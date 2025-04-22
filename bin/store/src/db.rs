use diesel::pg::PgConnection;
use diesel::r2d2::{self, ConnectionManager, Pool, PooledConnection};
use dotenv::dotenv;
use std::env;
use std::sync::{Arc, Mutex, OnceLock};

// Update the type definitions to use the imported types directly
pub type DbPool = Pool<ConnectionManager<PgConnection>>;
pub type DbPooledConnection = PooledConnection<ConnectionManager<PgConnection>>;
struct ConnectionCustomizer;

static POOL: OnceLock<DbPool> = OnceLock::new();

pub fn establish_connection() -> DbPool {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder()
        .max_size(20)
        .min_idle(Some(5)) // Keep minimum 5 connections ready
        .connection_timeout(std::time::Duration::from_secs(30)) // Increase timeout to 30 seconds
        .idle_timeout(Some(std::time::Duration::from_secs(300))) // Set idle timeout to 5 minutes
        .max_lifetime(Some(std::time::Duration::from_secs(3600))) // Set max connection lifetime to 1 hour
        .build(manager)
        .map_err(|e| {
            log::error!("Failed to create connection pool: {}", e);
            e
        })
        .expect("Failed to create pool")
}

// Get the global connection pool, initializing it if necessary
pub fn get_pool() -> &'static DbPool {
    POOL.get_or_init(establish_connection)
}

// Get a connection from the global pool - updated to use the correct error type
pub fn get_connection() -> DbPooledConnection {
    get_pool()
        .get()
        .map_err(|e| {
            log::error!("Failed to get database connection: {}", e);
            e
        })
        .expect("Failed to get database connection")
}
