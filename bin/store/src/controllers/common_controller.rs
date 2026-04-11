use crate::database::db::{self, DatabaseTypeConverter};

use crate::database::schema::verify::field_exists_in_table;
use crate::generated::table_enum::{generate_code, Table};
use crate::providers::operations::sync::sync_service::{insert, update};
use crate::structs::core::{ApiResponse, Auth, RequestBody};
use crate::utils::parse_filters::{build_sql_filter, SqlFilter};
use crate::utils::structs::FilterCriteria;
use actix_web::http;
use bytes::Bytes;
use csv::WriterBuilder;
use futures::{pin_mut, SinkExt};
use serde_json::Value;
use std::env;
use tokio_postgres::Client;
use tonic::Status;

use super::store_controller::ApiError;

#[derive(Debug)]
#[allow(warnings)]
pub enum AppError {
    DbConnection(String),
    CopyCommand(String),
    CsvConversion(String),
    DataSend(String),
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppError::DbConnection(msg) => write!(f, "Database connection failed: {}", msg),
            AppError::CopyCommand(msg) => write!(f, "COPY command failed: {}", msg),
            AppError::CsvConversion(msg) => write!(f, "CSV conversion failed: {}", msg),
            AppError::DataSend(msg) => write!(f, "Failed to send data to database: {}", msg),
        }
    }
}

impl From<csv::Error> for AppError {
    fn from(e: csv::Error) -> Self {
        AppError::CsvConversion(e.to_string())
    }
}

impl From<tokio_postgres::Error> for AppError {
    fn from(e: tokio_postgres::Error) -> Self {
        AppError::DbConnection(e.to_string())
    }
}

// /// Check if a table physically exists in the database by querying information_schema.
// pub async fn temp_table_exists_in_db(table_name: &str) -> Result<bool, AppError> {
//     let client = db::create_connection()
//         .await
//         .map_err(|e| AppError::DbConnection(e.to_string()))?;
//     let row = client
//         .query_one(
//             "SELECT EXISTS (SELECT 1 FROM information_schema.tables WHERE table_name = $1)",
//             &[&table_name],
//         )
//         .await
//         .map_err(|e| AppError::DbConnection(e.to_string()))?;
//     let exists: bool = row.get(0);
//     Ok(exists)
// }

pub fn migration_mode_enabled() -> bool {
    env::var("MIGRATION_MODE")
        .ok()
        .map(|v| {
            let v = v.trim();
            v.eq_ignore_ascii_case("true") || v == "1"
        })
        .unwrap_or(false)
}

pub async fn execute_copy(
    client: &Client,
    table_name: &str,
    columns: &[&str],
    csv_data: Vec<u8>,
) -> Result<(), AppError> {
    let temp_table_name = format!("temp_{}", table_name);

    // Quote column names to handle reserved keywords (e.g. "order")
    let quoted_columns: Vec<String> = columns.iter().map(|c| format!("\"{}\"", c)).collect();
    let columns_str = quoted_columns.join(",");

    let stmt = format!(
        "COPY {} ({}) FROM STDIN WITH (FORMAT csv)",
        table_name, columns_str
    );

    let temp_stmt = format!(
        "COPY {} ({}) FROM STDIN WITH (FORMAT csv)",
        temp_table_name, columns_str
    );

    // Wrap both COPYs in a single transaction so they both succeed or both roll back.
    client
        .execute("BEGIN", &[])
        .await
        .map_err(|e| AppError::DbConnection(format!("BEGIN transaction for table '{}': {}", table_name, e)))?;

    // COPY into main table
    let sink = client
        .copy_in(&stmt)
        .await
        .map_err(|e| AppError::CopyCommand(format!("table '{}': {}", table_name, e)))?;
    pin_mut!(sink);
    sink.send(Bytes::from(csv_data.clone()))
        .await
        .map_err(|e| AppError::DataSend(format!("table '{}': {}", table_name, e)))?;
    sink.close()
        .await
        .map_err(|e| AppError::DataSend(format!("table '{}' closing COPY stream: {}", table_name, e)))?;

    // COPY into temp table
    let sink = client
        .copy_in(&temp_stmt)
        .await
        .map_err(|e| AppError::CopyCommand(format!("temp table '{}': {}", temp_table_name, e)))?;
    pin_mut!(sink);
    sink.send(Bytes::from(csv_data))
        .await
        .map_err(|e| AppError::DataSend(format!("temp table '{}': {}", temp_table_name, e)))?;
    sink.close()
        .await
        .map_err(|e| AppError::DataSend(format!("temp table '{}' closing COPY stream: {}", temp_table_name, e)))?;

    // Both succeeded — commit
    client
        .execute("COMMIT", &[])
        .await
        .map_err(|e| AppError::DbConnection(format!("COMMIT transaction for table '{}': {}", table_name, e)))?;

    Ok(())
}

