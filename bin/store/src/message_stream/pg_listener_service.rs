use crate::message_stream::token_bucket::{Message, TokenBucket};
use futures::stream;
use futures::TryStreamExt;
use log::debug;
use log::{error, info, warn};
use once_cell::sync::OnceCell;
use std::sync::Arc;
use tokio::sync::mpsc;
use tokio::sync::Mutex;
use tokio::time::{interval, sleep, Duration};
use tokio_postgres::Client;

static INSTANCE: OnceCell<Arc<PgListenerService>> = OnceCell::new();
#[allow(warnings)]
pub struct PgListenerService {
    client: Mutex<Option<Client>>,
    main_stream: Arc<TokenBucket>,
    channel: String,
    subscribed_channels: Mutex<std::collections::HashSet<String>>,
    is_running: Mutex<bool>,
    is_paused: Mutex<bool>,
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

        let instance_clone = instance.clone();
        tokio::spawn(async move {
            loop {
                if let Err(e) = instance_clone.start().await {
                    error!("Failed to start PgListenerService: {}", e);
                    sleep(Duration::from_secs(5)).await;
                } else {
                    break;
                }
            }
        });

        Ok(())
    }

    pub fn new(channel: &str, capacity: usize) -> Arc<Self> {
        let main_stream = TokenBucket::new(&format!("pg_listener_{}", channel), capacity);
        let mut subscribed_channels = std::collections::HashSet::new();
        subscribed_channels.insert(channel.to_string());

        let service = Arc::new(Self {
            client: Mutex::new(None),
            main_stream,
            channel: channel.to_string(),
            subscribed_channels: Mutex::new(subscribed_channels),
            is_running: Mutex::new(false),
            is_paused: Mutex::new(false),
        });

        let service_clone = service.clone();
        tokio::spawn(async move {
            let notify = service_clone.main_stream.on_drain();
            loop {
                notify.notified().await;

                if service_clone.is_paused().await {
                    info!("Stream drained, resuming channels");
                    if let Err(e) = service_clone.resume_all_channels().await {
                        error!("Failed to resume channels after drain: {}", e);
                    }
                }
            }
        });

        let service_clone = service.clone();
        tokio::spawn(async move {
            let mut refresh_interval = interval(Duration::from_secs(30));
            refresh_interval.tick().await;

            loop {
                refresh_interval.tick().await;

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

    pub fn get_main_stream(&self) -> Arc<TokenBucket> {
        self.main_stream.clone()
    }

    pub async fn start(self: &Arc<Self>) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let mut is_running = self.is_running.lock().await;
        if *is_running {
            return Ok(());
        }

        *is_running = true;
        drop(is_running);

        let (client, mut connection) = crate::db::create_connection_with_polling().await?;

        let (tx, mut rx) = mpsc::unbounded_channel();

        let stream = stream::poll_fn(move |cx| connection.poll_message(cx)).map_err(|e| {
            error!("Connection error: {}", e);
            e
        });

        let connection_forwarder = stream.try_for_each(move |msg| {
            let tx = tx.clone();
            tx.send(msg).unwrap();
            futures::future::ready(Ok(()))
        });

        tokio::spawn(connection_forwarder);

        {
            let mut client_guard = self.client.lock().await;
            *client_guard = Some(client);
        }
        info!("Refreshing channels from database...");
        if let Err(e) = self.refresh_channels().await {
            error!("Failed to refresh channels: {}", e);
            return Err(e);
        }

        info!("Started listening on PostgreSQL channels");

        let service = Arc::clone(self);

        tokio::spawn(async move {
            while let Some(msg) = rx.recv().await {
                match msg {
                    tokio_postgres::AsyncMessage::Notification(notification) => {
                        service.process_notification(notification.clone()).await;
                    }
                    tokio_postgres::AsyncMessage::Notice(notice) => {
                        warn!("PostgreSQL Notice: {}", notice);
                    }
                    _ => {}
                }
            }

            error!("PostgreSQL notification stream ended");
            info!("Connection closed. Attempting to reconnect in 5 seconds...");
            Self::spawn_restart_task(service.clone());
        });

        Ok(())
    }

    async fn refresh_channels(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let client_guard = self.client.lock().await;
        if let Some(client) = &*client_guard {
            info!("About to query postgres_channels table");
            let result = client
                .query("SELECT channel_name FROM postgres_channels", &[])
                .await?;

            let mut channels = Vec::new();
            for row in result {
                let channel_name: String = row.get(0);
                channels.push(channel_name);
            }

            let mut subscribed_channels = self.subscribed_channels.lock().await;
            for channel in channels {
                if !subscribed_channels.contains(&channel) {
                    client
                        .batch_execute(&format!("LISTEN {};", channel))
                        .await?;
                    subscribed_channels.insert(channel.clone());
                    info!("Now listening on channel: {}", channel);
                }
            }
        } else {
            return Err("PostgreSQL client not initialized".into());
        }

        Ok(())
    }

    pub async fn stop(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let mut is_running = self.is_running.lock().await;
        if !*is_running {
            return Ok(());
        }

        *is_running = false;

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
    async fn process_notification(&self, notification: tokio_postgres::Notification) {
        let message = match serde_json::from_str::<serde_json::Value>(notification.payload()) {
            Ok(parsed_json) => Message(parsed_json),
            Err(e) => {
                log::error!("Failed to parse notification payload as JSON: {}", e);
                return;
            }
        };



        let msg = crate::message_stream::token_bucket::Message(message.0.clone());
        let has_capacity = self.main_stream.receive_message(msg).await;

        if !has_capacity {
            log::warn!("Main stream backpressured, pausing all channels");
            if let Err(e) = self.pause_all_channels().await {
                log::error!("Failed to pause channels due to backpressure: {}", e);
            }
        }

        debug!(
            "Received notification on channel {}: {}",
            notification.channel(),
            notification.payload()
        );
    }

    async fn send_notification(&self, payload: &str) -> Result<(), Box<dyn std::error::Error>> {
        let client_guard = self.client.lock().await;
        if let Some(client) = &*client_guard {
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
        info!("Attempting to reconnect in 5 seconds...");
        sleep(Duration::from_secs(5)).await;

        self.start().await
    }

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

            info!("All channels resumed");
        } else {
            return Err("PostgreSQL client not initialized".into());
        }

        Ok(())
    }

    async fn is_paused(&self) -> bool {
        *self.is_paused.lock().await
    }
}
