use once_cell::sync::Lazy;
use serde_json::Value;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use super::cache_config::CacheConfig;
use super::cache_factory::{CacheManager, CacheType};
use redis;

// Define a type alias for a JSON cache manager
type JsonCacheManager = CacheManager<String, Value>;

// Create a thread-safe singleton cache instance
static JSON_CACHE: Lazy<Arc<Mutex<JsonCacheManager>>> =
    Lazy::new(|| Arc::new(Mutex::new(CacheConfig::global().create_cache_manager())));

/// A thread-safe wrapper around CacheManager that provides direct method access
pub struct Cache {
    inner: Arc<Mutex<JsonCacheManager>>,
}
#[allow(warnings)]
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

    /// Insert a value into the cache with a TTL
    /// For in-memory cache, this just calls the regular insert method
    pub fn insert_with_ttl(&self, key: String, value: Value, _ttl: Duration) {
        // For in-memory cache, just use regular insert
        // For Redis, we would use the Redis-specific insert_with_ttl
        self.inner.lock().unwrap().insert_with_ttl(key, value, _ttl);
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

    pub fn remove_by_prefix(&self, prefix: &str) {
        match CacheConfig::global().cache_type {
            CacheType::Redis => {
                if let Some(conn_str) = CacheConfig::global().redis_connection.clone() {
                    if let Ok(client) = redis::Client::open(conn_str.as_str()) {
                        if let Ok(mut con) = client.get_connection() {
                            let pattern = format!("{}*", prefix);
                            let mut cursor: u64 = 0;
                            loop {
                                let res: (u64, Vec<String>) = match redis::cmd("SCAN")
                                    .arg(cursor)
                                    .arg("MATCH")
                                    .arg(&pattern)
                                    .arg("COUNT")
                                    .arg(1000)
                                    .query(&mut con)
                                {
                                    Ok(r) => r,
                                    Err(_) => break,
                                };
                                cursor = res.0;
                                let keys = res.1;
                                if !keys.is_empty() {
                                    let _: Result<(), _> =
                                        redis::cmd("DEL").arg(keys).query(&mut con);
                                }
                                if cursor == 0 {
                                    break;
                                }
                            }
                        }
                    }
                }
            }
            CacheType::InMemory => {
                let index_key = format!("{}_index", prefix);
                if let Some(v) = self.get(&index_key) {
                    if let Some(arr) = v.as_array() {
                        for k in arr {
                            if let Some(s) = k.as_str() {
                                let _ = self.remove(s);
                            }
                        }
                    }
                    let _ = self.remove(&index_key);
                }
            }
        }
    }

    pub fn add_index_key(&self, prefix: &str, key: &str) {
        if let CacheType::InMemory = CacheConfig::global().cache_type {
            let index_key = format!("{}_index", prefix);
            let mut keys: Vec<String> = self
                .get(&index_key)
                .and_then(|v| {
                    v.as_array().map(|arr| {
                        arr.iter()
                            .filter_map(|s| s.as_str().map(|s| s.to_string()))
                            .collect::<Vec<String>>()
                    })
                })
                .unwrap_or_default();
            if !keys.iter().any(|s| s == key) {
                keys.push(key.to_string());
                let json_arr = Value::Array(keys.into_iter().map(Value::String).collect());
                self.insert(index_key, json_arr);
            }
        }
    }
}

// Create a global static instance of Cache
pub static CACHE: Lazy<Cache> = Lazy::new(|| Cache::global());
