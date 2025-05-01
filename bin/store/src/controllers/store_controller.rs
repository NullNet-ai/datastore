use crate::db;
use crate::structs::structs::{ApiResponse, CreateRequestBody, QueryParams};
use crate::sync::sync_service::insert;
use crate::table_enum::Table;
use actix_web::error::BlockingError;
use actix_web::{HttpResponse, Responder, ResponseError, http, web};
use diesel::result::Error as DieselError;
use serde::Serialize;

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

pub async fn create_record(
    pool: web::Data<db::AsyncDbPool>,
    table: web::Path<String>,
    request: web::Json<CreateRequestBody>,
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
    if let Some(obj) = processed_record.as_object_mut() {
        if table_name == "packets" {
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
    let mut conn = match pool.get().await {
        Ok(conn) => conn,
        Err(e) => {
            return HttpResponse::InternalServerError().json(serde_json::json!({
                "error": format!("Failed to get database connection: {}", e)
            }));
        }
    };
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
    let result = match table.insert_record(&mut conn, processed_record.clone(), orig_request).await {
        Ok(record) => record,
        Err(e) => {
            return HttpResponse::InternalServerError().json(ApiError::from(e));
        }
    };

    let mut record_value: serde_json::Value = match serde_json::from_value(processed_record) {
        Ok(val) => val,
        Err(e) => {
            return HttpResponse::InternalServerError().json(ApiError {
                status: http::StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                message: format!("Failed to process record: {}", e),
            });
        }
    };
    println!("record_value: {:?}", record_value);

    if let Err(e) = insert(&inner_log_table, record_value.clone()).await {
        return HttpResponse::InternalServerError().json(ApiError {
            status: http::StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
            message: format!("Sync error: {}", e),
        });
    }


    let plucked_record=table.pluck_fields(&record_value, pluck_fields);

    let response = ApiResponse {
        success: true,
        message: format!("Record inserted into '{}'", &inner_log_table),
        count: 1,
        data: vec![plucked_record],
    };
    HttpResponse::Ok().json(response)
}
