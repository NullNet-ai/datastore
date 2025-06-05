use std::{collections::VecDeque, sync::Arc};
use tokio::sync::{Mutex, Notify};

#[derive(Debug, Clone)]
pub struct Message(pub String);

#[derive(Debug)]
pub struct TokenBucket {
    name: String,
    capacity: Mutex<usize>,
    tokens: Mutex<usize>,
    pub buffer: Mutex<VecDeque<Message>>,
    notify_drain: Arc<Notify>,
}

impl TokenBucket {
    pub fn new(name: &str, capacity: usize) -> Arc<Self> {
        Arc::new(Self {
            name: name.to_string(),
            capacity: Mutex::new(capacity),
            tokens: Mutex::new(capacity),
            buffer: Mutex::new(VecDeque::new()),
            notify_drain: Arc::new(Notify::new()),
        })
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub async fn receive_message(self: &Arc<Self>, msg: Message) {
        let mut tokens = self.tokens.lock().await;
        if *tokens > 0 {
            *tokens -= 1;
            self.buffer.lock().await.push_back(msg);
        }
    }

    pub async fn emit_message(self: &Arc<Self>) -> Option<Message> {
        let mut buffer = self.buffer.lock().await;
        let msg = buffer.pop_front();
        if msg.is_some() {
            let mut tokens = self.tokens.lock().await;
            *tokens += 1;
            let capacity = self.capacity.lock().await;
            if *tokens == *capacity {
                drop(capacity); // Release the lock before calling drain
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

    pub async fn decrement_tokens(&self) {
        let mut tokens = self.tokens.lock().await;
        if *tokens > 0 {
            *tokens -= 1;
        }
    }

    pub async fn get_high_watermark(&self) -> usize {
        *self.capacity.lock().await
    }

    pub async fn increment_tokens(&self) {
        let capacity = self.capacity.lock().await;
        let mut tokens = self.tokens.lock().await;
        if *tokens < *capacity {
            *tokens += 1;
            if *tokens == *capacity {
                drop(capacity); // Release the lock before calling drain
                self.drain().await;
            }
        }
    }

    pub async fn drain(&self) {
        self.notify_drain.notify_waiters();
    }

    pub fn on_drain(&self) -> Arc<Notify> {
        self.notify_drain.clone()
    }
}
