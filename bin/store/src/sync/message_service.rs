use crate::models::crdt_message_model::InsertCrdtMessage;
use serde_json::Value;

pub fn generate_messages_from_value(
    record: &Value,
    dataset: &String,
    operation: String,
) -> Vec<InsertCrdtMessage> {
    let object = record.as_object().expect("Expected a JSON object");

    let row = object.get("id") // Ensure it's a string
        .expect("Expected an `id` field of type string")
        .to_string();

    object.iter()
        .filter(|(key, value)| *key != "id" && !value.is_null()) // Skip the `id` field itself as it's used for `row`
        .map(|(key, value)| {
            InsertCrdtMessage {
                database: None,
                dataset: dataset.to_string(),
                group_id: "".to_string(),
                timestamp: "2025-03-11T00:00:00Z".to_string(),
                row: row.clone(),
                column: key.clone(),
                client_id: "client_id_placeholder".to_string(),
                value: value.to_string(),
                operation: operation.clone(),
                hypertable_timestamp: None,
            }
        })
        .collect()
}

