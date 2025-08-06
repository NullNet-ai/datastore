use crate::message_stream::token_bucket::TokenBucket;
use std::collections::{HashMap, HashSet, VecDeque};
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Clone)]
pub struct SharedStreamingState {
    pub backpressured_channels: Arc<Mutex<HashSet<String>>>,
    pub flushing_channels: Arc<Mutex<HashSet<String>>>,
    pub processing_channels: Arc<Mutex<HashSet<String>>>, // Tracks channels currently being processed
    pub active_channels: Arc<Mutex<HashMap<String, Arc<TokenBucket>>>>,
    pub organization_channels: Arc<Mutex<HashMap<String, HashSet<String>>>>,
    pub channel_organizations: Arc<Mutex<HashMap<String, String>>>, //maintaining both mappings avoids expensive reverse lookups and provides O(1) access in both directions.
    pub processing_queue: Arc<Mutex<VecDeque<String>>>,
}

impl SharedStreamingState {
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            backpressured_channels: Arc::new(Mutex::new(HashSet::new())),
            flushing_channels: Arc::new(Mutex::new(HashSet::new())),
            processing_channels: Arc::new(Mutex::new(HashSet::new())),
            active_channels: Arc::new(Mutex::new(HashMap::new())),
            organization_channels: Arc::new(Mutex::new(HashMap::new())),
            channel_organizations: Arc::new(Mutex::new(HashMap::new())),
            processing_queue: Arc::new(Mutex::new(VecDeque::new())),
        })
    }

    pub async fn register_channel(
        &self,
        channel_name: &str,
        organization_id: &str,
        token_bucket: Arc<TokenBucket>,
        _capacity: usize,
    ) -> Arc<TokenBucket> {
        {
            let mut channels = self.active_channels.lock().await;

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

    pub async fn mark_backpressured(&self, channel_name: &str) {
        let mut backpressured = self.backpressured_channels.lock().await;
        backpressured.insert(channel_name.to_string());
    }

    pub async fn remove_backpressured(&self, channel_name: &str) {
        let mut backpressured = self.backpressured_channels.lock().await;
        backpressured.remove(channel_name);
    }

    pub async fn is_backpressured(&self, channel_name: &str) -> bool {
        let backpressured = self.backpressured_channels.lock().await;
        backpressured.contains(channel_name)
    }

    pub async fn mark_flushing(&self, channel_name: &str) {
        let mut flushing = self.flushing_channels.lock().await;
        flushing.insert(channel_name.to_string());
    }

    pub async fn remove_flushing(&self, channel_name: &str) {
        let mut flushing = self.flushing_channels.lock().await;
        flushing.remove(channel_name);
    }

    pub async fn is_flushing(&self, channel_name: &str) -> bool {
        let flushing = self.flushing_channels.lock().await;
        flushing.contains(channel_name)
    }

    pub async fn get_channel_organization(&self, channel_name: &str) -> Option<String> {
        let channel_orgs = self.channel_organizations.lock().await;
        channel_orgs.get(channel_name).cloned()
    }

    pub async fn get_channel(&self, channel_name: &str) -> Option<Arc<TokenBucket>> {
        let channels = self.active_channels.lock().await;
        channels.get(channel_name).cloned()
    }

    pub async fn queue_for_processing(&self, channel_name: &str) {
        let mut queue = self.processing_queue.lock().await;

        if !queue.iter().any(|ch| ch == channel_name) {
            queue.push_back(channel_name.to_string());
        }
    }

    pub async fn dequeue_for_processing(&self) -> Option<String> {
        let mut queue = self.processing_queue.lock().await;
        queue.pop_front()
    }

    pub async fn mark_processing(&self, channel_name: &str) -> bool {
        let mut processing = self.processing_channels.lock().await;
        processing.insert(channel_name.to_string())
    }

    pub async fn remove_processing(&self, channel_name: &str) {
        let mut processing = self.processing_channels.lock().await;
        processing.remove(channel_name);
    }
}

static SHARED_STATE: std::sync::OnceLock<Arc<SharedStreamingState>> = std::sync::OnceLock::new();

pub fn get_shared_state() -> Arc<SharedStreamingState> {
    SHARED_STATE
        .get_or_init(|| SharedStreamingState::new())
        .clone()
}
