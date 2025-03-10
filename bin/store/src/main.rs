use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use diesel::prelude::*;
use serde_json::json;

mod db;
mod models;
mod schema;
mod table_enum;

use actix_web::error::BlockingError;
use diesel::result::Error as DieselError;
use models::{Item, NewItem};
use schema::items;
use serde::Serialize;

#[derive(Serialize)]
struct ApiError {
    message: String,
    status: String,
}
impl From<BlockingError> for ApiError
{
    fn from(error: BlockingError) -> Self {
        ApiError {
            status: "error".to_string(),
            message: format!("Internal server error: {:?}", error),
        }
    }
}
impl From<DieselError> for ApiError {
    fn from(error: DieselError) -> Self {
        ApiError {
            status: "error".to_string(),
            message: format!("Database error: {}", error),
        }
    }
}
async fn create_item(
    pool: web::Data<db::DbPool>,
    table_name: web::Path<String>,
    new_item: web::Json<NewItem>,
) -> impl Responder {
    let pool = pool.clone();
    let table_name = table_name.into_inner();

    let result = web::block(move || {
        let mut conn = pool.get().map_err(|e| ApiError {
            status: "error".to_string(),
            message: format!("Failed to get DB connection: {}", e),
        })?;

        diesel::insert_into(items::table)
            .values(&new_item.into_inner())
            .get_result::<Item>(&mut conn)
            .map_err(ApiError::from) // Convert Diesel error to `ApiError`
    })
        .await
        .map_err(ApiError::from); // Convert BlockingError to `ApiError`

    match result {
        Ok(item) => HttpResponse::Ok().json(item),
        Err(err) => HttpResponse::InternalServerError().json(json!(err)),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = db::establish_connection();
    println!("Database connected successfully.");

    let server_url = "127.0.0.1:3000";
    println!("Server is running on {}", server_url);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .route("/items", web::post().to(create_item))
    })
        .bind("127.0.0.1:3000")?
        .run()
        .await
}