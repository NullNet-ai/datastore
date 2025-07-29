use crate::models::crdt_message_model::CrdtMessageModel;
use crate::schema::verify::field_type_in_table;
use crate::structs::structs::ColumnValue;
use crate::table_enum::Table;
use base64::prelude::*;
use diesel_async::AsyncPgConnection;
use pluralizer::pluralize;
use serde_json::json;
use serde_json::{Map, Value};
use std::net::IpAddr;
use uuid::Uuid;

pub async fn apply(
    tx: &mut AsyncPgConnection,
    message: &CrdtMessageModel,
) -> Result<(), Box<dyn std::error::Error>> {
    let row = &message.row;
    let column = &message.column;
    let dataset = &message.dataset;
    let hypertable_timestamp = &message.hypertable_timestamp;

    let field_type_exists = field_type_in_table(&dataset, column);

    let field_type = match field_type_exists {
        Some(type_of_field) => type_of_field,
        None => {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                format!("Field '{}' doesn't exist in table '{}'", column, dataset),
            )))
        }
    };


    
    let value = convert_message_value_to_column_value(
        &message.value,
        &field_type.field_type,
        field_type.is_array,
        field_type.is_json,
        column,
    )?;
    // Handle hypertable timestamp
    let mut json_obj = serde_json::Map::new();
    let clean_id = row.trim_matches('"').to_string();
    json_obj.insert("id".to_string(), json!(clean_id));
    match value {
        ColumnValue::String(s) => {
            json_obj.insert(column.to_string(), json!(s));
        }
        ColumnValue::Array(arr) => {
            json_obj.insert(column.to_string(), json!(arr));
        }
        ColumnValue::Timestamp(dt) => {
            json_obj.insert(column.to_string(), json!(dt.naive_utc()));
        }
        ColumnValue::Integer(i) => {
            json_obj.insert(column.to_string(), json!(i));
        }
        ColumnValue::Float(f) => {
            json_obj.insert(column.to_string(), json!(f));
        }
        ColumnValue::Boolean(b) => {
            json_obj.insert(column.to_string(), json!(b));
        }
        ColumnValue::Json(json_val) => {
            // Preserve JSON structure instead of converting to string
            json_obj.insert(column.to_string(), json_val);
        }
        ColumnValue::Uuid(uuid_str) => {
            json_obj.insert(column.to_string(), json!(uuid_str));
        }
        ColumnValue::Binary(binary_data) => {
            // Convert binary to base64 for JSON representation
            let base64_str = base64::prelude::BASE64_STANDARD.encode(&binary_data);
            json_obj.insert(column.to_string(), json!(base64_str));
        }
        ColumnValue::Network(network_addr) => {
            json_obj.insert(column.to_string(), json!(network_addr));
        }
        ColumnValue::Numeric(numeric_str) => {
            // Try to parse as number for JSON, fallback to string
            if let Ok(num) = numeric_str.parse::<f64>() {
                json_obj.insert(column.to_string(), json!(num));
            } else {
                json_obj.insert(column.to_string(), json!(numeric_str));
            }
        }
        ColumnValue::None => {
            // Handle None case - insert null value or skip insertion
            json_obj.insert(column.to_string(), json!(null));
        }
    }

    json_obj = clean_extra_quotes(json_obj);
    let table = Table::from_str(dataset.as_str()).ok_or_else(|| {
        Box::new(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            format!("Unknown table: {}", dataset),
        ))
    })?;

    if let Some(ht_timestamp) = hypertable_timestamp {
        // Parse timestamp
        let timestamp_str = if ht_timestamp.contains('T')
            && !ht_timestamp.contains('Z')
            && !ht_timestamp.contains('+')
            && !ht_timestamp[10..].contains('-')
        {
            format!("{}+00:00", ht_timestamp)
        } else {
            ht_timestamp.to_string()
        };

        let timestamp = chrono::DateTime::parse_from_rfc3339(&timestamp_str).map_err(|e| {
            log::error!("Failed to parse timestamp '{}': {}", timestamp_str, e);
            Box::new(e) as Box<dyn std::error::Error>
        })?;
        json_obj.insert("timestamp".to_string(), json!(timestamp.naive_utc()));

        let json_values = serde_json::Value::Object(json_obj);

        match table.upsert_record_with_id_timestamp(tx, json_values).await {
            Ok(_) => return Ok(()),
            Err(e) => {
                print!("Error applying message: {}", e);
                Err(Box::new(e))
            }
        }
    } else {
        // Insert or update without hypertable timestamp
        let json_values = serde_json::Value::Object(json_obj);
        match table.upsert_record_with_id(tx, json_values).await {
            Ok(_) => return Ok(()),
            Err(e) => {
                log::error!("Error applying message: {}", e);
                Err(Box::new(e))
                // return error
            }
        }
    }
}

