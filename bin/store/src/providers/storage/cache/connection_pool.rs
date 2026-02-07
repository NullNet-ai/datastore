use redis::{Client, Connection, RedisError};
use std::collections::VecDeque;
use std::sync::{Arc, Condvar, Mutex};
use std::time::{Duration, Instant};
use thiserror::Error;

/// Connection pool configuration
#[derive(Debug, Clone)]
pub struct ConnectionPoolConfig {
    /// Maximum number of connections in the pool
    pub max_connections: usize,
    /// Minimum number of connections to maintain
    pub min_connections: usize,
    /// Connection timeout
    pub connection_timeout: Duration,
    /// Idle timeout before connection is closed (currently unused)
    #[allow(dead_code)]
    pub idle_timeout: Duration,
    /// Maximum lifetime of a connection (currently unused)
    #[allow(dead_code)]
    pub max_lifetime: Duration,
}

impl Default for ConnectionPoolConfig {
    fn default() -> Self {
        Self {
            max_connections: 10,
            min_connections: 2,
            connection_timeout: Duration::from_secs(30),
            idle_timeout: Duration::from_secs(300), // 5 minutes
            max_lifetime: Duration::from_secs(3600), // 1 hour
        }
    }
}

/// Connection pool statistics
#[derive(Debug, Clone)]
pub struct ConnectionPoolStats {
    #[allow(dead_code)]
    pub total_connections: usize,
    #[allow(dead_code)]
    pub idle_connections: usize,
    #[allow(dead_code)]
    pub active_connections: usize,
    #[allow(dead_code)]
    pub waiting_for_connection: usize,
}

/// Connection pool error types
#[derive(Error, Debug)]
pub enum ConnectionPoolError {
    #[error("Redis connection error: {0}")]
    ConnectionError(#[from] RedisError),

    #[error("Connection pool is at maximum capacity")]
    #[allow(dead_code)]
    PoolAtCapacity,

    #[error("Connection timeout after {0:?}")]
    ConnectionTimeout(Duration),

    #[error("Connection is too old (lifetime exceeded)")]
    #[allow(dead_code)]
    ConnectionTooOld,

    #[error("Connection is idle for too long")]
    #[allow(dead_code)]
    ConnectionIdleTooLong,
}

/// Pooled connection wrapper
pub struct PooledConnection {
    connection: Option<Connection>,
    pool: Arc<ConnectionPool>,
    #[allow(dead_code)]
    created_at: Instant,
    last_used: Instant,
}

impl PooledConnection {
    fn new(connection: Connection, pool: Arc<ConnectionPool>) -> Self {
        let now = Instant::now();
        Self {
            connection: Some(connection),
            pool,
            created_at: now,
            last_used: now,
        }
    }

    /// Get the underlying Redis connection
    pub fn connection(&mut self) -> &mut Connection {
        self.last_used = Instant::now();
        self.connection.as_mut().expect("Connection should exist")
    }

    /// Check if this connection is still valid (currently unused)
    #[allow(dead_code)]
    fn is_valid(&self, config: &ConnectionPoolConfig) -> bool {
        let now = Instant::now();

        // Check if connection is too old
        if now.duration_since(self.created_at) > config.max_lifetime {
            return false;
        }

        // Check if connection has been idle too long
        if now.duration_since(self.last_used) > config.idle_timeout {
            return false;
        }

        true
    }
}

impl Drop for PooledConnection {
    fn drop(&mut self) {
        if let Some(connection) = self.connection.take() {
            self.pool.return_connection(connection);
        }
    }
}

/// Thread-safe Redis connection pool
pub struct ConnectionPool {
    client: Arc<Client>,
    config: ConnectionPoolConfig,
    idle_connections: Arc<Mutex<VecDeque<Connection>>>,
    active_count: Arc<Mutex<usize>>,
    condition: Arc<Condvar>,
}

impl ConnectionPool {
    /// Create a new connection pool
    pub fn new(
        client: Arc<Client>,
        config: ConnectionPoolConfig,
    ) -> Result<Self, ConnectionPoolError> {
        let pool = Self {
            client,
            config: config.clone(),
            idle_connections: Arc::new(Mutex::new(VecDeque::new())),
            active_count: Arc::new(Mutex::new(0)),
            condition: Arc::new(Condvar::new()),
        };

        // Pre-create minimum connections
        for _ in 0..config.min_connections {
            match pool.create_connection() {
                Ok(conn) => {
                    pool.idle_connections.lock().unwrap().push_back(conn);
                }
                Err(e) => {
                    log::warn!("Failed to create initial connection: {}", e);
                }
            }
        }

        Ok(pool)
    }

