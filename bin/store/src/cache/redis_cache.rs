use super::cache_interface::CacheInterface;
use redis::{Client, Commands, RedisResult};
use serde::{de::DeserializeOwned, Serialize};
use std::fmt::Debug;
use std::hash::Hash;
use std::marker::PhantomData;
use std::sync::{Arc, Mutex};
use std::time::Duration;

/// Redis cache implementation
pub struct RedisCache<K, V>
where
    K: Eq + Hash + Clone + Debug + Send + Sync + Serialize,
    V: Clone + Send + Sync + Serialize + DeserializeOwned,
{
    client: Arc<Mutex<Client>>,
    default_ttl: Option<Duration>,
    _marker: PhantomData<(K, V)>,
}

impl<K, V> RedisCache<K, V>
where
    K: Eq + Hash + Clone + Debug + Send + Sync + Serialize,
    V: Clone + Send + Sync + Serialize + DeserializeOwned,
{
    /// Create a new Redis cache with the specified connection string
    pub fn new(
        connection_string: String,
        default_ttl: Option<Duration>,
    ) -> Result<Self, redis::RedisError> {
        let client = Client::open(connection_string)?;

        // Test the connection
        let mut con = client.get_connection()?;
        redis::cmd("PING").query::<String>(&mut con)?;

        log::info!("Successfully connected to Redis");

        Ok(Self {
            client: Arc::new(Mutex::new(client)),
            default_ttl,
            _marker: PhantomData,
        })
    }

    /// Get the default TTL
    pub fn default_ttl(&self) -> Option<Duration> {
        self.default_ttl
    }

    /// Set the default TTL
    pub fn set_default_ttl(&mut self, ttl: Option<Duration>) {
        self.default_ttl = ttl;
    }

    /// Insert a value with a specific TTL
    pub fn insert_with_ttl(
        &self,
        key: K,
        value: V,
        ttl: Duration,
    ) -> Result<(), redis::RedisError> {
        let client = self.client.lock().unwrap();
        let mut con = client.get_connection()?;

        let key_str = serde_json::to_string(&key).unwrap();
        let value_str = serde_json::to_string(&value).unwrap();

        con.set_ex(key_str, value_str, ttl.as_secs() as usize)?;
        Ok(())
    }
}

impl<K, V> CacheInterface<K, V> for RedisCache<K, V>
where
    K: Eq + Hash + Clone + Debug + Send + Sync + Serialize,
    V: Clone + Send + Sync + Serialize + DeserializeOwned,
{
    fn get(&self, key: &K) -> Option<V> {
        let client = self.client.lock().unwrap();
        if let Ok(mut con) = client.get_connection() {
            let key_str = serde_json::to_string(key).unwrap();
            if let Ok(value_str) = con.get::<_, String>(key_str) {
                if let Ok(value) = serde_json::from_str(&value_str) {
                    return Some(value);
                }
            }
        }
        None
    }

    fn insert(&self, key: K, value: V) {
        let client = self.client.lock().unwrap();
        if let Ok(mut con) = client.get_connection() {
            let key_str = serde_json::to_string(&key).unwrap();
            let value_str = serde_json::to_string(&value).unwrap();

            if let Some(ttl) = self.default_ttl {
                let _ = con.set_ex::<_, _, ()>(key_str, value_str, ttl.as_secs() as usize);
            } else {
                let _ = con.set::<_, _, ()>(key_str, value_str);
            }
        }
    }

    fn remove(&self, key: &K) -> Option<V> {
        let client = self.client.lock().unwrap();
        if let Ok(mut con) = client.get_connection() {
            let key_str = serde_json::to_string(key).unwrap();

            // Get the value first
            let value = self.get(key);

            // Then delete the key
            let _ = con.del::<_, ()>(key_str);

            return value;
        }
        None
    }

    fn clear(&self) {
        let client = self.client.lock().unwrap();
        if let Ok(mut con) = client.get_connection() {
            let _ = redis::cmd("FLUSHDB").query::<()>(&mut con);
        }
    }

    fn contains_key(&self, key: &K) -> bool {
        let client = self.client.lock().unwrap();
        if let Ok(mut con) = client.get_connection() {
            let key_str = serde_json::to_string(key).unwrap();
            if let Ok(exists) = con.exists::<_, bool>(key_str) {
                return exists;
            }
        }
        false
    }

    fn len(&self) -> usize {
        let client = self.client.lock().unwrap();
        if let Ok(mut con) = client.get_connection() {
            if let Ok(size) = redis::cmd("DBSIZE").query::<usize>(&mut con) {
                return size;
            }
        }
        0
    }

    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}
