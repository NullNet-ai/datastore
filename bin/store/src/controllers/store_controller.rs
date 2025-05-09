use crate::db;
use crate::structs::structs::{ApiResponse, RequestBody, QueryParams};
use crate::sync::sync_service::insert;
use crate::table_enum::Table;
use actix_web::error::BlockingError;
use actix_web::{http, web, HttpResponse, Responder, ResponseError};
use diesel::result::Error as DieselError;
use serde::Serialize;
use serde_json::Value;
use futures::{SinkExt, pin_mut};
use tokio_postgres::{NoTls, Client};
use crate::batch_sync::BatchSyncService;
use serde::Deserialize;
use crate::schema::verify::field_exists_in_table;
use crate::controllers::common_controller::{process_records, convert_json_to_csv, create_connection, execute_copy};

#[derive(Serialize)]
struct ApiError {
    message: String,
    status: u16,
}
impl From<BlockingError> for ApiError {
    fn from(error: BlockingError) -> Self {
        ApiError {
            status: error.status_code().as_u16(),
            message: format!("Internal server error: {:?}", error),
        }
    }
}

impl ApiError {
    fn new(status: http::StatusCode, message: impl Into<String>) -> Self {
        Self {
            status: status.as_u16(),
            message: message.into(),
        }
    }
}
impl From<DieselError> for ApiError {
    fn from(error: DieselError) -> Self {
        let status_code = match error {
            DieselError::NotFound => http::StatusCode::NOT_FOUND,
            DieselError::DatabaseError(_, _) => http::StatusCode::BAD_REQUEST,
            DieselError::DeserializationError(_) => http::StatusCode::UNPROCESSABLE_ENTITY,
            DieselError::SerializationError(_) => http::StatusCode::INTERNAL_SERVER_ERROR,
            DieselError::RollbackTransaction => http::StatusCode::INTERNAL_SERVER_ERROR,
            DieselError::AlreadyInTransaction => http::StatusCode::INTERNAL_SERVER_ERROR,
            _ => http::StatusCode::INTERNAL_SERVER_ERROR,
        };
        ApiError {
            status: status_code.as_u16(),
            message: format!("Database error: {}", error),
        }
    }
}

impl From<serde_json::Error> for ApiError {
    fn from(error: serde_json::Error) -> Self {
        Self::new(
            http::StatusCode::UNPROCESSABLE_ENTITY,
            format!("JSON processing error: {}", error),
        )
    }
}

pub async fn update_record(
    pool: web::Data<db::AsyncDbPool>,
    path_params: web::Path<(String, String)>,
    request: web::Json<RequestBody>,
    query: web::Query<QueryParams>,
) -> impl Responder {
    let pool = pool.clone();
    let orig_request = request;
    let mut request = orig_request.clone();
    request.process_record("update");
    let (table_name, record_id) = path_params.into_inner();
    let log_table = table_name.clone();
    let inner_log_table = log_table.clone();
    let mut processed_record = request.record.clone();
    let table = match Table::from_str(table_name.as_str()) {
        Some(t) => t,
        None => {
            return HttpResponse::BadRequest().json(ApiError {
                status: http::StatusCode::BAD_REQUEST.as_u16(),
                message: format!("Unknown table: {}", table_name),
            });
        }
    };
    if field_exists_in_table(&table_name, "hypertable_timestamp"){

        let mut conn = match pool.get().await {
            Ok(conn) => conn,
            Err(e) => {
                return HttpResponse::InternalServerError().json(serde_json::json!({
                    "error": format!("Failed to get database connection: {}", e)
                }));
            }
        };

        let timestamp_result = match table.get_hypertable_timestamp(&mut conn, &record_id).await {
            Ok(timestamp) => timestamp,
            Err(e) => {
                return HttpResponse::InternalServerError().json(ApiError::from(e));
            }
        };

        if let Some(obj) = processed_record.as_object_mut() {
            if let Some(timestamp) = timestamp_result {
                log::debug!("Found hypertable timestamp: {}", timestamp);
                obj.insert("hypertable_timestamp".to_string(), serde_json::Value::String(timestamp));
            } else {
                // If no timestamp found, use the timestamp from the record if available
                log::warn!("No hypertable_timestamp found: {}", record_id);
                //return error from here
                return HttpResponse::InternalServerError().json(ApiResponse {
                    message: format!("No hypertable_timestamp found: {}", record_id),
                    success:false,
                    count: 0,
                    data: vec![],

                });
               
            }
        }
    }


    let pluck_fields: Vec<String> = query
        .pluck
        .split(',')
        .map(|s| s.trim().to_string())
        .collect();
    

    let record_value: serde_json::Value = match serde_json::from_value(processed_record) {
        Ok(val) => val,
        Err(e) => {
            return HttpResponse::InternalServerError().json(ApiResponse {
                message: format!("Failed to process record: {}", e),
                success:false,
                count: 0,
                data: vec![],
            });
        }
    };


    // Use the update function from sync_service
    if let Err(e) = crate::sync::sync_service::update(&inner_log_table, record_value.clone(), &record_id).await {
        return HttpResponse::InternalServerError().json(ApiResponse {
            message: format!("Sync error: {}", e),
            success:false,
            count: 0,
            data: vec![],
        });
    }

    let plucked_record = table.pluck_fields(&record_value, pluck_fields);

    let response = ApiResponse {
        success: true,
        message: format!("Record updated in '{}'", &inner_log_table),
        count: 1,
        data: vec![plucked_record],
    };
    
    HttpResponse::Ok().json(response)
}

