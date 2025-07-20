use serde_json::Value;
use std::{collections::VecDeque, sync::Arc};
use tokio::sync::{Mutex, Notify};
use log::{debug, error, warn, info};

#[derive(Debug, Clone)]
#[allow(warnings)]
pub struct Message(pub Value);

#[derive(Debug)]
pub struct TokenBucket {
    name: String,
    capacity: Mutex<usize>,
    tokens: Mutex<usize>,
    pub buffer: Mutex<VecDeque<Message>>,
    notify_drain: Arc<Notify>,
    notify_message_available: Arc<Notify>,
    consumer_started: Mutex<bool>,
}
#[allow(warnings)]
impl TokenBucket {
    pub fn new(name: &str, capacity: usize) -> Arc<Self> {
        Arc::new(Self {
            name: name.to_string(),
            capacity: Mutex::new(capacity),
            tokens: Mutex::new(capacity),
            buffer: Mutex::new(VecDeque::new()),
            notify_drain: Arc::new(Notify::new()),
            notify_message_available: Arc::new(Notify::new()),
            consumer_started: Mutex::new(false),
        })
    }
    
    pub fn with_consumer(name: &str, capacity: usize) -> Arc<Self> {
        let bucket = Arc::new(Self {
            name: name.to_string(),
            capacity: Mutex::new(capacity),
            tokens: Mutex::new(capacity),
            buffer: Mutex::new(VecDeque::new()),
            notify_drain: Arc::new(Notify::new()),
            notify_message_available: Arc::new(Notify::new()),
            consumer_started: Mutex::new(false),
        });
        
        // Start single consumer task for sequential transmission
        bucket.start_sequential_consumer();
        bucket
    }
    
    /// Create a new TokenBucket without starting the consumer
    pub fn new_without_consumer(name: &str, capacity: usize) -> Arc<Self> {

        Arc::new(Self {
            name: name.to_string(),
            capacity: Mutex::new(capacity),
            tokens: Mutex::new(capacity),
            buffer: Mutex::new(VecDeque::new()),
            notify_drain: Arc::new(Notify::new()),
            notify_message_available: Arc::new(Notify::new()),
            consumer_started: Mutex::new(false),
        })
    }
    
