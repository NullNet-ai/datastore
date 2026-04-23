use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use super::cache_interface::CacheInterface;

/// In-memory cache implementation using a HashMap
pub struct InMemoryCache<K, V>
where
    K: Eq + Hash + Clone + Debug + Send + Sync,
    V: Clone + Send + Sync,
{
    cache: Arc<Mutex<HashMap<K, (V, Option<Instant>)>>>,
}

#[allow(warnings)]
impl<K, V> InMemoryCache<K, V>
where
    K: Eq + Hash + Clone + Debug + Send + Sync,
    V: Clone + Send + Sync,
{
    /// Create a new in-memory cache
    pub fn new() -> Self {
        Self {
            cache: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    fn purge_if_expired_locked(cache: &mut HashMap<K, (V, Option<Instant>)>, key: &K) -> bool {
        if let Some((_, exp)) = cache.get(key) {
            if let Some(t) = exp {
                if Instant::now() >= *t {
                    cache.remove(key);
                    return true;
                }
            }
        }
        false
    }
}

impl<K, V> Default for InMemoryCache<K, V>
where
    K: Eq + Hash + Clone + Debug + Send + Sync,
    V: Clone + Send + Sync,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<K, V> CacheInterface<K, V> for InMemoryCache<K, V>
where
    K: Eq + Hash + Clone + Debug + Send + Sync,
    V: Clone + Send + Sync,
{
    fn get(&self, key: &K) -> Option<V> {
        let mut cache = self.cache.lock().unwrap();
        if Self::purge_if_expired_locked(&mut cache, key) {
            return None;
        }
        cache.get(key).map(|(v, _)| v.clone())
    }

    fn insert(&self, key: K, value: V) {
        let mut cache = self.cache.lock().unwrap();
        cache.insert(key, (value, None));
    }

    fn insert_with_ttl(&self, key: K, value: V, ttl: Duration) {
        let mut cache = self.cache.lock().unwrap();
        let expires_at = Instant::now() + ttl;
        cache.insert(key, (value, Some(expires_at)));
    }

    fn remove(&self, key: &K) -> Option<V> {
        let mut cache = self.cache.lock().unwrap();
        cache.remove(key).map(|(v, _)| v)
    }

    fn clear(&self) {
        let mut cache = self.cache.lock().unwrap();
        cache.clear();
    }

    fn contains_key(&self, key: &K) -> bool {
        let mut cache = self.cache.lock().unwrap();
        if Self::purge_if_expired_locked(&mut cache, key) {
            return false;
        }
        cache.contains_key(key)
    }

    fn len(&self) -> usize {
        let cache = self.cache.lock().unwrap();
        cache.len()
    }

    fn is_empty(&self) -> bool {
        let cache = self.cache.lock().unwrap();
        cache.is_empty()
    }
}
