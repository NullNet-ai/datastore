use crate::message_stream::token_bucket::{Message, TokenBucket};
use futures::stream;
// use futures::StreamExt; // Add this import for next() and for_each
use futures::TryStreamExt; // Add this import for map_err
use log::{error, info};
use once_cell::sync::OnceCell;
use std::sync::Arc;
use tokio::sync::mpsc;
use tokio::sync::Mutex;
use tokio::time::{interval, sleep, Duration};
use tokio_postgres::{Client, NoTls}; //

static INSTANCE: OnceCell<Arc<PgListenerService>> = OnceCell::new();
#[allow(warnings)]
pub struct PgListenerService {
    client: Mutex<Option<Client>>,
    main_stream: Arc<TokenBucket>,
    channel: String,
    subscribed_channels: Mutex<std::collections::HashSet<String>>,
    is_running: Mutex<bool>,
    is_paused: Mutex<bool>,
    connection_string: Mutex<String>,
}
#[allow(warnings)]
impl PgListenerService {
    pub fn instance() -> Arc<Self> {
        INSTANCE
            .get_or_init(|| {
                let default_channel = "check";
                let default_capacity = 200_200;
                Self::new(default_channel, default_capacity)
            })
            .clone()
    }

    pub async fn initialize() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let instance = Self::instance();

        // Start the service in a background task
        let instance_clone = instance.clone();
        tokio::spawn(async move {
            loop {
                if let Err(e) = instance_clone.start().await {
                    error!("Failed to start PgListenerService: {}", e);
                    // Wait before retrying
                    sleep(Duration::from_secs(5)).await;
                } else {
                    // If start() returns Ok, it means it's already running or has been stopped intentionally
                    break;
                }
            }
        });

