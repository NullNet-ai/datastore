use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use diesel::prelude::*;
use serde_json::json;

mod db;
mod models;
mod schema;

use models::{Item, NewItem};
use schema::items;
use serde::Serialize;

#[derive(Serialize)]
struct ApiError {
    message: String,
}

async fn create_item(pool: web::Data<db::DbPool>, new_item: web::Json<NewItem>) -> impl Responder {
    let pool = pool.into_inner();

    match web::block(move || {
        let mut conn = pool.get().expect("Failed to get connection from pool");
        diesel::insert_into(items::table)
            .values(&new_item.into_inner())
            .get_result::<Item>(&mut conn)
    })
        .await
    {
        Ok(item) => HttpResponse::Ok().json({ "message" }),
        Err(_) => HttpResponse::InternalServerError().json(json!({"status": "error", "message": "Failed to create item"})),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = db::establish_connection();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .route("/items", web::post().to(create_item))
    })
        .bind("127.0.0.1:3000")?
        .run()
        .await
}