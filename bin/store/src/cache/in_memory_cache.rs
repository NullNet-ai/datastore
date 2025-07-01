use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;
use std::sync::{Arc, Mutex};

use super::cache_interface::CacheInterface;

/// In-memory cache implementation using a HashMap
pub struct InMemoryCache<K, V>
where
    K: Eq + Hash + Clone + Debug + Send + Sync,
    V: Clone + Send + Sync,
{
    cache: Arc<Mutex<HashMap<K, V>>>,
}

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
        let cache = self.cache.lock().unwrap();
        cache.get(key).cloned()
    }

    fn insert(&self, key: K, value: V) {
        let mut cache = self.cache.lock().unwrap();
        cache.insert(key, value);
    }

    fn remove(&self, key: &K) -> Option<V> {
        let mut cache = self.cache.lock().unwrap();
        cache.remove(key)
    }

    fn clear(&self) {
        let mut cache = self.cache.lock().unwrap();
        cache.clear();
    }

    fn contains_key(&self, key: &K) -> bool {
        let cache = self.cache.lock().unwrap();
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