        Ok(())
    }

    /// Creates a new PostgreSQL listener service with a token bucket for rate limiting
    pub fn new(channel: &str, capacity: usize) -> Arc<Self> {
        let main_stream = TokenBucket::new(&format!("pg_listener_{}", channel), capacity);
        let mut subscribed_channels = std::collections::HashSet::new();
        subscribed_channels.insert(channel.to_string());

        // Create the connection string
        let user = std::env::var("POSTGRES_USER").unwrap_or_else(|_| "admin".to_string());
        let password = std::env::var("POSTGRES_PASSWORD").unwrap_or_else(|_| "admin".to_string());
        let dbname = std::env::var("POSTGRES_DB").unwrap_or_else(|_| "nullnet".to_string());
        let host = std::env::var("POSTGRES_HOST").unwrap_or_else(|_| "localhost".to_string());
        let port = std::env::var("POSTGRES_PORT").unwrap_or_else(|_| "5433".to_string());

        let connection_string = format!(
            "host={} port={} user={} password={} dbname={}",
            host, port, user, password, dbname
        );

        let service = Arc::new(Self {
            client: Mutex::new(None),
            main_stream,
            channel: channel.to_string(),
            subscribed_channels: Mutex::new(subscribed_channels),
            is_running: Mutex::new(false),
            is_paused: Mutex::new(false),
            connection_string: Mutex::new(connection_string),
        });

        // Set up drain listener
        let service_clone = service.clone();
        tokio::spawn(async move {
            let notify = service_clone.main_stream.on_drain();
            loop {
                // Wait for the drain notification
                notify.notified().await;

                // Check if service is paused and resume channels if needed
                if service_clone.is_paused().await {
                    info!("Stream drained, resuming channels");
                    if let Err(e) = service_clone.resume_all_channels().await {
                        error!("Failed to resume channels after drain: {}", e);
                    }
                }
            }
        });

        // Set up periodic channel refresh (every 30 seconds)
        let service_clone = service.clone();
        tokio::spawn(async move {
            let mut refresh_interval = interval(Duration::from_secs(30));
            // Skip the first tick to avoid immediate refresh after startup
            refresh_interval.tick().await;

            loop {
                refresh_interval.tick().await;

                // Only refresh if the service is running and not paused
                if *service_clone.is_running.lock().await && !service_clone.is_paused().await {
                    info!("Performing periodic channel refresh...");
                    if let Err(e) = service_clone.refresh_channels().await {
                        error!("Failed to refresh channels during periodic refresh: {}", e);
                    }
                }
            }
        });

        service
    }
    
    /// Get the main stream token bucket
    pub fn get_main_stream(&self) -> Arc<TokenBucket> {
        self.main_stream.clone()
    }

    /// Start listening for PostgreSQL notifications
    pub async fn start(self: &Arc<Self>) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let mut is_running = self.is_running.lock().await;
        if *is_running {
            return Ok(());
        }

        *is_running = true;
        drop(is_running);

        // Get the connection string
        let connection_string = self.connection_string.lock().await.clone();

        // Connect to PostgreSQL
        let (client, mut connection) = tokio_postgres::connect(&connection_string, NoTls).await?;

        // Make transmitter and receiver
        let (tx, mut rx) = mpsc::unbounded_channel();

        // Create a stream from the connection and forward it to the channel
        let stream = stream::poll_fn(move |cx| connection.poll_message(cx)).map_err(|e| {
            error!("Connection error: {}", e);
            e
        });

        let connection_forwarder = stream.try_for_each(move |msg| {
            let tx = tx.clone();
            tx.send(msg).unwrap();
            futures::future::ready(Ok(()))
        });

        // Spawn a task to handle the connection
        tokio::spawn(connection_forwarder);

        // Store the client
        {
            let mut client_guard = self.client.lock().await;
            *client_guard = Some(client);
        }

        // Instead of listening to a default channel immediately,
        // refresh channels from the database first
        info!("Refreshing channels from database...");
        if let Err(e) = self.refresh_channels().await {
            error!("Failed to refresh channels: {}", e);
            // If we can't refresh channels, there's no point in continuing
            return Err(e);
        }

        info!("Started listening on PostgreSQL channels");

        // Clone what we need for the notification processing task
        let service = Arc::clone(self);

        // Wait for notifications in a separate thread
        tokio::spawn(async move {
            while let Some(msg) = rx.recv().await {
                match msg {
                    tokio_postgres::AsyncMessage::Notification(notification) => {
                        println!(
                            "Notification: channel={}, payload={}",
                            notification.channel(),
                            notification.payload()
                        );

                        // Process the notification through the token bucket
                        service.process_notification(notification.clone()).await;
                    }
                    tokio_postgres::AsyncMessage::Notice(notice) => {
                        println!("Notice: {}", notice);
                    }
                    _ => {}
                }
            }

            // If we get here, the stream has ended
            error!("PostgreSQL notification stream ended");

            // Attempt to reconnect
            info!("Connection closed. Attempting to reconnect in 5 seconds...");
            Self::spawn_restart_task(service.clone());
        });

        Ok(())
    }

    async fn refresh_channels(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let client_guard = self.client.lock().await;
        if let Some(client) = &*client_guard {
            // Query the database for channels
            info!("About to query postgres_channels table");
            let result = client
                .query("SELECT channel_name FROM postgres_channels", &[])
                .await?;

            // Extract channel names from the result
            let mut channels = Vec::new();
            for row in result {
                let channel_name: String = row.get(0);
                channels.push(channel_name);
            }

            // Subscribe to new channels
            let mut subscribed_channels = self.subscribed_channels.lock().await;
            for channel in channels {
                if !subscribed_channels.contains(&channel) {
                    client
                        .batch_execute(&format!("LISTEN {};", channel))
                        .await?;
                    subscribed_channels.insert(channel.clone());
                    info!("✅ Now listening on channel: {}", channel);
                }
            }
        } else {
            return Err("PostgreSQL client not initialized".into());
        }

        Ok(())
    }

    /// Stop listening for PostgreSQL notifications
    pub async fn stop(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let mut is_running = self.is_running.lock().await;
        if !*is_running {
            return Ok(());
        }

        *is_running = false;

        // Get the client and unlisten from all channels
        let mut client_guard = self.client.lock().await;
        if let Some(client) = client_guard.take() {
            let subscribed_channels = self.subscribed_channels.lock().await;
            for channel in &*subscribed_channels {
                if let Err(e) = client
                    .batch_execute(&format!("UNLISTEN {};", channel))
                    .await
                {
                    error!("Failed to unlisten from channel {}: {}", channel, e);
                }
            }
            info!("Stopped listening on all PostgreSQL channels");
        }

        Ok(())
    }

    fn spawn_restart_task(service: Arc<PgListenerService>) {
        tokio::spawn(async move {
            sleep(Duration::from_secs(5)).await;
            if let Err(e) = service.start().await {
                error!("Failed to restart service: {}", e);
            }
        });
    }
    /// Process a PostgreSQL notification through the token bucket
    async fn process_notification(&self, notification: tokio_postgres::Notification) {
        // Create a message from the notification payload
        let message = match serde_json::from_str::<serde_json::Value>(notification.payload()) {
            Ok(parsed_json) => Message(parsed_json),
            Err(e) => {
                log::error!("Failed to parse notification payload as JSON: {}", e);
                // Fallback to string if parsing fails
                Message(notification.payload().into())
            }
        };
        // Send the message to the token bucket
        self.main_stream.receive_message(message).await;

        // Check for backpressure after receiving the message
        let tokens_remaining = self.main_stream.get_tokens_remaining().await;
        if tokens_remaining == 0 && !*self.is_paused.lock().await {
            // Backpressure detected, pause all channels
            info!("⚠️ Backpressure detected, pausing channels");
            if let Err(e) = self.pause_all_channels().await {
                error!("Failed to pause channels: {}", e);
            }
        }

        info!(
            "Received notification on channel {}: {}",
            notification.channel(),
            notification.payload()
        );
    }

    /// Send a notification to the PostgreSQL channel
    async fn send_notification(&self, payload: &str) -> Result<(), Box<dyn std::error::Error>> {
        let client_guard = self.client.lock().await;
        if let Some(client) = &*client_guard {
            // Use NOTIFY to send a notification
            client
                .batch_execute(&format!(
                    "NOTIFY {}, '{}';",
                    self.channel,
                    payload.replace("'", "''")
                ))
                .await?;

            info!("Sent notification to channel {}: {}", self.channel, payload);
            Ok(())
        } else {
            Err("PostgreSQL client not initialized".into())
        }
    }

    /// Pause listening on all subscribed channels
    async fn pause_all_channels(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mut is_paused = self.is_paused.lock().await;
        *is_paused = true;

        let client_guard = self.client.lock().await;
        if let Some(client) = &*client_guard {
            let subscribed_channels = self.subscribed_channels.lock().await;
            for channel in &*subscribed_channels {
                client
                    .batch_execute(&format!("UNLISTEN {};", channel))
                    .await?;
                info!("Paused listening on channel: {}", channel);
            }
        } else {
            return Err("PostgreSQL client not initialized".into());
        }

        Ok(())
    }

    async fn reconnect(self: &Arc<Self>) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // Wait before reconnecting
        info!("Attempting to reconnect in 5 seconds...");
        sleep(Duration::from_secs(5)).await;

        // Directly call start without spawning a new task
        self.start().await
    }

    /// Resume listening on all subscribed channels
    async fn resume_all_channels(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mut is_paused = self.is_paused.lock().await;
        *is_paused = false;

        let client_guard = self.client.lock().await;
        if let Some(client) = &*client_guard {
            let subscribed_channels = self.subscribed_channels.lock().await;
            for channel in &*subscribed_channels {
                client
                    .batch_execute(&format!("LISTEN {};", channel))
                    .await?;
                info!("Resumed listening on channel: {}", channel);
            }

            info!("✅ All channels resumed");
        } else {
            return Err("PostgreSQL client not initialized".into());
        }

        Ok(())
    }

    /// Check if the service is currently paused
    async fn is_paused(&self) -> bool {
        *self.is_paused.lock().await
    }
}
