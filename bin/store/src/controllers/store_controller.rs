use crate::config::core::EnvConfig;
use crate::controllers::common_controller::{
    convert_json_to_csv, execute_copy, migration_mode_enabled, process_and_get_record_by_id,
    process_and_insert_record, process_and_update_record, process_records,
};
use crate::database::db::create_connection;
use crate::providers::operations::batch_sync::batch_sync::BatchSyncService;
use crate::providers::queries::aggregation_filter::AggregationSQLConstructor;
use crate::providers::queries::batch_update::BatchUpdateSQLConstructor;
use crate::providers::queries::find::{DynamicResult, SQLConstructor, Validation};
use crate::providers::queries::search_suggestion::{
    sql_constructor::SQLConstructor as SearchSQLContructor,
    structs::{AliasedJoinedEntity, FormatFilterResponse, SearchSuggestionCache},
    utils::{format_filters, generate_concatenated_expressions},
};
use crate::providers::storage::cache::cache;
use crate::providers::storage::get_valid_bucket_name;
use crate::providers::storage::minio::is_storage_disabled;
use crate::structs::core::{
    AggregationFilter, ApiResponse, Auth, BatchUpdateBody, GetByFilter, GroupAdvanceFilter,
    LogicalOperator, QueryParams, RequestBody, SearchSuggestionParams, SwitchAccountRequest,
    UpsertRequestBody,
};
use crate::structs::core::{FilterCriteria, FilterOperator};
use crate::utils::helpers::normalize_date_format;
use crate::{db, providers};
use actix_multipart::Multipart;
use actix_web::error::BlockingError;
use actix_web::{http, web, HttpResponse, Responder, ResponseError};
use actix_web::{HttpMessage, HttpRequest};
use aws_sdk_s3::primitives::ByteStream;
use chrono;
use chrono::Local;
use diesel::result::Error as DieselError;
use diesel::sql_query;
use diesel_async::RunQueryDsl;
use once_cell::sync::Lazy;
use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;
use sha1::{Digest, Sha1};
use std::collections::BTreeMap;
use std::collections::HashSet;
use std::fmt;
use std::sync::Mutex;
use tokio::fs::OpenOptions;
use tokio::io::AsyncWriteExt;
use ulid::Ulid;

use super::common_controller::{perform_upsert, sanitize_updates};
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

