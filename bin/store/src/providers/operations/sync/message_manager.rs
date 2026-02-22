use crate::database::db;
use crate::generated::models::crdt_message_model::CrdtMessageModel;
use crate::providers::operations::sync::message_service;
use diesel::result::Error as DieselError;
use once_cell::sync::OnceCell;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::mpsc;
use tokio::time::timeout;

use lazy_static::lazy_static;
use std::sync::atomic::{AtomicBool, Ordering};

/// Max messages per batch before flushing to DB.
const BATCH_SIZE: usize = 300;
/// When no message arrives for this long, flush any partial batch (avoids holding messages during low traffic).
const BATCH_FLUSH_TIMEOUT: Duration = Duration::from_millis(5);

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
        let mut batch: Vec<CrdtMessageModel> = Vec::with_capacity(BATCH_SIZE);
        let mut channel_closed = false;

        while !channel_closed {
            let (msg, closed) = if batch.is_empty() {
                let m = self.receiver.recv().await;
                let closed = m.is_none();
                (m, closed)
            } else {
                match timeout(BATCH_FLUSH_TIMEOUT, self.receiver.recv()).await {
                    Ok(Some(m)) => (Some(m), false),
                    Ok(None) => (None, true),
                    Err(_) => (None, false), // timeout: flush partial batch
                }
            };
            if closed {
                channel_closed = true;
            }

            let msg_is_none = msg.is_none();
            if let Some(message) = msg {
                QUEUE_EMPTY.store(false, Ordering::SeqCst);
                QUEUE_SIZE.fetch_add(1, Ordering::SeqCst);
                batch.push(message);
            }

            let should_flush = batch.len() >= BATCH_SIZE
                || channel_closed
                || (msg_is_none && !batch.is_empty());

            if should_flush && !batch.is_empty() {
                let n = batch.len();
                match self.process_batch(std::mem::take(&mut batch)).await {
                    Ok(_) => {
                        log::debug!("Batch of {} messages stored", n);
                    }
                    Err(e) => {
                        log::error!("Failed to store message batch: {}", e);
                    }
                }
                let prev = QUEUE_SIZE.fetch_sub(n, Ordering::SeqCst);
                if prev <= n {
                    QUEUE_EMPTY.store(true, Ordering::SeqCst);
                }
            }

            if channel_closed {
                break;
            }
        }
        QUEUE_EMPTY.store(true, Ordering::SeqCst);
    }

    async fn process_batch(
        &self,
        messages: Vec<CrdtMessageModel>,
    ) -> Result<(), DieselError> {
        if messages.is_empty() {
            return Ok(());
        }
        let mut conn = db::get_async_connection().await;
        message_service::insert_messages_batch(&mut conn, &messages).await?;
        Ok(())
    }
}
pub async fn save_pending_messages() -> Result<(), String> {
    use tokio::time::sleep;

    log::info!("Waiting for message queue to drain...");
    let initial_size = get_queue_size();
    log::info!("Current message queue size: {}", initial_size);

    // Wait for the background MessageManager to process all messages; do not force-reset
    // so that shutdown does not proceed with messages still in the channel.
    let max_wait_time = Duration::from_secs(300);
    let start_time = std::time::Instant::now();
    let mut last_log = std::time::Instant::now();

    while !is_queue_empty() {
        if start_time.elapsed() > max_wait_time {
            log::error!(
                "Exceeded maximum wait time ({}s) for queue drain. {} messages still pending. Shutting down anyway.",
                max_wait_time.as_secs(),
                get_queue_size()
            );
            return Err(format!(
                "Queue did not drain in time: {} messages remaining",
                get_queue_size()
            ));
        }

        sleep(Duration::from_millis(100)).await;

        if last_log.elapsed() >= Duration::from_secs(5) {
            let current_size = get_queue_size();
            log::info!(
                "Still waiting for message queue to drain... ({:.0}s elapsed) - {} messages remaining",
                start_time.elapsed().as_secs_f64(),
                current_size
            );
            last_log = std::time::Instant::now();
        }
    }

    log::info!("Message queue drained, proceeding with shutdown");
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
