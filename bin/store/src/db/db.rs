use tokio_postgres::{NoTls, Error, Client};
use dotenv::dotenv;
use std::env;
use std::format;
use std::eprintln;
pub struct Database {
    client: Client,
}

impl Database {
    pub async fn new() -> Result<Self, Error> {
        dotenv().ok();

        let host = env::var("POSTGRES_HOST").unwrap_or_else(|_| "localhost".to_string());
        let port = env::var("POSTGRES_PORT").unwrap_or_else(|_| "5432".to_string());
        let dbname = env::var("POSTGRES_DB").unwrap_or_else(|_| "nullnet".to_string());
        let user = env::var("POSTGRES_USER").unwrap_or_else(|_| "admin".to_string());
        let password = env::var("POSTGRES_PASSWORD").unwrap_or_else(|_| "admin".to_string());

        let connection_string = format!(
            "host={} port={} dbname={} user={} password={}",
            host, port, dbname, user, password
        );

        //This is used to establish a connection to the database, await is used to wait for the connection to be established
        let (client, connection) = tokio_postgres::connect(&connection_string, NoTls).await?;

        //This is used to spawn a new asynchronous task to keep the connection alive and not block the main thread
        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("Connection error: {}", e);
            }
        });

        Ok(Database { client })
    }

    pub async fn test_connection(&self) -> Result<(), Error> {
        self.client.query_one("SELECT 1", &[]).await?;
        Ok(())
    }
}