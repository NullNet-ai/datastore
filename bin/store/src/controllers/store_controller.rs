use crate::batch_sync::BatchSyncService;
use crate::controllers::common_controller::{
    convert_json_to_csv, execute_copy, process_and_get_record_by_id, process_and_insert_record,
    process_and_update_record, process_records,
};
use crate::db::create_connection;
use crate::providers::aggregation_filter::AggregationSQLConstructor;
use crate::providers::find::{DynamicResult, SQLConstructor, Validation};
use crate::providers::storage::get_valid_bucket_name;
use crate::providers::storage::minio::is_storage_disabled;
use crate::structs::structs::{
    AggregationFilter, ApiResponse, Auth, BatchUpdateBody, GetByFilter, QueryParams, RequestBody,
    SwitchAccountRequest, UpsertRequestBody,
};
use crate::utils::utils::table_exists;
use crate::{db, providers};
use actix_multipart::Multipart;
use actix_web::error::BlockingError;
use actix_web::test;
use actix_web::{http, web, HttpResponse, Responder, ResponseError};
use actix_web::{HttpMessage, HttpRequest};
use aws_sdk_s3::primitives::ByteStream;
use chrono;
use diesel::result::Error as DieselError;
use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;
use ulid::Ulid;
// use std::collections::HashMap;
// use diesel::prelude::*;
use std::fmt;
// use diesel::sql_types::*;
// use diesel::QueryableByName;
use diesel_async::RunQueryDsl;

use super::common_controller::{perform_batch_update, perform_upsert, sanitize_updates};
use futures_util::stream::StreamExt; // For processing multipart stream
use mime_guess; // For MIME type detection from file extensions
#[derive(Serialize, Debug)]
pub struct ApiError {
    pub message: String,
    pub status: u16,
}

impl From<Box<dyn std::error::Error>> for ApiError {
    fn from(error: Box<dyn std::error::Error>) -> Self {
        Self::new(
            http::StatusCode::INTERNAL_SERVER_ERROR,
            format!("Internal server error: {}", error),
        )
    }
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

    // Check if this is a root controller call
    let controller_type = extensions.get::<Option<String>>();
    let is_root_controller = controller_type
        .and_then(|opt| opt.as_ref())
        .map(|s| s == "root")
        .unwrap_or(false);

    // Perform different operations based on controller type
    if is_root_controller {
        log::info!(
            "Processing batch_update_records via root controller for table: {}",
            table_name.as_str()
        );
        // Add any root-specific logic here
    } else {
        log::info!(
            "Processing batch_update_records via simple controller for table: {}",
            table_name.as_str()
        );
        // Add any simple controller-specific logic here
    }

    // Check if this is a root controller call
    let controller_type = extensions.get::<Option<String>>();
    let is_root_controller = controller_type
        .and_then(|opt| opt.as_ref())
        .map(|s| s == "root")
        .unwrap_or(false);

    // Perform different operations based on controller type
    if is_root_controller {
        log::info!(
            "Processing update_record via root controller for table: {}, id: {}",
            table_name,
            record_id
        );
        // Add any root-specific logic here
    } else {
        log::info!(
            "Processing update_record via simple controller for table: {}, id: {}",
            table_name,
            record_id
        );
        // Add any simple controller-specific logic here
    }

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
        is_root_controller,
    )
    .await
    {
        Ok(response) => HttpResponse::Ok().json(response),
        Err(error) => {
            log::error!(
                "Error updating record in table '{}' with ID '{}': {:?}",
                table_name,
                record_id,
                error
            );
            let status_code = http::StatusCode::from_u16(error.status)
                .unwrap_or(http::StatusCode::INTERNAL_SERVER_ERROR);
            HttpResponse::build(status_code).json(ApiResponse {
                success: false,
                message: error.message,
                count: 0,
                data: vec![],
            })
        }
    }
}

pub async fn create_record(
    auth: HttpRequest,
    table: web::Path<String>,
    body: web::Json<serde_json::Value>,
    query: web::Query<QueryParams>,
    app_state: web::Data<providers::storage::AppState>,
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

    // Check if this is a root controller call
    let controller_type = extensions.get::<Option<String>>();
    let is_root_controller = controller_type
        .and_then(|opt| opt.as_ref())
        .map(|s| s == "root")
        .unwrap_or(false);

    let table_name = table.into_inner();

    // Perform different operations based on controller type
    if is_root_controller {
        log::info!(
            "Processing create_record via root controller for table: {}",
            table_name
        );
        // Add any root-specific logic here
    } else {
        log::info!(
            "Processing create_record via simple controller for table: {}",
            table_name
        );
        // Add any simple controller-specific logic here
    }

    // Special handling for organizations table - create bucket
    if table_name == "organizations" {
        log::info!("Creating organization record - will create corresponding bucket");

        // Extract organization name from the request body for bucket creation
        if let Some(org_name) = body.get("name").and_then(|v| v.as_str()) {
            log::info!("Creating bucket for organization: {}", org_name);

            // Generate valid bucket name using the organization name and ID
            let org_id = auth_data.organization_id.as_str();
            let bucket_name =
                providers::storage::minio::get_valid_bucket_name(org_name, Some(org_id));

            log::info!(
                "Generated bucket name: {} for organization: {}",
                bucket_name,
                org_name
            );

            // Create bucket using S3 client
            let s3_client = &app_state.s3_client;
            match s3_client.create_bucket().bucket(&bucket_name).send().await {
                Ok(_) => {
                    log::info!(
                        "Successfully created bucket '{}' for organization '{}'",
                        bucket_name,
                        org_name
                    );
                }
                Err(e) => {
                    // Check if error is because bucket already exists
                    let error_message = format!("{:?}", e);
                    if error_message.contains("BucketAlreadyExists")
                        || error_message.contains("BucketAlreadyOwnedByYou")
                    {
                        log::info!(
                            "Bucket '{}' already exists for organization '{}'",
                            bucket_name,
                            org_name
                        );
                    } else {
                        log::error!(
                            "Failed to create bucket '{}' for organization '{}': {:?}",
                            bucket_name,
                            org_name,
                            e
                        );
                    }
                }
            }
        } else {
            log::warn!("Organization name not found in request body for bucket creation");
        }
    }
    let pluck_fields: Vec<String> = query
        .pluck
        .split(',')
        .map(|s| s.trim().to_string())
        .collect();

    match process_and_insert_record(
        &table_name,
        body.into_inner(),
        Some(pluck_fields),
        &auth_data,
        is_root_controller,
    )
    .await
    {
        Ok(response) => HttpResponse::Ok().json(response),
        Err(error) => {
            log::error!(
                "Error creating record in table '{}': {:?}",
                table_name,
                error
            );
            let status_code = http::StatusCode::from_u16(error.status)
                .unwrap_or(http::StatusCode::INTERNAL_SERVER_ERROR);
            HttpResponse::build(status_code).json(ApiResponse {
                success: false,
                message: error.message,
                count: 0,
                data: vec![],
            })
        }
    }
}

