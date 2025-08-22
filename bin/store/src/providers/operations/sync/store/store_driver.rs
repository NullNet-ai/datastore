use crate::database::schema::verify::field_type_in_table;
use crate::generated::models::crdt_message_model::CrdtMessageModel;
use crate::generated::table_enum::Table;

use diesel_async::AsyncPgConnection;
use pluralizer::pluralize;
use serde_json::json;
use serde_json::{Map, Value};

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

    // Convert message value directly to JSON
    let json_value = parse_message_value_to_json(
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
    json_obj.insert(column.to_string(), json_value);

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
        } else if ht_timestamp.contains(' ') && !ht_timestamp.contains('T') {
            // Handle space-separated timestamps like "2025-08-20 21:44:41.082307"
            // Convert to RFC3339 format with T separator and UTC timezone
            let with_t = ht_timestamp.replace(' ', "T");
            format!("{}+00:00", with_t)
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

/// Bridge function to convert JSON value to ColumnValue using DatabaseTypeConverter
fn parse_message_value_to_json(
    value: &str,
    field_type: &str,
    is_array: bool,
    is_json: bool,
    column: &str,
) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    // Handle empty/null values
    if value.trim().is_empty()
        || value.trim() == "{}"
        || value.trim() == "[]"
        || value.trim() == "\"\""
    {
        return Ok(serde_json::Value::Null);
    }

    // Parse string value to JSON first for consistent handling
    let json_value = if is_array || is_plural_column(column) {
        // Handle array parsing
        if value.starts_with('[') && value.ends_with(']') {
            // Try to parse as JSON array
            match serde_json::from_str::<serde_json::Value>(value) {
                Ok(parsed) => parsed,
                Err(_) => {
                    // Fallback to PostgreSQL array format
                    let processed_array = process_pg_array(value)?;
                    serde_json::Value::Array(
                        processed_array
                            .into_iter()
                            .map(serde_json::Value::String)
                            .collect(),
                    )
                }
            }
        } else if value.starts_with('{') && value.ends_with('}') {
            // PostgreSQL array format
            let processed_array = process_pg_array(value)?;
            serde_json::Value::Array(
                processed_array
                    .into_iter()
                    .map(serde_json::Value::String)
                    .collect(),
            )
        } else {
            // Reject single values for array fields - they must be in proper array format
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                format!("Field '{}' expects an array format (e.g., [\"value1\", \"value2\"]) but received a simple string: '{}'", column, value),
            )));
        }
    } else {
        // For text fields, especially long ones like PostgreSQL functions,
        // treat as string directly to avoid JSON parsing issues
        if field_type == "text" || field_type == "varchar" {
            serde_json::Value::String(value.to_string())
        } else {
            // Try to parse as JSON first, fallback to string
            match serde_json::from_str::<serde_json::Value>(value) {
                Ok(parsed) => parsed,
                Err(_) => {
                    // If it's a quoted string, try to unquote it
                    if value.starts_with('"') && value.ends_with('"') && value.len() > 1 {
                        let unquoted = &value[1..value.len() - 1];
                        // Try parsing the unquoted value
                        match serde_json::from_str::<serde_json::Value>(unquoted) {
                            Ok(parsed) => parsed,
                            Err(_) => serde_json::Value::String(unquoted.to_string()),
                        }
                    } else {
                        serde_json::Value::String(value.to_string())
                    }
                }
            }
        }
    };

    // Handle null values
    if json_value.is_null() {
        return Ok(serde_json::Value::Null);
    }

    // Handle array types
    if is_array || is_plural_column(column) {
        if let serde_json::Value::Array(arr) = json_value {
            return Ok(serde_json::Value::Array(arr));
        } else {
            // If field is marked as array but value is not an array, reject it
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                format!("Field '{}' is an array field but received non-array value. Expected format: [\"value1\", \"value2\"]", column),
            )));
        }
    }

    // Handle timestamp types with special formatting
    if field_type == "timestamp" || field_type == "timestamptz" || column == "timestamp" {
        if let serde_json::Value::String(timestamp_str) = json_value {
            // Parse timestamp using the same logic as hypertable_timestamp
            let formatted_timestamp = if timestamp_str.contains('T')
                && !timestamp_str.contains('Z')
                && !timestamp_str.contains('+')
                && !timestamp_str[10..].contains('-')
            {
                format!("{}+00:00", timestamp_str)
            } else {
                timestamp_str.to_string()
            };

            let parsed_timestamp = chrono::DateTime::parse_from_rfc3339(&formatted_timestamp)
                .map_err(|e| {
                    log::error!("Failed to parse timestamp '{}': {}", formatted_timestamp, e);
                    Box::new(e) as Box<dyn std::error::Error>
                })?;
            // Use naive_utc() to match hypertable_timestamp handling
            return Ok(json!(parsed_timestamp.naive_utc()));
        }
    }

    // Handle JSON/JSONB types - preserve structure
    if is_json || field_type == "json" || field_type == "jsonb" {
        return Ok(json_value);
    }

    // For all other types, return the JSON value as-is
    Ok(json_value)
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
