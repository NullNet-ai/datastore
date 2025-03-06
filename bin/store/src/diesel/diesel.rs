use diesel::prelude::*;
use diesel_async::{RunQueryDsl, AsyncConnection, AsyncPgConnection};
use dotenv::dotenv;
use std::env;

pub struct Database {
    pub connection: AsyncPgConnection,
}

impl Database {
    pub async fn new() -> Self {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let connection = AsyncPgConnection::establish(&database_url)
            .await
            .expect(&format!("Error connecting to {}", database_url));
        println!("Successfully connected to the database");
        Database { connection }
    }
}