use actix_web::{web, App, HttpServer};
use controllers::main_controllers::{delete_chunk, get_chunk, get_chunk_status, sync};
use dotenv::dotenv;
use env_logger::Env;
use std::env;

mod controllers;
mod db;
mod models;
mod schema;
mod structs;
mod sync;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init_from_env(Env::default().default_filter_or("debug"));
    let port = env::var("PORT").unwrap_or_else(|_| "3002".to_string());
    let pool = db::establish_connection();
    println!("Database connected successfully.");

    let server_url = format!("0.0.0.0:{}", port);
    println!("Server is running on {}", server_url);

    HttpServer::new(move || {
        App::new().app_data(web::Data::new(pool.clone())).service(
            web::scope("/app")
                .route("/sync", web::post().to(sync))
                .service(
                    web::scope("/sync")
                        .route("/chunk", web::get().to(get_chunk))
                        .route("/chunk", web::delete().to(delete_chunk))
                        .route("/chunk/status", web::get().to(get_chunk_status)),
                )
                .route("/ping", web::get().to(|| async { "pong" })),
        )
    })
    .bind(server_url)?
    .run()
    .await
}