// Add this function to the store_controller.rs file

pub async fn get_by_id(
    auth: HttpRequest,
    path_params: web::Path<(String, String)>,
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

    // Check if this is a root controller call
    let controller_type = extensions.get::<Option<String>>();
    let is_root_controller = controller_type
        .and_then(|opt| opt.as_ref())
        .map(|s| s == "root")
        .unwrap_or(false);

    // Perform different operations based on controller type
    if is_root_controller {
        log::info!(
            "Processing get_by_id via root controller for table: {}, id: {}",
            table_name,
            record_id
        );
        // Add any root-specific logic here
    } else {
        log::info!(
            "Processing get_by_id via simple controller for table: {}, id: {}",
            table_name,
            record_id
        );
        // Add any simple controller-specific logic here
    }

    // Parse pluck fields from query parameters
    let pluck_fields: Vec<String> = if query.pluck.is_empty() {
        vec!["id".to_string()]
    } else {
        query
            .pluck
            .split(',')
            .map(|s| s.trim().to_string())
            .collect()
    };

    // Extract organization_id from auth_data
    let organization_id = Some(auth_data.organization_id.as_str());

    // Use the common function to get the record by ID
    match process_and_get_record_by_id(
        &table_name,
        &record_id,
        Some(pluck_fields),
        is_root_controller,
        organization_id,
    )
    .await
    {
        Ok(response) => HttpResponse::Ok().json(response),
        Err(_error) => {
            log::error!(
                "Error getting record from table '{}' with ID '{}': {:?}",
                table_name,
                record_id,
                _error
            );
            let status_code = http::StatusCode::from_u16(_error.status)
                .unwrap_or(http::StatusCode::INTERNAL_SERVER_ERROR);
            HttpResponse::build(status_code).json(ApiResponse {
                success: false,
                message: _error.message,
                count: 0,
                data: vec![],
            })
        }
    }
}

// ... existing code ...

#[derive(Deserialize)]
pub struct BatchInsertBody {
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
    let controller_type = extensions.get::<Option<String>>();
    let is_root_controller = controller_type
        .and_then(|opt| opt.as_ref())
        .map(|s| s == "root")
        .unwrap_or(false);

    // Perform different operations based on controller type
    if is_root_controller {
        log::info!(
            "Processing batch_update_records via root controller for table: {}",
            table_name.as_str()
        );
        // Add any root-specific logic here
    } else {
        log::info!(
            "Processing batch_update_records via simple controller for table: {}",
            table_name.as_str()
        );
        // Add any simple controller-specific logic here
    }
    let temp_table = format!("temp_{}", table_name);
    match table_exists(&temp_table) {
        Ok(_table) => {
            // Table exists, proceed with your logic using the table
        }
        Err(_error) => {
            // Table doesn't exist, return an error response
            return HttpResponse::BadRequest().json(ApiResponse {
                success: false,
                message: format!(
                    "Error checking table existence: temp table for {} is missing",
                    table_name
                ),
                count: 0,
                data: vec![],
            });
        }
    }

    let table_clone = table_name.clone();
    let batch_data = records.into_inner();
    let json_records = batch_data.records;

    if json_records.is_empty() {
        return HttpResponse::BadRequest().json(ApiResponse {
            success: false,
            message: "No records provided".to_string(),
            count: 0,
            data: vec![],
        });
    }
    let (processed_records, columns) = match process_records(
        json_records,
        &table_name,
        &auth_data.clone(),
        is_root_controller,
    ) {
        Ok((records, cols)) => (records, cols),
        Err(e) => {
            log::error!(
                "Error processing records for batch insert in table '{}': {}",
                table_name,
                e
            );
            return HttpResponse::BadRequest().json(ApiResponse {
                success: false,
                message: format!("Error processing records: {}", e),
                count: 0,
                data: vec![],
            });
        }
    };

    let csv_data = match convert_json_to_csv(&processed_records, &columns) {
        Ok(data) => data,
        Err(e) => {
            log::error!(
                "Error converting records to CSV for batch insert in table '{}': {:?}",
                table_name,
                e
            );
            return HttpResponse::BadRequest().json(ApiResponse {
                success: false,
                message: format!("Error converting records to CSV: {:?}", e),
                count: 0,
                data: vec![],
            });
        }
    };

    let client = match create_connection().await {
        Ok(client) => client,
        Err(e) => {
            log::error!(
                "Error creating database connection for batch insert in table '{}': {:?}",
                table_name,
                e
            );
            return HttpResponse::InternalServerError().json(ApiResponse {
                success: false,
                message: format!("Error creating database connection: {:?}", e),
                count: 0,
                data: vec![],
            });
        }
    };

    let column_refs: Vec<&str> = columns.iter().map(|s| s.as_str()).collect();

    match execute_copy(&client, &table_name, &column_refs, csv_data).await {
        Ok(_) => processed_records.clone(),
        Err(e) => {
            log::error!(
                "Error executing COPY command for batch insert in table '{}': {:?}",
                table_name,
                e
            );
            return HttpResponse::InternalServerError().json(ApiResponse {
                success: false,
                message: format!("Error executing COPY command: {:?}", e),
                count: 0,
                data: vec![],
            });
        }
    };

    // Convert JSON array to CSV in-memor

