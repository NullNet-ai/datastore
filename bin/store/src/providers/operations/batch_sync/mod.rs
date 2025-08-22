use crate::controllers::common_controller::process_and_update_record;
use crate::generated::table_enum::generate_code;
use crate::providers::operations::sync::sync_service::{insert, update};
use crate::structs::structs::Auth;
use log;
pub mod background_sync;
use serde_json::{json, Value};
use std::collections::VecDeque;
use std::sync::Arc;
use tokio::sync::mpsc::{self, UnboundedReceiver, UnboundedSender};
use tokio::sync::Mutex;

// Define message types
#[derive(Debug, Clone, PartialEq)]
pub enum MessageType {
    Insert,
    Update,
}

#[derive(Debug, Clone)]
pub struct CodeAssignmentMessage {
    pub table_name: String,
    pub id: String,
    pub entity_prefix: String,
    pub auth_data: Auth,
    pub is_batch: bool,
}
// Define a message structure
#[derive(Debug, Clone)]
#[allow(warnings)]
pub struct SyncMessage {
    pub table: String,
    pub record: Value,
    pub message_type: MessageType,
}

// Global senders that can be accessed from anywhere
static mut INSERT_SENDER: Option<Arc<UnboundedSender<SyncMessage>>> = None;
static mut UPDATE_SENDER: Option<Arc<UnboundedSender<SyncMessage>>> = None;
static mut CODE_ASSIGNMENT_SENDER: Option<Arc<UnboundedSender<CodeAssignmentMessage>>> = None;

// Queue service to manage the message queues
pub struct BatchSyncService {
    insert_receiver: Arc<Mutex<UnboundedReceiver<SyncMessage>>>,
    update_receiver: Arc<Mutex<UnboundedReceiver<SyncMessage>>>,
    code_assignment_receiver: Arc<Mutex<UnboundedReceiver<CodeAssignmentMessage>>>,
    insert_queue: Arc<Mutex<VecDeque<SyncMessage>>>,
    update_queue: Arc<Mutex<VecDeque<SyncMessage>>>,
    code_assignment_queue: Arc<Mutex<VecDeque<CodeAssignmentMessage>>>,
}
#[allow(warnings)]
impl BatchSyncService {
    // Initialize the queue service
    pub async fn init() -> Result<(), String> {
        // Create channels with buffer sizes
        let (insert_sender, insert_receiver) = mpsc::unbounded_channel::<SyncMessage>();
        let (update_sender, update_receiver) = mpsc::unbounded_channel::<SyncMessage>();
        let (code_sender, code_receiver) = mpsc::unbounded_channel::<CodeAssignmentMessage>();

        // Store the senders in global variables for access from anywhere
        unsafe {
            INSERT_SENDER = Some(Arc::new(insert_sender));
            UPDATE_SENDER = Some(Arc::new(update_sender));
            CODE_ASSIGNMENT_SENDER = Some(Arc::new(code_sender));
        }

        // Create the queue service
        let service = Self {
            insert_receiver: Arc::new(Mutex::new(insert_receiver)),
            update_receiver: Arc::new(Mutex::new(update_receiver)),
            code_assignment_receiver: Arc::new(Mutex::new(code_receiver)),
            insert_queue: Arc::new(Mutex::new(VecDeque::new())),
            update_queue: Arc::new(Mutex::new(VecDeque::new())),
            code_assignment_queue: Arc::new(Mutex::new(VecDeque::new())),
        };

        // Start the background processing tasks
        service.start_insert_processor().await;
        service.start_update_processor().await;
        service.start_code_assignment_processor().await;

        Ok(())
    }

    // TODO: Kindly revisit this

    // Get the global insert sender
    pub fn get_insert_sender() -> Option<Arc<UnboundedSender<SyncMessage>>> {
        unsafe { INSERT_SENDER.clone() }
    }

    // Get the global update sender
    pub fn get_update_sender() -> Option<Arc<UnboundedSender<SyncMessage>>> {
        unsafe { UPDATE_SENDER.clone() }
    }

    pub fn get_code_assignment_sender() -> Option<Arc<UnboundedSender<CodeAssignmentMessage>>> {
        unsafe { CODE_ASSIGNMENT_SENDER.clone() }
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

    async fn start_code_assignment_processor(&self) {
        let receiver = self.code_assignment_receiver.clone();
        let queue = self.code_assignment_queue.clone();

        tokio::spawn(async move {
            log::info!("Starting background code assignment processor");

            loop {
                if let Some(message) = Self::get_next_code_assignment(&queue).await {
                    if Self::process_code_assignment_message(message.clone()).await {
                        log::debug!("Processed code assignment message: {:?}", message);
                        log::debug!(
                            "Successfully processed code assignment for table: {}",
                            message.table_name
                        );
                    } else {
                        log::warn!(
                            "Failed to process code assignment for table: {}, will retry",
                            message.table_name
                        );
                        Self::add_to_code_assignment_queue(&queue, message).await;
                        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
                    }
                } else {
                    let mut rx = receiver.lock().await;
                    match rx.recv().await {
                        Some(message) => {
                            if !Self::process_code_assignment_message(message.clone()).await {
                                log::warn!(
                                    "Failed to process code assignment, adding to retry queue"
                                );
                                Self::add_to_code_assignment_queue(&queue, message).await;
                            }
                        }
                        None => {
                            log::error!("Code assignment channel closed, terminating processor");
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
            .map_err(|e| format!("Failed to send update message: {}", e))
    }

    async fn get_next_code_assignment(
        queue: &Arc<Mutex<VecDeque<CodeAssignmentMessage>>>,
    ) -> Option<CodeAssignmentMessage> {
        let mut queue_lock = queue.lock().await;
        queue_lock.pop_front()
    }

    async fn add_to_code_assignment_queue(
        queue: &Arc<Mutex<VecDeque<CodeAssignmentMessage>>>,
        message: CodeAssignmentMessage,
    ) {
        let mut queue_lock = queue.lock().await;
        queue_lock.push_back(message);
    }

    async fn process_code_assignment_message(message: CodeAssignmentMessage) -> bool {
        let table_name = message.table_name;
        let id = message.id;
        let entity_prefix = message.entity_prefix;
        let auth_data = message.auth_data;
        let is_batch = message.is_batch;
        let code = match generate_code(&table_name, &entity_prefix, 10000).await {
            Ok(code) => code,
            Err(e) => {
                log::error!("Error processing code assignment message: {}", e);
                return false; // Sync failed, will be retried
            }
        };
        // Try to update the record
        let record_obj = json!({
            "code": code,
            "is_batch": is_batch,
        });

        match process_and_update_record(
            &table_name,
            record_obj,
            &id,
            None,
            "update",
            &auth_data,
            false,
        )
        .await
        {
            Ok(_response) => true,
            Err(error) => {
                log::error!("Error processing code assignment message: {}", error);
                return false;
            }
        }
    }

    // Send a code assignment message to the queue
    pub async fn send_code_assignment_message(
        table_name: String,
        id: String,
        entity_prefix: String,
        auth_data: Auth,
        is_batch: bool,
    ) -> Result<(), String> {
        let sender = Self::get_code_assignment_sender()
            .ok_or_else(|| "Queue service not initialized".to_string())?;

        let message = CodeAssignmentMessage {
            table_name,
            id,
            entity_prefix,
            auth_data,
            is_batch,
        };

        sender
            .send(message)
            .map_err(|e| format!("Failed to send code assignment message: {}", e))
    }
}
