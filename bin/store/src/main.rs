use actix_web::{App, HttpServer, web};
use auth::auth_middleware::Authentication;
use dotenv::dotenv;
use std::env;
mod auth;
mod controllers;
mod db;
mod models;
mod schema;
mod structs;
mod sync;
mod table_enum;
use crate::sync::controllers::sync_endpoints_controller;
use crate::sync::sync_service::bg_sync;
use crate::sync::transactions::queue_service::QueueService;
use crate::sync::transactions::transaction_service::TransactionService;
use controllers::store_controller::create_record;
use env_logger::Env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("debug"));

    dotenv().ok();
    let port = env::var("PORT").unwrap_or_else(|_| "3001".to_string());
    let pool = db::establish_connection();
    println!("Database connected successfully.");
    TransactionService::initialize();
    if let Err(e) = QueueService::init() {
        log::error!("Failed to initialize queue: {}", e);
    } else {
        println!("Queue initialized successfully");
    }

    let server_url = format!("127.0.0.1:{}", port);
    println!("Server is running on {}", server_url);
    // ! not using async generator
    tokio::spawn(async {
        let mut conn = db::get_connection();

        if let Err(e) = bg_sync(&mut conn).await {
            log::error!("Error starting background sync: {}", e);
        }
    });

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .configure(sync_endpoints_controller::configure)
            .service(
                web::scope("/api/store")
                    .wrap(Authentication)
                    .route("/{table}", web::post().to(create_record)),
            )
    })
    .bind(server_url)?
    .run()
    .await
}