/// Convert message value to ColumnValue using centralized type conversion logic
/// This maintains the same behavior as the original logic while leveraging DatabaseTypeConverter
fn convert_message_value_to_column_value(
    value: &str,
    field_type: &str,
    is_array: bool,
    is_json: bool,
    column: &str,
) -> Result<ColumnValue, Box<dyn std::error::Error>> {

    // Handle empty/null values
    if value.trim().is_empty()
        || value.trim() == "{}"
        || value.trim() == "[]"
        || value.trim() == "\"\""
    {
        return Ok(ColumnValue::None);
    }

    // Handle array types
    if is_array || is_plural_column(column) {
        return Ok(ColumnValue::Array(process_pg_array(value)?));
    }

    // Handle timestamp types
    if field_type == "timestamp" || column == "timestamp" {
        let timestamp = value.trim_matches('"').to_string();
        let timestamp_str = if timestamp.contains('T')
            && !timestamp.contains('Z')
            && !timestamp.contains('+')
            && !timestamp[10..].contains('-')
        {
            format!("{}+00:00", timestamp)
        } else {
            timestamp.to_string()
        };

        let parsed_timestamp = chrono::DateTime::parse_from_rfc3339(&timestamp_str).map_err(|e| {
            log::error!("Failed to parse timestamp '{}': {}", timestamp_str, e);
            Box::new(e) as Box<dyn std::error::Error>
        })?;
        return Ok(ColumnValue::Timestamp(parsed_timestamp));
    }

    // Convert based on field type with enhanced PostgreSQL type support
    match field_type {
        "integer" | "int4" | "int8" | "int2" => {
            if let Ok(int_value) = value.parse::<i32>() {
                Ok(ColumnValue::Integer(int_value))
            } else {
                log::warn!("Failed to parse '{}' as integer, using as string", value);
                Ok(ColumnValue::String(value.to_string()))
            }
        }
        "float" | "float4" | "float8" | "real" | "double" => {
            if let Ok(float_value) = value.parse::<f64>() {
                Ok(ColumnValue::Float(float_value))
            } else {
                log::warn!("Failed to parse '{}' as float, using as string", value);
                Ok(ColumnValue::String(value.to_string()))
            }
        }
        "bool" | "boolean" => {
            if let Ok(bool_value) = value.to_lowercase().parse::<bool>() {
                Ok(ColumnValue::Boolean(bool_value))
            } else {
                log::warn!("Failed to parse '{}' as boolean, using as string", value);
                Ok(ColumnValue::String(value.to_string()))
            }
        }
        "json" | "jsonb" => {
            // Parse and preserve JSON structure
            match serde_json::from_str::<serde_json::Value>(value) {
                Ok(json_val) => Ok(ColumnValue::Json(json_val)),
                Err(_) => {
                    log::warn!("Invalid JSON value: {}, using as string", value);
                    Ok(ColumnValue::String(value.to_string()))
                }
            }
        }
        "uuid" => {
            // Strip quotes if present and validate UUID format
            let clean_value = value.trim_matches('"');
            if Uuid::parse_str(clean_value).is_ok() {
                Ok(ColumnValue::Uuid(clean_value.to_string()))
            } else {
                log::warn!("Invalid UUID format: {}, using as string", value);
                Ok(ColumnValue::String(value.to_string()))
            }
        }
        "inet" | "cidr" => {
            // Strip quotes if present and validate network address format
            let clean_value = value.trim_matches('"');
            if clean_value.parse::<IpAddr>().is_ok() || clean_value.contains('/') {
                Ok(ColumnValue::Network(clean_value.to_string()))
            } else {
                log::warn!("Invalid network address: {}, using as string", value);
                Ok(ColumnValue::String(value.to_string()))
            }
        }
        "numeric" | "decimal" => {
            // Keep numeric as string to preserve precision
            Ok(ColumnValue::Numeric(value.to_string()))
        }
        "bytea" => {
            // Handle binary data (base64 encoded)
            match base64::prelude::BASE64_STANDARD.decode(value) {
                Ok(binary_data) => Ok(ColumnValue::Binary(binary_data)),
                Err(_) => {
                    log::warn!("Invalid base64 binary data: {}, using as string", value);
                    Ok(ColumnValue::String(value.to_string()))
                }
            }
        }
        _ => {
            // Handle legacy JSON fields and fallback
            if is_json {
                match serde_json::from_str::<serde_json::Value>(value) {
                    Ok(json_val) => Ok(ColumnValue::Json(json_val)),
                    Err(_) => {
                        log::warn!("Invalid JSON value: {}, using as string", value);
                        Ok(ColumnValue::String(value.to_string()))
                    }
                }
            } else {
                // Default case - try to parse as number, fallback to string
                if let Ok(int_value) = value.parse::<i32>() {
                    Ok(ColumnValue::Integer(int_value))
                } else if let Ok(float_value) = value.parse::<f64>() {
                    Ok(ColumnValue::Float(float_value))
                } else {
                    Ok(ColumnValue::String(value.to_string()))
                }
            }
        }
    }
}

pub fn clean_extra_quotes(mut map: Map<String, Value>) -> Map<String, Value> {
    for (_key, value) in map.iter_mut() {
        if let Value::String(s) = value {
            // Check if string is wrapped in quotes
            if s.starts_with('"') && s.ends_with('"') {
                // Strip outer quotes
                *s = s.trim_matches('"').to_string();
            }
        }
    }
    map
}

fn is_plural_column(column: &str) -> bool {
    pluralize(column, 2, false) == column
}

fn process_pg_array(value: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    if value.is_empty() {
        return Ok(Vec::new());
    }

    // Try to parse as JSON array first
    if let Ok(json_array) = serde_json::from_str::<Vec<String>>(value) {
        return Ok(json_array);
    }

    // If it's a PostgreSQL array format
    if value.starts_with('{') && value.ends_with('}') {
        let processed: Vec<String> = value
            .trim_matches(|c| c == '{' || c == '}')
            .split(',')
            .map(|s| {
                s.trim()
                    .trim_matches('"') // Remove any quotes
                    .to_string()
            })
            .collect();
        return Ok(processed);
    }

    // If it's a single value, try to parse as number first
    if let Ok(_) = value.parse::<i32>() {
        return Ok(vec![value.to_string()]);
    }

    // Otherwise treat as a single string value
    Ok(vec![value.to_string()])
}
