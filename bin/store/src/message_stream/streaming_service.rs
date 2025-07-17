use crate::message_stream::pg_listener_service::PgListenerService;
use crate::message_stream::token_bucket::{Message, TokenBucket};
use crate::message_stream::gateway::broadcast_to_channel;
use crate::message_stream::stream_queue_service::StreamQueueService;
use crate::message_stream::shared_state::{get_shared_state, SharedStreamingState, ChannelStatus};
use log::{error, info, warn};
use serde_json::Value;
use socketioxide::SocketIo;
use std::sync::Arc;
use tokio::time::{sleep, Duration};

#[derive(Clone)]
pub struct MessageStreamingService {
    socket_io: SocketIo,
    queue_service: Arc<StreamQueueService>,
    shared_state: Arc<SharedStreamingState>,
}

impl MessageStreamingService {
    pub fn new(socket_io: SocketIo) -> Arc<Self> {
        Arc::new(Self {
            socket_io,
            queue_service: StreamQueueService::new(),
            shared_state: get_shared_state(),
        })
    }

    pub async fn initialize(self: &Arc<Self>) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        info!("Initializing MessageStreamingService...");
        
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
        
        tokio::spawn(async move {
            loop {
                // Get message from main stream
                let msg_opt = {
                    let mut buffer = main_stream.buffer.lock().await;
                    buffer.pop_front()
                };

                if let Some(message) = msg_opt {
                    // Extract channel name and organization_id from message
                    let (channel_name, organization_id) = if let Ok(payload) = serde_json::from_value::<Value>(message.0.clone()) {
                        let event_name = payload.get("event_name")
                            .and_then(|v| v.as_str())
                            .unwrap_or("default")
                            .to_string();
                        let org_id = payload.get("organization_id")
                            .and_then(|v| v.as_str())
                            .map(|s| s.to_string());
                        (event_name, org_id)
                    } else {
                        ("default".to_string(), None)
                    };

                    if let Some(org_id) = organization_id {
                        // Check if organization has authenticated clients
                        if let Some(_org_clients) = crate::message_stream::gateway::get_organization_clients(&org_id) {
                            // Handle the message using the existing handle_message logic
                            if let Err(e) = service.handle_message(&channel_name, &org_id, message.0).await {
                                error!("Error handling message for channel {}: {}", channel_name, e);
                            }
                        } else {
                            info!("No authenticated clients for organization {}, discarding message for channel {}", org_id, channel_name);
                        }
                    } else {
                        warn!("Message missing organization_id, discarding message for channel {}", channel_name);
                    }
                    
                    // Increment tokens for the main stream since we processed this message
                    main_stream.increment_tokens().await;
                } else {
                    // No messages in the buffer, wait a bit before checking again
                    tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
                }
            }
        });
        
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
        
        // Register with shared state
        self.shared_state.register_channel(
            channel_name,
            organization_id,
            token_bucket.clone(),
            capacity,
        ).await;
        
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
        let shared_state = Arc::clone(&self.shared_state);
        
