use serde::{de::DeserializeOwned, Serialize};
use std::fmt::Debug;
use std::hash::Hash;
use std::time::Duration;

use super::cache_interface::CacheInterface;
use super::in_memory_cache::InMemoryCache;
use super::redis_cache::RedisCache;

/// Enum representing the available cache types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CacheType {
    /// In-memory cache using a HashMap
    InMemory,
    /// Redis cache
    Redis,
}

/// Factory for creating cache instances
pub struct CacheFactory;

impl CacheFactory {
    /// Create a new cache instance based on the specified type
    pub fn create_cache<K, V>(
        cache_type: CacheType,
        redis_connection: Option<String>,
        ttl: Option<Duration>,
    ) -> Box<dyn CacheInterface<K, V>>
    where
        K: Eq + Hash + Clone + Debug + Send + Sync + Serialize + 'static,
        V: Clone + Send + Sync + Serialize + DeserializeOwned + 'static,
    {
        match cache_type {
            CacheType::InMemory => {
                log::info!("Using in-memory cache");
                Box::new(InMemoryCache::new())
            }
            CacheType::Redis => {
                let connection = redis_connection.unwrap_or_else(|| {
                    panic!("No Redis connection string provided, using default redis://127.0.0.1/");
                });

                log::info!("Attempting to connect to Redis at {}", connection);
                match RedisCache::new(connection.clone(), ttl) {
                    Ok(cache) => {
                        log::info!("Successfully connected to Redis at {}", connection);
                        Box::new(cache)
                    }
                    Err(e) => {
                        log::error!("Failed to connect to Redis at {}: {}", connection, e);
                        log::warn!("Falling back to in-memory cache");
                        Box::new(InMemoryCache::new())
                    }
                }
            }
        }
    }
}

/// Manager for handling cache instances
pub struct CacheManager<K, V>
where
    K: Eq + Hash + Clone + Debug + Send + Sync + Serialize + 'static,
    V: Clone + Send + Sync + Serialize + DeserializeOwned + 'static,
{
    cache: Box<dyn CacheInterface<K, V>>,
    current_type: CacheType,
}

#[allow(warnings)]
impl<K, V> CacheManager<K, V>
where
    K: Eq + Hash + Clone + Debug + Send + Sync + Serialize + 'static,
    V: Clone + Send + Sync + Serialize + DeserializeOwned + 'static,
{
    /// Create a new cache manager with the specified cache type
    pub fn new(
        cache_type: CacheType,
        redis_connection: Option<String>,
        ttl: Option<Duration>,
    ) -> Self {
        let cache = CacheFactory::create_cache(cache_type, redis_connection, ttl);
        Self {
            cache,
            current_type: cache_type,
        }
    }

    /// Get the current cache type
    pub fn cache_type(&self) -> CacheType {
        self.current_type
    }

    /// Switch to a different cache type
    pub fn switch_cache_type(
        &mut self,
        cache_type: CacheType,
        redis_connection: Option<String>,
        ttl: Option<Duration>,
    ) {
        if cache_type == self.current_type {
            return;
        }

        // Create the new cache
        let new_cache = CacheFactory::create_cache(cache_type, redis_connection, ttl);

        // Update the cache and type
        self.cache = new_cache;
        self.current_type = cache_type;
    }

    /// Get a value from the cache
    pub fn get(&self, key: &K) -> Option<V> {
        self.cache.get(key)
    }

    /// Insert a value into the cache
    pub fn insert(&self, key: K, value: V) {
        self.cache.insert(key, value);
    }

    /// Insert a value into the cache with TTL
    pub fn insert_with_ttl(&self, key: K, value: V, ttl: Duration) {
        self.cache.insert_with_ttl(key, value, ttl);
    }

    /// Remove a value from the cache
    pub fn remove(&self, key: &K) -> Option<V> {
        self.cache.remove(key)
    }

    /// Clear all entries from the cache
    pub fn clear(&self) {
        self.cache.clear();
    }

    /// Check if the cache contains a key
    pub fn contains_key(&self, key: &K) -> bool {
        self.cache.contains_key(key)
    }

    /// Get the number of entries in the cache
    pub fn len(&self) -> usize {
        self.cache.len()
    }

    /// Check if the cache is empty
    pub fn is_empty(&self) -> bool {
        self.cache.is_empty()
    }
}
