use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde_json::json;

mod db;
mod models;
mod schema;
mod table_enum;

use actix_web::error::BlockingError;
use diesel::result::Error as DieselError;
use models::NewItem;
use serde::Serialize;
use table_enum::Table;

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
async fn create_record(
    pool: web::Data<db::DbPool>,
    table: web::Path<String>,
    new_item: web::Json<NewItem>,
) -> impl Responder {
    let pool = pool.clone();
    let table_name = table.into_inner();

    let result = web::block(move || {
        let mut conn = pool.get().map_err(|e| ApiError {
            status: "error".to_string(),
            message: format!("Failed to get DB connection: {}", e),
        })?;

        let table = match table_name.as_str() {
            "items" => Table::Items,
            // Add other table mappings here
            _ => return Err(ApiError {
                status: "error".to_string(),
                message: format!("Unknown table: {}", table_name),
            }),
        };

        table.insert_query(&mut conn, new_item.into_inner()).map_err(ApiError::from)
    })
        .await
        .map_err(ApiError::from);

    match result {
        Ok(record) => HttpResponse::Ok().json(record),
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
            .route("/create/{table}", web::post().to(create_record))
    })
        .bind("127.0.0.1:3000")?
        .run()
        .await
}