pub fn process_records(
    records: Vec<Value>,
    table_name: &str,
    auth: &Auth,
    is_root_account: bool,
) -> Result<(Vec<Value>, Vec<String>), String> {
    let hypertable_exists = field_exists_in_table(table_name, "hypertable_timestamp");
    let mut processed_records = Vec::new();

    for record in records {
        // Run your custom processing logic
        let mut request_body = RequestBody { record };
        request_body.process_record("create", auth, is_root_account, table_name);

        // Hypertable timestamp logic
        if hypertable_exists {
            if let Some(obj) = request_body.record.as_object_mut() {
                if let Some(timestamp) = obj.get("timestamp") {
                    obj.insert("hypertable_timestamp".to_string(), timestamp.clone());
                }
            }
        }

        //insert is_batch to the record and set it to true
        if let Some(obj) = request_body.record.as_object_mut() {
            obj.insert("is_batch".to_string(), serde_json::Value::Bool(true));
            obj.insert(
                "sync_status".to_string(),
                serde_json::Value::String("complete".to_string()),
            );
        }

        processed_records.push(request_body.record);
    }

    let columns = extract_columns_owned(&processed_records)?;
    Ok((processed_records, columns))
}

fn extract_columns_owned(records: &[Value]) -> Result<Vec<String>, String> {
    records
        .first()
        .and_then(|v| v.as_object())
        .map(|obj| obj.keys().map(|k| k.to_string()).collect()) // Use to_string() to create owned Strings
        .ok_or_else(|| "Invalid record format".to_string())
}

pub fn convert_json_to_csv(
    records: &[Value],
    columns: &[String],
    table_name: &str,
) -> Result<Vec<u8>, AppError> {
    use crate::database::schema::verify::field_type_in_table;
    use std::collections::HashSet;

    // Build a set of columns that are JSON/JSONB so we can serialize them properly
    let json_columns: HashSet<&str> = columns
        .iter()
        .filter(|col| {
            field_type_in_table(table_name, col)
                .map(|info| info.is_json)
                .unwrap_or(false)
        })
        .map(|s| s.as_str())
        .collect();

    let mut wtr = WriterBuilder::new().has_headers(false).from_writer(vec![]);

    for record in records {
        let obj = record
            .as_object()
            .ok_or_else(|| AppError::CsvConversion("Invalid record format".to_string()))?;

        let row: Vec<String> = columns
            .iter()
            .map(|col| {
                let val = obj.get(col.as_str()).unwrap_or(&Value::Null);
                if json_columns.contains(col.as_str()) {
                    // For JSON/JSONB columns, always output valid JSON
                    match val {
                        Value::Null => String::new(),
                        _ => serde_json::to_string(val).unwrap_or_default(),
                    }
                } else {
                    serialize_value(val)
                }
            })
            .collect();

        wtr.write_record(&row)
            .map_err(|e| AppError::CsvConversion(e.to_string()))?;
    }

    wtr.into_inner()
        .map_err(|e| AppError::CsvConversion(e.to_string()))
}

