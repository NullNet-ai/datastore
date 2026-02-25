use crate::generated::models::crdt_message_model::CrdtMessageModel;
use crate::generated::schema::crdt_messages;
use crate::providers::operations::sync::hlc::hlc_service;
use diesel::prelude::*;
use diesel::result::Error as DieselError;
use diesel::upsert::excluded;
use diesel_async::AsyncPgConnection;
use diesel_async::RunQueryDsl;
use serde_json::Value;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};

pub async fn create_messages(
    mut tx: &mut AsyncPgConnection,
    record: &Value,
    dataset: &String,
    operation: String,
) -> Result<Vec<CrdtMessageModel>, DieselError> {
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

    let mut messages: Vec<CrdtMessageModel> = Vec::new();

    //check if the record is coming from the sync by checking is_batch field
    let is_batch = object
        .get("is_batch")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);

    if operation == "Update" || is_batch {
        let timestamp = hlc_service::HlcService::send(&mut tx).await.map_err(|e| {
            log::error!("Failed to generate HLC timestamp: {:?}", e);
            DieselError::DatabaseError(
                diesel::result::DatabaseErrorKind::Unknown,
                Box::new(format!("HLC timestamp generation failed: {}", e)),
            )
        })?;

        messages.push(CrdtMessageModel {
            database: None,
            dataset: dataset.to_string(),
            group_id: "".to_string(),
            timestamp,
            row: row.clone(),
            column: "sync_status".to_string(),
            client_id: "client_id_placeholder".to_string(),
            value: "consumed".to_string(),
            operation: operation.clone(),
            hypertable_timestamp: hypertable_timestamp.clone(),
        });
    }

    for (key, value) in object.iter() {
        if *key == "id" || value.is_null() || *key == "sync_status" {
            continue;
        }

        let timestamp = hlc_service::HlcService::send(&mut tx).await.map_err(|e| {
            log::error!("Failed to generate HLC timestamp: {:?}", e);
            DieselError::DatabaseError(
                diesel::result::DatabaseErrorKind::Unknown,
                Box::new(format!("HLC timestamp generation failed: {}", e)),
            )
        })?;

        messages.push(CrdtMessageModel {
            database: None,
            dataset: dataset.to_string(),
            group_id: "".to_string(),
            timestamp,
            row: row.clone(),
            column: key.clone(),
            client_id: "client_id_placeholder".to_string(),
            value: value.to_string(),
            operation: operation.clone(),
            hypertable_timestamp: hypertable_timestamp.clone(),
        });
    }

    if operation == "Insert" && !is_batch {
        let timestamp = hlc_service::HlcService::send(&mut tx).await.map_err(|e| {
            log::error!("Failed to generate HLC timestamp: {:?}", e);
            DieselError::DatabaseError(
                diesel::result::DatabaseErrorKind::Unknown,
                Box::new(format!("HLC timestamp generation failed: {}", e)),
            )
        })?;

        messages.push(CrdtMessageModel {
            database: None,
            dataset: dataset.to_string(),
            group_id: "".to_string(),
            timestamp,
            row: row.clone(),
            column: "sync_status".to_string(),
            client_id: "client_id_placeholder".to_string(),
            value: "complete".to_string(),
            operation: operation.clone(),
            hypertable_timestamp: hypertable_timestamp.clone(),
        });
    }

    Ok(messages)
}

#[allow(dead_code)]
pub async fn insert_message(
    tx: &mut AsyncPgConnection,
    mut message: CrdtMessageModel, // Changed to mutable
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
        .await
}

/// Wrapper so we can deduplicate by ON CONFLICT key (timestamp, group_id, row, column) using a set.
#[derive(Clone)]
struct ByConflictKey(CrdtMessageModel);

impl Hash for ByConflictKey {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.timestamp.hash(state);
        self.0.group_id.hash(state);
        self.0.row.hash(state);
        self.0.column.hash(state);
    }
}

impl PartialEq for ByConflictKey {
    fn eq(&self, other: &Self) -> bool {
        self.0.timestamp == other.0.timestamp
            && self.0.group_id == other.0.group_id
            && self.0.row == other.0.row
            && self.0.column == other.0.column
    }
}

impl Eq for ByConflictKey {}

/// Insert multiple messages in one statement. Messages are cleaned (row/value trim) before insert.
/// Batches are deduplicated by (timestamp, group_id, row, column) via a set so that ON CONFLICT
/// DO UPDATE never affects the same row twice (PostgreSQL error otherwise).
pub async fn insert_messages_batch(
    tx: &mut AsyncPgConnection,
    messages: &[CrdtMessageModel],
) -> Result<usize, DieselError> {
    if messages.is_empty() {
        return Ok(0);
    }
    let deduped: Vec<CrdtMessageModel> = messages
        .iter()
        .map(|m| ByConflictKey(m.clone()))
        .collect::<HashSet<_>>()
        .into_iter()
        .map(|k| k.0)
        .collect();

    let cleaned: Vec<CrdtMessageModel> = deduped
        .iter()
        .map(|m| CrdtMessageModel {
            row: m.row.trim_matches('"').to_string(),
            value: m.value.trim_matches('"').to_string(),
            ..m.clone()
        })
        .collect();

    diesel::insert_into(crdt_messages::table)
        .values(&cleaned)
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
        .await
}

pub async fn compare_messages(
    tx: &mut AsyncPgConnection,
    messages: Vec<CrdtMessageModel>,
) -> Result<Vec<(CrdtMessageModel, Option<CrdtMessageModel>)>, DieselError> {
    let mut result = Vec::new();

    // Use the iterator to process each message pair
    for result_item in find_existing_messages(tx, &messages).await {
        let (msg, existing_msg) = result_item?;

        // Clone the message to own it, and pair it with its existing counterpart
        let owned_msg = CrdtMessageModel {
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
pub async fn find_existing_messages<'a>(
    tx: &'a mut AsyncPgConnection,
    messages: &'a Vec<CrdtMessageModel>,
) -> Vec<Result<(&'a CrdtMessageModel, Option<CrdtMessageModel>), DieselError>> {
    let mut results = Vec::new();

    for message in messages.iter() {
        // Find the most recent existing message with the same dataset, column, and row
        let existing_message_result = crdt_messages::table
            .filter(crdt_messages::dataset.eq(&message.dataset))
            .filter(crdt_messages::column.eq(&message.column))
            .filter(crdt_messages::row.eq(&message.row))
            .order(crdt_messages::timestamp.desc())
            .limit(1)
            .first::<CrdtMessageModel>(tx)
            .await;

        let result = match existing_message_result {
            Ok(msg) => Ok((message, Some(msg))),
            Err(DieselError::NotFound) => Ok((message, None)),
            Err(e) => Err(e),
        };

        results.push(result);
    }

    results
}

pub async fn get_messages_since(
    conn: &mut AsyncPgConnection,
    timestamp_str: &str,
) -> Result<Vec<Value>, Box<dyn std::error::Error>> {
    use crate::generated::schema::crdt_messages;

    let results = crdt_messages::table
        .filter(crdt_messages::timestamp.gt(timestamp_str))
        .order(crdt_messages::timestamp.asc())
        .load::<CrdtMessageModel>(conn)
        .await?;

    // Convert CrdtMessage objects to Value objects
    let message_values: Vec<Value> = results
        .into_iter()
        .map(|msg| serde_json::to_value(msg).unwrap_or(Value::Null))
        .collect();

    Ok(message_values)
}
