use std::sync::{Arc, Mutex};
use serde::{de::DeserializeOwned, Serialize};
use serde_json::Value;
use once_cell::sync::Lazy;

use super::cache_config::CacheConfig;
use super::cache_factory::{CacheManager, CacheType};

// Define a type alias for a JSON cache manager
type JsonCacheManager = CacheManager<String, Value>;

// Create a thread-safe singleton cache instance
static JSON_CACHE: Lazy<Arc<Mutex<JsonCacheManager>>> = Lazy::new(|| {
    Arc::new(Mutex::new(CacheConfig::global().create_cache_manager()))
});

/// A thread-safe wrapper around CacheManager that provides direct method access
pub struct Cache {
    inner: Arc<Mutex<JsonCacheManager>>,
}

impl Cache {
    /// Get the global cache instance
    pub fn global() -> Self {
        Self {
            inner: JSON_CACHE.clone(),
        }
    }

    /// Get the current cache type
    pub fn cache_type(&self) -> CacheType {
        self.inner.lock().unwrap().cache_type()
    }

    /// Insert a value into the cache
    pub fn insert(&self, key: String, value: Value) {
        self.inner.lock().unwrap().insert(key, value);
    }

    pub fn get(&self, key: &str) -> Option<Value> {
        let string_key = key.to_string();
        self.inner.lock().unwrap().get(&string_key)
    }

    /// Remove a value from the cache
    pub fn remove(&self, key: &str) -> Option<Value> {
        let string_key = key.to_string();
        self.inner.lock().unwrap().remove(&string_key)
    }

    /// Clear all entries from the cache
    pub fn clear(&self) {
        self.inner.lock().unwrap().clear();
    }

    /// Check if the cache contains a key
    pub fn contains_key(&self, key: &str) -> bool {
        // Convert &str to &String by creating a temporary String
        let string_key = key.to_string();
        self.inner.lock().unwrap().contains_key(&string_key)
    }

    /// Get the number of entries in the cache
    pub fn len(&self) -> usize {
        self.inner.lock().unwrap().len()
    }

    /// Check if the cache is empty
    pub fn is_empty(&self) -> bool {
        self.inner.lock().unwrap().is_empty()
    }
}

// Create a global static instance of Cache
pub static CACHE: Lazy<Cache> = Lazy::new(|| Cache::global());