fn serialize_value(value: &Value) -> String {
    match value {
        Value::String(s) => s.clone(),
        Value::Null => String::new(),
        Value::Number(n) => n.to_string(),
        Value::Bool(b) => b.to_string(),
        Value::Array(arr) => {
            // If any element is an object or nested array, treat the whole thing as JSON
            let is_json_array = arr
                .iter()
                .any(|v| matches!(v, Value::Object(_) | Value::Array(_)));
            if is_json_array {
                serde_json::to_string(value).unwrap_or_default()
            } else {
                // Simple Postgres array literal for scalar arrays (text[], int[], etc.)
                format!(
                    "{{{}}}",
                    arr.iter()
                        .map(serialize_value)
                        .collect::<Vec<_>>()
                        .join(",")
                )
            }
        }
        Value::Object(_) => serde_json::to_string(value).unwrap_or_default(),
    }
}

//BATCH UPDATE FUNCTIONS

/// Convert serde_json::Value parameters to PostgreSQL-compatible types
/// This function now uses the centralized DatabaseTypeConverter for better error handling
/// and consistency across the application.
pub fn convert_params_to_sql_types(
    params: &[serde_json::Value],
) -> Result<Vec<Box<dyn tokio_postgres::types::ToSql + Sync + Send>>, String> {
    DatabaseTypeConverter::values_to_sql_params(params)
}

// Alternative implementation using the new batch_update provider:
// This demonstrates how to use the BatchUpdateSQLConstructor to reuse
// WHERE clause logic from find/sql_constructor.rs, avoiding code duplication
//
// pub async fn perform_batch_update_with_provider(
//     table_name: &str,
//     updates: Value,
//     filters: Vec<Value>,
//     is_root: bool,
//     organization_id: Option<String>,
// ) -> Result<(usize, Vec<Value>), String> {
//     let filter_criteria: Vec<crate::structs::core::FilterCriteria> =
//         serde_json::from_value(Value::Array(filters))
//             .map_err(|e| format!("Failed to parse filters: {}", e))?;
//
//     let mut batch_constructor = BatchUpdateSQLConstructor::new(table_name.to_string(), is_root);
//     if let Some(org_id) = organization_id {
//         batch_constructor = batch_constructor.with_organization_id(org_id);
//     }
//
//     // Option 1: Use simple approach (same as current implementation)
//     let update_result = build_sql_update(&updates, 1);
//     let (return_fields, update_sql, mut params) =
//         build_update_statement(update_result, &updates, table_name, vec![])
//             .map_err(|e| e.message)?;
//
//     let SqlFilter { sql: where_clause, params: where_params } =
//         batch_constructor.construct_where_clauses_simple(&filter_criteria);
//     params.extend(where_params);
//
//     let sql = format!(
//         "UPDATE {} SET {} WHERE {} RETURNING {}",
//         table_name, update_sql, where_clause, return_fields
//     );
//
//     // Option 2: Use advanced approach (leverages find/sql_constructor.rs logic)
//     // let where_clause = batch_constructor.construct_where_clauses_advanced(&filter_criteria)
//     //     .map_err(|e| format!("Failed to construct WHERE clause: {}", e))?;
//     // let sql = format!(
//     //     "UPDATE {} SET {}{} RETURNING {}",
//     //     table_name, update_sql, where_clause, return_fields
//     // );
//
//     // ... rest of the function remains the same
// }

pub fn sanitize_updates(mut record: serde_json::Map<String, Value>) -> Option<Value> {
    record.remove("version");
    record.remove("id");
    record.remove("timestamp");
    if record.is_empty() {
        None
    } else {
        Some(Value::Object(record))
    }
}

