use std::sync::OnceLock;
use std::time::Duration;

use super::cache_factory::{CacheManager, CacheType};

/// Global cache configuration
static CACHE_CONFIG: OnceLock<CacheConfig> = OnceLock::new();

/// Cache configuration
#[derive(Debug, Clone)]
pub struct CacheConfig {
    /// The type of cache to use
    pub cache_type: CacheType,
    /// The Redis connection string (if using Redis)
    pub redis_connection: Option<String>,
    /// The default TTL for cache entries
    pub ttl: Option<Duration>,
}

impl CacheConfig {
    /// Create a new cache configuration
    pub fn new(
        cache_type: CacheType,
        redis_connection: Option<String>,
        ttl: Option<Duration>,
    ) -> Self {
        Self {
            cache_type,
            redis_connection,
            ttl,
        }
    }

    /// Initialize the global cache configuration
    pub fn init(cache_type: CacheType, redis_connection: Option<String>, ttl: Option<Duration>) {
        let _ = CACHE_CONFIG.set(Self::new(cache_type, redis_connection, ttl));
    }

    /// Get the global cache configuration
    pub fn global() -> &'static CacheConfig {
        CACHE_CONFIG
            .get()
            .expect("Cache configuration not initialized")
    }

    /// Create a cache manager using the current configuration
    pub fn create_cache_manager<K, V>(&self) -> CacheManager<K, V>
    where
        K: std::hash::Hash
            + Eq
            + Clone
            + std::fmt::Debug
            + Send
            + Sync
            + serde::Serialize
            + 'static,
        V: Clone + Send + Sync + serde::Serialize + serde::de::DeserializeOwned + 'static,
    {
        CacheManager::new(self.cache_type, self.redis_connection.clone(), self.ttl)
    }
}