    /// Start the sequential consumer for this bucket (public method)
    pub fn start_consumer(self: &Arc<Self>) {
        self.start_sequential_consumer();
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub async fn receive_message(self: &Arc<Self>, msg: Message) -> bool {
        let mut tokens = self.tokens.lock().await;

        if *tokens > 0 {
            // Decrement token and always store message in bucket first
            *tokens -= 1;
            let remaining_tokens = *tokens;
            drop(tokens);
            
            // Always store message in bucket buffer to control burst
            let mut buffer = self.buffer.lock().await;
            buffer.push_back(msg.clone());
            drop(buffer);
            
            // Notify that a message is available for transmission
            self.notify_message_available.notify_one();
            
            // Return true if tokens still available (> 0), false when tokens = 0 (backpressured)
            remaining_tokens > 0
        } else {
            // No tokens available - bucket is backpressured
            false
        }
    }

    pub async fn emit_message(self: &Arc<Self>) -> Option<Message> {
        let mut buffer = self.buffer.lock().await;
        let msg = buffer.pop_front();
        
        if let Some(ref message) = msg {
            let capacity = *self.capacity.lock().await;
            let mut tokens = self.tokens.lock().await;
            let was_backpressured = *tokens == 0;
            
            // Restore token after message emission, but don't exceed capacity
            *tokens = std::cmp::min(*tokens + 1, capacity);
            let current_tokens = *tokens;
            
            // Extract message ID for logging
            let message_id = if let Ok(parsed) = serde_json::from_value::<serde_json::Map<String, serde_json::Value>>(message.0.clone()) {
                parsed.get("id").and_then(|v| v.as_str()).unwrap_or("unknown").to_string()
            } else {
                "unknown".to_string()
            };
            
            info!("📤 EMIT_MESSAGE - Message ID: {}, Buffer size: {}, Tokens: {}/{}, Bucket: {}", 
                  message_id, buffer.len(), current_tokens, capacity, self.name);
            
            // Emit drain when:
            // 1. Recovering from backpressure (was 0, now > 0), OR
            // 2. When buffer is empty and we have tokens available, OR
            // 3. When bucket reaches full capacity (ready to accept more messages)
            let buffer_empty = buffer.is_empty();
            if was_backpressured || (buffer_empty && current_tokens > 0) || current_tokens == capacity {
                info!("🔔 DRAIN NOTIFICATION - Reason: {}, Bucket: {}", 
                      if was_backpressured { "backpressure recovery" } 
                      else if buffer_empty { "buffer empty" } 
                      else { "full capacity" }, self.name);
                drop(buffer);
                drop(tokens);
                self.drain().await;
            }
        }
        msg
    }

    pub async fn set_tokens(&self, new_capacity: usize) {
        let mut tokens = self.tokens.lock().await;
        let mut capacity = self.capacity.lock().await;
        *tokens = std::cmp::min(*tokens, new_capacity);
        *capacity = new_capacity;
    }

    pub async fn get_tokens_remaining(&self) -> usize {
        *self.tokens.lock().await
    }

    pub async fn get_high_watermark(&self) -> usize {
        *self.capacity.lock().await
    }

    pub async fn drain(&self) {
        self.notify_drain.notify_waiters();
    }

    pub fn on_drain(&self) -> Arc<Notify> {
        self.notify_drain.clone()
    }
    
    pub fn on_message_available(&self) -> Arc<Notify> {
        self.notify_message_available.clone()
    }
    
    /// Start a single sequential consumer task for this bucket
    fn start_sequential_consumer(self: &Arc<Self>) {
        let bucket = Arc::clone(self);
        
        info!("🔧 START_SEQUENTIAL_CONSUMER CALLED - Bucket: {}", bucket.name);
        
        // Use try_lock to avoid blocking and prevent race conditions
        let should_start_consumer = {
            if let Ok(mut consumer_started) = bucket.consumer_started.try_lock() {
                if *consumer_started {
                    // Consumer already started, do nothing
                    warn!("Consumer already started for bucket: {}", bucket.name);
                    false
                } else {
                    *consumer_started = true;
                    info!("Starting new consumer for bucket: {}", bucket.name);
                    true
                }
            } else {
                // If try_lock fails, it means another thread is already setting up the consumer
                warn!("Consumer lock failed for bucket: {}", bucket.name);
                false
            }
        };
        
        if should_start_consumer {
            // Only spawn the task if we successfully set the flag
            info!("Spawning consumer task for bucket: {}", bucket.name);
            tokio::spawn(async move {
                info!("Consumer task started for bucket: {}", bucket.name);
                loop {
                    // Wait for message availability notification
                    info!("⏳ CONSUMER WAITING FOR NOTIFICATION - Bucket: {}", bucket.name);
                    bucket.notify_message_available.notified().await;
                    info!("🔔 CONSUMER NOTIFICATION RECEIVED - Bucket: {}", bucket.name);
                    
                    // Process messages one by one sequentially with rate limiting
                    let mut message_count = 0;
                    while let Some(message) = bucket.emit_message().await {
                        message_count += 1;
                        info!("📤 CONSUMER PROCESSING MESSAGE #{} - Bucket: {}", message_count, bucket.name);
                        bucket.transmit_to_channel(&message).await;

                        
                        // Add a significant delay to provide effective rate limiting
                        // This controls the transmission rate to prevent overwhelming the system
                        tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
                    }
                    info!("Consumer processed {} messages for bucket: {}", message_count, bucket.name);
                }
            });
        }
    }
    
    /// Transmit message to channel using gateway broadcast mechanism
    async fn transmit_to_channel(self: &Arc<Self>, message: &Message) {
        use crate::message_stream::gateway;
        
        // Parse the message to extract organization_id
        if let Ok(parsed_msg) = serde_json::from_value::<serde_json::Map<String, serde_json::Value>>(message.0.clone()) {
            if let Some(org_id) = parsed_msg.get("organization_id").and_then(|v| v.as_str()) {
                let channel_name = &self.name;
                
                // Get the global SocketIo instance and broadcast to the channel
                 if let Some(streaming_service) = gateway::STREAMING_SERVICE.get() {
                     let socket_io = streaming_service.get_socket_io();
                     gateway::broadcast_to_channel(socket_io, org_id, channel_name, message.0.clone());
                 } else {
                     error!("[{}] Streaming service not initialized", self.name);
                 }
            } else {
                error!("[{}] Message missing organization_id: {:?}", self.name, message.0);
            }
        } else {
            error!("[{}] Failed to parse message: {:?}", self.name, message.0);
        }
    }
}