//Insert request
type ControllerResult = Result<ApiResponse, ApiError>;
pub async fn process_record_for_insert<T: serde::Serialize>(
    record: T,
    table_name: &str,
    auth: &Auth,
    is_root_account: bool,
) -> Result<serde_json::Value, Status> {
    // Convert protobuf message to serde_json::Value
    let mut processed_record = serde_json::to_value(&record)
        .map_err(|e| Status::internal(format!("Failed to process record: {}", e)))?;

    // Process record through common processing logic
    let mut request_body = RequestBody {
        record: processed_record,
    };
    request_body.process_record("create", auth, is_root_account, table_name);
    processed_record = request_body.record;
    if field_exists_in_table(table_name, "hypertable_timestamp") {
        if let Some(obj) = processed_record.as_object_mut() {
            if let Some(timestamp) = obj.get("timestamp") {
                obj.insert("hypertable_timestamp".to_string(), timestamp.clone());
            }
        }
    }

    // Convert back to Value
    let record_value = serde_json::from_value(processed_record)
        .map_err(|e| Status::internal(format!("Failed to process record: {}", e)))?;

    Ok(record_value)
}

pub async fn process_and_insert_record(
    table_name: &str,
    mut record: Value,
    pluck_fields: Option<Vec<String>>,
    auth: &Auth,
    is_root_account: bool,
) -> ControllerResult {
    // In MIGRATION_MODE we trust the incoming 'code' (if any) and do NOT auto-generate/override it.
    if !migration_mode_enabled() {
        let code = generate_code(table_name, "", 100000).await.map_err(|e| {
            ApiError::new(
                http::StatusCode::INTERNAL_SERVER_ERROR,
                format!("Unable to generate code: {}", e),
            )
        })?;
        // assign code in the record (override any existing value)
        if let Value::Object(ref mut map) = record {
            map.insert("code".to_string(), Value::String(code));
        } else {
            return Err(ApiError::new(
                http::StatusCode::BAD_REQUEST,
                "Record must be an object".to_string(),
            ));
        }
    }
    let record_value = process_record_for_insert(record, table_name, auth, is_root_account)
        .await
        .map_err(|e| {
            ApiError::new(
                http::StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to process record: {}", e),
            )
        })?;

    // Get table instance
    let table = Table::from_str(table_name).ok_or_else(|| {
        ApiError::new(
            http::StatusCode::BAD_REQUEST,
            format!("Unknown table: {}", table_name),
        )
    })?;

    // If record has an id that already exists, perform update instead of insert (avoid duplicate key)
    let id_to_check = record_value
        .get("id")
        .and_then(|v| v.as_str())
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string());
    if let Some(id_val) = id_to_check {
        let mut conn = db::get_async_connection().await;
        let existing = table
            .get_by_id(
                &mut conn,
                &id_val,
                is_root_account,
                Some(auth.organization_id.clone()),
            )
            .await
            .map_err(|e| {
                ApiError::new(
                    http::StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Failed to check existing record: {}", e),
                )
            })?;
        if existing.is_some() {
            return process_and_update_record(
                table_name,
                record_value,
                &id_val,
                pluck_fields,
                "update",
                auth,
                is_root_account,
            )
            .await;
        }
    }

    // Insert record
    insert(&table_name.to_string(), record_value.clone())
        .await
        .map_err(|e| {
            ApiError::new(
                http::StatusCode::INTERNAL_SERVER_ERROR,
                format!("Sync error: {}", e),
            )
        })?;
    let plucked_record: serde_json::Value = match pluck_fields {
        Some(fields) => table.pluck_fields(&record_value, fields),
        None => record_value,
    };

    // Return success response
    Ok(ApiResponse {
        success: true,
        message: format!("Record inserted into '{}'", table_name),
        count: 1,
        data: vec![plucked_record],
    })
}

// Update request

