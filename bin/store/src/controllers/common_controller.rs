use crate::batch_sync::BatchSyncService;
use crate::db::{self, create_connection};
use crate::schema::verify::field_exists_in_table;
use crate::structs::structs::{ApiResponse, Auth, RequestBody, SqlUpdate};
use crate::sync::sync_service::{insert, update};
use crate::table_enum::{generate_code, Table};
use crate::utils::parse_filters::{build_sql_filter, SqlFilter};
use crate::utils::structs::FilterCriteria;
use actix_web::{http, HttpResponse};
use bytes::Bytes;
use csv::WriterBuilder;
use futures::{pin_mut, SinkExt};
use serde_json::{json, Value};
use tokio_postgres::Client;
use tonic::Status;

use super::store_controller::ApiError;

#[derive(Debug)]
pub enum AppError {
    DbConnection(String),
    CopyCommand(String),
    CsvConversion(String),
    DataSend(String),
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

pub async fn execute_copy(
    client: &Client,
    table_name: &str,
    columns: &[&str],
    csv_data: Vec<u8>,
) -> Result<(), AppError> {
    let temp_table_name = format!("temp_{}", table_name);
    
    // Create statements for both tables
    let stmt = format!(
        "COPY {} ({}) FROM STDIN WITH (FORMAT csv)",
        table_name,
        columns.join(",")
    );
    
    let temp_stmt = format!(
        "COPY {} ({}) FROM STDIN WITH (FORMAT csv)",
        temp_table_name,
        columns.join(",")
    );
    
    // Clone the client and data for parallel operations
    let client_clone = client.clone();
    let csv_data_clone = csv_data.clone();
    
    // Execute both copy operations in parallel
    let (main_result, temp_result) = tokio::join!(
        async {
            let sink = client
                .copy_in(&stmt)
                .await
                .map_err(|e| AppError::CopyCommand(e.to_string()))?;
            
            pin_mut!(sink);
            sink.send(Bytes::from(csv_data))
                .await
                .map_err(|e| AppError::DataSend(e.to_string()))?;
            
            sink.close()
                .await
                .map_err(|e| AppError::DataSend(e.to_string()))
        },
        async {
            let sink = client_clone
                .copy_in(&temp_stmt)
                .await
                .map_err(|e| AppError::CopyCommand(e.to_string()))?;
            
            pin_mut!(sink);
            sink.send(Bytes::from(csv_data_clone))
                .await
                .map_err(|e| AppError::DataSend(e.to_string()))?;
            
            sink.close()
                .await
                .map_err(|e| AppError::DataSend(e.to_string()))
        }
    );
    
    // Check results from both operations
    main_result?;
    temp_result?;
    
    Ok(())
}

pub fn process_records(
    records: Vec<Value>,
    table_name: &str,
    auth: &Auth,
) -> Result<(Vec<Value>, Vec<String>), String> {
    let hypertable_exists = field_exists_in_table(table_name, "hypertable_timestamp");
    let mut processed_records = Vec::new();

    for mut record in records {
        // Run your custom processing logic
        let mut request_body = RequestBody { record };
        request_body.process_record("create", auth);

        // Hypertable timestamp logic
        if hypertable_exists {
            if let Some(obj) = request_body.record.as_object_mut() {
                if let Some(timestamp) = obj.get("timestamp") {
                    obj.insert("hypertable_timestamp".to_string(), timestamp.clone());
                }
            }
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

pub fn convert_json_to_csv(records: &[Value], columns: &[String]) -> Result<Vec<u8>, AppError> {
    let mut wtr = WriterBuilder::new().has_headers(false).from_writer(vec![]);

    for record in records {
        let obj = record
            .as_object()
            .ok_or_else(|| AppError::CsvConversion("Invalid record format".to_string()))?;

        let row: Vec<String> = columns
            .iter()
            .map(|col| serialize_value(obj.get(col).unwrap_or(&Value::Null)))
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
        Value::Array(arr) => format!(
            "{{{}}}",
            arr.iter()
                .map(serialize_value)
                .collect::<Vec<_>>()
                .join(",")
        ),
        Value::Object(_) => serde_json::to_string(value).unwrap_or_default(),
    }
}

//BATCH UPDATE FUNCTIONS

pub fn convert_params_to_sql_types(
    params: &[serde_json::Value],
) -> Vec<Box<dyn tokio_postgres::types::ToSql + Sync + Send>> {
    let mut converted_values: Vec<Box<dyn tokio_postgres::types::ToSql + Sync + Send>> = Vec::new();

    for p in params {
        let boxed_value: Box<dyn tokio_postgres::types::ToSql + Sync + Send> = match p {
            serde_json::Value::Null => Box::new(None::<String>),
            serde_json::Value::Bool(b) => Box::new(*b),
            serde_json::Value::Number(n) => {
                if n.is_i64() {
                    let i_val = n.as_i64().unwrap();
                    if i_val >= i32::MIN as i64 && i_val <= i32::MAX as i64 {
                        Box::new(i_val as i32)
                    } else {
                        Box::new(i_val)
                    }
                } else if n.is_u64() {
                    let u_val = n.as_u64().unwrap();
                    if u_val <= i32::MAX as u64 {
                        Box::new(u_val as i32)
                    } else if u_val <= i64::MAX as u64 {
                        Box::new(u_val as i64)
                    } else {
                        Box::new(u_val.to_string())
                    }
                } else {
                    Box::new(n.as_f64().unwrap())
                }
            }
            serde_json::Value::String(s) => {
                // Try to parse as IpAddr first for inet fields
                if let Ok(ip) = s.parse::<std::net::IpAddr>() {
                    Box::new(ip)
                } else {
                    Box::new(s.clone())
                }
            }
            serde_json::Value::Array(arr) => {
                // Convert array elements to Vec<String>
                let string_array: Vec<String> = arr
                    .iter()
                    .map(|v| match v {
                        serde_json::Value::String(s) => s.clone(),
                        _ => v.to_string(),
                    })
                    .collect();
                Box::new(string_array)
            }
            _ => Box::new(format!("{}", p)),
        };
        converted_values.push(boxed_value);
    }

    converted_values
}

pub fn process_result_rows(
    rows: &[tokio_postgres::Row],
    update_fields: &[(&String, &serde_json::Value)],
    is_hypertable: bool,
) -> Vec<serde_json::Value> {
    let mut result_rows: Vec<serde_json::Value> = Vec::new();

    for row in rows {
        let mut obj = serde_json::Map::new();

        if let Ok(id) = row.try_get::<_, String>("id") {
            obj.insert("id".to_string(), serde_json::Value::String(id));
        }

        if let Ok(version) = row.try_get::<_, i32>("version") {
            obj.insert(
                "version".to_string(),
                serde_json::Value::Number(version.into()),
            );
        }

        if is_hypertable {
            if let Ok(timestamp) = row.try_get::<_, String>("hypertable_timestamp") {
                obj.insert(
                    "hypertable_timestamp".to_string(),
                    serde_json::Value::String(timestamp),
                );
            }
        }

        for (key, value) in update_fields.iter() {
            obj.insert(key.to_string(), (*value).clone());
        }

        result_rows.push(serde_json::Value::Object(obj));
    }

    result_rows
}

pub fn build_update_statement(
    update_result: Result<SqlUpdate, String>,
    updates: &serde_json::Value,
    table_name: &str,
    params: Vec<Value>,
) -> Result<(String, String, Vec<Value>), ApiResponse> {
    // Handle the update result
    let (mut update_sql, update_params) = match update_result {
        Ok(SqlUpdate { sql, params }) => (sql, params),
        Err(e) => {
            return Err(ApiResponse {
                success: false,
                message: e,
                count: 0,
                data: vec![],
            });
        }
    };

    // Add version increment
    update_sql = format!("{}, \"version\" = \"version\" + 1", update_sql);
    let mut params = params;
    params.extend(update_params);

    // Define return fields
    let return_fields = "id, version, updated_date, updated_time, updated_by";

    // Check for hypertable
    let is_hypertable = field_exists_in_table(table_name, "hypertable_timestamp");
    let hypertable_check = if is_hypertable {
        ", hypertable_timestamp"
    } else {
        ""
    };

    // Build updated fields list
    let updated_fields = if let Some(update_obj) = updates.as_object() {
        let fields: Vec<String> = update_obj
            .keys()
            .filter(|&k| k != "record")
            .map(|k| format!("\"{k}\""))
            .collect();

        if !fields.is_empty() {
            format!(", {}", fields.join(", "))
        } else {
            String::new()
        }
    } else {
        String::new()
    };

    Ok((
        return_fields.to_string() + hypertable_check + &updated_fields,
        update_sql,
        params,
    ))
}

pub fn build_sql_update(
    updates: &serde_json::Value,
    param_start_index: usize,
) -> Result<SqlUpdate, String> {
    let mut set_clauses = Vec::new();
    let mut params = Vec::new();
    let mut param_index = param_start_index;

    if let Some(update_map) = updates.as_object() {
        if update_map.is_empty() {
            return Err("No update fields provided".to_string());
        }

        for (key, value) in update_map {
            // Sanitize field name to prevent SQL injection
            let field = format!("\"{}\"", key.replace('\"', "\"\""));

            set_clauses.push(format!("{} = ${}", field, param_index));
            params.push(value.clone());
            param_index += 1;
        }

        Ok(SqlUpdate {
            sql: set_clauses.join(", "),
            params,
        })
    } else {
        Err("Updates must be an object".to_string())
    }
}

pub async fn perform_batch_update(
    table_name: &str,
    updates: Value,
    filters: Vec<Value>,
) -> Result<(usize, Vec<Value>), String> {
    let filter_criteria: Vec<FilterCriteria> = serde_json::from_value(Value::Array(filters))
        .map_err(|e| format!("Failed to parse filters: {}", e))?;

    let is_hypertable = field_exists_in_table(table_name, "hypertable_timestamp");
    let SqlFilter { sql, params } = build_sql_filter(&filter_criteria);
    let update_result = build_sql_update(&updates, params.len() + 1);
    let (return_fields, update_sql, params) =
        build_update_statement(update_result, &updates, table_name, params)
            .map_err(|e| e.message)?;

    let sql = format!(
        "UPDATE {} SET {} WHERE {} RETURNING {}",
        table_name, update_sql, sql, return_fields
    );

    let client = create_connection()
        .await
        .map_err(|e| format!("DB connection failed: {:?}", e))?;

    let converted_values = convert_params_to_sql_types(&params);
    let pg_params: Vec<&(dyn tokio_postgres::types::ToSql + Sync)> = converted_values
        .iter()
        .map(|b| b.as_ref() as &(dyn tokio_postgres::types::ToSql + Sync))
        .collect();

    let rows = client
        .query(&sql, &pg_params[..])
        .await
        .map_err(|e| format!("Failed to execute query: {}", e))?;

    let update_fields = updates
        .as_object()
        .map(|o| o.iter().filter(|(k, _)| *k != "record").collect::<Vec<_>>())
        .unwrap_or_default();

    let result_rows = process_result_rows(&rows, &update_fields, is_hypertable);

    for record in result_rows.iter() {
        BatchSyncService::send_update_message(table_name.to_string(), record.clone())
            .await
            .map_err(|e| format!("Sync error: {e}"))?;
    }

    Ok((rows.len(), result_rows))
}

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
) -> Result<serde_json::Value, Status> {
    // Convert protobuf message to serde_json::Value
    let mut processed_record = serde_json::to_value(&record)
        .map_err(|e| Status::internal(format!("Failed to process record: {}", e)))?;

    // Handle hypertable timestamp if needed
    if field_exists_in_table(table_name, "hypertable_timestamp") {
        if let Some(obj) = processed_record.as_object_mut() {
            if let Some(timestamp) = obj.get("timestamp") {
                obj.insert("hypertable_timestamp".to_string(), timestamp.clone());
            }
        }
    }

    // Process record through common processing logic
    let mut request_body = RequestBody {
        record: processed_record,
    };
    request_body.process_record("create", auth);
    processed_record = request_body.record;

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
) -> ControllerResult {
    // Process the record

    let entity_prefix_exists = if let Some(record_obj) = record.as_object() {
        record_obj.get("entity_prefix").and_then(|v| v.as_str())
    } else {
        None
    };
    let mut entity_prefix = "";
    // Return error if entity_prefix is not provided
    if entity_prefix_exists.is_none() {
        return Err(ApiError::new(
            http::StatusCode::BAD_REQUEST,
            "entity_prefix is required".to_string(),
        ));
    } else {
        entity_prefix = entity_prefix_exists.unwrap();
    }

    let code = generate_code(table_name, entity_prefix, 100000)
        .await
        .map_err(|e| {
            ApiError::new(
                http::StatusCode::INTERNAL_SERVER_ERROR,
                format!("Unable to generate code: {}", e),
            )
        })?;
    //assign code in the record
    if let Value::Object(ref mut map) = record {
        map.insert("code".to_string(), Value::String(code));
        map.remove("entity_prefix");
    } else {
        return Err(ApiError::new(
            http::StatusCode::BAD_REQUEST,
            "Record must be an object".to_string(),
        ));
    }
    let record_value = process_record_for_insert(record, table_name, auth)
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
    request_body.process_record(operation, auth);
    processed_record = request_body.record;

    // Check if record exists
    let mut conn = db::get_async_connection().await;
    let data_exists = table.get_by_id(&mut conn, record_id).await;
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
) -> ControllerResult {
    let table = Table::from_str(table_name).ok_or_else(|| {
        ApiError::new(
            http::StatusCode::BAD_REQUEST,
            format!("Unknown table: {}", table_name),
        )
    })?;

    let processed_record =
        process_record_for_update(record, table_name, id, &table, operation, auth)
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
        Some(fields) => table.pluck_fields(&processed_record, fields),
        None => processed_record,
    };

    Ok(ApiResponse {
        success: true,
        message: format!("Record updated in '{}'", table_name),
        count: 1,
        data: vec![plucked_record],
    })
}

//Common upsert
// ... existing code ...

pub async fn perform_upsert(
    table_name: &str,
    conflict_columns: Vec<String>,
    mut data: serde_json::Value,
    entity_prefix: String,
    pluck_fields: Option<Vec<String>>,
    auth: &Auth,
) -> Result<ApiResponse, ApiError> {
    // Build filters from conflict columns
    let filters = FilterCriteria::build_from_conflict_columns(conflict_columns, &data)
        .map_err(|e| ApiError::new(http::StatusCode::BAD_REQUEST, e))?;

    // Build SQL filter
    let SqlFilter { sql, params } = build_sql_filter(&filters.clone());
    let converted_params = convert_params_to_sql_types(&params);
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
        if let Value::Object(ref mut map) = data {
            map.insert("entity_prefix".to_string(), Value::String(entity_prefix));
        }
        // If the record doesn't exist, perform an insert
        process_and_insert_record(table_name, data, pluck_fields, auth).await
    } else {
        // If the record exists, perform an update
        process_and_update_record(table_name, data, &record_id, pluck_fields, "update", auth).await
    }
}
