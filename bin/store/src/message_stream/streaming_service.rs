use crate::message_stream::pg_listener_service::PgListenerService;
use crate::message_stream::token_bucket::{Message, TokenBucket};
use crate::message_stream::stream_queue_service::StreamQueueService;
use crate::message_stream::shared_state::{get_shared_state, SharedStreamingState};
use crate::message_stream::flush_connection_limiter::get_flush_limiter;
use log::{error, info, warn, debug};
use serde_json::Value;
use socketioxide::SocketIo;
use std::sync::Arc;
use std::env;


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
    
    pub fn get_socket_io(&self) -> &SocketIo {
        &self.socket_io
    }

    pub async fn initialize(self: &Arc<Self>) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        info!("Initializing MessageStreamingService...");
        
        let pg_listener = PgListenerService::instance();
        let main_stream = pg_listener.get_main_stream();
        
        self.start_routing_task(main_stream).await;
        self.start_processing_queue_handler().await;
        
        info!("MessageStreamingService initialized successfully");
        Ok(())
    }

    async fn start_routing_task(self: &Arc<Self>, main_stream: Arc<TokenBucket>) {
        let service = Arc::clone(self);


        
        tokio::spawn(async move {
            loop {
                // Wait for message availability notification
                main_stream.on_message_available().notified().await;
                
                // Process all available messages using proper emit flow
                while let Some(message) = main_stream.emit_message().await {
                    let (channel_name, organization_id) = if let Ok(payload) = serde_json::from_value::<Value>(message.0.clone()) {
                        let event_name = payload.get("event_name")
                            .and_then(|v| v.as_str());
                        let org_id = payload.get("organization_id")
                            .and_then(|v| v.as_str())
                            .map(|s| s.to_string());
                        
                        if event_name.is_none() {
                            log::error!("Missing event_name in message payload: {:?}", payload);
                            continue;
                        }
                        
                        if org_id.is_none() {
                            log::error!("Missing organization_id in message payload: {:?}", payload);
                            continue;
                        }
                        
                        (event_name.unwrap().to_string(), org_id)
                    } else {
                        log::error!("Failed to parse message payload as JSON: {:?}", message.0);
                        continue;
                    };

                    if let Some(org_id) = organization_id {
                        if let Some(_org_clients) = crate::message_stream::gateway::get_organization_clients(&org_id) {
                            // Route message through handle_message (proper flow)
                            if let Err(e) = service.handle_message(&channel_name, &org_id, message.0).await {
                                error!("Handle message error for channel {}: {}", channel_name, e);
                            }
                        }
                    }
                }
            }
        });
        
        info!("Message routing task started");
    }





    async fn start_drain_listener(&self, channel_name: String, bucket: Arc<TokenBucket>) {
        let service = Arc::new(self.clone());
        let drain_notifier = bucket.on_drain();
        let channel_name_clone = channel_name.clone();
        
        tokio::spawn(async move {
            loop {
                // Wait for drain event (when bucket becomes full)
                drain_notifier.notified().await;
                
                // First mark as flushing to prevent new messages from bypassing queue
                service.shared_state.mark_flushing(&channel_name_clone).await;
                
                // Then remove from backpressured state - channel is ready to receive
                service.shared_state.remove_backpressured(&channel_name_clone).await;
                
                // Check if there are actually queued messages to process
                let flush_limiter = get_flush_limiter();
                if let Ok((_permit, mut conn)) = flush_limiter.acquire_flush_connection().await {
                    match service.queue_service.has_queued_messages_with_conn(&mut conn, &channel_name_clone).await {
                        Ok(has_messages) => {
                            if has_messages {
                                // Process queued messages for this channel
                                if let Err(e) = service.process_queued_messages(&channel_name_clone).await {
                                    error!("Error processing queued messages for channel {}: {}", channel_name_clone, e);
                                }
                            } else {
                                // Remove from flushing since there are no messages to process
                                service.shared_state.remove_flushing(&channel_name_clone).await;
                            }
                        }
                        Err(e) => {
                            error!("Error checking queued messages for channel {}: {}", channel_name_clone, e);
                            // Remove from flushing on error
                            service.shared_state.remove_flushing(&channel_name_clone).await;
                        }
                    }
                } else {
                    error!("Failed to acquire connection to check queued messages for channel {}", channel_name_clone);
                    // Remove from flushing on connection error
                    service.shared_state.remove_flushing(&channel_name_clone).await;
                }
            }
        });
        
        info!("Drain listener started for channel {}", channel_name);
    }

    /// Start the processing queue handler for fairness
    pub async fn start_processing_queue_handler(&self) {
        let service = Arc::new(self.clone());
        
        tokio::spawn(async move {
            loop {
                // Check if there's a channel in the processing queue
                if let Some(channel_name) = service.shared_state.dequeue_for_processing().await {
                    // Only process if channel is still in flushing state
                    if service.shared_state.is_flushing(&channel_name).await {
                        info!("Processing queued channel {} from fairness queue", channel_name);
                        if let Err(e) = service.process_queued_messages(&channel_name).await {
                            error!("Error processing queued messages for channel {}: {}", channel_name, e);
                        }
                    }
                } else {
                    // No channels in queue, wait a bit before checking again
                    tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
                }
            }
        });
        
        info!("Processing queue handler started");
    }

    async fn process_queued_messages(&self, channel_name: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // Get the organization ID for this channel
        let _organization_id = if let Some(org_id) = self.shared_state.get_channel_organization(channel_name).await {
            org_id
        } else {
            warn!("No organization found for channel {}, skipping queue processing", channel_name);
            return Ok(());
        };
        
        // Channel should already be in flushing state from drain listener
        // Verify it's in flushing state
        if !self.shared_state.is_flushing(channel_name).await {
            warn!("Channel {} not in flushing state during queue processing, marking as flushing", channel_name);
            self.shared_state.mark_flushing(channel_name).await;
        }
        
        let bucket = if let Some(bucket) = self.shared_state.get_channel(channel_name).await {
            bucket
        } else {
            warn!("Channel {} not found, removing from flushing state", channel_name);
            self.shared_state.remove_flushing(channel_name).await;
            return Ok(());
        };
        
        // Acquire flush connection permit to limit concurrent database operations
        let flush_limiter = get_flush_limiter();
        let (_permit, mut conn) = match flush_limiter.acquire_flush_connection().await {
            Ok((permit, conn)) => (permit, conn),
            Err(e) => {
                error!("Failed to acquire flush connection for channel {}: {}", channel_name, e);
                self.shared_state.remove_flushing(channel_name).await;
                return Err(e);
            }
        };
        
        const BATCH_SIZE: usize = 500;
        
        // Process only ONE batch per turn to ensure fairness
        // Get batch of queued messages using shared connection
        let queued_items = self.queue_service.dequeue_batch_from_channel_with_conn(&mut conn, channel_name, BATCH_SIZE).await?;
        
        if !queued_items.is_empty() {
            let mut consumed_ids = Vec::new();
            let mut backpressured_during_flush = false;
            
            // Process each message in the batch
            for item in queued_items {
                // Try to send message directly through bucket (which will send to socket)
                let msg = Message(item.content);

                let has_capacity = bucket.receive_message(msg).await;
                
                // Track ALL consumed message IDs for deletion (regardless of backpressure)
                // The message is consumed by the bucket even if it returns false due to backpressure
                consumed_ids.push(item.id);
                
                if !has_capacity {
                    // Channel backpressured during processing - stop processing remaining items
                    warn!("Channel {} became backpressured during batch processing", channel_name);
                    backpressured_during_flush = true;
                    break;
                }
            }
            
            // If backpressured during flush, mark channel as backpressured FIRST
            if backpressured_during_flush {
                self.shared_state.mark_backpressured(channel_name).await;
            }
            
            // Delete ALL consumed messages from database in a single operation
            if !consumed_ids.is_empty() {
                self.queue_service.delete_processed_items_with_conn(&mut conn, &consumed_ids).await?;
            }
            
            // If backpressured during flush, remove from flushing and stop
            if backpressured_during_flush {
                self.shared_state.remove_flushing(channel_name).await;
                return Ok(());
            }
            
            // Check if there are more messages to process
            if self.queue_service.has_queued_messages_with_conn(&mut conn, channel_name).await? {
                // First delete the consumed messages (already done above)
                // Then add channel to processing queue for its turn while keeping it in flushing state
                self.shared_state.queue_for_processing(channel_name).await;
                return Ok(());
            }
        }
        
        // Remove from flushing state
        self.shared_state.remove_flushing(channel_name).await;
        
        Ok(())
    }

    pub async fn handle_message(
        &self,
        channel_name: &str,
        organization_id: &str,
        message: Value,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        if let Some(_org_clients) = crate::message_stream::gateway::get_organization_clients(organization_id) {
            // Queue message if channel is flushing or backpressured to maintain order
            let should_queue = self.shared_state.is_flushing(channel_name).await || self.shared_state.is_backpressured(channel_name).await;
            
            if should_queue {
                info!("Channel {} is flushing or backpressured, queuing message to maintain order", channel_name);
                // Use connection reuse for queue operations
                let flush_limiter = get_flush_limiter();
                let (_permit, mut conn) = flush_limiter.acquire_flush_connection().await?;
                self.queue_service.insert_to_queue_with_conn(&mut conn, channel_name, message).await?;
                return Ok(());
            }
            
            let token_bucket = self.shared_state.get_channel(channel_name).await;
            
            let bucket = if let Some(existing_bucket) = token_bucket {
                existing_bucket
            } else {
                // Create new token bucket for channel with authenticated clients
                info!("Creating new channel pipe for: {} (org: {})", channel_name, organization_id);
                
                let bucket_capacity = env::var("BUCKET_CAPACITY")
                    .ok()
                    .and_then(|s| s.parse::<usize>().ok())
                    .unwrap_or(1000);
                
                // Create token bucket without starting consumer yet
                let new_bucket = crate::message_stream::token_bucket::TokenBucket::new_without_consumer(
                    channel_name, 
                    bucket_capacity
                );
                
                // Register channel and get the actual bucket (may return existing if race condition)
                let actual_bucket = self.shared_state.register_channel(
                    channel_name,
                    organization_id,
                    new_bucket.clone(),
                    bucket_capacity,
                ).await;
                
                // Only start drain listener and consumer if we got back our new bucket (not an existing one)
                if Arc::ptr_eq(&new_bucket, &actual_bucket) {
                    // Start the sequential consumer for this new bucket
                    actual_bucket.start_consumer();
                    
                    // Start drain listener for this channel
                    self.start_drain_listener(channel_name.to_string(), actual_bucket.clone()).await;
                    
                    // Only add channel to organization mapping (no duplicate bucket registration)
                    crate::message_stream::gateway::add_channel_to_organization(organization_id, channel_name);
                    
                    info!("Channel pipe {} created with direct socket attachment and associated with organization {}", channel_name, organization_id);
                } else {
                    info!("Channel pipe {} already exists, using existing bucket", channel_name);
                }
                
                actual_bucket
            };
            
            // Try to receive message in token bucket
            let msg = Message(message.clone());
    
            let has_capacity = bucket.receive_message(msg).await;
            
            if !has_capacity {
                // Bucket became backpressured after storing the message - mark channel as backpressured
                // The message is already stored in the bucket's buffer, so we don't need to queue it
                warn!("Channel {} became backpressured after storing message", channel_name);
                self.shared_state.mark_backpressured(channel_name).await;
            }
            // Note: Message is automatically broadcast by the token bucket's consumer task
        } else {
            info!("No authenticated clients for organization {}, discarding message for channel {}", organization_id, channel_name);
        }
        
        Ok(())
    }


}