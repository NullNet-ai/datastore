use crate::sync::sync_service::insert;
use serde_json::Value;
use std::sync::Arc;
use tokio::sync::mpsc::{self, Receiver, Sender};
use tokio::sync::Mutex;
use std::collections::VecDeque;
use log;

// Define a message structure
#[derive(Debug, Clone)]
pub struct SyncMessage {
    pub table: String,
    pub record: Value,
}

// Global sender that can be accessed from anywhere
static mut SENDER: Option<Arc<Sender<SyncMessage>>> = None;

// Queue service to manage the message queue
pub struct BatchSyncService {
    receiver: Arc<Mutex<Receiver<SyncMessage>>>,
    queue: Arc<Mutex<VecDeque<SyncMessage>>>,
}

impl BatchSyncService {
    // Initialize the queue service
    pub async fn init() -> Result<(), String> {
        // Create a channel with a buffer size
        let (sender, receiver) = mpsc::channel::<SyncMessage>(100);
        
        // Store the sender in a global variable for access from anywhere
        unsafe {
            SENDER = Some(Arc::new(sender));
        }
        
        // Create the queue service
        let service = Self {
            receiver: Arc::new(Mutex::new(receiver)),
            queue: Arc::new(Mutex::new(VecDeque::new())),
        };
        
        // Start the background processing task
        service.start_background_processor().await;
        
        Ok(())
    }
    
    // Get the global sender
    pub fn get_sender() -> Option<Arc<Sender<SyncMessage>>> {
        unsafe {
            SENDER.clone()
        }
    }
    
    // Start the background processor
    async fn start_background_processor(&self) {
        let receiver = self.receiver.clone();
        let queue = self.queue.clone();
        
        // Spawn a new tokio task for background processing
        tokio::spawn(async move {
            log::info!("Starting background sync processor");
            
            loop {
                // Process any messages in the queue first
                if let Some(message) = Self::get_next_message(&queue).await {
                    if Self::process_message(message.clone()).await {
                        // Message processed successfully, remove from queue
                        log::debug!("Successfully processed message for table: {}", message.table);
                    } else {
                        // Processing failed, put it back in the queue for retry
                        log::warn!("Failed to process message for table: {}, will retry", message.table);
                        Self::add_to_queue(&queue, message).await;
                        
                        // Wait a bit before retrying to avoid hammering the system
                        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
                    }
                } else {
                    // No messages in queue, try to receive new ones
                    let mut rx = receiver.lock().await;
                    match rx.recv().await {
                        Some(message) => {
                            log::debug!("Received new message for table: {}", message.table);
                            
                            // Try to process the message immediately
                            if !Self::process_message(message.clone()).await {
                                // If processing fails, add to queue for retry
                                log::warn!("Failed to process new message, adding to retry queue");
                                Self::add_to_queue(&queue, message).await;
                            } else {
                                log::debug!("Successfully processed new message for table: {}", message.table);
                            }
                        }
                        None => {
                            // Channel closed, which shouldn't happen in normal operation
                            log::error!("Message channel closed, terminating background processor");
                            break;
                        }
                    }
                }
            }
        });
    }
    
    // Get the next message from the queue
    async fn get_next_message(queue: &Arc<Mutex<VecDeque<SyncMessage>>>) -> Option<SyncMessage> {
        let mut queue_lock = queue.lock().await;
        queue_lock.pop_front()
    }
    
    // Add a message to the queue
    async fn add_to_queue(queue: &Arc<Mutex<VecDeque<SyncMessage>>>, message: SyncMessage) {
        let mut queue_lock = queue.lock().await;
        queue_lock.push_back(message);
    }
    
    // Process a single message
    // Returns true if the message was processed successfully
    async fn process_message(message: SyncMessage) -> bool {
        // Try to insert the record
        match insert(&message.table, message.record.clone()).await {
            Ok(_) => true,  // Sync successful, mark as processed
            Err(e) => {
                log::error!("Error processing message: {}", e);
                false  // Sync failed, will be retried
            }
        }
    }
    
    // Send a message to the queue
    pub async fn send_message(table: String, record: Value) -> Result<(), String> {
        let sender = Self::get_sender().ok_or_else(|| "Queue service not initialized".to_string())?;
        
        let message = SyncMessage {
            table,
            record,
        };
        
        sender.send(message).await.map_err(|e| format!("Failed to send message: {}", e))
    }
}