use super::cache_interface::CacheInterface;
use super::connection_pool::{ConnectionPool, ConnectionPoolConfig, ConnectionPoolError};
use redis::Commands;
use serde::{de::DeserializeOwned, Serialize};
use std::fmt::Debug;
use std::hash::Hash;
use std::marker::PhantomData;
use std::sync::Arc;
use std::time::Duration;
use thiserror::Error;

/// Redis cache implementation
#[derive(Debug)]
pub struct RedisCache<K, V>
where
    K: Eq + Hash + Clone + Debug + Send + Sync + Serialize,
    V: Clone + Send + Sync + Serialize + DeserializeOwned,
{
    connection_pool: Arc<ConnectionPool>,
    default_ttl: Option<Duration>,
    _marker: PhantomData<(K, V)>,
}

/// Custom error type for Redis cache operations
#[derive(Error, Debug)]
pub enum RedisCacheError {
    #[error("Redis connection error: {0}")]
    ConnectionError(#[from] redis::RedisError),
    
    #[error("Connection pool error: {0}")]
    PoolError(#[from] ConnectionPoolError),
    
    #[error("Serialization error: {0}")]
    SerializationError(String),
}
#[allow(warnings)]
impl<K, V> RedisCache<K, V>
where
    K: Eq + Hash + Clone + Debug + Send + Sync + Serialize,
    V: Clone + Send + Sync + Serialize + DeserializeOwned,
{
    /// Create a new Redis cache with the specified connection string
    pub fn new(
        connection_string: String,
        default_ttl: Option<Duration>,
    ) -> Result<Self, RedisCacheError> {
        use redis::Client;
        
        let client = Arc::new(Client::open(connection_string)?);
        let pool_config = ConnectionPoolConfig::default();
        let connection_pool = Arc::new(ConnectionPool::new(client, pool_config)?);

        log::info!("Successfully created Redis connection pool");
        Ok(Self {
            connection_pool,
            default_ttl,
            _marker: PhantomData,
        })
    }

    /// Get connection pool statistics
    pub fn get_pool_stats(&self) -> super::connection_pool::ConnectionPoolStats {
        self.connection_pool.get_stats()
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
    ) -> Result<(), RedisCacheError> {
        let mut pooled_conn = self.connection_pool.get_connection()?;
        let conn = pooled_conn.connection();

        let key_str = serde_json::to_string(&key)
            .map_err(|e| RedisCacheError::SerializationError(format!("Failed to serialize key: {}", e)))?;
        let value_str = serde_json::to_string(&value)
            .map_err(|e| RedisCacheError::SerializationError(format!("Failed to serialize value: {}", e)))?;

        conn.set_ex::<_, _, ()>(key_str, value_str, ttl.as_secs() as usize)?;
        Ok(())
    }
}

impl<K, V> CacheInterface<K, V> for RedisCache<K, V>
where
    K: Eq + Hash + Clone + Debug + Send + Sync + Serialize,
    V: Clone + Send + Sync + Serialize + DeserializeOwned,
{
    fn get(&self, key: &K) -> Option<V> {
        let mut pooled_conn = match self.connection_pool.get_connection() {
            Ok(conn) => conn,
            Err(e) => {
                log::error!("Failed to get Redis connection: {}", e);
                return None;
            }
        };
        let conn = pooled_conn.connection();

        let key_str = match serde_json::to_string(key) {
            Ok(s) => s,
            Err(e) => {
                log::error!("Failed to serialize key: {}", e);
                return None;
            }
        };
        
        match conn.get::<_, Option<String>>(key_str) {
            Ok(Some(value_str)) => {
                match serde_json::from_str(&value_str) {
                    Ok(value) => Some(value),
                    Err(e) => {
                        log::error!("Failed to deserialize value: {}", e);
                        None
                    }
                }
            }
            Ok(None) => None,
            Err(e) => {
                log::debug!("Redis GET failed: {}", e);
                None
            }
        }
    }

    fn insert(&self, key: K, value: V) {
        let mut pooled_conn = match self.connection_pool.get_connection() {
            Ok(conn) => conn,
            Err(e) => {
                log::error!("Failed to get Redis connection for insert: {}", e);
                return;
            }
        };
        let conn = pooled_conn.connection();

        let key_str = match serde_json::to_string(&key) {
            Ok(s) => s,
            Err(e) => {
                log::error!("Failed to serialize key for insert: {}", e);
                return;
            }
        };
        
        let value_str = match serde_json::to_string(&value) {
            Ok(s) => s,
            Err(e) => {
                log::error!("Failed to serialize value for insert: {}", e);
                return;
            }
        };

        let result = if let Some(ttl) = self.default_ttl {
            conn.set_ex::<_, _, ()>(key_str, value_str, ttl.as_secs() as usize)
        } else {
            conn.set::<_, _, ()>(key_str, value_str)
        };

        if let Err(e) = result {
            log::error!("Failed to insert into Redis: {}", e);
        }
    }

    fn insert_with_ttl(&self, key: K, value: V, ttl: Duration) {
        let mut pooled_conn = match self.connection_pool.get_connection() {
            Ok(conn) => conn,
            Err(e) => {
                log::error!("Failed to get Redis connection for insert_with_ttl: {}", e);
                return;
            }
        };
        let conn = pooled_conn.connection();

        let key_str = match serde_json::to_string(&key) {
            Ok(s) => s,
            Err(e) => {
                log::error!("Failed to serialize key for insert_with_ttl: {}", e);
                return;
            }
        };
        
        let value_str = match serde_json::to_string(&value) {
            Ok(s) => s,
            Err(e) => {
                log::error!("Failed to serialize value for insert_with_ttl: {}", e);
                return;
            }
        };

        if let Err(e) = conn.set_ex::<_, _, ()>(key_str, value_str, ttl.as_secs() as usize) {
            log::error!("Failed to insert with TTL into Redis: {}", e);
        }
    }

    fn remove(&self, key: &K) -> Option<V> {
        let mut pooled_conn = match self.connection_pool.get_connection() {
            Ok(conn) => conn,
            Err(e) => {
                log::error!("Failed to get Redis connection for removal: {}", e);
                return None;
            }
        };
        let conn = pooled_conn.connection();

        let key_str = match serde_json::to_string(key) {
            Ok(s) => s,
            Err(e) => {
                log::error!("Failed to serialize key for removal: {}", e);
                return None;
            }
        };

        // Get the value first in the same connection to ensure consistency
        let value = match conn.get::<_, Option<String>>(&key_str) {
            Ok(Some(value_str)) => {
                match serde_json::from_str(&value_str) {
                    Ok(value) => Some(value),
                    Err(e) => {
                        log::error!("Failed to deserialize value during removal: {}", e);
                        None
                    }
                }
            }
            Ok(None) => None,
            Err(e) => {
                log::debug!("Redis GET during removal failed: {}", e);
                None
            }
        };

        // Delete the key if it existed
        if value.is_some() {
            if let Err(e) = conn.del::<_, ()>(key_str) {
                log::error!("Failed to delete key from Redis: {}", e);
            }
        }
        
        value
    }

    fn clear(&self) {
        let mut pooled_conn = match self.connection_pool.get_connection() {
            Ok(conn) => conn,
            Err(e) => {
                log::error!("Failed to get Redis connection for clear: {}", e);
                return;
            }
        };
        let conn = pooled_conn.connection();
        
        // Use FLUSHDB with ASYNC option for better performance and reliability
        match redis::cmd("FLUSHDB").arg("ASYNC").query::<()>(conn) {
            Ok(_) => {
                log::debug!("Redis database cleared successfully");
            }
            Err(e) => {
                log::error!("Failed to clear Redis database: {}", e);
                
                // Fallback: try to clear synchronously
                if let Err(e2) = redis::cmd("FLUSHDB").query::<()>(conn) {
                    log::error!("Fallback clear also failed: {}", e2);
                }
            }
        }
    }

    fn contains_key(&self, key: &K) -> bool {
        let mut pooled_conn = match self.connection_pool.get_connection() {
            Ok(conn) => conn,
            Err(e) => {
                log::error!("Failed to get Redis connection for contains_key: {}", e);
                return false;
            }
        };
        let conn = pooled_conn.connection();

        let key_str = match serde_json::to_string(key) {
            Ok(s) => s,
            Err(e) => {
                log::error!("Failed to serialize key for contains_key: {}", e);
                return false;
            }
        };
        
        match conn.exists::<_, bool>(key_str) {
            Ok(exists) => exists,
            Err(e) => {
                log::debug!("Redis EXISTS failed: {}", e);
                false
            }
        }
    }

    fn len(&self) -> usize {
        let mut pooled_conn = match self.connection_pool.get_connection() {
            Ok(conn) => conn,
            Err(e) => {
                log::error!("Failed to get Redis connection for len: {}", e);
                return 0;
            }
        };
        let conn = pooled_conn.connection();
        
        match redis::cmd("DBSIZE").query::<usize>(conn) {
            Ok(size) => size,
            Err(e) => {
                log::debug!("Redis DBSIZE failed: {}", e);
                0
            }
        }
    }

    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}
