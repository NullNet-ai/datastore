use once_cell::sync::OnceCell;
use tokio::sync::mpsc;
use std::sync::Arc;
use tokio::time::{sleep, Duration};
use crate::models::crdt_message_model::CrdtMessage;
use crate::db;
use crate::sync::message_service;

pub struct MessageManager {
    receiver: mpsc::Receiver<CrdtMessage>,
    initialized: bool,
}

pub static SENDER: OnceCell<Arc<mpsc::Sender<CrdtMessage>>> = OnceCell::new();

pub fn get_sender() -> Option<&'static Arc<mpsc::Sender<CrdtMessage>>> {
    if let Some(sender) = SENDER.get() {
        Some(sender)
    } else {
        log::error!("Message sender not initialized");
        None
    }
}

impl MessageManager {
    pub fn new(receiver: mpsc::Receiver<CrdtMessage>) -> Self {
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
    }

    async fn process_message(&self, message: CrdtMessage) -> Result<(), Box<dyn std::error::Error>> {
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

// Create a channel for sending messages to the background service
pub fn create_message_channel() -> mpsc::Sender<CrdtMessage> {
    let (sender, receiver) = mpsc::channel(10000); // Buffer size of 100
    
    // Spawn the background service
    let mut manager = MessageManager::new(receiver);
    tokio::spawn(async move {
        manager.start().await;
    });
    
    sender
}