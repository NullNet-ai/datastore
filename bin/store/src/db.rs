use diesel::pg::PgConnection;
use diesel::r2d2::{self, ConnectionManager, Pool, PooledConnection};
use dotenv::dotenv;
use std::env;
use std::sync::OnceLock;

use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use diesel_async::pooled_connection::deadpool::Pool as PoolAsync;
use once_cell::sync::Lazy;
use diesel_async::AsyncPgConnection;



// -- Sync Types --
pub type DbPool = Pool<ConnectionManager<PgConnection>>;
pub type DbPooledConnection = PooledConnection<ConnectionManager<PgConnection>>;

// -- Async Types --
pub type AsyncDbPool = PoolAsync<AsyncPgConnection>;
pub type AsyncDbPooledConnection = diesel_async::pooled_connection::deadpool::Object<AsyncPgConnection>;

static ASYNC_POOL: Lazy<AsyncDbPool> = Lazy::new(|| establish_async_pool());

pub fn establish_async_pool() -> AsyncDbPool {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let config = AsyncDieselConnectionManager::<AsyncPgConnection>::new(database_url);
     PoolAsync::builder(config)
        .max_size(20)
        .build().expect("Failed to create async pool")
}

// -- Sync Pool --
static SYNC_POOL: OnceLock<DbPool> = OnceLock::new();


pub fn establish_sync_pool() -> DbPool {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder()
        .max_size(20)
        .min_idle(Some(5))
        .connection_timeout(std::time::Duration::from_secs(30))
        .idle_timeout(Some(std::time::Duration::from_secs(300)))
        .max_lifetime(Some(std::time::Duration::from_secs(3600)))
        .build(manager)
        .expect("Failed to create sync pool")
}

pub fn get_sync_pool() -> &'static DbPool {
    SYNC_POOL.get_or_init(establish_sync_pool)
}
pub fn get_async_pool() -> &'static AsyncDbPool {
    &ASYNC_POOL
}

pub fn get_sync_connection() -> DbPooledConnection {
    get_sync_pool()
        .get()
        .unwrap_or_else(|e| {
            log::error!("Failed to get sync connection: {}", e);
            panic!("Sync connection failure");
        })
}


pub async fn get_async_connection() -> AsyncDbPooledConnection {
    get_async_pool()
        .get()
        .await
        .unwrap_or_else(|e| {
            log::error!("Failed to get async connection: {}", e);
            panic!("Async connection failure");
        })
}