use crate::batch_sync::BatchSyncService;
use crate::controllers::common_controller::{
    convert_json_to_csv, execute_copy, process_and_insert_record, process_and_update_record,
    process_records,
};
use crate::db;
use crate::db::create_connection;
use crate::structs::structs::Auth;
use crate::structs::structs::{
    ApiResponse, BatchUpdateBody, QueryParams, RequestBody, UpsertRequestBody,
};
use crate::table_enum::generate_code;
use crate::utils::utils::table_exists;
use actix_web::error::BlockingError;
use actix_web::{http, web, HttpResponse, Responder, ResponseError};
use actix_web::{HttpMessage, HttpRequest};
use diesel::result::Error as DieselError;
use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;
use std::fmt;

use super::common_controller::{perform_batch_update, perform_upsert, sanitize_updates};

#[derive(Serialize)]
pub struct ApiError {
    pub message: String,
    pub status: u16,
}
impl From<BlockingError> for ApiError {
    fn from(error: BlockingError) -> Self {
        ApiError {
            status: error.status_code().as_u16(),
            message: format!("Internal server error: {:?}", error),
        }
    }
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl ApiError {
    pub fn new(status: http::StatusCode, message: impl Into<String>) -> Self {
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
    auth: HttpRequest,
    pool: web::Data<db::AsyncDbPool>,
    path_params: web::Path<(String, String)>,
    request: web::Json<RequestBody>,
    query: web::Query<QueryParams>,
) -> impl Responder {
    let (table_name, record_id) = path_params.into_inner();
    let extensions = auth.extensions();
    let auth_data = match extensions.get::<Auth>() {
        Some(data) => data,
        None => {
            log::warn!("Auth data not found in request extensions");
            return HttpResponse::InternalServerError().json(ApiResponse {
                success: false,
                message: "Authentication information not available".to_string(),
                count: 0,
                data: vec![],
            });
        }
    };

    let pluck_fields: Vec<String> = if query.pluck.is_empty() {
        vec!["id".to_string()]
    } else {
        query
            .pluck
            .split(',')
            .map(|s| s.trim().to_string())
            .collect()
    };
    match process_and_update_record(
        &table_name,
        request.record.clone(),
        &record_id,
        Some(pluck_fields),
        "update",
        &auth_data,
    )
    .await
    {
        Ok(response) => HttpResponse::Ok().json(response),
        Err(error) => HttpResponse::build(http::StatusCode::from_u16(error.status).unwrap()).json(
            ApiResponse {
                success: false,
                message: error.message,
                count: 0,
                data: vec![],
            },
        ),
    }
}

pub async fn create_record(
    auth: HttpRequest,
    pool: web::Data<db::AsyncDbPool>,
    table: web::Path<String>,
    request: web::Json<RequestBody>,
    query: web::Query<QueryParams>,
) -> impl Responder {
    let extensions = auth.extensions();
    let auth_data = match extensions.get::<Auth>() {
        Some(data) => data,
        None => {
            log::warn!("Auth data not found in request extensions");
            return HttpResponse::InternalServerError().json(ApiResponse {
                success: false,
                message: "Authentication information not available".to_string(),
                count: 0,
                data: vec![],
            });
        }
    };
    let table_name = table.into_inner();
    let pluck_fields: Vec<String> = query
        .pluck
        .split(',')
        .map(|s| s.trim().to_string())
        .collect();
    //get entity_prefix from the request.record

    println!(
        "Organization ID: {}, Response Account: {}",
        auth_data.organization_id, auth_data.responsible_account
    );

    match process_and_insert_record(
        &table_name,
        request.record.clone(),
        Some(pluck_fields),
        &auth_data,
    )
    .await
    {
        Ok(response) => HttpResponse::Ok().json(response),
        Err(error) => HttpResponse::build(http::StatusCode::from_u16(error.status).unwrap()).json(
            ApiResponse {
                success: false,
                message: error.message,
                count: 0,
                data: vec![],
            },
        ),
    }
}

#[derive(Deserialize)]
pub struct BatchInsertBody {
    entity_prefix: Option<String>,
    records: Vec<Value>,
}

pub async fn batch_insert_records(
    auth: HttpRequest,
    table: web::Path<String>,
    records: web::Json<BatchInsertBody>,
) -> impl Responder {
    let table_name = table.into_inner();
    let extensions = auth.extensions();
    let auth_data = match extensions.get::<Auth>() {
        Some(data) => data,
        None => {
            log::warn!("Auth data not found in request extensions");
            return HttpResponse::InternalServerError().json(ApiResponse {
                success: false,
                message: "Authentication information not available".to_string(),
                count: 0,
                data: vec![],
            });
        }
    };
    let temp_table= format!("temp_{}", table_name);
    match table_exists(&temp_table) {
        Ok(table) => {
            // Table exists, proceed with your logic using the table
        },
        Err(error) => {
            // Table doesn't exist, return an error response
            return HttpResponse::BadRequest().json(ApiResponse {
                success: false,
                message: error.message,
                count: 0,
                data: vec![],
            });
        }
    }

    let table_clone = table_name.clone();
    let batch_data = records.into_inner();
    let json_records = batch_data.records;
    let entity_prefix_exists = batch_data.entity_prefix;
    let entity_prefix;

    match entity_prefix_exists {
        Some(prefix) => {
            entity_prefix = prefix;
        }
        None => {
            return HttpResponse::BadRequest().json(ApiResponse {
                success: false,
                message: "Entity prefix is required".to_string(),
                count: 0,
                data: vec![],
            })
        }
    }

    if json_records.is_empty() {
        return HttpResponse::BadRequest().json(ApiResponse {
            success: false,
            message: "No records provided".to_string(),
            count: 0,
            data: vec![],
        });
    }
    let (processed_records, columns) =
        match process_records(json_records, &table_name, &auth_data.clone()) {
            Ok((records, cols)) => (records, cols),
            Err(e) => {
                return HttpResponse::BadRequest().json(ApiResponse {
                    success: false,
                    message: format!("Error processing records: {}", e),
                    count: 0,
                    data: vec![],
                })
            }
        };

    let csv_data = match convert_json_to_csv(&processed_records, &columns) {
        Ok(data) => data,
        Err(e) => {
            return HttpResponse::BadRequest().json(ApiResponse {
                success: false,
                message: format!("Error converting records to CSV: {:?}", e),
                count: 0,
                data: vec![],
            })
        }
    };

    let client = match create_connection().await {
        Ok(client) => client,
        Err(e) => {
            return HttpResponse::InternalServerError().json(ApiResponse {
                success: false,
                message: format!("Error creating database connection: {:?}", e),
                count: 0,
                data: vec![],
            })
        }
    };

    let column_refs: Vec<&str> = columns.iter().map(|s| s.as_str()).collect();

    let records = match execute_copy(&client, &table_name, &column_refs, csv_data).await {
        Ok(_) => processed_records.clone(),
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
        if let Some(id) = record.get("id").and_then(|v| v.as_str()) {
            if let Err(e) = BatchSyncService::send_code_assignment_message(
                table_clone.clone(),
                id.to_string(),
                entity_prefix.clone(),
                auth_data.clone(),
            )
            .await
            {
                log::error!("Code assignment error with id {id}: {e}");
            }
        }
    }

    let response = ApiResponse {
        success: true,
        message: format!(
            "Inserted {} records into '{}'",
            processed_records.len(),
            table_name
        ),
        count: processed_records.len() as i32,
        data: processed_records, // Include the processed records in the response
    };

    HttpResponse::Ok().json(response)
}

pub async fn batch_update_records(
    auth: HttpRequest,
    pool: web::Data<db::AsyncDbPool>,
    table: web::Path<String>,
    request: web::Json<BatchUpdateBody>,
) -> impl Responder {
    let extensions = auth.extensions();
    let auth_data = match extensions.get::<Auth>() {
        Some(data) => data,
        None => {
            log::warn!("Auth data not found in request extensions");
            return HttpResponse::InternalServerError().json(ApiResponse {
                success: false,
                message: "Authentication information not available".to_string(),
                count: 0,
                data: vec![],
            });
        }
    };
    let table_name = table.into_inner();
    let batch_data = request.into_inner();
    let filters = batch_data.advance_filters;
    let mut updates = batch_data.updates;
    if updates
        .record
        .as_object()
        .map_or(true, |obj| obj.is_empty())
    {
        return HttpResponse::BadRequest().json(ApiResponse {
            success: false,
            message: "No update fields provided".to_string(),
            count: 0,
            data: vec![],
        });
    }
    updates.process_record("update", &auth_data);
    if let Some(record) = updates.record.as_object_mut() {
        record.remove("version");
    }
    let updates_value = match serde_json::to_value(updates) {
        Ok(Value::Object(map)) => {
            sanitize_updates(map).unwrap_or(Value::Object(Default::default()))
        }
        Ok(_) => Value::Object(Default::default()),
        Err(e) => {
            log::error!("Failed to serialize updates to JSON: {}", e);
            return HttpResponse::InternalServerError().json(ApiResponse {
                success: false,
                message: format!("Failed to process update data: {}", e),
                count: 0,
                data: vec![],
            });
        }
    };

    if updates_value.as_object().map_or(true, |o| o.is_empty()) {
        return HttpResponse::BadRequest().json(ApiResponse {
            success: false,
            message: "No valid fields to update".to_string(),
            count: 0,
            data: vec![],
        });
    }

    match perform_batch_update(&table_name, updates_value, filters).await {
        Ok((count, _)) => HttpResponse::Ok().json(ApiResponse {
            success: true,
            message: format!("Updated {} records in '{}'", count, table_name),
            count: count as i32,
            data: vec![],
        }),
        Err(e) => HttpResponse::InternalServerError().json(ApiResponse {
            success: false,
            message: e,
            count: 0,
            data: vec![],
        }),
    }

    //use the below code if you want to return the updated fields to the user, can be inefficient if the updated fields are large

    //print rows here
    // let mut json_rows: Vec<serde_json::Value> = Vec::new();
    // for row in &rows {
    //     let mut json_obj = serde_json::Map::new();

    //     // Extract id
    //     if let Ok(id) = row.try_get::<_, String>("id") {
    //         json_obj.insert("id".to_string(), serde_json::Value::String(id));
    //     }

    //     // Extract version
    //     if let Ok(version) = row.try_get::<_, i32>("version") {
    //         json_obj.insert("version".to_string(), serde_json::Value::Number(serde_json::Number::from(version)));
    //     }

    //     // Extract updated_date
    //     if let Ok(updated_date) = row.try_get::<_, String>("updated_date") {
    //         json_obj.insert("updated_date".to_string(), serde_json::Value::String(updated_date));
    //     }

    //     // Extract updated_time
    //     if let Ok(updated_time) = row.try_get::<_, String>("updated_time") {
    //         json_obj.insert("updated_time".to_string(), serde_json::Value::String(updated_time));
    //     }

    //     // Extract updated_by
    //     if let Ok(updated_by) = row.try_get::<_, String>("updated_by") {
    //         json_obj.insert("updated_by".to_string(), serde_json::Value::String(updated_by));
    //     }

    //     // Extract hypertable_timestamp if it exists
    //     if field_exists_in_table(&table_name, "hypertable_timestamp") {
    //         if let Ok(timestamp) = row.try_get::<_, String>("hypertable_timestamp") {
    //             json_obj.insert("hypertable_timestamp".to_string(), serde_json::Value::String(timestamp));
    //         }
    //     }

    //     // Extract any updated fields
    //     if let Some(update_obj) = updates.as_object() {
    //         for key in update_obj.keys() {
    //             if key != "record" {
    //                 // Try to get the value as different types
    //                 if let Ok(val) = row.try_get::<_, String>(key.as_str()) {
    //                     json_obj.insert(key.clone(), serde_json::Value::String(val));
    //                 } else if let Ok(val) = row.try_get::<_, i32>(key.as_str()) {
    //                     json_obj.insert(key.clone(), serde_json::Value::Number(serde_json::Number::from(val)));
    //                 } else if let Ok(val) = row.try_get::<_, i64>(key.as_str()) {
    //                     json_obj.insert(key.clone(), serde_json::Value::Number(serde_json::Number::from(val)));
    //                 } else if let Ok(val) = row.try_get::<_, f64>(key.as_str()) {
    //                     if let Some(num) = serde_json::Number::from_f64(val) {
    //                         json_obj.insert(key.clone(), serde_json::Value::Number(num));
    //                     }
    //                 } else if let Ok(val) = row.try_get::<_, bool>(key.as_str()) {
    //                     json_obj.insert(key.clone(), serde_json::Value::Bool(val));
    //                 }
    //             }
    //         }
    //     }

    //     json_rows.push(serde_json::Value::Object(json_obj));
    // }
}

pub async fn batch_delete_records(
    auth: HttpRequest,
    pool: web::Data<db::AsyncDbPool>,
    table: web::Path<String>,
    request: web::Json<BatchUpdateBody>,
) -> impl Responder {
    let extensions = auth.extensions();
    let auth_data = match extensions.get::<Auth>() {
        Some(data) => data,
        None => {
            log::warn!("Auth data not found in request extensions");
            return HttpResponse::InternalServerError().json(ApiResponse {
                success: false,
                message: "Authentication information not available".to_string(),
                count: 0,
                data: vec![],
            });
        }
    };
    let table_name = table.into_inner();
    let batch_data = request.into_inner();
    let filters = batch_data.advance_filters;

    // Create delete updates (setting tombstone and status)
    let mut delete_updates = RequestBody {
        record: serde_json::json!({}),
    };

    // Process the record through the common processing logic
    delete_updates.process_record("delete", &auth_data);
    if let Some(record) = delete_updates.record.as_object_mut() {
        record.remove("version");
    }

    let updates_value = delete_updates.record;

    match perform_batch_update(&table_name, updates_value, filters).await {
        Ok((count, _)) => HttpResponse::Ok().json(ApiResponse {
            success: true,
            message: format!("Deleted {} records in '{}'", count, table_name),
            count: count as i32,
            data: vec![],
        }),
        Err(e) => HttpResponse::InternalServerError().json(ApiResponse {
            success: false,
            message: e,
            count: 0,
            data: vec![],
        }),
    }
}

// ... existing code ...

pub async fn upsert(
    auth: HttpRequest,
    pool: web::Data<db::AsyncDbPool>,
    table_name: web::Path<String>,
    request_body: web::Json<UpsertRequestBody>,
    query: web::Query<QueryParams>,
) -> impl Responder {
    let extensions = auth.extensions();
    let auth_data = match extensions.get::<Auth>() {
        Some(data) => data,
        None => {
            log::warn!("Auth data not found in request extensions");
            return HttpResponse::InternalServerError().json(ApiResponse {
                success: false,
                message: "Authentication information not available".to_string(),
                count: 0,
                data: vec![],
            });
        }
    };
    let table_name = table_name.into_inner();
    let request_body = request_body.into_inner();

    // Extract pluck fields from query if provided
    let pluck_fields = if !query.pluck.is_empty() {
        Some(
            query
                .pluck
                .split(',')
                .map(|s| s.trim().to_string())
                .collect(),
        )
    } else {
        None
    };

    // Call the reusable function
    match perform_upsert(
        &table_name,
        request_body.conflict_columns,
        request_body.data,
        request_body.entity_prefix,
        pluck_fields,
        &auth_data,
    )
    .await
    {
        Ok(response) => HttpResponse::Ok().json(response),
        Err(error) => HttpResponse::build(http::StatusCode::from_u16(error.status).unwrap()).json(
            ApiResponse {
                success: false,
                message: error.message,
                count: 0,
                data: vec![],
            },
        ),
    }
}

pub async fn delete_record(
    auth: HttpRequest,
    pool: web::Data<db::AsyncDbPool>,
    path_params: web::Path<(String, String)>,
) -> impl Responder {
    let (table_name, record_id) = path_params.into_inner();
    let extensions = auth.extensions();
    let auth_data = match extensions.get::<Auth>() {
        Some(data) => data,
        None => {
            log::warn!("Auth data not found in request extensions");
            return HttpResponse::InternalServerError().json(ApiResponse {
                success: false,
                message: "Authentication information not available".to_string(),
                count: 0,
                data: vec![],
            });
        }
    };

    // Create delete updates (setting tombstone and status)
    let delete_updates = serde_json::json!({});

    match process_and_update_record(
        &table_name,
        delete_updates,
        &record_id,
        None,
        "delete",
        &auth_data,
    )
    .await
    {
        Ok(mut response) => {
            // Parse the response as Value to modify it
            let mut response_value: serde_json::Value =
                serde_json::from_str(&serde_json::to_string(&response).unwrap()).unwrap();
            if let Some(obj) = response_value.as_object_mut() {
                obj["message"] = serde_json::Value::String(format!(
                    "Record with ID '{}' deleted successfully from '{}'",
                    record_id, table_name
                ));
            }
            HttpResponse::Ok().json(response_value)
        }
        Err(error) => HttpResponse::build(http::StatusCode::from_u16(error.status).unwrap()).json(
            ApiResponse {
                success: false,
                message: error.message,
                count: 0,
                data: vec![],
            },
        ),
    }
}
