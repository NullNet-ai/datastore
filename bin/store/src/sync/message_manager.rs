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
}

pub fn is_queue_empty() -> bool {
    QUEUE_EMPTY.load(Ordering::SeqCst)
}

pub struct MessageManager {
    receiver: mpsc::Receiver<CrdtMessageModel>,
    initialized: bool,
}

pub static SENDER: OnceCell<Arc<mpsc::Sender<CrdtMessageModel>>> = OnceCell::new();

pub fn get_sender() -> Option<&'static Arc<mpsc::Sender<CrdtMessageModel>>> {
    if let Some(sender) = SENDER.get() {
        Some(sender)
    } else {
        log::error!("Message sender not initialized");
        None
    }
}

impl MessageManager {
    pub fn new(receiver: mpsc::Receiver<CrdtMessageModel>) -> Self {
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
            match self.process_message(message).await {
                Ok(_) => {
                    log::debug!("Successfully processed message");
                }
                Err(e) => {
                    log::error!("Error processing message: {}", e);
                }
            }

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

    while !is_queue_empty() {
        // Wait a bit before checking again
        sleep(Duration::from_millis(100)).await;

        // Log progress periodically
        static mut COUNTER: u32 = 0;
        unsafe {
            COUNTER += 1;
            if COUNTER % 50 == 0 {
                // Log every ~5 seconds
                log::info!(
                    "Still waiting for message queue to drain... ({} seconds)",
                    COUNTER / 10
                );
            }
        }
    }

    log::info!("Message queue is empty, safe to shut down");
    Ok(())
}

// Create a channel for sending messages to the background service
pub fn create_message_channel() -> mpsc::Sender<CrdtMessageModel> {
    let (sender, receiver) = mpsc::channel(10000); // Buffer size of 100

    // Spawn the background service
    let mut manager = MessageManager::new(receiver);
    tokio::spawn(async move {
        manager.start().await;
    });

    sender
}
