use std::collections::{HashMap, HashSet};
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
    
    /// Channels that are currently being processed/routed
    pub processing_channels: Arc<Mutex<HashSet<String>>>,
    
    /// Global channel registry for dashboard/monitoring access
    pub channel_registry: Arc<Mutex<HashMap<String, ChannelInfo>>>,
}

#[derive(Debug, Clone)]
pub struct ChannelInfo {
    pub name: String,
    pub organization_id: String,
    pub capacity: usize,
    pub created_at: std::time::Instant,
}

impl SharedStreamingState {
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            backpressured_channels: Arc::new(Mutex::new(HashSet::new())),
            flushing_channels: Arc::new(Mutex::new(HashSet::new())),
            active_channels: Arc::new(Mutex::new(HashMap::new())),
            organization_channels: Arc::new(Mutex::new(HashMap::new())),
            channel_organizations: Arc::new(Mutex::new(HashMap::new())),
            processing_channels: Arc::new(Mutex::new(HashSet::new())),
            channel_registry: Arc::new(Mutex::new(HashMap::new())),
        })
    }
    
    /// Register a new channel with its token bucket
    pub async fn register_channel(
        &self,
        channel_name: &str,
        organization_id: &str,
        token_bucket: Arc<TokenBucket>,
        capacity: usize,
    ) {
        // Add to active channels
        {
            let mut channels = self.active_channels.lock().await;
            channels.insert(channel_name.to_string(), token_bucket);
        }
        
        // Add to organization mapping
        {
            let mut org_channels = self.organization_channels.lock().await;
            org_channels
                .entry(organization_id.to_string())
                .or_insert_with(HashSet::new)
                .insert(channel_name.to_string());
        }
        
        // Add reverse mapping
        {
            let mut channel_orgs = self.channel_organizations.lock().await;
            channel_orgs.insert(channel_name.to_string(), organization_id.to_string());
        }
        
        // Add to registry
        {
            let mut registry = self.channel_registry.lock().await;
            registry.insert(channel_name.to_string(), ChannelInfo {
                name: channel_name.to_string(),
                organization_id: organization_id.to_string(),
                capacity,
                created_at: std::time::Instant::now(),
            });
        }
    }
    
    /// Unregister a channel and clean up all related state
    pub async fn unregister_channel(&self, channel_name: &str) {
        // Get organization_id before removing
        let organization_id = {
            let channel_orgs = self.channel_organizations.lock().await;
            channel_orgs.get(channel_name).cloned()
        };
        
        // Remove from active channels
        {
            let mut channels = self.active_channels.lock().await;
            channels.remove(channel_name);
        }
        
        // Remove from backpressured set
        {
            let mut backpressured = self.backpressured_channels.lock().await;
            backpressured.remove(channel_name);
        }
        
        // Remove from flushing set
        {
            let mut flushing = self.flushing_channels.lock().await;
            flushing.remove(channel_name);
        }
        
        // Remove from processing set
        {
            let mut processing = self.processing_channels.lock().await;
            processing.remove(channel_name);
        }
        
        // Remove from organization mapping
        if let Some(org_id) = &organization_id {
            let mut org_channels = self.organization_channels.lock().await;
            if let Some(channels) = org_channels.get_mut(org_id) {
                channels.remove(channel_name);
                if channels.is_empty() {
                    org_channels.remove(org_id);
                }
            }
        }
        
        // Remove reverse mapping
        {
            let mut channel_orgs = self.channel_organizations.lock().await;
            channel_orgs.remove(channel_name);
        }
        
        // Remove from registry
        {
            let mut registry = self.channel_registry.lock().await;
            registry.remove(channel_name);
        }
    }
    
    /// Mark a channel as backpressured
    pub async fn mark_backpressured(&self, channel_name: &str) {
        let mut backpressured = self.backpressured_channels.lock().await;
        backpressured.insert(channel_name.to_string());
    }
    
    /// Remove backpressure status from a channel
    pub async fn clear_backpressured(&self, channel_name: &str) {
        let mut backpressured = self.backpressured_channels.lock().await;
        backpressured.remove(channel_name);
    }
    
    /// Check if a channel is backpressured
    pub async fn is_backpressured(&self, channel_name: &str) -> bool {
        let backpressured = self.backpressured_channels.lock().await;
        backpressured.contains(channel_name)
    }
    
    /// Mark a channel as flushing
    pub async fn mark_flushing(&self, channel_name: &str) -> bool {
        let mut flushing = self.flushing_channels.lock().await;
        if flushing.contains(channel_name) {
            false // Already flushing
        } else {
            flushing.insert(channel_name.to_string());
            true // Successfully marked as flushing
        }
    }
    
    /// Remove flushing status from a channel
    pub async fn clear_flushing(&self, channel_name: &str) {
        let mut flushing = self.flushing_channels.lock().await;
        flushing.remove(channel_name);
    }
    
    /// Check if a channel is flushing
    pub async fn is_flushing(&self, channel_name: &str) -> bool {
        let flushing = self.flushing_channels.lock().await;
        flushing.contains(channel_name)
    }
    
    /// Check if a channel should queue messages (flushing or backpressured)
    pub async fn should_queue_message(&self, channel_name: &str) -> bool {
        let flushing = self.flushing_channels.lock().await;
        let backpressured = self.backpressured_channels.lock().await;
        flushing.contains(channel_name) || backpressured.contains(channel_name)
    }
    
    /// Get a channel's token bucket
    pub async fn get_channel(&self, channel_name: &str) -> Option<Arc<TokenBucket>> {
        let channels = self.active_channels.lock().await;
        channels.get(channel_name).cloned()
    }
    
    /// Get all active channel names
    pub async fn get_active_channel_names(&self) -> Vec<String> {
        let channels = self.active_channels.lock().await;
        channels.keys().cloned().collect()
    }
    
    /// Get channels for an organization
    pub async fn get_organization_channels(&self, organization_id: &str) -> Vec<String> {
        let org_channels = self.organization_channels.lock().await;
        org_channels
            .get(organization_id)
            .map(|channels| channels.iter().cloned().collect())
            .unwrap_or_default()
    }
    
    /// Get organization for a channel
    pub async fn get_channel_organization(&self, channel_name: &str) -> Option<String> {
        let channel_orgs = self.channel_organizations.lock().await;
        channel_orgs.get(channel_name).cloned()
    }
    
    /// Mark a channel as being processed
    pub async fn mark_processing(&self, channel_name: &str) -> bool {
        let mut processing = self.processing_channels.lock().await;
        if processing.contains(channel_name) {
            false // Already processing
        } else {
            processing.insert(channel_name.to_string());
            true // Successfully marked as processing
        }
    }
    
    /// Remove processing status from a channel
    pub async fn clear_processing(&self, channel_name: &str) {
        let mut processing = self.processing_channels.lock().await;
        processing.remove(channel_name);
    }
    
    /// Get comprehensive status for a channel
    pub async fn get_channel_status(&self, channel_name: &str) -> Option<ChannelStatus> {
        let channels = self.active_channels.lock().await;
        let flushing = self.flushing_channels.lock().await;
        let backpressured = self.backpressured_channels.lock().await;
        let processing = self.processing_channels.lock().await;
        let registry = self.channel_registry.lock().await;
        
        if let Some(bucket) = channels.get(channel_name) {
            let info = registry.get(channel_name);
            Some(ChannelStatus {
                channel_name: channel_name.to_string(),
                organization_id: info.map(|i| i.organization_id.clone()).unwrap_or_default(),
                tokens_remaining: bucket.get_tokens_remaining().await,
                high_watermark: bucket.get_high_watermark().await,
                capacity: info.map(|i| i.capacity).unwrap_or(0),
                is_flushing: flushing.contains(channel_name),
                is_backpressured: backpressured.contains(channel_name),
                is_processing: processing.contains(channel_name),
                created_at: info.map(|i| i.created_at).unwrap_or_else(std::time::Instant::now),
            })
        } else {
            None
        }
    }
    
    /// Get status for all channels
    pub async fn get_all_channel_statuses(&self) -> Vec<ChannelStatus> {
        let channel_names = self.get_active_channel_names().await;
        let mut statuses = Vec::new();
        
        for channel_name in channel_names {
            if let Some(status) = self.get_channel_status(&channel_name).await {
                statuses.push(status);
            }
        }
        
        statuses
    }
}

#[derive(Debug, Clone)]
pub struct ChannelStatus {
    pub channel_name: String,
    pub organization_id: String,
    pub tokens_remaining: usize,
    pub high_watermark: usize,
    pub capacity: usize,
    pub is_flushing: bool,
    pub is_backpressured: bool,
    pub is_processing: bool,
    pub created_at: std::time::Instant,
}

/// Global instance for shared state
static SHARED_STATE: std::sync::OnceLock<Arc<SharedStreamingState>> = std::sync::OnceLock::new();

/// Get the global shared state instance
pub fn get_shared_state() -> Arc<SharedStreamingState> {
    SHARED_STATE
        .get_or_init(|| SharedStreamingState::new())
        .clone()
}

/// Initialize the shared state (call this once at startup)
pub fn initialize_shared_state() -> Arc<SharedStreamingState> {
    get_shared_state()
}