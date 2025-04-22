use crate::db::DbPooledConnection;
use crate::models::crdt_message_model::CrdtMessage;
use crate::schema::schema::crdt_messages;
use crate::sync::hlc::hlc_service;
use diesel::prelude::*;
use diesel::result::Error as DieselError;
use diesel::upsert::excluded;
use serde_json::Value;

pub fn create_messages(
    mut tx: &mut DbPooledConnection,
    record: &Value,
    dataset: &String,
    operation: String,
) -> Result<Vec<CrdtMessage>, DieselError> {
    let object = record.as_object().expect("Expected a JSON object");

    let row = object
        .get("id")
        .ok_or_else(|| DieselError::NotFound)
        .map_err(|_| {
            DieselError::QueryBuilderError(
                "Record does not have an id, make sure record has an id".into(),
            )
        })?
        .to_string();

    let hypertable_timestamp = object
        .get("hypertable_timestamp")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());

    let messages: Vec<CrdtMessage> = object
        .iter()
        .filter(|(key, value)| *key != "id" && !value.is_null()) // Skip the `id` field itself as it's used for `row`
        .map(|(key, value)| CrdtMessage {
            database: None,
            dataset: dataset.to_string(),
            group_id: "".to_string(),
            timestamp: hlc_service::HlcService::send(&mut tx).unwrap(),
            row: row.clone(),
            column: key.clone(),
            client_id: "client_id_placeholder".to_string(),
            value: value.to_string(),
            operation: operation.clone(),
            hypertable_timestamp: hypertable_timestamp.clone(),
        })
        .collect();
    Ok::<Vec<CrdtMessage>, DieselError>(messages)
}

pub fn insert_message(
    tx: &mut DbPooledConnection,
    mut message: CrdtMessage, // Changed to mutable
) -> Result<usize, DieselError> {
    // Clean fields once upfront
    message.row = message.row.trim_matches('"').to_string();
    message.value = message.value.trim_matches('"').to_string();

    diesel::insert_into(crdt_messages::table)
        .values(&message)
        .on_conflict((
            crdt_messages::timestamp,
            crdt_messages::group_id,
            crdt_messages::row,
            crdt_messages::column,
        ))
        .do_update()
        .set((
            crdt_messages::database.eq(excluded(crdt_messages::database)),
            crdt_messages::dataset.eq(excluded(crdt_messages::dataset)),
            crdt_messages::client_id.eq(excluded(crdt_messages::client_id)),
            crdt_messages::value.eq(excluded(crdt_messages::value)),
            crdt_messages::operation.eq(excluded(crdt_messages::operation)),
            crdt_messages::hypertable_timestamp.eq(excluded(crdt_messages::hypertable_timestamp)),
        ))
        .execute(tx)
}

pub fn compare_messages(
    tx: &mut DbPooledConnection,
    messages: Vec<CrdtMessage>,
) -> Result<Vec<(CrdtMessage, Option<CrdtMessage>)>, DieselError> {
    let mut result = Vec::new();

    // Use the iterator to process each message pair
    for result_item in find_existing_messages(tx, &messages) {
        let (msg, existing_msg) = result_item?;

        // Clone the message to own it, and pair it with its existing counterpart
        let owned_msg = CrdtMessage {
            database: msg.database.clone(),
            dataset: msg.dataset.clone(),
            group_id: msg.group_id.clone(),
            timestamp: msg.timestamp.clone(),
            row: msg.row.clone(),
            column: msg.column.clone(),
            client_id: msg.client_id.clone(),
            value: msg.value.clone(),
            operation: msg.operation.clone(),
            hypertable_timestamp: msg.hypertable_timestamp.clone(),
        };

        // Add the pair to the result vector
        result.push((owned_msg, existing_msg));
    }

    Ok(result)
}
pub fn find_existing_messages<'a>(
    tx: &'a mut DbPooledConnection,
    messages: &'a Vec<CrdtMessage>,
) -> impl Iterator<Item = Result<(&'a CrdtMessage, Option<CrdtMessage>), DieselError>> + 'a {
    messages.iter().map(move |message| {
        // Find the most recent existing message with the same dataset, column, and row
        let existing_message = crdt_messages::table
            .filter(crdt_messages::dataset.eq(&message.dataset))
            .filter(crdt_messages::column.eq(&message.column))
            .filter(crdt_messages::row.eq(&message.row))
            .order(crdt_messages::timestamp.desc())
            .limit(1)
            .first::<CrdtMessage>(tx)
            .optional()?;

        Ok((message, existing_message))
    })
}

pub fn get_messages_since(
    conn: &mut DbPooledConnection,
    timestamp_str: &str,
) -> Result<Vec<Value>, Box<dyn std::error::Error>> {
    use crate::schema::schema::crdt_messages;

    let results = crdt_messages::table
        .filter(crdt_messages::timestamp.gt(timestamp_str))
        .load::<CrdtMessage>(conn)?;

    // Convert CrdtMessage objects to Value objects
    let message_values: Vec<Value> = results
        .into_iter()
        .map(|msg| serde_json::to_value(msg).unwrap_or(Value::Null))
        .collect();

    Ok(message_values)
}
