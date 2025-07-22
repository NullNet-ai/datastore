use crate::message_stream::pg_listener_service::PgListenerService;
use crate::message_stream::token_bucket::{Message, TokenBucket};
use crate::message_stream::stream_queue_service::StreamQueueService;
use crate::message_stream::shared_state::{get_shared_state, SharedStreamingState};
use crate::message_stream::flush_connection_limiter::get_flush_limiter;
use log::{error, info, warn};
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

                main_stream.on_message_available().notified().await;
                

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

                            if let Err(e) = service.handle_message(&channel_name, &org_id, message.0).await {
                                error!("Handle message error for channel {}: {}", channel_name, e);
                            }
                        } else {
                            log::info!("No clients connected for organization {}, discarding message for channel {}", org_id, channel_name);
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

                drain_notifier.notified().await;
                

                let is_flushing = service.shared_state.is_flushing(&channel_name_clone).await;
                let is_backpressured = service.shared_state.is_backpressured(&channel_name_clone).await;
                

                if is_flushing {
                    if is_backpressured {
                        service.shared_state.remove_backpressured(&channel_name_clone).await;
                    }
                } else if is_backpressured {
                    service.shared_state.mark_flushing(&channel_name_clone).await;
                    service.shared_state.remove_backpressured(&channel_name_clone).await;
                } else {
                    continue;
                }
                
               
                

                let flush_limiter = get_flush_limiter();
                if let Ok((_permit, mut conn)) = flush_limiter.acquire_flush_connection().await {
                    match service.queue_service.has_queued_messages(&mut conn, &channel_name_clone).await {
                        Ok(has_messages) => {
                            if has_messages {
                                if let Err(e) = service.process_queued_messages(&channel_name_clone).await {
                                    error!("Error processing queued messages for channel {}: {}", channel_name_clone, e);
                                }
                            } else {
                                service.shared_state.remove_flushing(&channel_name_clone).await;
                            }
                        }
                        Err(e) => {
                            error!("Error checking queued messages for channel {}: {}", channel_name_clone, e);

                            service.shared_state.remove_flushing(&channel_name_clone).await;
                        }
                    }
                } else {
                    error!("Failed to acquire connection to check queued messages for channel {}", channel_name_clone);

                    service.shared_state.remove_flushing(&channel_name_clone).await;
                }
            }
        });
        
        info!("Drain listener started for channel {}", channel_name);
    }

    pub async fn start_processing_queue_handler(&self) {
        let service = Arc::new(self.clone());
        
        tokio::spawn(async move {
            loop {

                if let Some(channel_name) = service.shared_state.dequeue_for_processing().await {

                    if service.shared_state.is_flushing(&channel_name).await {
                        info!("Processing queued channel {} from fairness queue", channel_name);
                        if let Err(e) = service.process_queued_messages(&channel_name).await {
                            error!("Error processing queued messages for channel {}: {}", channel_name, e);
                        }
                    }
                } else {
                    tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
                }
            }
        });
        
        info!("Processing queue handler started");
    }

    async fn process_queued_messages(&self, channel_name: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {

        let _organization_id = if let Some(org_id) = self.shared_state.get_channel_organization(channel_name).await {
            org_id
        } else {
            warn!("No organization found for channel {}, skipping queue processing", channel_name);
            return Ok(());
        };
        

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
        

        let queued_items = self.queue_service.dequeue_batch_from_channel(&mut conn, channel_name, BATCH_SIZE).await?;
        
        if !queued_items.is_empty() {
            let mut consumed_ids = Vec::new();
            let mut backpressured_during_flush = false;
            

            for item in queued_items {

                let msg = Message(item.content);

                let has_capacity = bucket.receive_message(msg).await;
                

                consumed_ids.push(item.id);
                
                if !has_capacity {

                    warn!("Channel {} became backpressured during batch processing", channel_name);
                    backpressured_during_flush = true;
                    break;
                }
            }
            

            if backpressured_during_flush {
                self.shared_state.mark_backpressured(channel_name).await;
            }
            

            if !consumed_ids.is_empty() {
                self.queue_service.delete_processed_items(&mut conn, &consumed_ids).await?;
            }
            

            if backpressured_during_flush {
                self.shared_state.remove_flushing(channel_name).await;
                return Ok(());
            }
            

            if self.queue_service.has_queued_messages(&mut conn, channel_name).await? {

                self.shared_state.queue_for_processing(channel_name).await;
                return Ok(());
            }
        }
        

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

            let should_queue = self.shared_state.is_flushing(channel_name).await || self.shared_state.is_backpressured(channel_name).await;
            
            if should_queue {
                info!("Channel {} is flushing or backpressured, queuing message to maintain order", channel_name);

                let flush_limiter = get_flush_limiter();
                let (_permit, mut conn) = flush_limiter.acquire_flush_connection().await?;
                self.queue_service.insert_to_queue(&mut conn, channel_name, message).await?;
                return Ok(());
            }
            
            let token_bucket = self.shared_state.get_channel(channel_name).await;
            
            let bucket = if let Some(existing_bucket) = token_bucket {
                existing_bucket
            } else {

                info!("Creating new channel pipe for: {} (org: {})", channel_name, organization_id);
                
                let bucket_capacity = env::var("BUCKET_CAPACITY")
                    .ok()
                    .and_then(|s| s.parse::<usize>().ok())
                    .unwrap_or(1000);
                

                let new_bucket = crate::message_stream::token_bucket::TokenBucket::new_without_consumer(
                    channel_name, 
                    bucket_capacity
                );
                

                let actual_bucket = self.shared_state.register_channel(
                    channel_name,
                    organization_id,
                    new_bucket.clone(),
                    bucket_capacity,
                ).await;
                

                if Arc::ptr_eq(&new_bucket, &actual_bucket) {

                    actual_bucket.start_consumer();
                    

                    self.start_drain_listener(channel_name.to_string(), actual_bucket.clone()).await;
                    

                    crate::message_stream::gateway::add_channel_to_organization(organization_id, channel_name);
                    
                    info!("Channel pipe {} created with direct socket attachment and associated with organization {}", channel_name, organization_id);
                } else {
                    info!("Channel pipe {} already exists, using existing bucket", channel_name);
                }
                
                actual_bucket
            };
            

            let msg = Message(message.clone());
    
            let has_capacity = bucket.receive_message(msg).await;
            
            if !has_capacity {

                warn!("Channel {} became backpressured after storing message", channel_name);
                self.shared_state.mark_backpressured(channel_name).await;
            }

        } else {
            info!("No authenticated clients for organization {}, discarding message for channel {}", organization_id, channel_name);
        }
        
        Ok(())
    }


}