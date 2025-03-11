use crate::db;
use crate::models::item_model::InsertItem;
use crate::models::packet_model::InsertPacket;
use crate::structs::structs::{ApiResponse, CreateRequestBody, QueryParams};
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
    request: web::Json<CreateRequestBody>,
    query: web::Query<QueryParams>,
) -> impl Responder {
    let pool = pool.clone();
    let table_name = table.into_inner();
    let log_table = table_name.clone();
    let inner_log_table = log_table.clone();
    let processed_record = match request.process_record() {
        Ok(processed) => processed,
        Err(e) => return HttpResponse::BadRequest().json(ApiError {
            status: "error".to_string(),
            message: e,
        }),
    };
    let pluck_fields: Vec<String> = query.pluck
        .split(',')
        .map(|s| s.trim().to_string())
        .collect();
    let result = web::block(move || {
        let mut conn = pool.get().map_err(|e| ApiError {
            status: "error".to_string(),
            message: format!("Failed to get DB connection: {}", e),
        })?;
        let table = match table_name.as_str() {
            "items" => {
                Table::Items
            }
            "packets" => {
                Table::Packets
            }
            // Add other table mappings here
            _ => return Err(ApiError {
                status: "error".to_string(),
                message: format!("Unknown table: {}", &log_table),
            }),
        };

        // The insert_query function now returns a string directly
        match table_name.as_str() {
            "items" => {
                let parsed_item: InsertItem = serde_json::from_value(processed_record.clone()).map_err(|e| ApiError {
                    status: "error".to_string(),
                    message: format!("Failed to parse item: {}", e),
                })?;
                table.insert_item(&mut conn, parsed_item).map_err(ApiError::from)
            }
            "packets" => {

                let mut modified_record = processed_record.clone();
                if let Some(timestamp) = modified_record.get("timestamp") {
                    modified_record["hypertable_timestamp"] = timestamp.clone();
                }

                let parsed_packet: InsertPacket = serde_json::from_value(modified_record.clone()).map_err(|e| ApiError {
                    status: "error".to_string(),
                    message: format!("Failed to parse packet: {}", e),
                })?;
                table.insert_packet(&mut conn, parsed_packet).map_err(ApiError::from)
            }
            _ => unreachable!(), // We've already checked this above
        }
    })
        .await
        .map_err(ApiError::from);

    match result {
        Ok(Ok(record)) => {
            // Parse the record string into a JSON value
            let mut record_value: serde_json::Value = serde_json::from_str(&record).unwrap_or(serde_json::Value::Null);
            
            // Apply pluck filter if specified
            if !pluck_fields.is_empty() && record_value.is_object() {
                let obj = record_value.as_object_mut().unwrap();
                let keys: Vec<String> = obj.keys().cloned().collect();
                
                for key in keys {
                    if !pluck_fields.contains(&key) {
                        obj.remove(&key);
                    }
                }
            }

            // Create the response
            let response = ApiResponse {
                success: true,
                message: format!("Record inserted into '{}'", &inner_log_table),
                count: 1,
                data: vec![record_value],
            };

            HttpResponse::Ok().json(response)
        }
        Ok(Err(err)) => HttpResponse::InternalServerError().json(json!(err)),
        Err(err) => HttpResponse::InternalServerError().json(json!(err)),
    }
}