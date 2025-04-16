use actix_web::{App, HttpServer, web};
use dotenv::dotenv;
use std::env;
use controllers::controllers::{get_chunk, delete_chunk, sync};

mod controllers;
mod db;
mod models;
mod schema;
mod structs;
mod sync; 

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
                web::scope("/app")
                    .route("/sync", web::post().to(sync))
                    .service(
                        web::scope("/sync")
                            .route("/chunk", web::get().to(get_chunk))
                            .route("/chunk", web::delete().to(delete_chunk))
                    )
                    .route("/ping", web::get().to(|| async { "pong" }))
            )
    })
    .bind(server_url)?
    .run()
    .await
}
