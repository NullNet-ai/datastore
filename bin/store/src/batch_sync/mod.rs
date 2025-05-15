use crate::sync::sync_service::{insert, update};
use log;
use serde_json::Value;
use std::collections::VecDeque;
use std::sync::Arc;
use tokio::sync::mpsc::{self, Receiver, Sender};
use tokio::sync::Mutex;

// Define message types
#[derive(Debug, Clone, PartialEq)]
pub enum MessageType {
    Insert,
    Update,
}

// Define a message structure
#[derive(Debug, Clone)]
pub struct SyncMessage {
    pub table: String,
    pub record: Value,
    pub message_type: MessageType,
}

// Global senders that can be accessed from anywhere
static mut INSERT_SENDER: Option<Arc<Sender<SyncMessage>>> = None;
static mut UPDATE_SENDER: Option<Arc<Sender<SyncMessage>>> = None;

// Queue service to manage the message queues
pub struct BatchSyncService {
    insert_receiver: Arc<Mutex<Receiver<SyncMessage>>>,
    update_receiver: Arc<Mutex<Receiver<SyncMessage>>>,
    insert_queue: Arc<Mutex<VecDeque<SyncMessage>>>,
    update_queue: Arc<Mutex<VecDeque<SyncMessage>>>,
}

impl BatchSyncService {
    // Initialize the queue service
    pub async fn init() -> Result<(), String> {
        // Create channels with buffer sizes
        let (insert_sender, insert_receiver) = mpsc::channel::<SyncMessage>(100);
        let (update_sender, update_receiver) = mpsc::channel::<SyncMessage>(100);

        // Store the senders in global variables for access from anywhere
        unsafe {
            INSERT_SENDER = Some(Arc::new(insert_sender));
            UPDATE_SENDER = Some(Arc::new(update_sender));
        }

        // Create the queue service
        let service = Self {
            insert_receiver: Arc::new(Mutex::new(insert_receiver)),
            update_receiver: Arc::new(Mutex::new(update_receiver)),
            insert_queue: Arc::new(Mutex::new(VecDeque::new())),
            update_queue: Arc::new(Mutex::new(VecDeque::new())),
        };

        // Start the background processing tasks
        service.start_insert_processor().await;
        service.start_update_processor().await;

        Ok(())
    }

    // Get the global insert sender
    pub fn get_insert_sender() -> Option<Arc<Sender<SyncMessage>>> {
        unsafe { INSERT_SENDER.clone() }
    }

    // Get the global update sender
    pub fn get_update_sender() -> Option<Arc<Sender<SyncMessage>>> {
        unsafe { UPDATE_SENDER.clone() }
    }

    // Start the insert processor
    async fn start_insert_processor(&self) {
        let receiver = self.insert_receiver.clone();
        let queue = self.insert_queue.clone();

        // Spawn a new tokio task for background processing
        tokio::spawn(async move {
            log::info!("Starting background insert processor");

            loop {
                // Process any messages in the queue first
                if let Some(message) = Self::get_next_message(&queue).await {
                    if Self::process_insert_message(message.clone()).await {
                        // Message processed successfully, remove from queue
                        log::debug!(
                            "Successfully processed insert message for table: {}",
                            message.table
                        );
                    } else {
                        // Processing failed, put it back in the queue for retry
                        log::warn!(
                            "Failed to process insert message for table: {}, will retry",
                            message.table
                        );
                        Self::add_to_queue(&queue, message).await;

                        // Wait a bit before retrying to avoid hammering the system
                        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
                    }
                } else {
                    // No messages in queue, try to receive new ones
                    let mut rx = receiver.lock().await;
                    match rx.recv().await {
                        Some(message) => {
                            log::debug!("Received new insert message for table: {}", message.table);

                            // Try to process the message immediately
                            if !Self::process_insert_message(message.clone()).await {
                                // If processing fails, add to queue for retry
                                log::warn!(
                                    "Failed to process new insert message, adding to retry queue"
                                );
                                Self::add_to_queue(&queue, message).await;
                            } else {
                                log::debug!(
                                    "Successfully processed new insert message for table: {}",
                                    message.table
                                );
                            }
                        }
                        None => {
                            // Channel closed, which shouldn't happen in normal operation
                            log::error!(
                                "Insert message channel closed, terminating background processor"
                            );
                            break;
                        }
                    }
                }
            }
        });
    }

