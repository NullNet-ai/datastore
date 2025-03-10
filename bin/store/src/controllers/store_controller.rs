use crate::db;
use crate::models::NewItem;
use crate::table_enum::Table;
use actix_web::error::BlockingError;
use actix_web::{web, HttpResponse, Responder};
use diesel::result::Error as DieselError;
use serde::Serialize;
use serde_json::json;


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


pub async fn create_record(
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