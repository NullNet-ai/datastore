use crate::message_stream::message_broker::BrokerService;
use crate::message_stream::pg_listener_service::PgListenerService;
use crate::message_stream::token_bucket::{Message, TokenBucket};
use crate::message_stream::gateway::broadcast_to_channel;
use crate::message_stream::stream_queue_service::StreamQueueService;
use log::{error, info, warn};
use serde_json::Value;
use socketioxide::SocketIo;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::time::{sleep, Duration};

pub struct MessageStreamingService {
    broker: Arc<BrokerService>,
    socket_io: SocketIo,
    channel_pipes: Arc<Mutex<HashMap<String, Arc<TokenBucket>>>>,
    #[allow(dead_code)]
    queue_service: Arc<StreamQueueService>,
}

impl MessageStreamingService {
    pub fn new(socket_io: SocketIo) -> Arc<Self> {
        Arc::new(Self {
            broker: BrokerService::new(),
            socket_io,
            channel_pipes: Arc::new(Mutex::new(HashMap::new())),
            queue_service: StreamQueueService::new(),
        })
    }

    pub async fn initialize(self: &Arc<Self>) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        info!("Initializing MessageStreamingService...");
        
        // Start the queue cleanup task
        self.broker.spawn_queue_cleanup_task();
        
        // Get the main stream from PgListenerService
        let pg_listener = PgListenerService::instance();
        let main_stream = pg_listener.get_main_stream();
        
        // Start the routing task
        self.start_routing_task(main_stream).await;
        
