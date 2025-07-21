use std::collections::{HashMap, HashSet, VecDeque};
use std::sync::Arc;
use tokio::sync::Mutex;
use crate::message_stream::token_bucket::TokenBucket;

/// Centralized shared state for all message streaming services
/// This eliminates duplication and provides a single source of truth
#[derive(Clone)]
pub struct SharedStreamingState {
    /// Channels that are currently backpressured (no token capacity)
    pub backpressured_channels: Arc<Mutex<HashSet<String>>>,
    
    /// Channels that are currently flushing from queue
    pub flushing_channels: Arc<Mutex<HashSet<String>>>,
    
    /// Active channel pipes (token buckets) by channel name
    pub active_channels: Arc<Mutex<HashMap<String, Arc<TokenBucket>>>>,
    
    /// Organization to channels mapping
    pub organization_channels: Arc<Mutex<HashMap<String, HashSet<String>>>>,
    
    /// Channel to organization mapping (reverse lookup)
    pub channel_organizations: Arc<Mutex<HashMap<String, String>>>,
    
    /// Processing queue for channels that need continued flushing (fairness queue)
    pub processing_queue: Arc<Mutex<VecDeque<String>>>,

}


impl SharedStreamingState {
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            backpressured_channels: Arc::new(Mutex::new(HashSet::new())),
            flushing_channels: Arc::new(Mutex::new(HashSet::new())),
            active_channels: Arc::new(Mutex::new(HashMap::new())),
            organization_channels: Arc::new(Mutex::new(HashMap::new())),
            channel_organizations: Arc::new(Mutex::new(HashMap::new())),
            processing_queue: Arc::new(Mutex::new(VecDeque::new())),
        })
    }
    
    /// Register a new channel with its token bucket (returns existing if already registered)
    pub async fn register_channel(
        &self,
        channel_name: &str,
        organization_id: &str,
        token_bucket: Arc<TokenBucket>,
        _capacity: usize,
    ) -> Arc<TokenBucket> {
        {
            let mut channels = self.active_channels.lock().await;
            // Check if channel already exists to prevent duplicate registration
            if let Some(existing_bucket) = channels.get(channel_name) {
                return existing_bucket.clone();
            }
            channels.insert(channel_name.to_string(), token_bucket.clone());
        }
        
        {
            let mut org_channels = self.organization_channels.lock().await;
            org_channels
                .entry(organization_id.to_string())
                .or_insert_with(HashSet::new)
                .insert(channel_name.to_string());
        }
        
        {
            let mut channel_orgs = self.channel_organizations.lock().await;
            channel_orgs.insert(channel_name.to_string(), organization_id.to_string());
        }
        
        token_bucket
    }
    

    
    /// Mark a channel as backpressured
    pub async fn mark_backpressured(&self, channel_name: &str) {
        let mut backpressured = self.backpressured_channels.lock().await;
        backpressured.insert(channel_name.to_string());
    }
    
    /// Remove a channel from backpressured state
    pub async fn remove_backpressured(&self, channel_name: &str) {
        let mut backpressured = self.backpressured_channels.lock().await;
        backpressured.remove(channel_name);
    }
    
    /// Get all backpressured channels

    
    /// Check if a channel is backpressured
    pub async fn is_backpressured(&self, channel_name: &str) -> bool {
        let backpressured = self.backpressured_channels.lock().await;
        backpressured.contains(channel_name)
    }
    
    /// Mark a channel as flushing
    pub async fn mark_flushing(&self, channel_name: &str) {
        let mut flushing = self.flushing_channels.lock().await;
        flushing.insert(channel_name.to_string());
    }
    
    /// Remove a channel from flushing state
    pub async fn remove_flushing(&self, channel_name: &str) {
        let mut flushing = self.flushing_channels.lock().await;
        flushing.remove(channel_name);
    }
    
    /// Check if a channel is flushing
    pub async fn is_flushing(&self, channel_name: &str) -> bool {
        let flushing = self.flushing_channels.lock().await;
        flushing.contains(channel_name)
    }
    
    /// Get the organization ID for a channel
    pub async fn get_channel_organization(&self, channel_name: &str) -> Option<String> {
        let channel_orgs = self.channel_organizations.lock().await;
        channel_orgs.get(channel_name).cloned()
    }
    
    /// Get a channel's token bucket
    pub async fn get_channel(&self, channel_name: &str) -> Option<Arc<TokenBucket>> {
        let channels = self.active_channels.lock().await;
        channels.get(channel_name).cloned()
    }
    
    /// Add a channel to the processing queue for continued flushing
    pub async fn queue_for_processing(&self, channel_name: &str) {
        let mut queue = self.processing_queue.lock().await;
        // Only add if not already in queue
        if !queue.iter().any(|ch| ch == channel_name) {
            queue.push_back(channel_name.to_string());
        }
    }
    
    /// Get the next channel from processing queue
    pub async fn dequeue_for_processing(&self) -> Option<String> {
        let mut queue = self.processing_queue.lock().await;
        queue.pop_front()
    }

}

/// Global instance for shared state
static SHARED_STATE: std::sync::OnceLock<Arc<SharedStreamingState>> = std::sync::OnceLock::new();

/// Get the global shared state instance
pub fn get_shared_state() -> Arc<SharedStreamingState> {
    SHARED_STATE
        .get_or_init(|| SharedStreamingState::new())
        .clone()
}