use crate::db;
use crate::models::crdt_message_model::CrdtMessageModel;
use crate::sync::message_service;
use once_cell::sync::OnceCell;
use std::sync::Arc;
use tokio::sync::mpsc;
use tokio::time::{sleep, Duration};

use lazy_static::lazy_static;
use std::sync::atomic::{AtomicBool, Ordering};

lazy_static! {
    static ref QUEUE_EMPTY: AtomicBool = AtomicBool::new(true);
    static ref QUEUE_SIZE: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
}

pub fn is_queue_empty() -> bool {
    QUEUE_EMPTY.load(Ordering::SeqCst)
}

pub fn get_queue_size() -> usize {
    QUEUE_SIZE.load(Ordering::SeqCst)
}

#[allow(warnings)]
pub fn set_queue_size(size: usize) {
    QUEUE_SIZE.store(size, Ordering::SeqCst);
}

pub struct MessageManager {
    receiver: mpsc::UnboundedReceiver<CrdtMessageModel>,
    initialized: bool,
}

pub static SENDER: OnceCell<Arc<mpsc::UnboundedSender<CrdtMessageModel>>> = OnceCell::new();

pub fn get_sender() -> Option<&'static Arc<mpsc::UnboundedSender<CrdtMessageModel>>> {
    if let Some(sender) = SENDER.get() {
        Some(sender)
    } else {
        log::error!("Message sender not initialized");
        None
    }
}

impl MessageManager {
    pub fn new(receiver: mpsc::UnboundedReceiver<CrdtMessageModel>) -> Self {
        MessageManager {
            receiver,
            initialized: false,
        }
    }

    pub async fn start(&mut self) {
        if self.initialized {
            return;
        }

        self.initialized = true;

        while let Some(message) = self.receiver.recv().await {
            QUEUE_EMPTY.store(false, Ordering::SeqCst);
            // Increment queue size when message is received
            let current_size = QUEUE_SIZE.fetch_add(1, Ordering::SeqCst);
            log::debug!("Message received. Queue size: {}", current_size + 1);

            match self.process_message(message).await {
                Ok(_) => {
                    log::debug!("Successfully processed message");
                }
                Err(e) => {
                    log::error!("Error processing message: {}", e);
                }
            }

            // Decrement queue size after processing
            let new_size = QUEUE_SIZE.fetch_sub(1, Ordering::SeqCst) - 1;
            if new_size == 0 {
                QUEUE_EMPTY.store(true, Ordering::SeqCst);
            }
            log::debug!("Message processed. Queue size: {}", new_size);

            // Add a small delay to prevent CPU overuse
            sleep(Duration::from_millis(10)).await;
        }
        QUEUE_EMPTY.store(true, Ordering::SeqCst);
    }

    async fn process_message(
        &self,
        message: CrdtMessageModel,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut conn = db::get_async_connection().await;

        // Store the message
        match message_service::insert_message(&mut conn, message).await {
            Ok(_) => {
                log::debug!("Message stored successfully");
                Ok(())
            }
            Err(e) => {
                log::error!("Failed to store message: {}", e);
                Err(Box::new(e))
            }
        }
    }
}
pub async fn save_pending_messages() -> Result<(), String> {
    log::info!("Waiting for message queue to drain...");
    let initial_size = get_queue_size();
    log::info!("Current message queue size: {}", initial_size);

    // Add a maximum wait time (e.g., 30 seconds)
    let max_wait_time = Duration::from_secs(30);
    let start_time = std::time::Instant::now();
    let mut last_size = initial_size;
    let mut stuck_count = 0;

    while !is_queue_empty() {
        // Check if we've exceeded the maximum wait time
        if start_time.elapsed() > max_wait_time {
            log::warn!("Exceeded maximum wait time for queue drain. Forcing shutdown with {} messages remaining", get_queue_size());
            // Force reset the counter to allow shutdown
            QUEUE_SIZE.store(0, Ordering::SeqCst);
            QUEUE_EMPTY.store(true, Ordering::SeqCst);
            break;
        }

        // Wait a bit before checking again
        sleep(Duration::from_millis(100)).await;

        // Log progress periodically
        static mut COUNTER: u32 = 0;
        unsafe {
            COUNTER += 1;
            if COUNTER % 50 == 0 {
                // Log every ~5 seconds
                let current_size = get_queue_size();
                log::info!(
                    "Still waiting for message queue to drain... ({} seconds) - {} messages remaining",
                    COUNTER / 10,
                    current_size
                );

                // Check if the size hasn't changed for multiple iterations
                if current_size == last_size && current_size > 0 {
                    stuck_count += 1;

                    // If stuck for too long (15 seconds with no change), reset the counter
                    if stuck_count >= 3 {
                        log::warn!("Queue size hasn't changed for 15 seconds. Possible counter desynchronization. Resetting counter.");
                        QUEUE_SIZE.store(0, Ordering::SeqCst);
                        QUEUE_EMPTY.store(true, Ordering::SeqCst);
                        break;
                    }
                } else {
                    stuck_count = 0;
                }

                last_size = current_size;
            }
        }
    }

    log::info!("Message queue is empty or timeout reached, proceeding with shutdown");
    Ok(())
}

// Create a channel for sending messages to the background service
pub fn create_message_channel() -> mpsc::UnboundedSender<CrdtMessageModel> {
    let (sender, receiver) = mpsc::unbounded_channel(); // Buffer size of 100

    // Spawn the background service
    let mut manager = MessageManager::new(receiver);
    tokio::spawn(async move {
        manager.start().await;
    });

    sender
}
