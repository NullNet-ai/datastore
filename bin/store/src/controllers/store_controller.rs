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
    let mut request = request.into_inner();
    let pool = pool.clone();
    let table_name = table.into_inner();
    let log_table = table_name.clone();
    let inner_log_table = log_table.clone();
    request.process_record("create");
    let processed_record = request.record.clone();
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
    let result = match table.insert_record(&mut conn, processed_record.clone()).await {
        Ok(record) => record,
        Err(e) => {
            return HttpResponse::InternalServerError().json(ApiError::from(e));
        }
    };

    let mut record_value: serde_json::Value = match serde_json::from_str(&result) {
        Ok(val) => val,
        Err(e) => {
            return HttpResponse::InternalServerError().json(ApiError {
                status: http::StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                message: format!("Failed to parse record: {}", e),
            });
        }
    };

    if let Err(e) = insert(&inner_log_table, record_value.clone()).await {
        return HttpResponse::InternalServerError().json(ApiError {
            status: http::StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
            message: format!("Sync error: {}", e),
        });
    }

    if !pluck_fields.is_empty() && record_value.is_object() {
        let obj = record_value.as_object_mut().unwrap();
        let keys: Vec<String> = obj.keys().cloned().collect();

        for key in keys {
            if !pluck_fields.contains(&key) {
                obj.remove(&key);
            }
        }
    }

    let response = ApiResponse {
        success: true,
        message: format!("Record inserted into '{}'", &inner_log_table),
        count: 1,
        data: vec![record_value],
    };
    HttpResponse::Ok().json(response)
}
