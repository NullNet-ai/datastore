use actix_web::{web, App, HttpServer, Responder};
use dotenv::dotenv;
use std::env;
mod controllers;
mod db;
mod diesel;
mod types;
mod providers;
mod utility;

async fn hello() -> impl Responder {
    "Hello, World!"
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let db_pool = diesel::create_pool();
    let port: String = env::var("PORT").unwrap_or("3000".to_string());

    HttpServer::new(move || {
        App::new()
            .app_data(db_pool.clone())
            .configure(controllers::store_controller::init_routes)
            .route("/", web::get().to(hello))
    })
        .bind(format!("127.0.0.1:{}", port))?
        .run()
        .await
}