        info!("MessageStreamingService initialized successfully");
        Ok(())
    }

    async fn start_routing_task(self: &Arc<Self>, main_stream: Arc<TokenBucket>) {
        let service = Arc::clone(self);
        
        // Spawn the main pipe routing task with our custom routing logic
        self.broker.spawn_main_pipe_routing_task(
            main_stream,
            move |message: &Message| -> String {
                // Extract event_name from the message
                // The message payload should contain event_name which becomes the channel name
                if let Ok(payload) = serde_json::from_value::<Value>(message.0.clone()) {
                    if let Some(event_name) = payload.get("event_name").and_then(|v| v.as_str()) {
                        return event_name.to_string();
                    }
                }
                
                // Fallback to a default channel if event_name is not found
                "default".to_string()
            },
            move |channel_name: &str, message: &Message| {
                let service_clone = Arc::clone(&service);
                let channel_name = channel_name.to_string();
                let message = message.clone();
                
                async move {
                    // Extract organization_id from the message
                    let organization_id = if let Ok(payload) = serde_json::from_value::<Value>(message.0.clone()) {
                        payload.get("organization_id").and_then(|v| v.as_str()).map(|s| s.to_string())
                    } else {
                        None
                    };
                    
                    if let Some(org_id) = organization_id {
                        // Check if organization has authenticated clients
                        if let Some(_org_clients) = crate::message_stream::gateway::get_organization_clients(&org_id) {
                            // Check if channel already exists
                            {
                                let pipes = service_clone.channel_pipes.lock().await;
                                if let Some(existing_pipe) = pipes.get(&channel_name) {
                                    return Some(existing_pipe.clone());
                                }
                            }
                            
                            // Channel doesn't exist but organization has authenticated clients
                            // Create a new token bucket for this channel
                            info!("Creating new channel pipe for: {} (org: {})", channel_name, org_id);
                            let token_bucket = crate::message_stream::token_bucket::TokenBucket::new(
                                &format!("{}_{}", org_id, channel_name), 
                                1000
                            );
                            
                            // Store the channel pipe
                            {
                                let mut pipes = service_clone.channel_pipes.lock().await;
                                // Double-check it wasn't created while we were waiting for the lock
                                if let Some(existing_pipe) = pipes.get(&channel_name) {
                                    return Some(existing_pipe.clone());
                                }
                                pipes.insert(channel_name.clone(), token_bucket.clone());
                            }
                            
                            // Register with the broker
                            service_clone.broker.register_pipe(token_bucket.clone()).await;
                            
                            // Register with the global token bucket registry for dashboard access
                            crate::message_stream::gateway::register_token_bucket(&channel_name, 1000);
                            
                            // Add channel to organization's channels
                            crate::message_stream::gateway::add_channel_to_organization(&org_id, &channel_name);
                            
                            info!("Channel pipe {} created and associated with organization {}", channel_name, org_id);
                            return Some(token_bucket);
                        } else {
                            // No authenticated clients for this organization
                            info!("No authenticated clients for organization {}, discarding message for channel {}", org_id, channel_name);
                            return None;
                        }
                    } else {
                        // No organization_id in message
                        warn!("Message missing organization_id, discarding message for channel {}", channel_name);
                        return None;
                    }
                }
            },
        );
        
        info!("Message routing task started");
    }

    pub async fn register_channel(
        self: &Arc<Self>,
        channel_name: &str,
        organization_id: &str,
        capacity: usize,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        info!("Registering channel: {} for organization: {}", channel_name, organization_id);
        
        // Create a new token bucket for this channel
        let token_bucket = TokenBucket::new(&format!("{}_{}", organization_id, channel_name), capacity);
        
        // Store the channel pipe
        {
            let mut pipes = self.channel_pipes.lock().await;
            pipes.insert(channel_name.to_string(), token_bucket.clone());
        }
        
        // Register with the broker
        self.broker.register_pipe(token_bucket.clone()).await;
        
        // Register with the global token bucket registry for dashboard access
        crate::message_stream::gateway::register_token_bucket(channel_name, capacity);
        
        // Set up drain listener for queue flushing
        Arc::clone(self).setup_drain_listener(channel_name, organization_id, token_bucket).await;
        
        info!("Channel {} registered successfully", channel_name);
        Ok(())
    }

    async fn setup_drain_listener(
        self: Arc<Self>,
        channel_name: &str,
        organization_id: &str,
        token_bucket: Arc<TokenBucket>,
    ) {
        let service = Arc::clone(&self);
        let channel_name = channel_name.to_string();
        let organization_id = organization_id.to_string();
        let notify = token_bucket.on_drain();
        
        tokio::spawn(async move {
            loop {
                notify.notified().await;
                info!("Channel {} drained, flushing queue", channel_name);
                
                if let Err(e) = service.flush_channel_queue(&channel_name, &organization_id).await {
                    error!("Error flushing queue for channel {}: {}", channel_name, e);
                }
            }
        });
    }

    async fn flush_channel_queue(
        &self,
        channel_name: &str,
        organization_id: &str,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // Get queued items from database
        let queue_items = self.queue_service.get_queue_items(channel_name, 1000).await?;
        
        if queue_items.is_empty() {
            return Ok(());
        }
        
        info!("Flushing {} queued messages for channel {}", queue_items.len(), channel_name);
        
        // Get the token bucket for this channel
        let token_bucket = {
            let pipes = self.channel_pipes.lock().await;
            pipes.get(channel_name).cloned()
        };
        
        if let Some(bucket) = token_bucket {
            for item in queue_items {
                // Check if bucket has capacity
                if bucket.get_tokens_remaining().await > 0 {
                    // Send message directly to bucket (bypass handle_message to avoid infinite loop)
                    let message = Message(item.content.clone());
                    bucket.receive_message(message).await;
                    
                    // Broadcast to Socket.IO clients
                    broadcast_to_channel(
                        &self.socket_io,
                        organization_id,
                        channel_name,
                        item.content,
                    );
                    
                    // Delete from queue only after successful processing
                    self.queue_service.delete_from_queue(&item.id).await?;
                } else {
                    // Bucket is backpressured again, stop processing
                    // DO NOT save back to queue - this would create infinite loop
                    warn!("Channel {} backpressured again during queue flush, stopping", channel_name);
                    break;
                }
            }
        } else {
            // Channel doesn't exist, discard all queued messages for this channel
            warn!("Channel {} no longer exists, discarding {} queued messages", channel_name, queue_items.len());
            for item in queue_items {
                self.queue_service.delete_from_queue(&item.id).await?;
            }
        }
        
        Ok(())
    }

    pub async fn get_or_create_channel_pipe(
        &self,
        channel_name: &str,
        organization_id: &str,
    ) -> Arc<TokenBucket> {
        // First, try to get existing pipe
        {
            let pipes = self.channel_pipes.lock().await;
            if let Some(pipe) = pipes.get(channel_name) {
                return pipe.clone();
            }
        }
        
        // Pipe doesn't exist, create it
        info!("Creating new channel pipe for: {} (org: {})", channel_name, organization_id);
        
        // Create a new token bucket for this channel with default capacity
        let token_bucket = TokenBucket::new(&format!("{}_{}", organization_id, channel_name), 1000);
        
        // Store the channel pipe
        {
            let mut pipes = self.channel_pipes.lock().await;
            // Double-check it wasn't created while we were waiting for the lock
            if let Some(existing_pipe) = pipes.get(channel_name) {
                return existing_pipe.clone();
            }
            pipes.insert(channel_name.to_string(), token_bucket.clone());
        }
        
        // Register with the broker
        self.broker.register_pipe(token_bucket.clone()).await;
        
        // Register with the global token bucket registry for dashboard access
        crate::message_stream::gateway::register_token_bucket(channel_name, 1000);
        
        // Note: We skip setting up the drain listener here since we don't have access to Arc<Self>
        // The drain listener should be set up when the channel is properly registered
        
        info!("Channel pipe {} created successfully", channel_name);
        token_bucket
    }

    pub async fn handle_message(
        &self,
        channel_name: &str,
        organization_id: &str,
        message: Value,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // Check if organization has authenticated clients
        if let Some(_org_clients) = crate::message_stream::gateway::get_organization_clients(organization_id) {
            // Check if channel exists
            let token_bucket = {
                let pipes = self.channel_pipes.lock().await;
                pipes.get(channel_name).cloned()
            };
            
            let bucket = if let Some(existing_bucket) = token_bucket {
                existing_bucket
            } else {
                // Channel doesn't exist but organization has authenticated clients
                // Create a new token bucket for this channel
                info!("Creating new channel pipe for: {} (org: {})", channel_name, organization_id);
                let new_bucket = crate::message_stream::token_bucket::TokenBucket::new(
                    &format!("{}_{}", organization_id, channel_name), 
                    1000
                );
                
                // Store the channel pipe
                {
                    let mut pipes = self.channel_pipes.lock().await;
                    // Double-check it wasn't created while we were waiting for the lock
                    if let Some(existing_pipe) = pipes.get(channel_name) {
                        existing_pipe.clone()
                    } else {
                        pipes.insert(channel_name.to_string(), new_bucket.clone());
                        
                        // Register with the broker
                        self.broker.register_pipe(new_bucket.clone()).await;
                        
                        // Register with the global token bucket registry for dashboard access
                        crate::message_stream::gateway::register_token_bucket(channel_name, 1000);
                        
                        // Add channel to organization's channels
                        crate::message_stream::gateway::add_channel_to_organization(organization_id, channel_name);
                        
                        info!("Channel pipe {} created and associated with organization {}", channel_name, organization_id);
                        new_bucket
                    }
                }
            };
            
            // Check if bucket has capacity
            if bucket.get_tokens_remaining().await > 0 {
                // Send message directly
                let msg = Message(message.clone());
                bucket.receive_message(msg).await;
                
                // Broadcast to Socket.IO clients
                broadcast_to_channel(
                    &self.socket_io,
                    organization_id,
                    channel_name,
                    message,
                );
            } else {
                // Bucket is backpressured, save to database queue
                warn!("Channel {} is backpressured, saving to queue", channel_name);
                self.queue_service.insert_to_queue(channel_name, message).await?;
            }
        } else {
            // No authenticated clients for this organization, discard message
            info!("No authenticated clients for organization {}, discarding message for channel {}", organization_id, channel_name);
        }
        
        Ok(())
    }

    pub async fn unregister_channel(&self, channel_name: &str) {
        info!("Unregistering channel: {}", channel_name);
        
        // Remove from our local storage
        {
            let mut pipes = self.channel_pipes.lock().await;
            pipes.remove(channel_name);
        }
        
        // Unregister from broker
        self.broker.unregister_pipe(channel_name).await;
        
        info!("Channel {} unregistered", channel_name);
    }

    pub async fn get_active_channels(&self) -> Vec<String> {
        self.broker.get_active_pipe_names().await
    }

    // Start a background task to process messages from all channels
    pub async fn start_message_processing_loop(self: &Arc<Self>) {
        loop {
            // Get all active channels
            let channel_names = self.get_active_channels().await;
            
            for channel_name in channel_names {
                // Process messages from each channel
                if let Err(e) = self.process_channel_messages(&channel_name).await {
                    error!("Error processing messages for channel {}: {}", channel_name, e);
                }
            }
            
            // Sleep briefly before next iteration
            sleep(Duration::from_millis(100)).await;
        }
    }

    async fn process_channel_messages(
        &self,
        channel_name: &str,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let token_bucket = {
            let pipes = self.channel_pipes.lock().await;
            pipes.get(channel_name).cloned()
        };
        
        if let Some(bucket) = token_bucket {
            // Emit messages from the bucket
            while let Some(message) = bucket.emit_message().await {
                // Extract organization_id from message or use a default approach
                let organization_id = self.extract_organization_id(&message).unwrap_or_else(|| "default".to_string());
                
                // Broadcast to Socket.IO clients
                broadcast_to_channel(
                    &self.socket_io,
                    &organization_id,
                    channel_name,
                    message.0,
                );
            }
        }
        
        Ok(())
    }

    fn extract_organization_id(&self, message: &Message) -> Option<String> {
        if let Ok(payload) = serde_json::from_value::<Value>(message.0.clone()) {
            payload.get("organization_id")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string())
        } else {
            None
        }
    }
}