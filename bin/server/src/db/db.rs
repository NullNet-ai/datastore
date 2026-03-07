use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use dotenv::dotenv;
use std::env;
use std::sync::OnceLock;

// Update the type definitions to use the imported types directly
pub type DbPool = Pool<ConnectionManager<PgConnection>>;
pub type DbPooledConnection = PooledConnection<ConnectionManager<PgConnection>>;

static POOL: OnceLock<DbPool> = OnceLock::new();

pub fn establish_connection() -> DbPool {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let max_size = env::var("DB_POOL_SIZE")
        .ok()
        .and_then(|v| v.parse::<u32>().ok())
        .unwrap_or(25);
    Pool::builder()
        .max_size(max_size)
        .build(manager)
        .expect("Failed to create pool")
}

// Get the global connection pool, initializing it if necessary
pub fn get_pool() -> &'static DbPool {
    POOL.get_or_init(establish_connection)
}

// Get a connection from the global pool - updated to use the correct error type
pub fn get_connection() -> DbPooledConnection {
    get_pool().get().expect("Failed to get database connection")
}
