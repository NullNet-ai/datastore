use crate::message_stream::token_bucket::Message;
use crate::message_stream::token_bucket::TokenBucket;
use std::{
    collections::{HashMap, HashSet, VecDeque},
    sync::Arc,
    time::Instant,
};
use tokio::sync::Mutex;
use tokio::time::{sleep, Duration};

pub struct BrokerService {
    pub queues: Mutex<HashMap<String, (VecDeque<Message>, Instant)>>,
    pub active_pipes: Mutex<HashMap<String, Arc<TokenBucket>>>,
    pub backpressured_pipes: Mutex<HashSet<String>>,
}
#[allow(warnings)]
impl BrokerService {
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            queues: Mutex::new(HashMap::new()),
            active_pipes: Mutex::new(HashMap::new()),
            backpressured_pipes: Mutex::new(HashSet::new()),
        })
    }

    pub async fn register_pipe(self: &Arc<Self>, pipe: Arc<TokenBucket>) {
        let pipe_name = pipe.name();
        let mut pipes = self.active_pipes.lock().await;
        pipes.insert(pipe_name.clone(), pipe.clone());

        // Set up drain event listener for this pipe
        let broker = Arc::downgrade(self); // Now self is &Arc<BrokerService>
        let pipe_name_clone = pipe_name.clone();
        let notify = pipe.on_drain();

        tokio::spawn(async move {
            loop {
                // Wait for drain notification
                notify.notified().await;

                // Try to upgrade the weak reference
                if let Some(broker) = broker.upgrade() {
                    // Process buffered messages when drain occurs
                    broker.handle_pipe_drain(&pipe_name_clone).await;
                } else {
                    // Broker has been dropped, exit the loop
                    break;
                }
            }
        });
    }

    pub async fn handle_pipe_drain(&self, pipe_name: &str) {
        // Check if pipe is in backpressured set
        let is_backpressured = {
            let backpressured = self.backpressured_pipes.lock().await;
            backpressured.contains(pipe_name)
        };

        if is_backpressured {
            // Get the pipe
            if let Some(pipe) = self.get_pipe(pipe_name).await {
                // Try to drain the buffer
                let mut should_remove = true;

                // Process messages until pipe is full or queue is empty
                loop {
                    let mut queues = self.queues.lock().await;
                    if let Some((queue, _)) = queues.get_mut(pipe_name) {
                        if queue.is_empty() {
                            break; // No more messages to process
                        }

                        // Check if pipe has tokens available
                        if pipe.get_tokens_remaining().await == 0 {
                            should_remove = false; // Still backpressured
                            break;
                        }

                        // Process one message
                        if let Some(msg) = queue.pop_front() {
                            drop(queues); // Release lock before async call
                            pipe.receive_message(msg).await;
                        } else {
                            break;
                        }
                    } else {
                        break; // No queue for this pipe
                    }
                }

                // Clean up if needed
                if should_remove {
                    let mut backpressured = self.backpressured_pipes.lock().await;
                    backpressured.remove(pipe_name);

                    // Also remove empty queue
                    let mut queues = self.queues.lock().await;
                    if let Some((queue, _)) = queues.get(pipe_name) {
                        if queue.is_empty() {
                            queues.remove(pipe_name);
                        }
                    }
                }
            }
        }
    }

    pub async fn unregister_pipe(&self, pipe_name: &str) {
        let mut pipes = self.active_pipes.lock().await;
        pipes.remove(pipe_name);

        // Also clean up from backpressured set
        let mut backpressured = self.backpressured_pipes.lock().await;
        backpressured.remove(pipe_name);

        // Clean up any queued messages
        self.empty_queue(pipe_name).await;
    }

    pub async fn get_pipe(&self, pipe_name: &str) -> Option<Arc<TokenBucket>> {
        let pipes = self.active_pipes.lock().await;
        pipes.get(pipe_name).cloned()
    }

    pub async fn get_active_pipe_names(&self) -> Vec<String> {
        let pipes = self.active_pipes.lock().await;
        pipes.keys().cloned().collect()
    }

    pub async fn buffer_message(&self, pipe_name: &str, msg: Message) {
        // Add to backpressured set
        let mut backpressured = self.backpressured_pipes.lock().await;
        backpressured.insert(pipe_name.to_string());
        drop(backpressured);

        // Add message to queue
        let mut queues = self.queues.lock().await;
        let (queue, timestamp) = queues
            .entry(pipe_name.to_string())
            .or_insert((VecDeque::new(), Instant::now()));
        queue.push_back(msg);
        *timestamp = Instant::now();
    }

    pub async fn try_drain(&self, pipe_name: &str) {
        // Get the pipe from active_pipes
        let pipe_opt = self.get_pipe(pipe_name).await;

        if let Some(pipe) = pipe_opt {
            let mut processed = false;
            let mut should_remove = false;

            // Process messages in a loop, re-acquiring the lock each time
            loop {
                // Scope the lock to ensure it's dropped at the end of each iteration
                let msg_opt = {
                    let mut queues = self.queues.lock().await;
                    if let Some((queue, _)) = queues.get_mut(pipe_name) {
                        if queue.is_empty() {
                            should_remove = processed;
                            break;
                        }

                        if pipe.get_tokens_remaining().await > 0 {
                            queue.pop_front()
                        } else {
                            // Mark as backpressured and exit
                            let mut backpressured = self.backpressured_pipes.lock().await;
                            backpressured.insert(pipe_name.to_string());
                            break;
                        }
                    } else {
                        break;
                    }
                };

                // Process the message outside the lock scope
                if let Some(msg) = msg_opt {
                    pipe.receive_message(msg).await;
                    processed = true;
                } else {
                    break;
                }
            }

            // Clean up if needed
            if should_remove {
                let mut backpressured = self.backpressured_pipes.lock().await;
                backpressured.remove(pipe_name);

                // Also remove empty queue
                self.queues.lock().await.remove(pipe_name);
            }
        }
    }

    pub async fn empty_queue(&self, pipe_name: &str) {
        self.queues.lock().await.remove(pipe_name);

        // Also remove from backpressured set
        let mut backpressured = self.backpressured_pipes.lock().await;
        backpressured.remove(pipe_name);
    }

    pub fn spawn_queue_cleanup_task(self: &Arc<Self>) {
        let this = Arc::clone(self);
        tokio::spawn(async move {
            loop {
                sleep(Duration::from_secs(5)).await;
                let now = Instant::now();
                let mut queues = this.queues.lock().await;
                let mut to_remove = Vec::new();

                queues.retain(|name, (q, last)| {
                    let keep = !q.is_empty() && now.duration_since(*last) < Duration::from_secs(30);
                    if !keep {
                        to_remove.push(name.clone());
                    }
                    keep
                });

                drop(queues);

                // Also clean up backpressured set
                if !to_remove.is_empty() {
                    let mut backpressured = this.backpressured_pipes.lock().await;
                    for name in to_remove {
                        backpressured.remove(&name);
                    }
                }
            }
        });
    }

    pub async fn route_from_main_pipe(
        &self,
        main_pipe: &Arc<TokenBucket>,
        get_target_pipe: impl Fn(&Message) -> String,
    ) {
        loop {
            let msg_opt = {
                let mut buffer = main_pipe.buffer.lock().await;
                buffer.pop_front()
            };

            if let Some(msg) = msg_opt {
                let target_name = get_target_pipe(&msg);

                // Get the target pipe from active_pipes
                let target_pipe = self.get_pipe(&target_name).await;

                if let Some(pipe) = target_pipe {
                    if pipe.get_tokens_remaining().await > 0 {
                        pipe.receive_message(msg).await;
                        main_pipe.increment_tokens().await;
                    } else {
                        // Buffer the message and mark pipe as backpressured
                        self.buffer_message(&target_name, msg).await;
                    }
                } else {
                    println!("Pipe {} not found.", target_name);
                    // Increment tokens for the main pipe since we're not using this message
                    main_pipe.increment_tokens().await;
                }
            } else {
                // No messages in the buffer, wait a bit before checking again
                tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
            }
        }
    }

    pub fn spawn_main_pipe_routing_task(
        self: &Arc<Self>,
        main_pipe: Arc<TokenBucket>,
        get_target_pipe: impl Fn(&Message) -> String + Send + 'static,
    ) {
        let this = Arc::clone(self);
        let main_pipe_clone = Arc::clone(&main_pipe);

        tokio::spawn(async move {
            this.route_from_main_pipe(&main_pipe_clone, get_target_pipe)
                .await;
        });
    }
}
