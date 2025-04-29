use crate::models::crdt_message_model::CrdtMessage;
use crate::structs::structs::ColumnValue;
use crate::table_enum::Table;
use diesel_async::AsyncPgConnection;

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
        ColumnValue::String(message.value.clone())
    };
    // Handle hypertable timestamp
    let mut values = std::collections::HashMap::new();
    values.insert("id".to_string(), serde_json::Value::String(row.to_string()));
    values.insert(column.to_string(), value.to_json_value());
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
        values.insert(
            "timestamp".to_string(),
            serde_json::Value::String(timestamp.to_string()),
        );

        match table.upsert_record_with_id_timestamp(tx, &values).await {
            Ok(_) => return Ok(()),
            Err(e) => {
                print!("Error applying message: {}", e);
                return Ok(());
            }
        }
    } else {
        // Insert or update without hypertable timestamp
        match table.upsert_record_with_id(tx, &values).await {
            Ok(_) => return Ok(()),
            Err(e) => {
                print!("Error applying message: {}", e);
                return Ok(());
            }
        }
    }
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