    /// Get a connection from the pool
    pub fn get_connection(self: &Arc<Self>) -> Result<PooledConnection, ConnectionPoolError> {
        let start_time = Instant::now();

        loop {
            // Try to get an idle connection
            {
                let mut idle_conns = self.idle_connections.lock().unwrap();
                if let Some(mut conn) = idle_conns.pop_front() {
                    // Validate the connection
                    if self.validate_connection(&mut conn) {
                        *self.active_count.lock().unwrap() += 1;
                        return Ok(PooledConnection::new(conn, Arc::clone(self)));
                    }
                }
            }

            // Check if we can create a new connection
            {
                let active_count = *self.active_count.lock().unwrap();
                if active_count < self.config.max_connections {
                    match self.create_connection() {
                        Ok(conn) => {
                            *self.active_count.lock().unwrap() += 1;
                            return Ok(PooledConnection::new(conn, Arc::clone(self)));
                        }
                        Err(e) => {
                            log::warn!("Failed to create new connection: {}", e);
                        }
                    }
                }
            }

            // Check timeout
            if Instant::now().duration_since(start_time) > self.config.connection_timeout {
                return Err(ConnectionPoolError::ConnectionTimeout(
                    self.config.connection_timeout,
                ));
            }

            // Wait for a connection to become available
            let active_count = self.active_count.lock().unwrap();
            let (_guard, _timeout_result) = self
                .condition
                .wait_timeout(active_count, Duration::from_millis(100))
                .unwrap();
        }
    }

    /// Return a connection to the pool
    fn return_connection(&self, mut connection: Connection) {
        *self.active_count.lock().unwrap() -= 1;

        // Validate connection before returning to pool
        if self.validate_connection(&mut connection) {
            self.idle_connections.lock().unwrap().push_back(connection);
        }

        // Notify waiting threads
        self.condition.notify_one();
    }

    /// Create a new Redis connection
    fn create_connection(&self) -> Result<Connection, ConnectionPoolError> {
        let mut conn = self.client.get_connection()?;

        // Test the connection with multiple retries
        let mut retries = 3;
        while retries > 0 {
            match redis::cmd("PING").query::<String>(&mut conn) {
                Ok(response) if response == "PONG" => {
                    log::trace!("New connection created and validated successfully");
                    return Ok(conn);
                }
                Ok(response) => {
                    log::warn!(
                        "Connection test failed with unexpected response: '{}'",
                        response
                    );
                    if retries == 1 {
                        return Err(ConnectionPoolError::ConnectionError(
                            redis::RedisError::from((
                                redis::ErrorKind::ResponseError,
                                "Invalid PING response",
                            )),
                        ));
                    }
                }
                Err(e) => {
                    log::warn!(
                        "Connection test failed: {}, retries left: {}",
                        e,
                        retries - 1
                    );
                    if retries == 1 {
                        return Err(e.into());
                    }
                }
            }
            retries -= 1;
            std::thread::sleep(Duration::from_millis(100));
        }

        // This should not be reached, but just in case
        Err(ConnectionPoolError::ConnectionError(
            redis::RedisError::from((
                redis::ErrorKind::ResponseError,
                "Connection creation failed after retries",
            )),
        ))
    }

    /// Validate a connection is still working
    fn validate_connection(&self, connection: &mut Connection) -> bool {
        // Try PING command with timeout protection
        match redis::cmd("PING").query::<String>(connection) {
            Ok(response) if response == "PONG" => {
                log::trace!("Connection validation successful");
                true
            }
            Ok(response) => {
                log::warn!(
                    "Connection validation failed: unexpected response '{}'",
                    response
                );
                false
            }
            Err(e) => {
                log::debug!("Connection validation failed: {}", e);
                false
            }
        }
    }

    /// Get pool statistics
    pub fn get_stats(&self) -> ConnectionPoolStats {
        let idle_conns = self.idle_connections.lock().unwrap();
        let active_count = *self.active_count.lock().unwrap();

        ConnectionPoolStats {
            total_connections: idle_conns.len() + active_count,
            idle_connections: idle_conns.len(),
            active_connections: active_count,
            waiting_for_connection: 0, // This would require more complex tracking
        }
    }

    /// Shutdown the connection pool (currently unused)
    #[allow(dead_code)]
    pub fn shutdown(&self) {
        let mut idle_conns = self.idle_connections.lock().unwrap();
        idle_conns.clear();
        *self.active_count.lock().unwrap() = 0;
    }
}

// Implement Clone for ConnectionPool to allow Arc<ConnectionPool>
impl Clone for ConnectionPool {
    fn clone(&self) -> Self {
        Self {
            client: Arc::clone(&self.client),
            config: self.config.clone(),
            idle_connections: Arc::clone(&self.idle_connections),
            active_count: Arc::clone(&self.active_count),
            condition: Arc::clone(&self.condition),
        }
    }
}

// Implement Debug for ConnectionPool since redis::Connection doesn't implement Debug
impl std::fmt::Debug for ConnectionPool {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let stats = self.get_stats();
        f.debug_struct("ConnectionPool")
            .field("config", &self.config)
            .field("stats", &stats)
            .finish()
    }
}