impl std::error::Error for ApiError {}

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
        Ok(response) => {
            invalidate_table_cache_prefix(&table_name);
            HttpResponse::Ok().json(response)
        }
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
    app_state: Option<web::Data<providers::storage::AppState>>,
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
            if std::env::var("DISABLE_STORAGE").unwrap_or("false".to_string()) == "false" {
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
                let s3_client = &app_state.unwrap().s3_client;
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
        Ok(response) => {
            invalidate_table_cache_prefix(&table_name);
            HttpResponse::Ok().json(response)
        }
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
    // Create a single DB connection upfront — reused for temp table check and COPY
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

    // In migration mode, skip the temp table check — it was created in the migration's Phase 1.
    if !migration_mode_enabled() {
        let temp_table = format!("temp_{}", table_name);
        let check_result = client
            .query_one(
                "SELECT EXISTS (SELECT 1 FROM pg_class WHERE relname = $1 AND relkind = 'r')",
                &[&temp_table],
            )
            .await;
        match check_result {
            Ok(row) => {
                let exists: bool = row.get(0);
                if !exists {
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
            Err(e) => {
                log::error!("Error checking temp table existence: {}", e);
                return HttpResponse::InternalServerError().json(ApiResponse {
                    success: false,
                    message: format!("Error checking temp table existence: {}", e),
                    count: 0,
                    data: vec![],
                });
            }
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

    let csv_data = match convert_json_to_csv(&processed_records, &columns, &table_name) {
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

    let column_refs: Vec<&str> = columns.iter().map(|s| s.as_str()).collect();

    match execute_copy(&client, &table_name, &column_refs, csv_data).await {
        Ok(_) => processed_records.clone(),
        Err(e) => {
            log::error!("Batch insert failed for table '{}': {}", table_name, e);
            return HttpResponse::InternalServerError().json(ApiResponse {
                success: false,
                message: format!("{}", e),
                count: 0,
                data: vec![],
            });
        }
    };

    // Convert JSON array to CSV in-memor

    if !migration_mode_enabled() {
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

    invalidate_table_cache_prefix(&table_name);
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

    // Use the new batch_update provider
    let batch_constructor = BatchUpdateSQLConstructor::new(table_name.clone(), is_root_controller)
        .with_organization_id(auth_data.organization_id.clone());

    // Convert updates_value to SET clause string
    let set_clause = if let Some(obj) = updates_value.as_object() {
        obj.iter()
            .map(|(k, v)| {
                let value_str = match v {
                    Value::String(s) => format!("'{}'", s.replace("'", "''")),
                    Value::Number(n) => n.to_string(),
                    Value::Bool(b) => b.to_string(),
                    Value::Null => "NULL".to_string(),
                    Value::Array(arr) => {
                        // Convert JSON array to PostgreSQL array format
                        let array_elements: Vec<String> = arr
                            .iter()
                            .map(|item| match item {
                                Value::String(s) => format!("'{}'", s.replace("'", "''")),
                                _ => serde_json::to_string(item)
                                    .unwrap_or_else(|_| "NULL".to_string()),
                            })
                            .collect();
                        format!("ARRAY[{}]", array_elements.join(", "))
                    }
                    Value::Object(_) => {
                        // For objects, use JSONB casting
                        format!(
                            "'{}'::jsonb",
                            serde_json::to_string(v)
                                .unwrap_or_else(|_| "NULL".to_string())
                                .replace("'", "''")
                        )
                    }
                };
                format!("{} = {}", k, value_str)
            })
            .collect::<Vec<_>>()
            .join(", ")
    } else {
        return HttpResponse::BadRequest().json(ApiResponse {
            success: false,
            message: "Invalid update data format".to_string(),
            count: 0,
            data: vec![],
        });
    };

    let sql_result = batch_constructor.construct_batch_update_advanced(&set_clause, &filters);
    log::debug!(
        "SQL Query for batch updates: {}",
        sql_result
            .clone()
            .unwrap_or_else(|e| format!("Failed to construct SQL query: {}", e))
    );

    match sql_result {
        Ok(sql_query) => {
            let mut conn = db::get_async_connection().await;

            match diesel::sql_query(&sql_query).execute(&mut conn).await {
                Ok(count) => {
                    invalidate_table_cache_prefix(&table_name);
                    HttpResponse::Ok().json(ApiResponse {
                        success: true,
                        message: format!("Updated {} records in '{}'", count, table_name),
                        count: count as i32,
                        data: vec![],
                    })
                }
                Err(e) => {
                    log::error!(
                        "Error executing batch update in table '{}': {}",
                        table_name,
                        e
                    );
                    HttpResponse::InternalServerError().json(ApiResponse {
                        success: false,
                        message: format!("Database error: {}", e),
                        count: 0,
                        data: vec![],
                    })
                }
            }
        }
        Err(e) => {
            log::error!(
                "Error constructing batch update SQL for table '{}': {}",
                table_name,
                e
            );
            HttpResponse::InternalServerError().json(ApiResponse {
                success: false,
                message: format!("SQL construction error: {}", e),
                count: 0,
                data: vec![],
            })
        }
    }
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

    // Use the new batch_update provider for delete operations
    let batch_constructor = BatchUpdateSQLConstructor::new(table_name.clone(), is_root_controller)
        .with_organization_id(auth_data.organization_id.clone());

    // Convert updates_value to SET clause string
    let set_clause = if let Some(obj) = updates_value.as_object() {
        obj.iter()
            .map(|(k, v)| {
                format!(
                    "{} = {}",
                    k,
                    serde_json::to_string(v).unwrap_or_else(|_| "NULL".to_string())
                )
            })
            .collect::<Vec<_>>()
            .join(", ")
    } else {
        return HttpResponse::BadRequest().json(ApiResponse {
            success: false,
            message: "Invalid delete data format".to_string(),
            count: 0,
            data: vec![],
        });
    };

    let sql_result = batch_constructor.construct_batch_update_advanced(&set_clause, &filters);

    match sql_result {
        Ok(sql_query) => {
            let mut conn = db::get_async_connection().await;

            match diesel::sql_query(&sql_query).execute(&mut conn).await {
                Ok(count) => {
                    invalidate_table_cache_prefix(&table_name);
                    HttpResponse::Ok().json(ApiResponse {
                        success: true,
                        message: format!("Deleted {} records in '{}'", count, table_name),
                        count: count as i32,
                        data: vec![],
                    })
                }
                Err(e) => {
                    log::error!(
                        "Error executing batch delete in table '{}': {}",
                        table_name,
                        e
                    );
                    HttpResponse::InternalServerError().json(ApiResponse {
                        success: false,
                        message: format!("Database error: {}", e),
                        count: 0,
                        data: vec![],
                    })
                }
            }
        }
        Err(e) => {
            log::error!(
                "Error constructing batch delete SQL for table '{}': {}",
                table_name,
                e
            );
            HttpResponse::InternalServerError().json(ApiResponse {
                success: false,
                message: format!("SQL construction error: {}", e),
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
        Some(pluck_fields),
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
            invalidate_table_cache_prefix(&table_name);
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
    let mut parameters = request_body.into_inner();
    let table = path_params.into_inner();
    let is_root = auth
        .extensions()
        .get::<Auth>()
        .map_or(false, |auth_data| auth_data.is_root_account);

    let headers = auth.headers();
    let timezone = headers
        .get("timezone")
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string());
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

    parameters.date_format = normalize_date_format(&parameters.date_format);

    // Clone parameters for debug logging (since they will be moved to SQLConstructor)
    let parameters_for_debug = parameters.clone();

    let filter_key_payload = serde_json::json!({
        "params": &parameters_for_debug,
        "org": &organization_id,
        "tz": &timezone,
        "is_root": is_root
    });
    let filter_str = serde_json::to_string(&filter_key_payload).unwrap_or_default();
    let mut hasher = Sha1::new();
    hasher.update(filter_str.as_bytes());
    let filter_hash = format!("{:x}", hasher.finalize());
    let cache_key = format!("{}_cache_{}", table, filter_hash);

    if let Some(cached) = cache.get(&cache_key) {
        if let Some(arr) = cached.as_array() {
            let data: Vec<Value> = arr.clone();
            return HttpResponse::Ok().json(ApiResponse {
                success: true,
                message: format!(
                    "Filter operation completed for table: {} (from cache)",
                    &table
                ),
                count: data.len() as i32,
                data,
            });
        }
    }

    // Create SQLConstructor with organization_id if available
    let mut sql_constructor = SQLConstructor::new(parameters, table.clone(), is_root, timezone);
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

    log::debug!("@@@parameters: {:?}", parameters_for_debug.clone());
    // Get a connection from the pool
    let mut conn = db::get_async_connection().await;
    // Enhanced debug logging to file
    if EnvConfig::default().debug {
        log::debug!("QUERY: {}", query);
        // Also write to debug log file
        if let Err(e) = write_query_to_debug_log(&query, &table, false).await {
            log::warn!("Failed to write debug query log: {}", e);
        }

        // Convert parameters to stringified JSON object
        match serde_json::to_string_pretty(&parameters_for_debug) {
            Ok(params_json) => {
                if let Err(e) = write_query_to_debug_log(&params_json, &table, true).await {
                    log::warn!("Failed to write debug parameters log: {}", e);
                }
            }
            Err(e) => {
                log::warn!("Failed to serialize parameters to JSON: {}", e);
            }
        }
    }

    log::debug!("@@@parameters: {:?}", parameters_for_debug.clone());
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

    cache.insert_with_ttl(
        cache_key.clone(),
        serde_json::Value::Array(data.clone()),
        std::time::Duration::from_millis(EnvConfig::default().find_cache_ttl_ms),
    );
    cache.add_index_key(&format!("{}_cache_", table), &cache_key);

    HttpResponse::Ok().json(ApiResponse {
        success: true,
        message: format!("Filter operation completed for table: {}", &table),
        count: data.len() as i32,
        data,
    })
}

fn invalidate_table_cache_prefix(table: &str) {
    cache.remove_by_prefix(&format!("{}_cache_", table));
}
/// Count route: POST /api/store/{table}/count
/// Uses the same filter parsing as get_by_filter and aggregation_filter.
/// Returns count of distinct rows matching the filters.
pub async fn count_by_filter(
    auth: HttpRequest,
    path_params: web::Path<String>,
    request_body: web::Json<GetByFilter>,
) -> impl Responder {
    let mut parameters = request_body.into_inner();
    let table = path_params.into_inner();
    let is_root = auth
        .extensions()
        .get::<Auth>()
        .map_or(false, |auth_data| auth_data.is_root_account);

    let headers = auth.headers();
    let timezone = headers
        .get("timezone")
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string());

    let extensions = auth.extensions();
    let organization_id = match extensions.get::<Auth>() {
        Some(auth_data) => Some(auth_data.organization_id.clone()),
        None => {
            log::warn!("Auth data not found in request extensions");
            None
        }
    };

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

    parameters.date_format = normalize_date_format(&parameters.date_format);

    // Build a stable cache key based on filter params, org, tz, and root flag
    let parameters_for_debug = parameters.clone();
    let filter_key_payload = serde_json::json!({
        "params": &parameters_for_debug,
        "org": &organization_id,
        "tz": &timezone,
        "is_root": is_root
    });
    let filter_str = serde_json::to_string(&filter_key_payload).unwrap_or_default();
    let mut hasher = Sha1::new();
    hasher.update(filter_str.as_bytes());
    let filter_hash = format!("{:x}", hasher.finalize());
    let cache_key = format!("{}_cache_count_{}", table, filter_hash);

    if let Some(cached) = cache.get(&cache_key) {
        let cached_count = if let Some(n) = cached.as_i64() {
            Some(n)
        } else {
            cached.get("count").and_then(|v| v.as_i64())
        };
        if let Some(n) = cached_count {
            return HttpResponse::Ok().json(ApiResponse {
                success: true,
                message: format!("Count completed for table: {} (from cache)", &table),
                count: n as i32,
                data: vec![serde_json::json!({ "count": n })],
            });
        }
    }

    let mut sql_constructor = SQLConstructor::new(parameters, table.clone(), is_root, timezone);
    if let Some(org_id) = organization_id {
        sql_constructor = sql_constructor.with_organization_id(org_id);
    }

    let query = match sql_constructor.construct_count() {
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

    let mut conn = db::get_async_connection().await;
    let final_query = format!("SELECT row_to_json(t) FROM ({}) t", query);

    log::info!("Count query: {:?}", final_query);
    let results = match diesel::dsl::sql_query(&final_query)
        .load::<DynamicResult>(&mut conn)
        .await
    {
        Ok(results) => results,
        Err(e) => {
            log::error!("Error executing count query for table '{}': {:?}", table, e);
            return HttpResponse::InternalServerError().json(ApiResponse {
                success: false,
                message: format!("Query execution error: {}", e),
                count: 0,
                data: vec![],
            });
        }
    };

    let count_value: i64 = results
        .get(0)
        .and_then(|r| r.row_to_json.as_ref())
        .and_then(|j| j.get("count"))
        .and_then(|v| v.as_i64())
        .unwrap_or(0);

    cache.insert_with_ttl(
        cache_key.clone(),
        serde_json::json!(count_value),
        std::time::Duration::from_millis(EnvConfig::default().find_cache_ttl_ms),
    );
    cache.add_index_key(&format!("{}_cache_", table), &cache_key);

    HttpResponse::Ok().json(ApiResponse {
        success: true,
        message: format!("Count completed for table: {}", &table),
        count: count_value as i32,
        data: vec![serde_json::json!({ "count": count_value })],
    })
}

// Removed helper in favor of centralized cache methods

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

    // Extract timezone: prefer body over header for consistency with find, count, search suggestion
    let header_timezone = auth
        .headers()
        .get("timezone")
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string());
    let body_timezone = parameters.timezone.clone();
    let timezone = match (header_timezone, body_timezone) {
        (_, Some(tz)) => Some(tz),    // Body takes precedence
        (Some(tz), None) => Some(tz), // Header fallback
        (None, None) => None,
    };

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
    let mut sql_constructor =
        AggregationSQLConstructor::new(parameters, table.clone(), is_root, timezone);
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
    log::debug!("@@@@Extracted organization_id: {:?}", organization_id);
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
    query: web::Query<std::collections::HashMap<String, String>>,
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

    // First, get file metadata from database
    let pluck_fields = vec![
        "mimetype".to_string(),
        "download_path".to_string(),
        "size".to_string(),
        "etag".to_string(),
        "tags".to_string(),
        "filename".to_string(),
        "path".to_string(),
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

    let mut mimetype = file_metadata
        .get("mimetype")
        .and_then(|v| v.as_str())
        .unwrap_or("application/octet-stream")
        .to_string();
    let download_path = file_metadata
        .get("download_path")
        .and_then(|v| v.as_str())
        .unwrap_or(&file_id);
    let file_etag = file_metadata
        .get("etag")
        .and_then(|v| v.as_str())
        .unwrap_or("");
    let filename = file_metadata
        .get("filename")
        .and_then(|v| v.as_str())
        .unwrap_or("");
    let tags_contains_thumbnail = file_metadata
        .get("tags")
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter().any(|t| {
                t.as_str()
                    .map(|s| s.eq_ignore_ascii_case("thumbnail"))
                    .unwrap_or(false)
            })
        })
        .unwrap_or(false);
    let is_thumbnail_requested = query
        .get("q")
        .map(|v| v.eq_ignore_ascii_case("thumbnail"))
        .unwrap_or(false);

    let base_bucket_name =
        std::env::var("STORAGE_BUCKET_NAME").unwrap_or_else(|_| app_state.bucket_name.clone());
    let bucket_name = base_bucket_name.clone();

    let mut effective_download_path = download_path.to_string();

    // If thumbnail is NOT requested but the current record is a thumbnail, try to resolve original
    if !is_thumbnail_requested && tags_contains_thumbnail {
        let db_pluck = vec![
            "download_path".to_string(),
            "mimetype".to_string(),
            "size".to_string(),
            "id".to_string(),
            "tags".to_string(),
            "filename".to_string(),
            "path".to_string(),
        ];
        let mut adv_filters: Vec<FilterCriteria> = Vec::new();
        if let Some(fname) = file_metadata.get("filename").and_then(|v| v.as_str()) {
            if !fname.is_empty() {
                adv_filters.push(FilterCriteria::Criteria {
                    field: "filename".to_string(),
                    entity: None,
                    operator: FilterOperator::Equal,
                    values: vec![serde_json::Value::String(fname.to_string())],
                    case_sensitive: Some(false),
                    parse_as: "text".to_string(),
                    match_pattern: None,
                    is_search: None,
                    has_group_count: None,
                });
                adv_filters.push(FilterCriteria::LogicalOperator {
                    operator: LogicalOperator::And,
                });
            }
        }
        // Ensure path does NOT include thumbnail
        adv_filters.push(FilterCriteria::Criteria {
            field: "path".to_string(),
            entity: None,
            operator: FilterOperator::NotContains,
            values: vec![serde_json::Value::String("thumbnail".to_string())],
            case_sensitive: Some(false),
            parse_as: "text".to_string(),
            match_pattern: None,
            is_search: None,
            has_group_count: None,
        });
        adv_filters.push(FilterCriteria::LogicalOperator {
            operator: LogicalOperator::And,
        });
        // Ensure same organization
        adv_filters.push(FilterCriteria::Criteria {
            field: "organization_id".to_string(),
            entity: None,
            operator: FilterOperator::Equal,
            values: vec![serde_json::Value::String(auth_data.organization_id.clone())],
            case_sensitive: Some(false),
            parse_as: "text".to_string(),
            match_pattern: None,
            is_search: None,
            has_group_count: None,
        });

        let parameters = GetByFilter {
            pluck: db_pluck.clone(),
            pluck_object: Default::default(),
            pluck_group_object: Default::default(),
            advance_filters: adv_filters,
            group_advance_filters: vec![],
            joins: vec![],
            group_by: None,
            concatenate_fields: vec![],
            multiple_sort: vec![],
            date_format: "YYYY-mm-dd".to_string(),
            order_by: "updated_time".to_string(),
            order_direction: "DESC".to_string(),
            is_case_sensitive_sorting: None,
            offset: 0,
            limit: 1,
            distinct_by: None,
            timezone: None,
            time_format: "HH24:MI".to_string(),
        };
        let mut sql_constructor = SQLConstructor::new(
            parameters.clone(),
            "files".to_string(),
            is_root_controller,
            None,
        );
        sql_constructor = sql_constructor.with_organization_id(auth_data.organization_id.clone());
        if let Ok(query_sql) = sql_constructor.construct() {
            let final_query = format!("SELECT row_to_json(t) FROM ({}) t", query_sql);
            let mut conn = db::get_async_connection().await;
            if let Ok(results) = diesel::dsl::sql_query(&final_query)
                .load::<DynamicResult>(&mut conn)
                .await
            {
                if let Some(first) = results.into_iter().filter_map(|r| r.row_to_json).next() {
                    if let Some(obj) = first.as_object() {
                        if let Some(dp) = obj.get("download_path").and_then(|v| v.as_str()) {
                            effective_download_path = dp.to_string();
                        } else if let Some(pv) = obj.get("path").and_then(|v| v.as_str()) {
                            let prefix = format!("{}/", bucket_name);
                            let key = if pv.starts_with(&prefix) {
                                pv.strip_prefix(&prefix).unwrap_or(pv)
                            } else {
                                pv
                            };
                            effective_download_path = key.to_string();
                        }
                        if let Some(mt) = obj.get("mimetype").and_then(|v| v.as_str()) {
                            mimetype = mt.to_string();
                        }
                    }
                }
            }
        }
    }

    if is_thumbnail_requested && !tags_contains_thumbnail {
        let db_pluck = vec![
            "download_path".to_string(),
            "mimetype".to_string(),
            "size".to_string(),
            "id".to_string(),
            "tags".to_string(),
            "filename".to_string(),
            "path".to_string(),
        ];
        let mut adv_filters: Vec<FilterCriteria> = Vec::new();
        if !filename.is_empty() {
            adv_filters.push(FilterCriteria::Criteria {
                field: "filename".to_string(),
                entity: None,
                operator: FilterOperator::Equal,
                values: vec![serde_json::Value::String(filename.to_string())],
                case_sensitive: Some(false),
                parse_as: "text".to_string(),
                match_pattern: None,
                is_search: None,
                has_group_count: None,
            });
            adv_filters.push(FilterCriteria::LogicalOperator {
                operator: LogicalOperator::And,
            });
        }
        adv_filters.push(FilterCriteria::Criteria {
            field: "tags".to_string(),
            entity: None,
            operator: FilterOperator::Contains,
            values: vec![serde_json::Value::String("thumbnail".to_string())],
            case_sensitive: Some(false),
            parse_as: "text".to_string(),
            match_pattern: None,
            is_search: None,
            has_group_count: None,
        });
        adv_filters.push(FilterCriteria::LogicalOperator {
            operator: LogicalOperator::And,
        });
        adv_filters.push(FilterCriteria::Criteria {
            field: "path".to_string(),
            entity: None,
            operator: FilterOperator::Contains,
            values: vec![serde_json::Value::String("thumbnail".to_string())],
            case_sensitive: Some(false),
            parse_as: "text".to_string(),
            match_pattern: None,
            is_search: None,
            has_group_count: None,
        });
        adv_filters.push(FilterCriteria::LogicalOperator {
            operator: LogicalOperator::And,
        });
        adv_filters.push(FilterCriteria::Criteria {
            field: "organization_id".to_string(),
            entity: None,
            operator: FilterOperator::Equal,
            values: vec![serde_json::Value::String(auth_data.organization_id.clone())],
            case_sensitive: Some(false),
            parse_as: "text".to_string(),
            match_pattern: None,
            is_search: None,
            has_group_count: None,
        });
        let parameters = GetByFilter {
            pluck: db_pluck.clone(),
            pluck_object: Default::default(),
            pluck_group_object: Default::default(),
            advance_filters: adv_filters,
            group_advance_filters: vec![],
            joins: vec![],
            group_by: None,
            concatenate_fields: vec![],
            multiple_sort: vec![],
            date_format: "YYYY-mm-dd".to_string(),
            order_by: "updated_time".to_string(),
            order_direction: "DESC".to_string(),
            is_case_sensitive_sorting: None,
            offset: 0,
            limit: 1,
            distinct_by: None,
            timezone: None,
            time_format: "HH24:MI".to_string(),
        };
        let mut sql_constructor = SQLConstructor::new(
            parameters.clone(),
            "files".to_string(),
            is_root_controller,
            None,
        );
        sql_constructor = sql_constructor.with_organization_id(auth_data.organization_id.clone());
        if let Ok(query_sql) = sql_constructor.construct() {
            let final_query = format!("SELECT row_to_json(t) FROM ({}) t", query_sql);
            let mut conn = db::get_async_connection().await;
            if let Ok(results) = diesel::dsl::sql_query(&final_query)
                .load::<DynamicResult>(&mut conn)
                .await
            {
                if let Some(first) = results.into_iter().filter_map(|r| r.row_to_json).next() {
                    if let Some(obj) = first.as_object() {
                        if let Some(dp) = obj.get("download_path").and_then(|v| v.as_str()) {
                            effective_download_path = dp.to_string();
                        } else if let Some(pv) = obj.get("path").and_then(|v| v.as_str()) {
                            let prefix = format!("{}/", bucket_name);
                            let key = if pv.starts_with(&prefix) {
                                pv.strip_prefix(&prefix).unwrap_or(pv)
                            } else {
                                pv
                            };
                            effective_download_path = key.to_string();
                        }
                        if let Some(mt) = obj.get("mimetype").and_then(|v| v.as_str()) {
                            mimetype = mt.to_string();
                        }
                    }
                }
            }
        }
    }

    let variant_label = if is_thumbnail_requested {
        "thumbnail"
    } else {
        "original"
    };
    let temp_file_ttl_secs: u64 = EnvConfig::default().temporary_file_ttl_secs;
    let cache_dir = std::env::temp_dir()
        .join("store_cache")
        .join(auth_data.organization_id.clone());
    let cache_file_path = cache_dir.join(format!("{}_{}.cache", file_id, variant_label));
    log::debug!("Cache file path: {:?}", cache_file_path);
    if let Ok(meta) = tokio::fs::metadata(&cache_file_path).await {
        if let Ok(modified) = meta.modified() {
            let now = std::time::SystemTime::now();
            if now
                .duration_since(modified)
                .unwrap_or(std::time::Duration::from_secs(0))
                <= std::time::Duration::from_secs(temp_file_ttl_secs)
            {
                if let Ok(bytes_vec) = tokio::fs::read(&cache_file_path).await {
                    use futures_util::stream;
                    let cached_bytes = bytes::Bytes::from(bytes_vec);
                    let byte_stream =
                        stream::once(async move { Ok::<_, std::io::Error>(cached_bytes) });
                    let actual_content_type = mimetype.to_string();
                    let is_image = actual_content_type.starts_with("image/");
                    let filename = effective_download_path.split('/').last().unwrap_or("file");
                    let content_disposition = if is_image {
                        format!("inline; filename=\"{}\"", filename)
                    } else {
                        format!("attachment; filename=\"{}\"", filename)
                    };
                    return HttpResponse::Ok()
                        .content_type(actual_content_type)
                        .insert_header(("Cache-Control", "public, max-age=3600"))
                        .insert_header(("Accept-Ranges", "bytes"))
                        .insert_header(("Content-Disposition", content_disposition))
                        .streaming(byte_stream);
                }
            }
        }
    }

    // Stream file from S3
    let s3_client = &app_state.s3_client;

    let s3_key = effective_download_path.as_str();
    match s3_client
        .get_object()
        .bucket(&bucket_name)
        .key(s3_key)
        .send()
        .await
    {
        Ok(output) => {
            let actual_content_type = mimetype.to_string();

            let content_length = output.content_length().unwrap_or(0);

            match output.body.collect().await {
                Ok(data) => {
                    let bytes = data.into_bytes();

                    use futures_util::stream;
                    let bytes_for_stream = bytes.clone();
                    let byte_stream =
                        stream::once(async move { Ok::<_, std::io::Error>(bytes_for_stream) });
                    let cache_dir_clone = cache_dir.clone();
                    let cache_file_path_clone = cache_file_path.clone();
                    let to_write_vec = bytes.to_vec();
                    let ttl_for_task = temp_file_ttl_secs;
                    tokio::spawn(async move {
                        let _ = tokio::fs::create_dir_all(&cache_dir_clone).await;
                        let _ = tokio::fs::write(&cache_file_path_clone, &to_write_vec).await;
                        tokio::spawn(async move {
                            tokio::time::sleep(std::time::Duration::from_secs(ttl_for_task)).await;
                            let _ = tokio::fs::remove_file(&cache_file_path_clone).await;
                        });
                    });

                    let is_image = actual_content_type.starts_with("image/");
                    let filename = s3_key.split('/').last().unwrap_or("file");

                    let content_disposition = if is_image {
                        format!("inline; filename=\"{}\"", filename)
                    } else {
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
            let normalize_etag = |s: &str| {
                s.trim()
                    .trim_start_matches("\\\"")
                    .trim_end_matches("\\\"")
                    .trim_end_matches('\\')
                    .trim_matches('"')
                    .to_string()
            };
            let target_etag = normalize_etag(file_etag);
            if target_etag.is_empty() {
                return HttpResponse::NotFound().json(ApiResponse {
                    success: false,
                    message: "File not found in storage".to_string(),
                    count: 0,
                    data: vec![],
                });
            }
            let mut found_key: Option<String> = None;
            let db_pluck = vec![
                "download_path".to_string(),
                "mimetype".to_string(),
                "id".to_string(),
            ];
            let parameters = GetByFilter {
                pluck: db_pluck.clone(),
                pluck_object: Default::default(),
                pluck_group_object: Default::default(),
                advance_filters: vec![
                    FilterCriteria::Criteria {
                        field: "etag".to_string(),
                        entity: None,
                        operator: FilterOperator::Equal,
                        values: vec![serde_json::Value::String(target_etag.clone())],
                        case_sensitive: Some(false),
                        parse_as: "text".to_string(),
                        match_pattern: None,
                        is_search: None,
                        has_group_count: None,
                    },
                    FilterCriteria::LogicalOperator {
                        operator: LogicalOperator::And,
                    },
                    FilterCriteria::Criteria {
                        field: "organization_id".to_string(),
                        entity: None,
                        operator: FilterOperator::Equal,
                        values: vec![serde_json::Value::String(auth_data.organization_id.clone())],
                        case_sensitive: Some(false),
                        parse_as: "text".to_string(),
                        match_pattern: None,
                        is_search: None,
                        has_group_count: None,
                    },
                ],
                group_advance_filters: vec![],
                joins: vec![],
                group_by: None,
                concatenate_fields: vec![],
                multiple_sort: vec![],
                date_format: "YYYY-mm-dd".to_string(),
                order_by: "id".to_string(),
                order_direction: "ASC".to_string(),
                is_case_sensitive_sorting: None,
                offset: 0,
                limit: 1,
                distinct_by: None,
                timezone: None,
                time_format: "HH24:MI".to_string(),
            };
            let mut sql_constructor = SQLConstructor::new(
                parameters.clone(),
                "files".to_string(),
                is_root_controller,
                None,
            )
            .with_organization_id(auth_data.organization_id.clone());
            if let Ok(query) = sql_constructor.construct() {
                let final_query = format!("SELECT row_to_json(t) FROM ({}) t", query);
                let mut conn = db::get_async_connection().await;
                if let Ok(results) = diesel::dsl::sql_query(&final_query)
                    .load::<DynamicResult>(&mut conn)
                    .await
                {
                    if let Some(first) = results.into_iter().filter_map(|r| r.row_to_json).next() {
                        if let Some(obj) = first.as_object() {
                            if let Some(dp) = obj.get("download_path").and_then(|v| v.as_str()) {
                                found_key = Some(dp.to_string());
                            }
                            if mimetype == "application/octet-stream" {
                                if let Some(mt) = obj.get("mimetype").and_then(|v| v.as_str()) {
                                    mimetype = mt.to_string();
                                }
                            }
                        }
                    }
                }
            }
            if found_key.is_none() {
                let org_name = std::env::var("DEFAULT_ORGANIZATION_NAME")
                    .unwrap_or_else(|_| "default".to_string());
                let valid_prefix =
                    get_valid_bucket_name(&org_name, Some(auth_data.organization_id.as_str()));
                let mut continuation: Option<String> = None;
                loop {
                    let mut req = s3_client
                        .list_objects_v2()
                        .bucket(&bucket_name)
                        .prefix(&valid_prefix);
                    if let Some(token) = continuation.as_deref() {
                        req = req.continuation_token(token);
                    }
                    match req.send().await {
                        Ok(list_output) => {
                            for object in list_output.contents() {
                                let obj_etag = object.e_tag().unwrap_or("");
                                if !obj_etag.is_empty() && normalize_etag(obj_etag) == target_etag {
                                    if let Some(key) = object.key() {
                                        found_key = Some(key.to_string());
                                        break;
                                    }
                                }
                            }
                            if found_key.is_some() {
                                break;
                            }
                            if let Some(next) = list_output.next_continuation_token() {
                                continuation = Some(next.to_string());
                            } else {
                                break;
                            }
                        }
                        Err(err) => {
                            log::error!(
                                "ListObjectsV2 error while searching by ETag for file {}: {:?}",
                                file_id,
                                err
                            );
                            break;
                        }
                    }
                }
                if found_key.is_none() {
                    let mut continuation_all: Option<String> = None;
                    loop {
                        let mut req = s3_client.list_objects_v2().bucket(&bucket_name);
                        if let Some(token) = continuation_all.as_deref() {
                            req = req.continuation_token(token);
                        }
                        match req.send().await {
                            Ok(list_output) => {
                                for object in list_output.contents() {
                                    let obj_etag = object.e_tag().unwrap_or("");
                                    if !obj_etag.is_empty()
                                        && normalize_etag(obj_etag) == target_etag
                                    {
                                        if let Some(key) = object.key() {
                                            found_key = Some(key.to_string());
                                            break;
                                        }
                                    }
                                }
                                if found_key.is_some() {
                                    break;
                                }
                                if let Some(next) = list_output.next_continuation_token() {
                                    continuation_all = Some(next.to_string());
                                } else {
                                    break;
                                }
                            }
                            Err(err) => {
                                log::error!(
                                    "Bucket-wide ListObjectsV2 error while searching by ETag for file {}: {:?}",
                                    file_id,
                                    err
                                );
                                break;
                            }
                        }
                    }
                }
            }
            if let Some(fallback_key) = found_key {
                match s3_client
                    .get_object()
                    .bucket(&bucket_name)
                    .key(&fallback_key)
                    .send()
                    .await
                {
                    Ok(output) => {
                        let update_body =
                            serde_json::json!({"id": file_id, "download_path": &fallback_key});
                        let _ = process_and_update_record(
                            "files",
                            update_body,
                            file_id.as_str(),
                            None,
                            "update",
                            &auth_data,
                            false,
                        )
                        .await;
                        let actual_content_type = mimetype.to_string();
                        let content_length = output.content_length().unwrap_or(0);
                        match output.body.collect().await {
                            Ok(data) => {
                                let bytes = data.into_bytes();
                                use futures_util::stream;
                                let bytes_for_stream = bytes.clone();
                                let byte_stream = stream::once(async move {
                                    Ok::<_, std::io::Error>(bytes_for_stream)
                                });
                                let cache_dir_clone = cache_dir.clone();
                                let cache_file_path_clone = cache_file_path.clone();
                                let to_write_vec = bytes.to_vec();
                                let ttl_for_task = temp_file_ttl_secs;
                                tokio::spawn(async move {
                                    let _ = tokio::fs::create_dir_all(&cache_dir_clone).await;
                                    let _ = tokio::fs::write(&cache_file_path_clone, &to_write_vec)
                                        .await;
                                    tokio::spawn(async move {
                                        tokio::time::sleep(std::time::Duration::from_secs(
                                            ttl_for_task,
                                        ))
                                        .await;
                                        let _ =
                                            tokio::fs::remove_file(&cache_file_path_clone).await;
                                    });
                                });
                                let is_image = actual_content_type.starts_with("image/");
                                let filename = fallback_key.split('/').last().unwrap_or("file");
                                let content_disposition = if is_image {
                                    format!("inline; filename=\"{}\"", filename)
                                } else {
                                    format!("attachment; filename=\"{}\"", filename)
                                };
                                return HttpResponse::Ok()
                                    .content_type(actual_content_type)
                                    .insert_header(("Content-Length", content_length.to_string()))
                                    .insert_header(("Cache-Control", "public, max-age=3600"))
                                    .insert_header(("Accept-Ranges", "bytes"))
                                    .insert_header(("Content-Disposition", content_disposition))
                                    .streaming(byte_stream);
                            }
                            Err(err) => {
                                log::error!(
                                    "Error reading S3 object body (fallback by ETag) for file {}: {:?}",
                                    file_id,
                                    err
                                );
                                return HttpResponse::InternalServerError().json(ApiResponse {
                                    success: false,
                                    message: "Failed to read file content".to_string(),
                                    count: 0,
                                    data: vec![],
                                });
                            }
                        }
                    }
                    Err(err) => {
                        log::error!(
                            "Fallback get_object by ETag failed for file {}: {:?}",
                            file_id,
                            err
                        );
                        HttpResponse::NotFound().json(ApiResponse {
                            success: false,
                            message: "File not found in storage".to_string(),
                            count: 0,
                            data: vec![],
                        })
                    }
                }
            } else {
                HttpResponse::NotFound().json(ApiResponse {
                    success: false,
                    message: "File not found in storage".to_string(),
                    count: 0,
                    data: vec![],
                })
            }
        }
    }
}

pub async fn upload_file(
    auth: HttpRequest,
    app_state: web::Data<providers::storage::AppState>,
    mut multipart: Multipart,
) -> impl Responder {
    // Check if storage is disabled
    if is_storage_disabled() {
        log::info!("Storage is disabled (DISABLE_STORAGE=true), returning mock upload response");
        return HttpResponse::Ok().json(ApiResponse {
            success: false,
            message: "Upload failed (storage disabled)".to_string(),
            count: 0,
            data: vec![],
        });
    }

    // Check for Auth data early and abort if missing
    let extensions = auth.extensions();
    let auth_data = match extensions.get::<Auth>() {
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

    if let Some(content_type_header) = auth.headers().get(actix_web::http::header::CONTENT_TYPE) {
        log::info!("Incoming Content-Type header: {:?}", content_type_header);
    }
    let name = "files";
    let client = app_state.s3_client.clone();
    let bucket_name = app_state.bucket_name.clone();
    let mut uploaded_files_count = 0;
    let mut file_metadata = Vec::new();
    let pluck_fields = vec!["id".to_string()];
    while let Some(field_result) = multipart.next().await {
        log::debug!("Processing field result: {:?}", field_result);
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
            // Use organization ID from auth data, fallback to DEFAULT_ORGANIZATION_ID env var
            let org_id = if !auth_data.organization_id.is_empty() {
                auth_data.organization_id.clone()
            } else {
                std::env::var("DEFAULT_ORGANIZATION_ID").unwrap_or_else(|_| String::new())
            };
            // Get organization name for path structure
            let organization_name = std::env::var("DEFAULT_ORGANIZATION_NAME")
                .unwrap_or_else(|_| "default".to_string());

            // Create the new path structure: STORAGE_BUCKET_NAME/organization_name/file_id.extension
            let valid_bucket_name =
                get_valid_bucket_name(&organization_name, Some(org_id.as_str()));
            let new_unique_filename = format!(
                "{}/{}",
                valid_bucket_name,
                format!("{}.{}", new_id, extension)
            );

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
                                    // Extract ID from filename (format: "organization_name/ID.extension")
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
                    let mut actual_filename = final_filename.clone();
                    let normalized_filename =
                        format!("{}/{}.{}", valid_bucket_name, actual_id, extension);
                    if actual_filename != normalized_filename {
                        let _ = client
                            .copy_object()
                            .bucket(&bucket_name)
                            .key(&normalized_filename)
                            .copy_source(format!("{}/{}", bucket_name, actual_filename))
                            .send()
                            .await;
                        actual_filename = normalized_filename;
                    }

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
                        "organization_id": auth_data.organization_id.clone(),
                        // "created_by": auth_data.account_organization_id.clone(),
                        // "updated_by": auth_data.responsible_account.clone(),
                        "deleted_by": "",
                        "requested_by": auth_data.responsible_account.clone(),
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
                        "uploaded_by": auth_data.account_organization_id.clone(),
                        // "downloaded_by": "",
                        "etag": get_output.e_tag().unwrap_or("Unknown"),
                        "version_id": get_output.version_id().unwrap_or(""),
                        "download_path": actual_filename.clone(),
                        "presigned_url": "", // TODO: Generate presigned URL if needed
                        "presigned_url_expire": 0, // TODO: Set expiration timestamp
                        // "last_modified": get_output.last_modified()
                        //     .map(|dt| dt.to_string())
                        //     .unwrap_or_else(|| "Unknown".to_string())
                    });
                    // For existing files, try to save to database (will handle duplicates gracefully)

                    let insert_result = process_and_insert_record(
                        name,
                        metadata.clone(),
                        Some(pluck_fields.clone()),
                        &auth_data,
                        auth_data.is_root_account,
                    )
                    .await;
                    if let Err(e) = insert_result {
                        log::warn!(
                            "Failed to upsert existing file metadata into '{}': {}",
                            name,
                            e.message
                        );
                    }
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
                        "organization_id": auth_data.organization_id.clone(),
                        "created_by": auth_data.account_organization_id.clone(),
                        "updated_by": auth_data.responsible_account.clone(),
                        "deleted_by": "",
                        "requested_by": auth_data.responsible_account.clone(),
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
                        "uploaded_by": auth_data.account_organization_id.clone(),
                        "downloaded_by": "",
                        "etag": put_output.e_tag().unwrap_or("Unknown"),
                        "version_id": put_output.version_id().unwrap_or(""),
                        "download_path": final_filename.clone(),
                        "presigned_url": "", // TODO: Generate presigned URL if needed
                        "presigned_url_expire": 0 // TODO: Set expiration timestamp
                    });
                    dbg!(format!(
                        "Complete file metadata for uploaded file: {:?}",
                        metadata
                    ));
                    // Save file metadata to the database using process_and_insert_record

                    let insert_result = process_and_insert_record(
                        name,
                        metadata.clone(),
                        Some(pluck_fields.clone()),
                        &auth_data,
                        auth_data.is_root_account,
                    )
                    .await;
                    if let Err(e) = insert_result {
                        log::error!(
                            "Failed to insert uploaded file metadata into '{}': {}",
                            name,
                            e.message
                        );
                    }
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
    use crate::providers::operations::auth::auth_service;
    use crate::providers::operations::organizations::auth_service as org_auth_service;
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
    let signed_in_account = claims.signed_in_account;
    // .map(|s| json!({"account_id": s}))
    // .unwrap_or_else(|| json!({}));
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
        "as_root": true
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

pub async fn search_suggestions(
    auth: HttpRequest,
    path_params: web::Path<String>,
    request_body: web::Json<SearchSuggestionParams>,
) -> impl Responder {
    let table = path_params.into_inner();
    let parameters: SearchSuggestionParams = request_body.into_inner();
    let is_root = auth
        .extensions()
        .get::<Auth>()
        .map_or(false, |auth_data| auth_data.is_root_account);

    let headers = auth.headers();
    let header_timezone = headers
        .get("timezone")
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string());
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

    let SearchSuggestionParams {
        advance_filters,
        group_advance_filters,
        joins,
        concatenate_fields,
        date_format,
        time_format,
        timezone: body_timezone,
        ..
    } = &parameters;

    // Prefer body timezone over header for consistency across find, count, aggregation, and search suggestion
    let timezone = match (header_timezone.clone(), body_timezone) {
        (_, Some(tz)) => Some(tz.to_string()),
        (Some(tz), None) => Some(tz.to_string()),
        (None, None) => None,
    };

    // Convert parameters to stringified JSON object
    match serde_json::to_string_pretty(&parameters.clone()) {
        Ok(params_json) => {
            if let Err(e) = write_query_to_debug_log(&params_json, &table, true).await {
                log::warn!("Failed to write debug parameters log: {}", e);
            }
        }
        Err(e) => {
            log::warn!("Failed to serialize parameters to JSON: {}", e);
        }
    }

    if advance_filters.is_empty() && group_advance_filters.is_empty() {
        return HttpResponse::BadRequest().json(ApiResponse {
            success: false,
            message: format!("No advance or group filters provided"),
            count: 0,
            data: vec![],
        });
    }

    let json_params_string = match (|| -> Result<String, serde_json::Error> {
        let value: Value = serde_json::to_value(&parameters)?;
        if let Value::Object(map) = value {
            let sorted: BTreeMap<String, Value> = map.into_iter().collect();
            serde_json::to_string(&sorted)
        } else {
            serde_json::to_string(&parameters)
        }
    })() {
        Ok(s) => s,
        Err(e) => {
            log::error!("Failed to serialize parameters to JSON: {}", e);
            return HttpResponse::UnprocessableEntity().json(ApiResponse {
                success: false,
                message: format!("JSON parameters processing error: {}", e),
                count: 0,
                data: vec![],
            });
        }
    };

    let mv_hash_input = {
        let org_key = organization_id.as_deref().unwrap_or("no_org");
        format!("is_root:{}|org:{}|{}", is_root, org_key, json_params_string)
    };
    let mv_hash = SearchSuggestionCache::hash_string(&mv_hash_input);
    let mv_name = format!("mv_ss_{}", &mv_hash);

    if let Some(cached_value) = SearchSuggestionCache::get_mv_results(&mv_hash) {
        let started_at = std::time::Instant::now();
        let data: Vec<Value> = cached_value.as_array().cloned().unwrap_or_default();
        if SearchSuggestionCache::set_mv_refresh_trigger_if_absent(&mv_hash) {
            let mv_name = mv_name.clone();
            tokio::spawn(async move {
                if let Err(e) = refresh_materialized_view_once(mv_name).await {
                    log::warn!("MV refresh failed: {}", e.message);
                }
            });
        }

        let took_ms = started_at.elapsed().as_millis();
        log::debug!(
            "Search Suggestion: Redis cached response took {}ms (matview: {})",
            took_ms,
            mv_name
        );

        return HttpResponse::Ok().json(ApiResponse {
            success: true,
            message: format!(
                "[Redis]: Search suggestions operation completed for table: {}",
                &table
            ),
            count: data.len() as i32,
            data,
        });
    }

    if let Ok(client) = create_connection().await {
        if let Ok(rows) = client
            .query(
                "SELECT matviewname, schemaname, matviewowner FROM pg_matviews WHERE matviewname = $1",
                &[&mv_name.as_str()],
            )
            .await
        {
            if let Some(row) = rows.get(0) {
                let matviewname: String = row.get(0);
                let schemaname: String = row.get(1);
                let final_query =
                    format!("SELECT row_to_json(t) FROM {}.{} t", schemaname, matviewname);
                if let Ok(res_rows) = client.query(final_query.as_str(), &[]).await {
                    let data: Vec<serde_json::Value> = res_rows
                        .into_iter()
                        .map(|rr| {
                            let data: Value = rr.get(0);
                            let data = data.get("row_to_json").cloned().unwrap_or(data);

                            if let Some(data) = data.get("results").and_then(|v| v.get("data")) {
                                data.clone()
                            } else {
                                data
                            }
                        })
                        .filter(|item| !item.is_null())
                        .collect();

                    SearchSuggestionCache::set_mv_results(&mv_hash, Value::Array(data.clone()));
                    SearchSuggestionCache::set_mv_refresh_trigger_if_absent(&mv_hash);

                    log::debug!(
                        "Search Suggestion: Materialized View data found from matview: {}",
                        matviewname
                    );
                    return HttpResponse::Ok().json(ApiResponse {
                        success: true,
                        message: format!(
                            "[Materialized View]: Search suggestions operation completed for table: {}",
                            &table
                        ),
                        count: data.len() as i32,
                        data,
                    });
                }
            }
        }
    }

    // get the aliased entities
    let mut aliased_joined_entities = Vec::new();

    for join in joins {
        let (to_entity, to_alias) = if join.r#type == "self" {
            (
                join.field_relation.from.entity.clone(),
                join.field_relation.from.alias.clone(),
            )
        } else {
            (
                join.field_relation.to.entity.clone(),
                join.field_relation.to.alias.clone(),
            )
        };

        if let Some(alias) = to_alias {
            aliased_joined_entities.push(AliasedJoinedEntity { to_entity, alias });
        }
    }

    // format filters
    let mut search_term = String::new();
    let mut filtered_fields = Value::Object(serde_json::Map::new());
    let mut formatted_advance_filters: Vec<Value> = Vec::new();
    let mut formatted_group_advance_filters: Vec<Value> = Vec::new();
    if !group_advance_filters.is_empty() {
        for grouped_filters in group_advance_filters {
            let filters = match grouped_filters {
                GroupAdvanceFilter::Criteria { filters, .. } => filters.clone(),
                GroupAdvanceFilter::Operator { filters, .. } => filters.clone(),
            };

            let FormatFilterResponse {
                formatted_filters,
                search_term: _search_term,
                filtered_fields: _filtered_fields,
            } = format_filters(
                filters,
                Some(&aliased_joined_entities),
                &table,
                filtered_fields.clone(),
                search_term.clone(),
            );

            // Update the outer scope variables
            filtered_fields = _filtered_fields;
            search_term = _search_term;

            // Create a new GroupAdvanceFilter with the formatted filters
            let updated_group_filter = match grouped_filters {
                GroupAdvanceFilter::Criteria { operator, .. } => GroupAdvanceFilter::Criteria {
                    operator: operator.clone(),
                    filters: formatted_filters
                        .into_iter()
                        .filter_map(|v| serde_json::from_value(v).ok())
                        .collect(),
                },
                GroupAdvanceFilter::Operator { operator, .. } => GroupAdvanceFilter::Operator {
                    operator: operator.clone(),
                    filters: formatted_filters
                        .into_iter()
                        .filter_map(|v| serde_json::from_value(v).ok())
                        .collect(),
                },
            };

            // Convert to Value for the final result
            formatted_group_advance_filters
                .push(serde_json::to_value(updated_group_filter).unwrap_or(Value::Null));
        }
    } else {
        let FormatFilterResponse {
            formatted_filters: _formatted_advance_filters,
            search_term: _search_term,
            filtered_fields: _filtered_fields,
        } = format_filters(
            advance_filters.clone(),
            Some(&aliased_joined_entities),
            &table,
            filtered_fields,
            search_term,
        );
        search_term = _search_term;
        filtered_fields = _filtered_fields;
        formatted_advance_filters = _formatted_advance_filters;
    }

    // generate concatenated fields
    let concatenated_expressions = generate_concatenated_expressions(
        concatenate_fields.clone(),
        Some(date_format.as_str()),
        timezone.as_deref(),
        time_format,
    );

    // get connection to Diesel
    let mut conn = db::get_async_connection().await;
    // generate json build object query (use resolved timezone with body precedence)
    let mut sql_constructor: SearchSQLContructor<SearchSuggestionParams> =
        SearchSQLContructor::new(parameters, table.clone(), is_root, timezone.clone());
    if let Some(org_id) = organization_id {
        sql_constructor = sql_constructor.with_organization_id(org_id);
    }

    let query = match sql_constructor.construct(
        &filtered_fields,
        &formatted_advance_filters,
        &formatted_group_advance_filters,
        &search_term.clone(),
        &concatenated_expressions.clone(),
    ) {
        Ok(sql) => sql,
        Err(e) => {
            return HttpResponse::BadRequest().json(ApiResponse {
                success: false,
                message: format!("Invalid Search configuration: {}", e),
                count: 0,
                data: vec![],
            });
        }
    };
    if let Err(e) = write_query_to_debug_log(&query, &table, false).await {
        log::warn!("Failed to write debug query log: {}", e);
    }

    log::debug!("Search Suggestion Query: {}", query);
    let mv_ready = match ensure_materialized_view(mv_name.clone(), query.clone()).await {
        Ok(_) => true,
        Err(e) => {
            log::warn!(
                "Materialized view setup failed for {}: {}",
                mv_name,
                e.message
            );
            false
        }
    };

    let final_query = if mv_ready {
        format!("SELECT row_to_json FROM \"{}\"", mv_name)
    } else {
        format!("SELECT row_to_json(t) AS row_to_json FROM ({}) t", query)
    };

    // execute query
    let results = match diesel::dsl::sql_query(&final_query)
        .load::<DynamicResult>(&mut conn)
        .await
    {
        Ok(results) => results,
        Err(e) => {
            return HttpResponse::InternalServerError().json(ApiResponse {
                success: false,
                message: format!("Query execution error: {}", e),
                count: 0,
                data: vec![],
            });
        }
    };

    let data: Vec<serde_json::Value> = results
        .into_iter()
        .filter_map(|result| result.row_to_json)
        .map(|data| {
            if let Some(data) = data.get("results").and_then(|v| v.get("data")) {
                data.clone()
            } else {
                data
            }
        })
        .filter(|item| !item.is_null())
        .collect();

    SearchSuggestionCache::set_mv_results(&mv_hash, Value::Array(data.clone()));
    SearchSuggestionCache::set_mv_refresh_trigger_if_absent(&mv_hash);

    HttpResponse::Ok().json(ApiResponse {
        success: true,
        message: format!(
            "Search suggestions operation completed for table: {}",
            &table
        ),
        count: data.len() as i32,
        data: if data.len() > 0 { data } else { vec![] },
    })
}

/// Helper function to write query debug logs to file
async fn write_query_to_debug_log(
    query: &str,
    table: &str,
    is_body_params: bool,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Create logs directory in root folder (current directory)
    let logs_dir = match std::env::current_dir() {
        Ok(current_dir) => current_dir.join("logs"),
        Err(e) => return Err(Box::new(e)),
    };
    tokio::fs::create_dir_all(&logs_dir).await?;

    // Create filename with current date
    let current_date = Local::now().format("%Y-%m-%d_%H-%M-%S").to_string();
    let log_file = logs_dir.join(format!("sql_queries_{}.log", current_date));

    // Format the log entry
    let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S%.3f").to_string();
    let formatted_query = query.replace("\n", " ").trim().to_string();

    // Add parameters indicator if body params
    let params_indicator = if is_body_params {
        "Body Parameters:"
    } else {
        "Query Parameters:"
    };

    let log_entry = format!(
        "[{}] Table: {}\n{}: {}\n{}\n",
        timestamp,
        table,
        params_indicator,
        formatted_query,
        "-".repeat(80)
    );

    // Open file in append mode
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(log_file)
        .await?;

    // Write the log entry
    file.write_all(log_entry.as_bytes()).await?;
    file.flush().await?;

    Ok(())
}
static ACTIVE_MV_REFRESH: Lazy<Mutex<HashSet<String>>> = Lazy::new(|| Mutex::new(HashSet::new()));

async fn ensure_materialized_view(mv_name: String, base_query: String) -> Result<(), ApiError> {
    let mut conn = db::get_async_connection().await;
    let create_mv_sql = format!(
        r#"CREATE MATERIALIZED VIEW IF NOT EXISTS "{}" AS
           SELECT
             (md5(row_to_json(t)::text) || ':' || (row_number() OVER ())::text) AS row_key,
             row_to_json(t) AS row_to_json
           FROM ({}) t"#,
        mv_name, base_query
    );
    if let Err(e) = sql_query(&create_mv_sql).execute(&mut conn).await {
        log::warn!("Failed to create materialized view {}: {}", mv_name, e);
        return Err(ApiError {
            message: format!("Failed to create materialized view {}: {}", mv_name, e),
            status: 500,
        });
    }

    // Ensure a unique index exists to allow CONCURRENTLY refresh
    let create_idx_sql = format!(
        r#"CREATE UNIQUE INDEX IF NOT EXISTS "{}_uix_row_key" ON "{}" (row_key)"#,
        mv_name, mv_name
    );
    if let Err(e) = sql_query(&create_idx_sql).execute(&mut conn).await {
        log::warn!(
            "Failed to create unique index for materialized view {}: {}",
            mv_name,
            e
        );
        return Ok(());
    }

    Ok(())
}

async fn refresh_materialized_view_once(mv_name: String) -> Result<(), ApiError> {
    {
        let mut set = ACTIVE_MV_REFRESH.lock().unwrap();
        if !set.insert(mv_name.clone()) {
            return Ok(());
        }
    }

    let refresh_sql = format!(r#"REFRESH MATERIALIZED VIEW CONCURRENTLY "{}""#, mv_name);
    let mut conn = db::get_async_connection().await;
    let result = sql_query(&refresh_sql).execute(&mut conn).await;

    {
        let mut set = ACTIVE_MV_REFRESH.lock().unwrap();
        set.remove(&mv_name);
    }

    match result {
        Ok(_) => Ok(()),
        Err(e) => Err(ApiError {
            message: format!("Failed to refresh materialized view {}: {}", mv_name, e),
            status: 500,
        }),
    }
}
/// Schema verification endpoint
/// Verifies that a table exists and optionally checks if specified fields exist in the table
#[derive(Deserialize)]
pub struct SchemaVerificationRequest {
    pub entity: String,
    pub fields: Option<Vec<String>>,
}

#[derive(Serialize)]
pub struct SchemaVerificationResponse {
    pub exists: bool,
    pub available_fields: Vec<String>,
    pub missing_fields: Vec<String>,
}

pub async fn verify_schema(
    _auth: HttpRequest,
    request_body: web::Json<SchemaVerificationRequest>,
) -> impl Responder {
    use crate::database::schema::verify::{field_exists_in_table, get_table_fields};

    let request = request_body.into_inner();
    let table_name = request.entity;

    // Check if table exists by trying to get its fields
    let available_fields = match get_table_fields(&table_name) {
        Some(fields) => fields,
        None => {
            // Table doesn't exist
            return HttpResponse::Ok().json(SchemaVerificationResponse {
                exists: false,
                available_fields: vec![],
                missing_fields: request.fields.unwrap_or_default(),
            });
        }
    };

    // Table exists, now check requested fields if provided
    let mut missing_fields = Vec::new();

    if let Some(fields_to_check) = request.fields {
        for field in &fields_to_check {
            if !field_exists_in_table(&table_name, field) {
                missing_fields.push(field.clone());
            }
        }
    }

    HttpResponse::Ok().json(SchemaVerificationResponse {
        exists: true,
        available_fields,
        missing_fields,
    })
}
