use crate::db::DbPooledConnection;
use crate::models::crdt_message_model::InsertCrdtMessage;
use crate::sync::hlc::hlc_service;
use diesel::result::Error as DieselError;
use serde_json::Value;

pub fn create_messages(
    mut tx: &mut DbPooledConnection,
    record: &Value,
    dataset: &String,
    operation: String,
) -> Result<Vec<InsertCrdtMessage>, DieselError> {
    let object = record.as_object().expect("Expected a JSON object");

    let row = object
        .get("id") // Ensure it's a string
        .expect("Expected an `id` field of type string")
        .to_string();

    let messages: Vec<InsertCrdtMessage> = object
        .iter()
        .filter(|(key, value)| *key != "id" && !value.is_null()) // Skip the `id` field itself as it's used for `row`
        .map(|(key, value)| InsertCrdtMessage {
            database: None,
            dataset: dataset.to_string(),
            group_id: "".to_string(),
            timestamp: hlc_service::HlcService::send(&mut tx).unwrap(),
            row: row.clone(),
            column: key.clone(),
            client_id: "client_id_placeholder".to_string(),
            value: value.to_string(),
            operation: operation.clone(),
            hypertable_timestamp: None,
        })
        .collect();
    Ok::<Vec<InsertCrdtMessage>, DieselError>(messages)
}