        tokio::spawn(async move {
            loop {
                notify.notified().await;
                info!("Channel {} drained, removing from backpressured and starting flush", channel_name);
                
                // Remove from backpressured channels when drained
                shared_state.clear_backpressured(&channel_name).await;
                info!("Resumed receiving data for channel {}", channel_name);
                
                // Start flushing the queue
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
        // Check if already flushing to prevent concurrent flushes
        if self.shared_state.is_flushing(channel_name).await {
            info!("Channel {} is already flushing, skipping", channel_name);
            return Ok(());
        }
        
        // Mark as flushing
        self.shared_state.mark_flushing(channel_name).await;
        
        let result = self.flush_channel_queue_internal(channel_name, organization_id).await;
        
        // Always remove from flushing channels when done
        self.shared_state.clear_flushing(channel_name).await;
        
        result
    }
    
    async fn flush_channel_queue_internal(
        &self,
        channel_name: &str,
        organization_id: &str,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        loop {
            // Get batch of queued items from database
            let queue_items = self.queue_service.get_queue_items(channel_name, 1000).await?;
            
            if queue_items.is_empty() {
                info!("No more queued items for channel {}, flush complete", channel_name);
                break;
            }
            
            info!("Flushing {} queued messages for channel {}", queue_items.len(), channel_name);
            
            // Get the token bucket for this channel
            let token_bucket = self.shared_state.get_channel(channel_name).await;
            
            if let Some(bucket) = token_bucket {
                let mut messages_processed = 0;
                let mut items_to_delete = Vec::new();
                
                for item in queue_items {
                    // Check if bucket has capacity
                    if bucket.get_tokens_remaining().await > 0 {
                        // Send message directly to bucket
                         let message = Message(item.content.clone());
                         bucket.receive_message(message).await;
                         
                         // Broadcast to Socket.IO clients
                         broadcast_to_channel(
                             &self.socket_io,
                             organization_id,
                             channel_name,
                             item.content,
                         );
                         
                         items_to_delete.push(item.id.clone());
                         messages_processed += 1;
                    } else {
                        // No tokens available, mark as backpressured and stop
                        warn!("Channel {} has no tokens available during queue flush after {} messages", channel_name, messages_processed);
                        
                        self.shared_state.mark_backpressured(channel_name).await;
                        
                        break;
                    }
                }
                
                // Delete processed items from queue
                for item_id in items_to_delete {
                    if let Err(e) = self.queue_service.delete_from_queue(&item_id).await {
                        error!("Failed to delete queue item {}: {}", item_id, e);
                    }
                }
                
                info!("Processed {} messages from queue for channel {}", messages_processed, channel_name);
                
                // If we processed fewer messages than available, we hit backpressure
                if messages_processed == 0 {
                    info!("No messages could be processed for channel {}, stopping flush", channel_name);
                    break;
                }
                
                // Check if channel is backpressured before continuing
                if self.shared_state.is_backpressured(channel_name).await {
                    info!("Channel {} is backpressured, stopping flush", channel_name);
                    break;
                }
                
                // Continue with next batch if there might be more items
                // Small delay to prevent tight loop
                tokio::task::yield_now().await;
                
            } else {
                // Channel doesn't exist, discard all queued messages for this channel
                warn!("Channel {} no longer exists, discarding {} queued messages", channel_name, queue_items.len());
                for item in queue_items {
                    if let Err(e) = self.queue_service.delete_from_queue(&item.id).await {
                        error!("Failed to delete orphaned queue item {}: {}", item.id, e);
                    }
                }
                break;
            }
        }
        
        Ok(())
    }

    pub async fn get_or_create_channel_pipe(
        &self,
        channel_name: &str,
        organization_id: &str,
    ) -> Arc<TokenBucket> {
        // Check if channel already exists in shared state
        if let Some(existing_pipe) = self.shared_state.get_channel(channel_name).await {
            return existing_pipe;
        }
        
        // Pipe doesn't exist, create it
        info!("Creating new channel pipe for: {} (org: {})", channel_name, organization_id);
        
        // Create a new token bucket for this channel with default capacity
        let token_bucket = TokenBucket::new(&format!("{}_{}", organization_id, channel_name), 1000);
        
        // Register with shared state
        self.shared_state.register_channel(
            channel_name,
            organization_id,
            token_bucket.clone(),
            1000, // capacity
        ).await;
        
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
            // Check if channel is currently flushing or backpressured
            let should_queue = self.shared_state.is_flushing(channel_name).await || self.shared_state.is_backpressured(channel_name).await;
            
            if should_queue {
                // Channel is flushing or backpressured, queue the message to maintain order
                info!("Channel {} is flushing or backpressured, queuing message to maintain order", channel_name);
                self.queue_service.insert_to_queue(channel_name, message).await?;
                return Ok(());
            }
            
            // Check if channel exists
            let token_bucket = self.shared_state.get_channel(channel_name).await;
            
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
                
                // Register with shared state
                self.shared_state.register_channel(
                    channel_name,
                    organization_id,
                    new_bucket.clone(),
                    1000, // capacity
                ).await;
                
                // Note: No broker registration needed anymore
                
                // Register with the global token bucket registry for dashboard access
                crate::message_stream::gateway::register_token_bucket(channel_name, 1000);
                
                // Add channel to organization's channels
                crate::message_stream::gateway::add_channel_to_organization(organization_id, channel_name);
                
                info!("Channel pipe {} created and associated with organization {}", channel_name, organization_id);
                new_bucket
            };
            
            // Check if bucket has capacity before sending
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
                 warn!("Channel {} became backpressured, saving message to queue", channel_name);
                 
                 // Mark channel as backpressured
                 self.shared_state.mark_backpressured(channel_name).await;
                 
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
        
        // Unregister from shared state (this handles all cleanup)
        self.shared_state.unregister_channel(channel_name).await;
        
        info!("Channel {} unregistered", channel_name);
    }

    pub async fn get_active_channels(&self) -> Vec<String> {
        self.shared_state.get_active_channel_names().await
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
        let token_bucket = self.shared_state.get_channel(channel_name).await;
        
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
    
    /// Get the current status of a channel (for debugging/monitoring)
    pub async fn get_channel_status(&self, channel_name: &str) -> Option<ChannelStatus> {
        self.shared_state.get_channel_status(channel_name).await
    }
    
    /// Manually trigger queue flushing for a channel (for debugging/admin purposes)
    pub async fn trigger_manual_flush(
        &self,
        channel_name: &str,
        organization_id: &str,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        info!("Manually triggering flush for channel {}", channel_name);
        self.flush_channel_queue(channel_name, organization_id).await
    }
}