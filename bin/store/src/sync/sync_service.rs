use crate::db;
use crate::db::DbPooledConnection;
use crate::models::crdt_message_model::InsertCrdtMessage;
use crate::structs::structs::Clock;
use crate::sync::message_service::{compare_messages, create_messages};
use diesel::result::Error as DieselError;
use diesel::Connection;
use serde_json::Value;
use crate::sync::message_service;
use crate::sync::hlc::hlc_service::HlcService;


pub async fn insert(table: &String, row: Value) -> Result<(), DieselError> {
    let operation = "Insert".to_string();
    let mut conn = db::get_connection();
    conn.transaction(|mut tx| {
        let messages: Vec<InsertCrdtMessage> = create_messages(&mut tx, &row, table, operation)?;

        Ok::<(), DieselError>(())
    })?;

    Ok(())
}


pub async fn send_messages(mut tx: &mut DbPooledConnection, messages: Vec<InsertCrdtMessage>) -> Result<(), Box<dyn std::error::Error>> {
    self::apply_messages(&mut tx, messages).await?;
    Ok(())
}

async fn apply_messages(mut tx: &mut DbPooledConnection, messages: Vec<InsertCrdtMessage>) -> Result<(), Box<dyn std::error::Error>> {
    //use messageService.compareMessages here
    let existing_messages = compare_messages(&mut tx, messages.clone())?;

    for (msg, existing_msg) in existing_messages {
        // Check if there's no existing message or if the new message has a newer timestamp
        if existing_msg.is_none() || existing_msg.as_ref().unwrap().timestamp < msg.timestamp {
            // Apply the message (equivalent to storeService.apply)
            apply_message(&mut tx, &msg)?;
        }

        // Check if there's no existing message or if the timestamps are different
        if existing_msg.is_none() || existing_msg.as_ref().unwrap().timestamp != msg.timestamp {
            // Insert the timestamp into the clock (equivalent to clockService.insertTimestamp)
            let inserted_timestamp:Clock = HlcService::insert_timestamp(&mut tx, msg.timestamp)?;
            let mut updated_msg = msg.clone();
            updated_msg.group_id = std::env::var("GROUP_ID").unwrap_or_else(|_| "my-group".to_string());
            updated_msg.client_id = inserted_timestamp.timestamp.node_id;

            // Insert the message with additional fields (equivalent to messagesService.insertMessage)
            message_service::insert_message(&mut tx, updated_msg)?;
        }
    }

    Ok(())
}