    for record in processed_records.iter() {
        if let Some(id) = record.get("id").and_then(|v| v.as_str()) {
            if let Err(e) = BatchSyncService::send_code_assignment_message(
                table_clone.clone(),
                id.to_string(),
                "".to_string(),
                auth_data.clone(),
                true,
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

    // Check if this is a root controller call
    let controller_type = extensions.get::<Option<String>>();
    let is_root_controller = controller_type
        .and_then(|opt| opt.as_ref())
        .map(|s| s == "root")
        .unwrap_or(false);

    // Perform different operations based on controller type
    if is_root_controller {
        log::info!(
            "Processing batch_update_records via root controller for table: {}",
            table
        );
        // Add any root-specific logic here
    } else {
        log::info!(
            "Processing batch_update_records via simple controller for table: {}",
            table
        );
        // Add any simple controller-specific logic here
    }

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
    updates.process_record("update", &auth_data, is_root_controller, &table_name);
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
        Err(e) => {
            log::error!(
                "Error performing batch update in table '{}': {}",
                table_name,
                e
            );
            HttpResponse::InternalServerError().json(ApiResponse {
                success: false,
                message: e,
                count: 0,
                data: vec![],
            })
        }
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

    // Check if this is a root controller call
    let controller_type = extensions.get::<Option<String>>();
    let is_root_controller = controller_type
        .and_then(|opt| opt.as_ref())
        .map(|s| s == "root")
        .unwrap_or(false);

    // Perform different operations based on controller type
    if is_root_controller {
        log::info!(
            "Processing batch_delete_records via root controller for table: {}",
            table.as_str()
        );
        // Add any root-specific logic here
    } else {
        log::info!(
            "Processing batch_delete_records via simple controller for table: {}",
            table.as_str()
        );
        // Add any simple controller-specific logic here
    }

    let table_name = table.into_inner();
    let batch_data = request.into_inner();
    let filters = batch_data.advance_filters;

    // Create delete updates (setting tombstone and status)
    let mut delete_updates = RequestBody {
        record: serde_json::json!({}),
    };

    // Process the record through the common processing logic
    delete_updates.process_record("delete", &auth_data, is_root_controller, &table_name);
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
        Err(e) => {
            log::error!(
                "Error performing batch delete in table '{}': {}",
                table_name,
                e
            );
            HttpResponse::InternalServerError().json(ApiResponse {
                success: false,
                message: e,
                count: 0,
                data: vec![],
            })
        }
    }
}

// ... existing code ...

pub async fn upsert(
    auth: HttpRequest,
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

    // Check if this is a root controller call
    let controller_type = extensions.get::<Option<String>>();
    let is_root_controller = controller_type
        .and_then(|opt| opt.as_ref())
        .map(|s| s == "root")
        .unwrap_or(false);

    // Perform different operations based on controller type
    if is_root_controller {
        log::info!(
            "Processing upsert via root controller for table: {}",
            table_name
        );
        // Add any root-specific logic here
    } else {
        log::info!(
            "Processing upsert via simple controller for table: {}",
            table_name
        );
        // Add any simple controller-specific logic here
    }

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
        pluck_fields,
        &auth_data,
        is_root_controller,
    )
    .await
    {
        Ok(response) => HttpResponse::Ok().json(response),
        Err(error) => {
            log::error!(
                "Error performing upsert in table '{}': {}",
                table_name,
                error.message
            );
            let status_code = http::StatusCode::from_u16(error.status)
                .unwrap_or(http::StatusCode::INTERNAL_SERVER_ERROR);
            HttpResponse::build(status_code).json(ApiResponse {
                success: false,
                message: error.message,
                count: 0,
                data: vec![],
            })
        }
    }
}

pub async fn delete_record(
    auth: HttpRequest,
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

    // Check if this is a root controller call
    let controller_type = extensions.get::<Option<String>>();
    let is_root_controller = controller_type
        .and_then(|opt| opt.as_ref())
        .map(|s| s == "root")
        .unwrap_or(false);

    // Perform different operations based on controller type
    if is_root_controller {
        log::info!(
            "Processing delete_record via root controller for table: {}, id: {}",
            table_name,
            record_id
        );
        // Add any root-specific logic here
    } else {
        log::info!(
            "Processing delete_record via simple controller for table: {}, id: {}",
            table_name,
            record_id
        );
        // Add any simple controller-specific logic here
    }

    let _organization_id = &auth_data.organization_id;

    // Create delete updates (setting tombstone and status)
    let delete_updates = serde_json::json!({});

    match process_and_update_record(
        &table_name,
        delete_updates,
        &record_id,
        None,
        "delete",
        &auth_data,
        is_root_controller,
    )
    .await
    {
        Ok(response) => {
            // Parse the response as Value to modify it
            // let mut response_value: serde_json::Value =
            //     serde_json::from_str(&serde_json::to_string(&response).unwrap()).unwrap();

            let mut response_value: serde_json::Value = serde_json::to_value(&response)
                .unwrap_or_else(|e| {
                    log::error!("Failed to convert response: {}", e);
                    serde_json::json!({
                        "success": false,
                        "message": "Failed to process response, while updating, in process and update record",
                        "count": 0,
                        "data": []
                    })
                });
            if let Some(obj) = response_value.as_object_mut() {
                obj["message"] = serde_json::Value::String(format!(
                    "Record with ID '{}' deleted successfully from '{}'",
                    record_id, table_name
                ));
            }
            HttpResponse::Ok().json(response_value)
        }
        Err(error) => {
            log::error!(
                "Error deleting record from table '{}' with ID '{}': {}",
                table_name,
                record_id,
                error.message
            );
            let status_code = http::StatusCode::from_u16(error.status)
                .unwrap_or(http::StatusCode::INTERNAL_SERVER_ERROR);
            HttpResponse::build(status_code).json(ApiResponse {
                success: false,
                message: error.message,
                count: 0,
                data: vec![],
            })
        }
    }
}

//get by filter

pub async fn get_by_filter(
    auth: HttpRequest,
    path_params: web::Path<String>,
    request_body: web::Json<GetByFilter>,
) -> impl Responder {
    let parameters = request_body.into_inner();
    let table = path_params.into_inner();
    let is_root = auth
        .extensions()
        .get::<Auth>()
        .map_or(false, |auth_data| auth_data.is_root_account);

    // Extract organization_id from auth context
    let extensions = auth.extensions();
    let organization_id = match extensions.get::<Auth>() {
        Some(auth_data) => Some(auth_data.organization_id.clone()),
        None => {
            log::warn!("Auth data not found in request extensions");
            None
        }
    };

    // Check if this is a root controller call
    let controller_type = extensions.get::<Option<String>>();
    let is_root_controller = controller_type
        .and_then(|opt| opt.as_ref())
        .map(|s| s == "root")
        .unwrap_or(false);

    // Perform different operations based on controller type
    if is_root_controller {
        log::info!(
            "Processing get_by_filter via root controller for table: {}",
            table
        );
        // Add any root-specific logic here
    } else {
        log::info!(
            "Processing get_by_filter via simple controller for table: {}",
            table
        );
        // Add any simple controller-specific logic here
    }

    let validation = Validation::new(&parameters, &table);
    let ApiResponse {
        success,
        message,
        count,
        data,
    } = validation.exec();
    if !success {
        return HttpResponse::BadRequest().json(ApiResponse {
            success,
            message,
            count,
            data,
        });
    }

    // Create SQLConstructor with organization_id if available
    let mut sql_constructor = SQLConstructor::new(parameters, table.clone(), is_root);
    if let Some(org_id) = organization_id {
        sql_constructor = sql_constructor.with_organization_id(org_id);
    }

    let query = match sql_constructor.construct() {
        Ok(sql) => sql,
        Err(e) => {
            return HttpResponse::BadRequest().json(ApiResponse {
                success: false,
                message: format!("Invalid filter configuration: {}", e),
                count: 0,
                data: vec![],
            });
        }
    };

    // Get a connection from the pool
    let mut conn = db::get_async_connection().await;

    // Wrap your original query with row_to_json
    // This is slower approach by flixible
    // TODO: create a better way of handling dynamic queries
    let final_query = format!("SELECT row_to_json(t) FROM ({}) t", query);

    let results = match diesel::dsl::sql_query(&final_query)
        .load::<DynamicResult>(&mut conn)
        .await
    {
        Ok(results) => results,
        Err(e) => {
            log::error!("Error executing query for table '{}': {:?}", table, e);
            return HttpResponse::InternalServerError().json(ApiResponse {
                success: false,
                message: format!("Query execution error: {}", e),
                count: 0,
                data: vec![],
            });
        }
    };
    // Extract JSON values directly (no string parsing needed)
    let data: Vec<serde_json::Value> = results
        .into_iter()
        .filter_map(|result| result.row_to_json)
        .collect();

    HttpResponse::Ok().json(ApiResponse {
        success: true,
        message: format!("Filter operation completed for table: {}", &table),
        count: data.len() as i32,
        data,
    })
}

//aggregation filter

pub async fn aggregation_filter(
    auth: HttpRequest,
    request_body: web::Json<AggregationFilter>,
) -> impl Responder {
    let parameters = request_body.into_inner();
    let table = parameters.entity.clone();
    let is_root = auth
        .extensions()
        .get::<Auth>()
        .map_or(false, |auth_data| auth_data.is_root_account);
    // Extract organization_id from auth context
    let extensions = auth.extensions();
    let organization_id = match extensions.get::<Auth>() {
        Some(auth_data) => Some(auth_data.organization_id.clone()),
        None => {
            log::warn!("Auth data not found in request extensions");
            None
        }
    };

    // Create AggregationSQLConstructor with organization_id if available
    let mut sql_constructor = AggregationSQLConstructor::new(parameters, table.clone(), is_root);
    if let Some(org_id) = organization_id {
        sql_constructor = sql_constructor.with_organization_id(org_id);
    }

    let query = match sql_constructor.construct_aggregation() {
        Ok(sql) => sql,
        Err(e) => {
            return HttpResponse::BadRequest().json(ApiResponse {
                success: false,
                message: format!("Invalid aggregation configuration: {}", e),
                count: 0,
                data: vec![],
            });
        }
    };

    let mut conn = db::get_async_connection().await;

    // Wrap your original query with row_to_json
    let final_query = format!("SELECT row_to_json(t) FROM ({}) t", query);

    let results = match diesel::dsl::sql_query(&final_query)
        .load::<DynamicResult>(&mut conn)
        .await
    {
        Ok(results) => results,
        Err(e) => {
            log::error!(
                "Error executing aggregation query for table '{}': {:?}",
                table,
                e
            );
            return HttpResponse::InternalServerError().json(ApiResponse {
                success: false,
                message: format!("Query execution error: {}", e),
                count: 0,
                data: vec![],
            });
        }
    };

    // Extract JSON values directly (no string parsing needed)
    let data: Vec<serde_json::Value> = results
        .into_iter()
        .filter_map(|result| result.row_to_json)
        .collect();

    HttpResponse::Ok().json(ApiResponse {
        success: true,
        message: format!("Aggregation operation completed for table: {}", &table),
        count: data.len() as i32,
        data,
    })
}

// files implementation
// Query
// TODO: get file metadat from database
pub async fn get_file_by_id(
    auth: HttpRequest,
    path_params: web::Path<String>,
    query: web::Query<QueryParams>,
) -> impl Responder {
    let file_id = path_params.into_inner();

    // Extract auth data for organization context
    let extensions = auth.extensions();
    let auth_data = match extensions.get::<Auth>() {
        Some(data) => data,
        None => {
            log::warn!("Auth data not found in request extensions");
            return HttpResponse::Unauthorized().json(ApiResponse {
                success: false,
                message: "Authentication required".to_string(),
                count: 0,
                data: vec![],
            });
        }
    };
    dbg!(format!("authdata {:?} file_id {:?}", auth_data, file_id));
    // Check if this is a root controller call
    let controller_type = extensions.get::<Option<String>>();
    let is_root_controller = controller_type
        .and_then(|opt| opt.as_ref())
        .map(|s| s == "root")
        .unwrap_or(false);

    // Log the operation
    if is_root_controller {
        log::info!(
            "Processing get_file_by_id via root controller for file_id: {}",
            file_id
        );
    } else {
        log::info!(
            "Processing get_file_by_id via simple controller for file_id: {}",
            file_id
        );
    }

    // Parse pluck fields from query parameters or use default file fields
    let pluck_fields: Vec<String> = if query.pluck.is_empty() {
        vec![
            "id".to_string(),
            "image_url".to_string(),
            "filename".to_string(),
            "mimetype".to_string(),
            "size".to_string(),
            "download_path".to_string(),
            "presigned_url".to_string(),
            "created_date".to_string(),
            "created_time".to_string(),
            "updated_date".to_string(),
            "updated_time".to_string(),
            "organization_id".to_string(),
        ]
    } else {
        query
            .pluck
            .split(',')
            .map(|s| s.trim().to_string())
            .collect()
    };

    // Extract organization_id from auth_data
    let organization_id = Some(auth_data.organization_id.as_str());

    // Use the common controller to get file metadata from database
    match process_and_get_record_by_id(
        "files",
        &file_id,
        Some(pluck_fields),
        is_root_controller,
        organization_id,
    )
    .await
    {
        Ok(response) => HttpResponse::Ok().json(response),
        Err(error) => {
            log::error!("Error retrieving file {}: {:?}", file_id, error);
            let status_code = http::StatusCode::from_u16(error.status)
                .unwrap_or(http::StatusCode::INTERNAL_SERVER_ERROR);
            HttpResponse::build(status_code).json(ApiResponse {
                success: false,
                message: error.message,
                count: 0,
                data: vec![],
            })
        }
    }
}
// Download file by ID with streaming response for client preview
pub async fn download_file_by_id(
    auth: HttpRequest,
    path_params: web::Path<String>,
    _query: web::Query<std::collections::HashMap<String, String>>,
    app_state: web::Data<providers::storage::AppState>,
) -> HttpResponse {
    let file_id = path_params.into_inner();

    // Extract auth data for organization context
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

    // Check if this is a root controller call
    let controller_type = extensions.get::<Option<String>>();
    let is_root_controller = controller_type
        .and_then(|opt| opt.as_ref())
        .map(|s| s == "root")
        .unwrap_or(false);

    // Log the operation
    if is_root_controller {
        log::info!(
            "Processing download_file_by_id via root controller for file_id: {}",
            file_id
        );
    } else {
        log::info!(
            "Processing download_file_by_id via simple controller for file_id: {}",
            file_id
        );
    }

    // First, get file metadata from database
    let pluck_fields = vec![
        "mimetype".to_string(),
        "download_path".to_string(),
        "size".to_string(),
    ];

    // Extract organization_id from auth_data
    let organization_id = Some(auth_data.organization_id.as_str());
    let file_metadata = match process_and_get_record_by_id(
        "files",
        &file_id,
        Some(pluck_fields),
        is_root_controller,
        organization_id,
    )
    .await
    {
        Ok(response) => {
            if response.success && !response.data.is_empty() {
                response.data[0].clone()
            } else {
                return HttpResponse::NotFound().json(ApiResponse {
                    success: false,
                    message: "File not found".to_string(),
                    count: 0,
                    data: vec![],
                });
            }
        }
        Err(e) => {
            log::error!("Error retrieving file metadata {}: {:?}", file_id, e);
            return HttpResponse::InternalServerError().json(ApiResponse {
                success: false,
                message: format!("Failed to retrieve file metadata: {}", e),
                count: 0,
                data: vec![],
            });
        }
    };

    // Extract file information from metadata
    let mimetype = file_metadata
        .get("mimetype")
        .and_then(|v| v.as_str())
        .unwrap_or("application/octet-stream");
    let download_path = file_metadata
        .get("download_path")
        .and_then(|v| v.as_str())
        .unwrap_or(&file_id);

    // Get bucket name with organization context
    let base_bucket_name =
        std::env::var("STORAGE_BUCKET_NAME").unwrap_or_else(|_| app_state.bucket_name.clone());
    let bucket_name = get_valid_bucket_name(&base_bucket_name, organization_id);

    // Extract just the filename from download_path (remove bucket name if present)
    let s3_key = if download_path.contains('/') {
        // If download_path contains '/', take the part after the last '/'
        download_path.split('/').last().unwrap_or(&file_id)
    } else {
        // If no '/', use the download_path as is (it's just the filename)
        download_path
    };

    // Stream file from S3
    let s3_client = &app_state.s3_client;

    match s3_client
        .get_object()
        .bucket(&bucket_name)
        .key(s3_key)
        .send()
        .await
    {
        Ok(output) => {
            // Use the mimetype from database for proper content type handling
            // This ensures correct MIME type detection for preview functionality
            let actual_content_type = mimetype.to_string();

            // Capture content length before consuming the body
            let content_length = output.content_length().unwrap_or(0);

            // Convert the S3 body stream to bytes and create a streaming response
            match output.body.collect().await {
                Ok(data) => {
                    let bytes = data.into_bytes();

                    // Create a stream from the bytes for efficient streaming
                    use futures_util::stream;
                    let byte_stream = stream::once(async move { Ok::<_, std::io::Error>(bytes) });

                    // Determine if this is an image for inline display
                    let is_image = actual_content_type.starts_with("image/");
                    let filename = s3_key.split('/').last().unwrap_or("file");

                    // Set Content-Disposition for proper preview behavior
                    let content_disposition = if is_image {
                        // For images, use inline disposition to enable preview in browsers/Postman
                        format!("inline; filename=\"{}\"", filename)
                    } else {
                        // For non-images, use attachment to trigger download
                        format!("attachment; filename=\"{}\"", filename)
                    };

                    HttpResponse::Ok()
                        .content_type(actual_content_type)
                        .insert_header(("Content-Length", content_length.to_string()))
                        .insert_header(("Cache-Control", "public, max-age=3600"))
                        .insert_header(("Accept-Ranges", "bytes"))
                        .insert_header(("Content-Disposition", content_disposition))
                        .streaming(byte_stream)
                }
                Err(e) => {
                    log::error!("Error reading S3 object body for file {}: {:?}", file_id, e);
                    HttpResponse::InternalServerError().json(ApiResponse {
                        success: false,
                        message: "Failed to read file content".to_string(),
                        count: 0,
                        data: vec![],
                    })
                }
            }
        }
        Err(e) => {
            log::error!("Error downloading file {} from S3: {:?}", file_id, e);
            HttpResponse::NotFound().json(ApiResponse {
                success: false,
                message: "File not found in storage".to_string(),
                count: 0,
                data: vec![],
            })
        }
    }
}

pub async fn upload_file(
    req: HttpRequest,
    app_state: web::Data<providers::storage::AppState>,
    mut multipart: Multipart,
) -> impl Responder {
    // Check if storage is disabled
    if is_storage_disabled() {
        log::info!("Storage is disabled (DISABLE_STORAGE=true), returning mock upload response");

        // Generate mock file metadata for disabled storage
        let mock_id = Ulid::new().to_string();
        let mock_metadata = serde_json::json!({
            "id": mock_id,
            "status": "mock_uploaded",
            "previous_status": "",
            "created_date": chrono::Utc::now().format("%Y-%m-%d").to_string(),
            "created_time": chrono::Utc::now().format("%H:%M:%S").to_string(),
            "updated_date": chrono::Utc::now().format("%Y-%m-%d").to_string(),
            "updated_time": chrono::Utc::now().format("%H:%M:%S").to_string(),
            "organization_id": "",
            "created_by": "",
            "updated_by": "",
            "deleted_by": "",
            "requested_by": "",
            "tags": [],
            "image_url": format!("mock-bucket/{}.png", mock_id),
            "fieldname": "files",
            "originalname": "mock-file.png",
            "encoding": "7bit",
            "mimetype": "image/png",
            "destination": "mock-bucket",
            "filename": format!("{}.png", mock_id),
            "path": format!("mock-bucket/{}.png", mock_id),
            "size": 1024,
            "uploaded_by": "",
            "downloaded_by": "",
            "etag": "mock-etag",
            "version_id": "",
            "download_path": format!("mock-bucket/{}.png", mock_id),
            "presigned_url": "",
            "presigned_url_expire": 0
        });

        return HttpResponse::Ok().json(ApiResponse {
            success: true,
            message: "Mock upload successful (storage disabled)".to_string(),
            count: 1,
            data: vec![mock_metadata],
        });
    }

    // Check for Auth data early and abort if missing
    let extensions = req.extensions();
    let _auth_data = match extensions.get::<Auth>() {
        Some(data) => data,
        None => {
            log::error!("Auth data not found in request extensions - aborting upload process");
            return HttpResponse::Unauthorized().json(ApiResponse {
                success: false,
                message: "Authentication required for file upload".to_string(),
                count: 0,
                data: vec![],
            });
        }
    };

    if let Some(content_type_header) = req.headers().get(actix_web::http::header::CONTENT_TYPE) {
        log::info!("Incoming Content-Type header: {:?}", content_type_header);
    }
    let name = "files";
    let client = app_state.s3_client.clone();
    let bucket_name = app_state.bucket_name.clone();
    let mut uploaded_files_count = 0;
    let mut file_metadata = Vec::new();
    let pluck_fields = vec!["id".to_string()];
    while let Some(field_result) = multipart.next().await {
        let mut field = match field_result {
            Ok(field) => field,
            Err(e) => {
                log::error!("Error getting field from multipart: {:?}", e);
                return HttpResponse::InternalServerError().body(format!("Multipart error: {}", e));
            }
        };

        let content_disposition = field.content_disposition();
        let fname = content_disposition.get_filename().map(|s| s.to_string());
        let field_name = content_disposition.get_name().unwrap_or("file").to_string();

        // Get content type from multipart field
        let field_content_type = field.content_type().map(|ct| ct.to_string());

        // Determine the best content type using multiple sources
        let content_type = if let Some(fname_ref) = &fname {
            // First try to detect MIME type from file extension
            let mime_from_extension = mime_guess::from_path(fname_ref).first_or_octet_stream();
            let detected_mime = mime_from_extension.to_string();

            log::info!(
                "MIME type detection for '{}': detected='{}', field_provided={:?}",
                fname_ref,
                detected_mime,
                field_content_type
            );

            // Use detected MIME type if it's not generic, otherwise fall back to field content type
            if detected_mime != "application/octet-stream" {
                log::info!("Using detected MIME type: {}", detected_mime);
                detected_mime
            } else {
                let fallback = field_content_type.unwrap_or("application/octet-stream".to_string());
                log::info!("Using fallback MIME type: {}", fallback);
                fallback
            }
        } else {
            // No filename available, use field content type
            let fallback = field_content_type.unwrap_or("application/octet-stream".to_string());
            log::info!(
                "No filename available, using field content type: {}",
                fallback
            );
            fallback
        };

        if let Some(fname) = fname {
            // Generate unique filename with ID.extension format
            let extension = std::path::Path::new(&fname)
                .extension()
                .and_then(|ext| ext.to_str())
                .unwrap_or("png"); // Default to png if no extension

            // Generate a new ID for potential upload
            let new_id = Ulid::new().to_string();
            let new_unique_filename = format!("{}.{}", new_id, extension);

            // First, try to find if this file already exists by listing all files with same extension
            let list_result = client
                .list_objects_v2()
                .bucket(&bucket_name)
                .prefix("") // List all files
                .send()
                .await;

            let mut existing_file_key: Option<String> = None;
            let mut existing_id: Option<String> = None;

            // Read the uploaded file content to compare with existing files
            let mut file_data = Vec::new();
            while let Some(chunk) = field.next().await {
                if let Ok(bytes) = chunk {
                    file_data.extend_from_slice(&bytes);
                }
            }

            // Check if any existing files match our content by comparing file sizes and content
            if let Ok(list_output) = list_result {
                let objects = list_output.contents();
                for object in objects {
                    if let Some(key) = object.key() {
                        // Check if this is a file with the same extension
                        if key.ends_with(&format!(".{}", extension)) {
                            // Try to get the object and compare content
                            if let Ok(existing_obj) = client
                                .get_object()
                                .bucket(&bucket_name)
                                .key(key)
                                .send()
                                .await
                            {
                                // Compare file sizes first (quick check)
                                if existing_obj.content_length().unwrap_or(0)
                                    == file_data.len() as i64
                                {
                                    // If sizes match, this might be the same file
                                    // Extract ID from filename (format: "ID.extension")
                                    if let Some(filename) = key.split('/').last() {
                                        if let Some(id_part) = filename.split('.').next() {
                                            existing_file_key = Some(key.to_string());
                                            existing_id = Some(id_part.to_string());
                                            break;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // If we found an existing file, use its ID and key
            let (final_id, final_filename) = if let (Some(existing_key), Some(existing_id_val)) =
                (existing_file_key.clone(), existing_id.clone())
            {
                (existing_id_val, existing_key)
            } else {
                // No existing file found, use new ID
                (new_id.clone(), new_unique_filename.clone())
            };

            // Check if file exists using the determined filename
            let get_result = client
                .get_object()
                .bucket(&bucket_name)
                .key(&final_filename)
                .send()
                .await;

            match get_result {
                Ok(get_output) => {
                    // File already exists in MinIO, use the extracted ID from filename
                    let actual_id = final_id.clone();
                    let actual_filename = final_filename.clone();

                    dbg!(format!(
                        "Found existing file with ID '{}' and filename '{}'",
                        actual_id, actual_filename
                    ));

                    // File already exists in MinIO, return comprehensive metadata
                    let metadata = serde_json::json!({
                        "id": actual_id.clone(),
                        // "categories": [],
                        // "code": "",
                        // "tombstone": false,
                        "status": "already_exists",
                        "previous_status": "",
                        // "version": 1,
                        // "created_date": chrono::Utc::now().format("%Y-%m-%d").to_string(),
                        // "created_time": chrono::Utc::now().format("%H:%M:%S").to_string(),
                        "updated_date": get_output.last_modified()
                            .map(|dt| {
                                // Convert AWS SDK DateTime to chrono DateTime
                                let timestamp = dt.as_secs_f64();
                                let chrono_dt = chrono::DateTime::from_timestamp(timestamp as i64, (timestamp.fract() * 1_000_000_000.0) as u32)
                                    .unwrap_or_else(|| chrono::Utc::now());
                                chrono_dt.format("%Y-%m-%d").to_string()
                            })
                            .unwrap_or_else(|| "Unknown".to_string()),
                        "updated_time": get_output.last_modified()
                            .map(|dt| {
                                // Convert AWS SDK DateTime to chrono DateTime
                                let timestamp = dt.as_secs_f64();
                                let chrono_dt = chrono::DateTime::from_timestamp(timestamp as i64, (timestamp.fract() * 1_000_000_000.0) as u32)
                                    .unwrap_or_else(|| chrono::Utc::now());
                                chrono_dt.format("%H:%M:%S").to_string()
                            })
                            .unwrap_or_else(|| "Unknown".to_string()),
                        "organization_id": "", // TODO: Extract from auth context
                        "created_by": "", // TODO: Extract from auth context
                        "updated_by": "", // TODO: Extract from auth context
                        "deleted_by": "",
                        "requested_by": "", // TODO: Extract from auth context
                        // "timestamp": chrono::Utc::now().timestamp(),
                        "tags": [],
                        "image_url": format!("{}/{}", bucket_name, actual_filename),
                        "fieldname": field_name.clone(),
                        "originalname": fname.clone(),
                        "encoding": "7bit", // Default encoding for multipart
                        "mimetype": content_type.clone(),
                        "destination": bucket_name.clone(),
                        "filename": actual_filename.clone(),
                        "path": format!("{}/{}", bucket_name, actual_filename),
                        "size": get_output.content_length().unwrap_or(0),
                        // "uploaded_by": "", // TODO: Extract from auth context
                        // "downloaded_by": "",
                        "etag": get_output.e_tag().unwrap_or("Unknown"),
                        "version_id": get_output.version_id().unwrap_or(""),
                        "download_path": format!("{}/{}", bucket_name, actual_filename),
                        "presigned_url": "", // TODO: Generate presigned URL if needed
                        "presigned_url_expire": 0, // TODO: Set expiration timestamp
                        // "last_modified": get_output.last_modified()
                        //     .map(|dt| dt.to_string())
                        //     .unwrap_or_else(|| "Unknown".to_string())
                    });
                    // For existing files, try to save to database (will handle duplicates gracefully)
                    let auth_data = _auth_data;

                    // Use create_record function to save metadata
                    let req = test::TestRequest::default()
                        .insert_header(("content-type", "application/json"))
                        .to_http_request();
                    req.extensions_mut().insert(auth_data.clone());

                    let table_path = web::Path::from(name.to_string());
                    let body = web::Json(metadata.clone());
                    let query = web::Query(QueryParams {
                        pluck: pluck_fields.join(","),
                    });

                    let _response =
                        create_record(req, table_path, body, query, app_state.clone()).await;
                    // For existing files, add metadata regardless of database operation result
                    file_metadata.push(metadata.clone());
                    log::info!(
                        "File '{}' already exists in MinIO with unique name '{}', skipping upload",
                        fname,
                        actual_filename
                    );

                    // File data already read earlier for comparison, no need to read again
                    continue;
                }
                Err(_) => {
                    // File doesn't exist in MinIO, proceed with upload
                    log::info!("File '{}' doesn't exist in MinIO, proceeding with upload using filename '{}'", fname, final_filename);
                }
            }

            // File data already read earlier for comparison, use it for upload
            // ✅ Convert Vec<u8> -> ByteStream
            let byte_stream = ByteStream::from(file_data.clone());

            // ✅ Upload to AWS S3 first with unique filename
            let upload_result = client
                .put_object()
                .bucket(&bucket_name)
                .key(&final_filename)
                .body(byte_stream)
                .send()
                .await;

            match upload_result {
                Ok(put_output) => {
                    uploaded_files_count += 1;
                    log::info!(
                        "Successfully uploaded '{}' to AWS S3 with unique name '{}'.",
                        fname,
                        final_filename
                    );
                    // File uploaded successfully to MinIO - add comprehensive metadata
                    let metadata = serde_json::json!({
                        "id": final_id.clone(),
                        // "categories": [],
                        // "code": "",
                        // "tombstone": false,
                        "status": "uploaded",
                        "previous_status": "",
                        // "version": 1,
                        "created_date": chrono::Utc::now().format("%Y-%m-%d").to_string(),
                        "created_time": chrono::Utc::now().format("%H:%M:%S").to_string(),
                        "updated_date": chrono::Utc::now().format("%Y-%m-%d").to_string(),
                        "updated_time": chrono::Utc::now().format("%H:%M:%S").to_string(),
                        "organization_id": "", // TODO: Extract from auth context
                        "created_by": "", // TODO: Extract from auth context
                        "updated_by": "", // TODO: Extract from auth context
                        "deleted_by": "",
                        "requested_by": "", // TODO: Extract from auth context
                        // "timestamp": chrono::Utc::now().timestamp(),
                        "tags": [],
                        "image_url": format!("{}/{}", bucket_name, final_filename),
                        "fieldname": field_name,
                        "originalname": fname.clone(),
                        "encoding": "7bit", // Default encoding for multipart
                        "mimetype": content_type.clone(),
                        "destination": bucket_name.clone(),
                        "filename": final_filename.clone(),
                        "path": format!("{}/{}", bucket_name, final_filename),
                        "size": file_data.len(),
                        "uploaded_by": "", // TODO: Extract from auth context
                        "downloaded_by": "",
                        "etag": put_output.e_tag().unwrap_or("Unknown"),
                        "version_id": put_output.version_id().unwrap_or(""),
                        "download_path": format!("{}/{}", bucket_name, final_filename),
                        "presigned_url": "", // TODO: Generate presigned URL if needed
                        "presigned_url_expire": 0 // TODO: Set expiration timestamp
                    });
                    dbg!(format!(
                        "Complete file metadata for uploaded file: {:?}",
                        metadata
                    ));
                    // Save file metadata to the database using process_and_insert_record
                    let auth_data = _auth_data;

                    // Use create_record function to save metadata
                    let req = test::TestRequest::default()
                        .insert_header(("content-type", "application/json"))
                        .to_http_request();
                    req.extensions_mut().insert(auth_data.clone());

                    let table_path = web::Path::from(name.to_string());
                    let body = web::Json(metadata.clone());
                    let query = web::Query(QueryParams {
                        pluck: pluck_fields.join(","),
                    });

                    let _response =
                        create_record(req, table_path, body, query, app_state.clone()).await;
                    log::info!("Attempted to save file metadata to database for '{}' with unique name '{}' using create_record", fname, final_filename);
                    // Add the metadata to response
                    file_metadata.push(metadata);
                }
                Err(e) => {
                    log::error!("AWS S3 upload error for '{}': {:?}", fname, e);
                    return HttpResponse::InternalServerError()
                        .body(format!("Upload failed: {}", e));
                }
            }
        }
    }

    let existing_count = file_metadata
        .iter()
        .filter(|m| m["status"] == "already_exists")
        .count();

    let response_message = if existing_count > 0 {
        if uploaded_files_count > 0 {
            format!(
                "Uploaded {} new file(s), {} file(s) already existed in MinIO",
                uploaded_files_count, existing_count
            )
        } else {
            format!("All {} file(s) already exist in MinIO", existing_count)
        }
    } else {
        format!("Successfully uploaded {} file(s)", uploaded_files_count)
    };

    HttpResponse::Ok().json(ApiResponse {
        success: true,
        message: response_message,
        count: (uploaded_files_count + existing_count) as i32,
        data: file_metadata,
    })
}

pub async fn switch_account(request: web::Json<SwitchAccountRequest>) -> impl Responder {
    use crate::auth::auth_service;
    use crate::organizations::auth_service as org_auth_service;
    use serde_json::json;

    // Verify the token
    let claims = match auth_service::verify(&request.data.token) {
        Ok(claims) => claims,
        Err(e) => {
            log::error!("Token verification failed: {}", e);
            return HttpResponse::Unauthorized().json(json!({
                "success": false,
                "message": "Invalid token"
            }));
        }
    };

    // Extract account information from claims
    let account = &claims.account;
    let signed_in_account = claims
        .previously_logged_in
        .map(|s| json!({"account_id": s}))
        .unwrap_or_else(|| json!({}));
    let organization_id = &account.organization_id;
    let account_id = &account.account_id;
    let account_organization_id = Some(account.account_organization_id.as_str());

    // Get the logged in account
    let logged_account = match org_auth_service::get_account(
        account_id,
        Some(organization_id),
        account_organization_id,
        None, // account_id for lookup
    )
    .await
    {
        Ok(Some(account)) => account,
        Ok(None) => {
            return HttpResponse::BadRequest().json(json!({
                "success": false,
                "message": "[Switch Account]: Logged in account not found"
            }));
        }
        Err(e) => {
            log::error!("Error fetching logged account: {}", e);
            return HttpResponse::InternalServerError().json(json!({
                "success": false,
                "message": "Internal server error"
            }));
        }
    };

    // Get the target account
    let logged_account_id = logged_account.get("id").and_then(|v| v.as_str());
    let target_account = match org_auth_service::get_account(
        account_id,
        Some(&request.data.organization_id),
        None, // account_organization_id
        logged_account_id,
    )
    .await
    {
        Ok(Some(account)) => account,
        Ok(None) => {
            return HttpResponse::BadRequest().json(json!({
                "success": false,
                "message": "[Switch Account]: Target account not found"
            }));
        }
        Err(e) => {
            log::error!("Error fetching target account: {}", e);
            return HttpResponse::InternalServerError().json(json!({
                "success": false,
                "message": "Internal server error"
            }));
        }
    };

    // Create new token value
    let new_token_value = json!({
        "account": target_account,
        "signed_in_account": signed_in_account,
        "as_root": false
    });

    // Sign the new token
    let token = match org_auth_service::sign(&new_token_value).await {
        Ok(token) => token,
        Err(e) => {
            log::error!("Token generation failed: {}", e);
            return HttpResponse::Forbidden().json(json!({
                "success": false,
                "message": "[Switch Account]: Token not generated"
            }));
        }
    };

    HttpResponse::Ok().json(json!({
        "success": true,
        "message": "Account switched successfully",
        "token": token
    }))
}
