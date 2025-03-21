use crate::db::DbPooledConnection;
use crate::models::crdt_message_model::{GetCrdtMessage, InsertCrdtMessage};
use crate::schema::schema::crdt_messages;
use crate::sync::hlc::hlc_service;
use diesel::prelude::*;

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
            hypertable_timestamp: hypertable_timestamp.clone(),
        })
        .collect();
    Ok::<Vec<InsertCrdtMessage>, DieselError>(messages)
}

pub fn insert_message(
    tx: &mut DbPooledConnection,
    message: InsertCrdtMessage,
) -> Result<usize, DieselError> {
    let existing = crdt_messages::table
        .filter(crdt_messages::timestamp.eq(&message.timestamp))
        .filter(crdt_messages::group_id.eq(&message.group_id))
        .filter(crdt_messages::row.eq(&message.row))
        .filter(crdt_messages::column.eq(&message.column))
        .first::<GetCrdtMessage>(tx)
        .optional()?;

    match existing {
        Some(_) => diesel::update(crdt_messages::table)
            .filter(crdt_messages::timestamp.eq(&message.timestamp))
            .filter(crdt_messages::group_id.eq(&message.group_id))
            .filter(crdt_messages::row.eq(&message.row))
            .filter(crdt_messages::column.eq(&message.column))
            .set((
                crdt_messages::database.eq(message.database),
                crdt_messages::dataset.eq(message.dataset),
                crdt_messages::client_id.eq(message.client_id),
                crdt_messages::value.eq(message.value),
                crdt_messages::operation.eq(message.operation),
                crdt_messages::hypertable_timestamp.eq(message.hypertable_timestamp),
            ))
            .execute(tx),
        // If it doesn't exist, insert it
        None => diesel::insert_into(crdt_messages::table)
            .values(message)
            .execute(tx),
    }
}

pub fn compare_messages(
    tx: &mut DbPooledConnection,
    messages: Vec<InsertCrdtMessage>,
) -> Result<Vec<(InsertCrdtMessage, Option<GetCrdtMessage>)>, DieselError> {
    let mut result = Vec::new();

    // Use the iterator to process each message pair
    for result_item in find_existing_messages(tx, &messages) {
        let (msg, existing_msg) = result_item?;

        // Clone the message to own it, and pair it with its existing counterpart
        let owned_msg = InsertCrdtMessage {
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
    messages: &'a Vec<InsertCrdtMessage>,
) -> impl Iterator<Item = Result<(&'a InsertCrdtMessage, Option<GetCrdtMessage>), DieselError>> + 'a
{
    messages.iter().map(move |message| {
        // Find the most recent existing message with the same dataset, column, and row
        let existing_message = crdt_messages::table
            .filter(crdt_messages::dataset.eq(&message.dataset))
            .filter(crdt_messages::column.eq(&message.column))
            .filter(crdt_messages::row.eq(&message.row))
            .order(crdt_messages::timestamp.desc())
            .limit(1)
            .first::<GetCrdtMessage>(tx)
            .optional()?;

        Ok((message, existing_message))
    })
}