pub async fn process_record_for_update<T: serde::Serialize>(
    record: T,
    table_name: &str,
    record_id: &str,
    table: &Table,
    operation: &str,
    auth: &Auth,
    is_root_account: bool,
) -> Result<serde_json::Value, Status> {
    // Convert to serde_json::Value
    let mut processed_record = serde_json::to_value(&record)
        .map_err(|e| Status::internal(format!("Failed to process record: {}", e)))?;

    if let Some(obj) = processed_record.as_object_mut() {
        obj.insert(
            "id".to_string(),
            serde_json::Value::String(record_id.to_string().clone()),
        );
    }

    // Process record through common processing logic
    let mut request_body = RequestBody {
        record: processed_record.clone(),
    };
    request_body.process_record(operation, auth, is_root_account, table_name);
    processed_record = request_body.record;

    // Check if record exists
    let mut conn = db::get_async_connection().await;
    let data_exists = table
        .get_by_id(
            &mut conn,
            record_id,
            is_root_account,
            Some(auth.organization_id.clone()),
        )
        .await;
    match data_exists {
        Ok(record) => {
            if record.is_none() {
                return Err(Status::not_found(format!(
                    "Record with ID '{}' not found in '{}'",
                    record_id, table_name
                )));
            }
        }
        Err(e) => {
            return Err(Status::internal(format!("Failed to get record: {}", e)));
        }
    }

    // Handle hypertable timestamp if needed
    if field_exists_in_table(table_name, "hypertable_timestamp") {
        let timestamp_result = match table.get_hypertable_timestamp(&mut conn, record_id).await {
            Ok(timestamp) => timestamp,
            Err(e) => {
                return Err(Status::internal(format!(
                    "Failed to get hypertable_timestamp: {}",
                    e
                )));
            }
        };

        if let Some(obj) = processed_record.as_object_mut() {
            if let Some(timestamp) = timestamp_result {
                log::debug!("Found hypertable timestamp: {}", timestamp);
                obj.insert(
                    "hypertable_timestamp".to_string(),
                    serde_json::Value::String(timestamp),
                );
            } else {
                log::warn!("No hypertable_timestamp found: {}", record_id);
                return Err(Status::internal(format!(
                    "Failed to insert hypertable timestamp in record"
                )));
            }
        }
    }

    Ok(processed_record)
}

pub async fn process_and_update_record(
    table_name: &str,
    record: Value,
    id: &str,
    pluck_fields: Option<Vec<String>>,
    operation: &str,
    auth: &Auth,
    is_root_account: bool,
) -> ControllerResult {
    let table = Table::from_str(table_name).ok_or_else(|| {
        ApiError::new(
            http::StatusCode::BAD_REQUEST,
            format!("Unknown table: {}", table_name),
        )
    })?;

    let processed_record = process_record_for_update(
        record,
        table_name,
        id,
        &table,
        operation,
        auth,
        is_root_account,
    )
    .await
    .map_err(|status| {
        ApiError::new(
            http::StatusCode::INTERNAL_SERVER_ERROR,
            status.message().to_string(),
        )
    })?;

    update(
        &table_name.to_string(),
        processed_record.clone(),
        &id.to_string(),
    )
    .await
    .map_err(|e| {
        ApiError::new(
            http::StatusCode::INTERNAL_SERVER_ERROR,
            format!("Sync error: {}", e),
        )
    })?;

    let plucked_record: serde_json::Value = match pluck_fields {
        Some(fields) => {
            // Check if processed_record contains all required fields
            let missing_fields: Vec<&String> = fields
                .iter()
                .filter(|field| {
                    !processed_record
                        .as_object()
                        .map(|obj| obj.contains_key(*field))
                        .unwrap_or(false)
                })
                .collect();

            if !missing_fields.is_empty() {
                // If fields are missing, fetch the complete record using get_by_id
                let mut conn = db::get_async_connection().await;
                let complete_record = table
                    .get_by_id(
                        &mut conn,
                        id,
                        is_root_account,
                        Some(auth.organization_id.clone()),
                    )
                    .await
                    .map_err(|e| {
                        ApiError::new(
                            http::StatusCode::INTERNAL_SERVER_ERROR,
                            format!("Failed to get complete record: {}", e),
                        )
                    })?;

                match complete_record {
                    Some(record_value) => table.pluck_fields(&record_value, fields),
                    None => {
                        return Err(ApiError::new(
                            http::StatusCode::NOT_FOUND,
                            format!("Record with ID '{}' not found in '{}'", id, table_name),
                        ));
                    }
                }
            } else {
                // All fields are present in processed_record
                table.pluck_fields(&processed_record, fields)
            }
        }
        None => processed_record,
    };

    Ok(ApiResponse {
        success: true,
        message: format!("Record updated in '{}'", table_name),
        count: 1,
        data: vec![plucked_record],
    })
}

