use crate::models::crdt_message_model::CrdtMessage;
use crate::structs::structs::ColumnValue;
use crate::table_enum::Table;
use diesel_async::AsyncPgConnection;
use pluralizer::pluralize;
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

    let value = if message.value.trim().is_empty() || message.value.trim() == "{}" || message.value.trim() == "[]" || message.value.trim() == "\"\"" {
        ColumnValue::None
    }else if is_plural_column(column) {
        ColumnValue::Array(process_pg_array(&message.value)?)
    } else if column == "timestamp" {
        // Parse timestamp
        let timestamp = message.value.trim_matches('"').to_string();
        let timestamp_str = if timestamp.contains('T')
            && !timestamp.contains('Z')
            && !timestamp.contains('+')
            && !timestamp[10..].contains('-')
        {
            format!("{}+00:00", timestamp)
        } else {
            timestamp.to_string()
        };

        ColumnValue::Timestamp(
            chrono::DateTime::parse_from_rfc3339(&timestamp_str).map_err(|e| {
                log::error!("Failed to parse timestamp '{}': {}", timestamp_str, e);
                Box::new(e) as Box<dyn std::error::Error>
            })?,
        )
    } else {
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
