use std::fmt::Debug;
use std::hash::Hash;

/// Generic interface for cache implementations
pub trait CacheInterface<K, V>: Send + Sync
where
    K: Eq + Hash + Clone + Debug + Send + Sync,
    V: Clone + Send + Sync,
{
    /// Get a value from the cache
    fn get(&self, key: &K) -> Option<V>;

    /// Insert a value into the cache
    fn insert(&self, key: K, value: V);

    /// Remove a value from the cache
    fn remove(&self, key: &K) -> Option<V>;

    /// Clear all entries from the cache
    fn clear(&self);

    /// Check if the cache contains a key
    fn contains_key(&self, key: &K) -> bool;

    /// Get the number of entries in the cache
    fn len(&self) -> usize;

    /// Check if the cache is empty
    fn is_empty(&self) -> bool;
}
