use crate::models::crdt_message_model::CrdtMessage;
use crate::structs::structs::ColumnValue;
use crate::table_enum::Table;
use diesel_async::AsyncPgConnection;
use serde_json::json;
use serde_json::{Map, Value};

pub async fn apply(
    tx: &mut AsyncPgConnection,
    message: &CrdtMessage,
) -> Result<(), Box<dyn std::error::Error>> {
    let row = &message.row;
    let column = &message.column;
    let dataset = &message.dataset;
    let hypertable_timestamp = &message.hypertable_timestamp;

    let value = if is_plural_column(column) {
        ColumnValue::Array(process_pg_array(&message.value)?)
    } else if column == "timestamp" {
        ColumnValue::Timestamp(
            chrono::DateTime::parse_from_rfc3339(&message.value)
                .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?,
        )
    } else {
            // Try to parse as integer first
            if let Ok(int_value) = message.value.parse::<i32>() {
                ColumnValue::Integer(int_value)
            } else {
                ColumnValue::String(message.value.clone())
            }
 };
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
        let timestamp = chrono::DateTime::parse_from_rfc3339(ht_timestamp)
        .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;
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
    column.ends_with('s')
}

fn process_pg_array(value: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    if value.is_empty() {
        return Ok(Vec::new()); // Return an empty array for empty string
    }

    // Validate that we have an array-like structure
    if !value.starts_with('{') || !value.ends_with('}') {
        // If the value doesn't look like a PG array, throw an error
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "Expected an array after processing",
        )));
    }

    // Parse PostgreSQL array string to a Rust vector
    // Remove the curly braces and split by commas
    let processed: Vec<String> = value
        .trim_matches(|c| c == '{' || c == '}')
        .split(',')
        .map(|s| s.trim().to_string())
        .collect();

    Ok(processed)
}
