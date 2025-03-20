use crate::db::DbPooledConnection;
use crate::models::crdt_message_model::InsertCrdtMessage;
use crate::schema::schema;
use crate::structs::structs::{ColumnValue, Id};
use diesel::Column;
use crate::schema::schema::*;
use diesel::Table;

pub async fn apply(tx: &mut DbPooledConnection, message: &InsertCrdtMessage) -> Result<(), Box<dyn std::error::Error>> {
    let row = &message.row;
    let column = &message.column;
    let dataset = &message.dataset;
    let hypertable_timestamp = &message.hypertable_timestamp;
    let operation = &message.operation;

    let value = if is_plural_column(column) {
        ColumnValue::Array(process_pg_array(&message.value)?)
    } else if column == "timestamp" {
        ColumnValue::Timestamp(
            chrono::DateTime::parse_from_rfc3339(&message.value)
                .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?
        )
    } else {
        ColumnValue::String(message.value.clone())
    };
    // Handle hypertable timestamp
    if let Some(ht_timestamp) = hypertable_timestamp {
        // Parse timestamp
        let timestamp = chrono::DateTime::parse_from_rfc3339(ht_timestamp)
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;
       

        // Insert or update with hypertable timestamp
        match insert_with_hypertable_timestamp(tx, dataset, row, column, &value, &timestamp) {
            Ok(_) => return Ok(()),
            Err(e) => {
                print!("Error applying message: {}", e);
                return Ok(());
            }
        }
    } else {
        // Insert or update without hypertable timestamp
        match insert_without_hypertable_timestamp(tx, dataset, row, column, &value) {
            Ok(_) => return Ok(()),
            Err(e) => {
                print!("Error applying message: {}", e);
                return Ok(());
            }
        }
    }

    Ok(())
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

fn insert_with_hypertable_timestamp(
    tx: &mut DbPooledConnection,
    dataset: &str,
    row: &str,
    column: &str,
    value: &ColumnValue,
    timestamp: &chrono::DateTime<chrono::FixedOffset>,
) -> Result<(), diesel::result::Error> {
    // Get the table from the dataset
    let table = get_table_by_dataset(dataset);

    // Create a dynamic values map
    let mut values = std::collections::HashMap::new();
    values.insert("id".to_string(), serde_json::Value::String(row.to_string()));
    values.insert("timestamp".to_string(), serde_json::Value::String(timestamp.to_string()));
    values.insert(column.to_string(), value.to_json_value());

    // Convert to JSON for insertion
    let json_values = serde_json::to_value(values).unwrap();

    // Use diesel's json insert capabilities
    diesel::insert_into(table)
        .values(json_values)
        .on_conflict((get_id_column(dataset), get_timestamp_column(dataset)))
        .do_update()
        .set(json_values)
        .execute(tx)
        .map(|_| ())
}

fn insert_without_hypertable_timestamp(
    tx: &mut DbPooledConnection,
    dataset: &str,
    row: &str,
    column: &str,
    value: &ColumnValue,
) -> Result<(), diesel::result::Error> {
    // Get the table from the dataset
    let table = get_table_by_dataset(dataset);

    // Create a dynamic values map
    let mut values = std::collections::HashMap::new();
    values.insert("id".to_string(), serde_json::Value::String(row.to_string()));
    values.insert(column.to_string(), value.to_json_value());

    // Convert to JSON for insertion
    let json_values = serde_json::to_value(values).unwrap();

    // Use diesel's json insert capabilities
    diesel::insert_into(table)
        .values(json_values)
        .on_conflict(get_id_column(dataset))
        .do_update()
        .set(json_values)
        .execute(tx)
        .map(|_| ())
}

fn get_id_column(dataset: &str) -> Id {

    match dataset {
        "packets" => Id::Uuid(uuid::Uuid::new_v4()), // Example for UUID-based ID
        _ => panic!("Unknown dataset: {}", dataset),
    }
}

fn get_timestamp_column(dataset: &str) -> impl Column {
    match dataset {
        "packets" => schema::packets::timestamp,
        // Add more tables as needed
        _ => panic!("Unknown dataset: {}", dataset),
    }
}
