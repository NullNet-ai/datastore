use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use std::env;
mod controllers;
mod db;
mod models;
mod schema;
mod structs;
mod sync;
mod table_enum;
mod auth;

use controllers::store_controller::create_record;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    //use port from the env
    dotenv().ok();
    let port = env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let pool = db::establish_connection();
    println!("Database connected successfully.");

    let server_url = format!("127.0.0.1:{}", port);
    println!("Server is running on {}", server_url);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(web::scope("/api/store").route("/{table}", web::post().to(create_record)))
    })
        .bind(server_url)?
        .run()
        .await
}