pub async fn create_record(
    pool: web::Data<db::AsyncDbPool>,
    table: web::Path<String>,
    request: web::Json<RequestBody>,
    query: web::Query<QueryParams>,
) -> impl Responder {
    let pool = pool.clone();
    let orig_request = request;
    let mut request = orig_request.clone();
    request.process_record("create");
    let table_name = table.into_inner();
    let log_table = table_name.clone();
    let inner_log_table = log_table.clone();
    let mut processed_record = request.record.clone();

    if field_exists_in_table(&table_name, "hypertable_timestamp"){
        if let Some(obj) = processed_record.as_object_mut() {
            if let Some(timestamp) = obj.get("timestamp") {
                obj.insert("hypertable_timestamp".to_string(), timestamp.clone());
            }
        }
    }

    let pluck_fields: Vec<String> = query
        .pluck
        .split(',')
        .map(|s| s.trim().to_string())
        .collect();
    let table = match Table::from_str(table_name.as_str()) {
        Some(t) => t,
        None => {
            return HttpResponse::BadRequest().json(ApiError {
                status: http::StatusCode::BAD_REQUEST.as_u16(),
                message: format!("Unknown table: {log_table}"),
            });
        }
    };

    // Execute the insert operation directly in async context
    // let result = match table.insert_record(&mut conn, processed_record.clone(), orig_request).await {
    //     Ok(record) => record,
    //     Err(e) => {
    //         return HttpResponse::InternalServerError().json(ApiError::from(e));
    //     }
    // };

    let record_value: serde_json::Value = match serde_json::from_value(processed_record) {
        Ok(val) => val,
        Err(e) => {
            return HttpResponse::InternalServerError().json(ApiError {
                status: http::StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                message: format!("Failed to process record: {}", e),
            });
        }
    };

    if let Err(e) = insert(&inner_log_table, record_value.clone()).await {
        return HttpResponse::InternalServerError().json(ApiError {
            status: http::StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
            message: format!("Sync error: {}", e),
        });
    }

    let plucked_record = table.pluck_fields(&record_value, pluck_fields);

    let response = ApiResponse {
        success: true,
        message: format!("Record inserted into '{}'", &inner_log_table),
        count: 1,
        data: vec![plucked_record],
    };
    HttpResponse::Ok().json(response)
}

#[derive(Deserialize)]
pub struct BatchInsertBody {
    entity_prefix: Option<String>,
    records: Vec<Value>,
}

pub async fn batch_insert_records(
    table: web::Path<String>,
    records: web::Json<BatchInsertBody>,
) -> impl Responder {
    let table_name = table.into_inner();
    let table_clone = table_name.clone();
    let batch_data = records.into_inner();
    let json_records = batch_data.records;

    if json_records.is_empty() {
        return HttpResponse::BadRequest().json(ApiResponse{
            success: false,
            message: "No records provided".to_string(),
            count: 0,
            data: vec![],
        })
    }
    let (processed_records, columns) = match process_records(json_records, &table_name) {
        Ok((records, cols)) => (records, cols),
        Err(e) => return HttpResponse::BadRequest().json(ApiResponse {
            success: false,
            message: format!("Error processing records: {}", e),
            count: 0,
            data: vec![],
        })
    };

    let csv_data =match convert_json_to_csv(&processed_records, &columns){
        Ok(data) => data,
        Err(e) => return HttpResponse::BadRequest().json(ApiResponse {
            success: false,
            message: format!("Error converting records to CSV: {:?}", e),
            count: 0,
            data: vec![],
        })
    };

    let client = match create_connection().await {
        Ok(client) => client,
        Err(e) => return HttpResponse::InternalServerError().json(ApiResponse {
            success: false,
            message: format!("Error creating database connection: {:?}", e),
            count: 0,
            data: vec![],
        })
    };

    let column_refs: Vec<&str> = columns.iter().map(|s| s.as_str()).collect();

    let records=match  execute_copy(&client, &table_name, &column_refs, csv_data)
        .await {
        Ok(_) => {
            processed_records.clone()
        }
        Err(e) => {
            return HttpResponse::InternalServerError().json(ApiResponse {
                success: false,
                message: format!("Error executing COPY command: {:?}", e),
                count: 0,
                data: vec![],
            })
        }
    };

    // Convert JSON array to CSV in-memor
  
    for record in processed_records.iter() {
        if let Err(e) = BatchSyncService::send_message(table_clone.clone(), record.clone()).await {
            return HttpResponse::InternalServerError().json(ApiResponse {
                success: false,
                message: format!("Sync error: {e}"),
                count: 0,
                data: vec![],
            });
        }
    }

    let response = ApiResponse {
        success: true,
        message: format!("Inserted {} records into '{}'", processed_records.len(), table_name),
        count: processed_records.len() as i32,
        data: processed_records, // Include the processed records in the response
    };
    
    HttpResponse::Ok().json(response)
}

fn extract_columns(records: &[Value]) -> Result<Vec<&str>, String> {
    records.first()
        .and_then(|v| v.as_object())
        .map(|obj| obj.keys().map(|k| k.as_str()).collect())
        .ok_or_else(|| "Invalid record format".to_string())
}

fn serialize_value(value: &Value) -> String {
    match value {
        Value::String(s) => s.clone(),
        Value::Null => String::new(),
        Value::Number(n) => n.to_string(),
        Value::Bool(b) => b.to_string(),
        Value::Array(arr) => format!("{{{}}}", arr.iter()
            .map(serialize_value)
            .collect::<Vec<_>>()
            .join(",")),
        Value::Object(_) => serde_json::to_string(value).unwrap_or_default(),
    }
}