    // Start the update processor
    async fn start_update_processor(&self) {
        let receiver = self.update_receiver.clone();
        let queue = self.update_queue.clone();

        // Spawn a new tokio task for background processing
        tokio::spawn(async move {
            log::info!("Starting background update processor");

            loop {
                // Process any messages in the queue first
                if let Some(message) = Self::get_next_message(&queue).await {
                    if Self::process_update_message(message.clone()).await {
                        // Message processed successfully, remove from queue
                        log::debug!(
                            "Successfully processed update message for table: {}",
                            message.table
                        );
                    } else {
                        // Processing failed, put it back in the queue for retry
                        log::warn!(
                            "Failed to process update message for table: {}, will retry",
                            message.table
                        );
                        Self::add_to_queue(&queue, message).await;

                        // Wait a bit before retrying to avoid hammering the system
                        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
                    }
                } else {
                    // No messages in queue, try to receive new ones
                    let mut rx = receiver.lock().await;
                    match rx.recv().await {
                        Some(message) => {
                            log::debug!("Received new update message for table: {}", message.table);

                            // Try to process the message immediately
                            if !Self::process_update_message(message.clone()).await {
                                // If processing fails, add to queue for retry
                                log::warn!(
                                    "Failed to process new update message, adding to retry queue"
                                );
                                Self::add_to_queue(&queue, message).await;
                            } else {
                                log::debug!(
                                    "Successfully processed new update message for table: {}",
                                    message.table
                                );
                            }
                        }
                        None => {
                            // Channel closed, which shouldn't happen in normal operation
                            log::error!(
                                "Update message channel closed, terminating background processor"
                            );
                            break;
                        }
                    }
                }
            }
        });
    }

    // Get the next message from a queue
    async fn get_next_message(queue: &Arc<Mutex<VecDeque<SyncMessage>>>) -> Option<SyncMessage> {
        let mut queue_lock = queue.lock().await;
        queue_lock.pop_front()
    }

    // Add a message to a queue
    async fn add_to_queue(queue: &Arc<Mutex<VecDeque<SyncMessage>>>, message: SyncMessage) {
        let mut queue_lock = queue.lock().await;
        queue_lock.push_back(message);
    }

    // Process an insert message
    async fn process_insert_message(message: SyncMessage) -> bool {
        // Try to insert the record
        match insert(&message.table, message.record.clone()).await {
            Ok(_) => true, // Sync successful, mark as processed
            Err(e) => {
                log::error!("Error processing insert message: {}", e);
                false // Sync failed, will be retried
            }
        }
    }

    // Process an update message
    async fn process_update_message(message: SyncMessage) -> bool {
        // Try to update the record
        //extract id from the message
        let id = match message.record.get("id").and_then(|v| v.as_str()) {
            Some(id) => id.to_string(),
            None => {
                log::error!(
                    "Error processing update message: ID field not found or not a string in record"
                );
                return false; // Cannot process without ID, will be retried
            }
        };
        let id_string = id.to_string();
        let mut record_obj = message.record.clone();

        // Check if record is an object and then remove the id
        if let Some(obj) = record_obj.as_object_mut() {
            obj.remove("id");
        }
        match update(&message.table, record_obj, &id_string).await {
            Ok(_) => true, // Sync successful, mark as processed
            Err(e) => {
                log::error!("Error processing update message: {}", e);
                false // Sync failed, will be retried
            }
        }
    }

    // Send an insert message to the queue
    pub async fn send_insert_message(table: String, record: Value) -> Result<(), String> {
        let sender =
            Self::get_insert_sender().ok_or_else(|| "Queue service not initialized".to_string())?;

        let message = SyncMessage {
            table,
            record,
            message_type: MessageType::Insert,
        };

        sender
            .send(message)
            .await
            .map_err(|e| format!("Failed to send insert message: {}", e))
    }

    // Send an update message to the queue
    pub async fn send_update_message(table: String, record: Value) -> Result<(), String> {
        let sender =
            Self::get_update_sender().ok_or_else(|| "Queue service not initialized".to_string())?;

        let message = SyncMessage {
            table,
            record,
            message_type: MessageType::Update,
        };

        sender
            .send(message)
            .await
            .map_err(|e| format!("Failed to send update message: {}", e))
    }
}
