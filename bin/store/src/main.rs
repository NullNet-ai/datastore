use actix_web::{web, App, HttpServer, Responder};
use dotenv::dotenv;
use std::env;

mod controllers;
use controllers::store_controller;

mod db;
mod diesel;
mod types;
async fn hello() -> impl Responder {
    "Hello, World!"
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let db = diesel::Database::new().await;
    let port: String = env::var("PORT").unwrap_or("3000".to_string());

    HttpServer::new(|| {
        App::new()
            .service(web::scope("/api/store").configure(store_controller::init_routes))
            .route("/", web::get().to(hello))
    })
        .bind(format!("127.0.0.1:{}", port))?
        .run()
        .await
}