//getById

// Add this function after the perform_upsert function

pub async fn process_and_get_record_by_id(
    table_name: &str,
    id: &str,
    pluck_fields: Option<Vec<String>>,
    is_root_account: bool,
    organization_id: Option<&str>,
) -> ControllerResult {
    let table = Table::from_str(table_name).ok_or_else(|| {
        ApiError::new(
            http::StatusCode::BAD_REQUEST,
            format!("Unknown table: {}", table_name),
        )
    })?;

    // Create database connection
    let mut conn = db::get_async_connection().await;

    // Get record by ID
    let record = table
        .get_by_id(
            &mut conn,
            id,
            is_root_account,
            organization_id.map(|s| s.to_string()),
        )
        .await
        .map_err(|e| {
            ApiError::new(
                http::StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to get record: {}", e),
            )
        })?;

    // Check if record exists
    match record {
        Some(record_value) => {
            // Apply pluck fields if provided
            let plucked_record: serde_json::Value = match pluck_fields {
                Some(fields) => table.pluck_fields(&record_value, fields),
                None => record_value,
            };

            Ok(ApiResponse {
                success: true,
                message: format!("Record found in '{}'", table_name),
                count: 1,
                data: vec![plucked_record],
            })
        }
        None => Err(ApiError::new(
            http::StatusCode::NOT_FOUND,
            format!("Record with ID '{}' not found in '{}'", id, table_name),
        )),
    }
}

//Common upsert

pub async fn perform_upsert(
    table_name: &str,
    conflict_columns: Vec<String>,
    data: serde_json::Value,
    pluck_fields: Option<Vec<String>>,
    auth: &Auth,
    is_root_account: bool,
) -> Result<ApiResponse, ApiError> {
    // Build filters from conflict columns
    let filters = FilterCriteria::build_from_conflict_columns(conflict_columns, &data)
        .map_err(|e| ApiError::new(http::StatusCode::BAD_REQUEST, e))?;

    // Build SQL filter
    let SqlFilter { sql, params } = build_sql_filter(&filters.clone())
        .map_err(|e| ApiError::new(http::StatusCode::BAD_REQUEST, e.to_string()))?;
    let converted_params = convert_params_to_sql_types(&params).map_err(|e| {
        ApiError::new(
            http::StatusCode::BAD_REQUEST,
            format!("Failed to convert parameters: {}", e),
        )
    })?;
    let query = format!("SELECT id FROM {} WHERE {} LIMIT 1", table_name, sql);
    let pg_params: Vec<&(dyn tokio_postgres::types::ToSql + Sync)> = converted_params
        .iter()
        .map(|b| b.as_ref() as &(dyn tokio_postgres::types::ToSql + Sync))
        .collect();

    // Create database connection
    let conn = db::create_connection().await.map_err(|e| {
        ApiError::new(
            http::StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to establish database connection: {}", e),
        )
    })?;

    // Check if record exists
    let row = conn.query_opt(&query, &pg_params[..]).await.map_err(|e| {
        ApiError::new(
            http::StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to execute query: {}", e),
        )
    })?;

    // Get record ID if exists, otherwise empty string
    let record_id = match row {
        Some(row) => row.get::<_, String>(0),
        None => "".to_string(),
    };

    // Either insert or update based on existence
    if record_id.is_empty() {
        // If the record doesn't exist, perform an insert
        process_and_insert_record(table_name, data, pluck_fields, auth, is_root_account).await
    } else {
        // If the record exists, perform an update
        process_and_update_record(
            table_name,
            data,
            &record_id,
            pluck_fields,
            "update",
            auth,
            is_root_account,
        )
        .await
    }
}
