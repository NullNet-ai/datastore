use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use diesel_async::pooled_connection::deadpool::Pool as PoolAsync;
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use diesel_async::AsyncPgConnection;
use dotenv::dotenv;
use once_cell::sync::Lazy;
use std::env;
use std::sync::OnceLock;
use tokio_postgres::{Client, NoTls};

// -- Sync Types --
#[allow(warnings)]
pub type DbPool = Pool<ConnectionManager<PgConnection>>;
#[allow(warnings)]
pub type DbPooledConnection = PooledConnection<ConnectionManager<PgConnection>>;

// -- Async Types --
pub type AsyncDbPool = PoolAsync<AsyncPgConnection>;
pub type AsyncDbPooledConnection =
    diesel_async::pooled_connection::deadpool::Object<AsyncPgConnection>;

static ASYNC_POOL: Lazy<AsyncDbPool> = Lazy::new(|| establish_async_pool());

pub fn establish_async_pool() -> AsyncDbPool {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let config = AsyncDieselConnectionManager::<AsyncPgConnection>::new(database_url);
    PoolAsync::builder(config)
        .max_size(20)
        .build()
        .expect("Failed to create async pool")
}

// -- Sync Pool --
#[allow(warnings)]
static SYNC_POOL: OnceLock<DbPool> = OnceLock::new();
#[allow(warnings)]
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
#[allow(warnings)]
pub fn get_sync_pool() -> &'static DbPool {
    SYNC_POOL.get_or_init(establish_sync_pool)
}
pub fn get_async_pool() -> &'static AsyncDbPool {
    &ASYNC_POOL
}
#[allow(warnings)]
pub fn get_sync_connection() -> DbPooledConnection {
    get_sync_pool().get().unwrap_or_else(|e| {
        log::error!("Failed to get sync connection: {}", e);
        panic!("Sync connection failure");
    })
}

pub async fn get_async_connection() -> AsyncDbPooledConnection {
    get_async_pool().get().await.unwrap_or_else(|e| {
        log::error!("Failed to get async connection: {}", e);
        panic!("Async connection failure");
    })
}

//raw database connection
pub async fn create_connection() -> Result<Client, Box<dyn std::error::Error>> {
    let user = env::var("POSTGRES_USER").unwrap_or_else(|_| "admin".to_string());
    let password = env::var("POSTGRES_PASSWORD").unwrap_or_else(|_| "admin".to_string());
    let dbname = env::var("POSTGRES_DB").unwrap_or_else(|_| "test".to_string());
    let host = env::var("POSTGRES_HOST").unwrap_or_else(|_| "localhost".to_string());
    let port = env::var("POSTGRES_PORT").unwrap_or_else(|_| "5433".to_string());

    let connection_string = format!(
        "host={} port={} user={} password={} dbname={}",
        host, port, user, password, dbname
    );

    let (client, connection) = tokio_postgres::connect(&connection_string, NoTls)
        .await
        .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            log::error!("PostgreSQL connection error: {}", e);
        }
    });

    Ok(client)
}
