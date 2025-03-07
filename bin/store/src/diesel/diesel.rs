use deadpool_diesel::postgres::{Manager, Pool};
use dotenv::dotenv;
use std::env;

pub type DbPool = Pool;

pub fn create_pool() -> DbPool {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let manager = Manager::new(database_url, deadpool_diesel::Runtime::Tokio1);

    Pool::builder(manager)
        .build()
        .expect("Failed to create database pool")
}
