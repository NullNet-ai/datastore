use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use std::env;

mod controllers;
mod db;
mod models;
mod schema;
mod structs;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let port = env::var("PORT").unwrap_or_else(|_| "3002".to_string());
    let pool = db::establish_connection();
    println!("Database connected successfully.");

    let server_url = format!("127.0.0.1:{}", port);
    println!("Server is running on {}", server_url);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(
                web::scope("/api/server")
                    // Add your routes here
            )
    })
    .bind(server_url)?
    .run()
    .await
}