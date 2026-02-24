use crate::database::db;
use crate::generated::models::crdt_message_model::CrdtMessageModel;
use crate::providers::operations::sync::message_service;
use diesel::result::Error as DieselError;
use once_cell::sync::OnceCell;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::mpsc;
use tokio::sync::Mutex;
use tokio::time::timeout;

use lazy_static::lazy_static;
use std::sync::atomic::{AtomicBool, Ordering};

/// Max messages per batch before flushing to DB.
const BATCH_SIZE: usize = 300;
/// When no message arrives for this long, flush any partial batch (avoids holding messages during low traffic).
const BATCH_FLUSH_TIMEOUT: Duration = Duration::from_millis(5);
/// Interval between retries of failed batches (local in-memory queue only).
const FAILED_BATCH_RETRY_INTERVAL: Duration = Duration::from_secs(5 * 60);

lazy_static! {
    static ref QUEUE_EMPTY: AtomicBool = AtomicBool::new(true);
    static ref QUEUE_SIZE: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
    /// In-memory queue of batches that failed to insert. Retried every 5 minutes.
    static ref FAILED_BATCH_QUEUE: Arc<Mutex<Vec<Vec<CrdtMessageModel>>>> =
        Arc::new(Mutex::new(Vec::new()));
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
                        log::error!("Failed to store message batch: {} (batch queued for retry)", e);
                        // Batch was already pushed to FAILED_BATCH_QUEUE inside process_batch
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
        match message_service::insert_messages_batch(&mut conn, &messages).await {
            Ok(_) => Ok(()),
            Err(e) => {
                // Queue failed batch for retry (local in-memory only)
                FAILED_BATCH_QUEUE.lock().await.push(messages);
                Err(e)
            }
        }
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

    log::info!("Message queue drained, waiting for failed-batch queue to flush...");

    // Wait for failed-batch queue to empty: run retry passes until empty or timeout
    let flush_start = std::time::Instant::now();
    let mut last_flush_log = std::time::Instant::now();
    loop {
        let to_retry: Vec<Vec<CrdtMessageModel>> = {
            let mut q = FAILED_BATCH_QUEUE.lock().await;
            std::mem::take(q.as_mut())
        };
        if to_retry.is_empty() {
            log::info!("Failed-batch queue flushed, proceeding with shutdown");
            return Ok(());
        }
        if flush_start.elapsed() > max_wait_time {
            let pending_count = to_retry.len();
            let mut q = FAILED_BATCH_QUEUE.lock().await;
            q.extend(to_retry);
            log::error!(
                "Exceeded maximum wait time ({}s) for failed-batch queue flush. {} batch(es) still pending. Shutting down anyway.",
                max_wait_time.as_secs(),
                pending_count
            );
            return Err(format!(
                "Failed-batch queue did not flush in time: {} batch(es) remaining",
                pending_count
            ));
        }
        log::info!(
            "Shutdown flush: retrying {} failed batch(es)",
            to_retry.len()
        );
        let mut requeue = Vec::new();
        for batch in to_retry {
            if batch.is_empty() {
                continue;
            }
            let n = batch.len();
            match try_insert_batch(&batch).await {
                Ok(_) => {
                    log::debug!("Shutdown flush: batch of {} messages stored", n);
                }
                Err(e) => {
                    log::warn!("Shutdown flush: batch of {} messages failed: {} (will requeue)", n, e);
                    requeue.push(batch);
                }
            }
        }
        if !requeue.is_empty() {
            let mut q = FAILED_BATCH_QUEUE.lock().await;
            q.extend(requeue);
        }
        sleep(Duration::from_millis(100)).await;
        if last_flush_log.elapsed() >= Duration::from_secs(5) {
            let remaining = FAILED_BATCH_QUEUE.lock().await.len();
            log::info!(
                "Still waiting for failed-batch queue to flush... ({:.0}s elapsed) - {} batch(es) remaining",
                flush_start.elapsed().as_secs_f64(),
                remaining
            );
            last_flush_log = std::time::Instant::now();
        }
    }
}

/// Runs in the background: every 5 minutes, retries failed batches from the local in-memory queue.
/// Successful inserts remove the batch; failures re-queue it for the next cycle.
async fn run_failed_batch_retry_loop() {
    use tokio::time::sleep;
    loop {
        sleep(FAILED_BATCH_RETRY_INTERVAL).await;
        let to_retry: Vec<Vec<CrdtMessageModel>> = {
            let mut q = FAILED_BATCH_QUEUE.lock().await;
            std::mem::take(q.as_mut())
        };
        if to_retry.is_empty() {
            continue;
        }
        log::info!(
            "Retrying {} failed batch(es) from local queue",
            to_retry.len()
        );
        let mut requeue = Vec::new();
        for batch in to_retry {
            if batch.is_empty() {
                continue;
            }
            let n = batch.len();
            match try_insert_batch(&batch).await {
                Ok(_) => {
                    log::info!("Retry succeeded for batch of {} messages", n);
                }
                Err(e) => {
                    log::warn!("Retry failed for batch of {} messages: {} (will requeue)", n, e);
                    requeue.push(batch);
                }
            }
        }
        if !requeue.is_empty() {
            let mut q = FAILED_BATCH_QUEUE.lock().await;
            q.extend(requeue);
        }
    }
}

async fn try_insert_batch(messages: &[CrdtMessageModel]) -> Result<(), DieselError> {
    let mut conn = db::get_async_connection().await;
    message_service::insert_messages_batch(&mut conn, messages).await?;
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

    // Spawn the failed-batch retry loop (local queue only, every 5 mins)
    tokio::spawn(run_failed_batch_retry_loop());

    sender
}
