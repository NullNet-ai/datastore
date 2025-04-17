use crate::db;
use crate::db::DbPooledConnection;
use crate::models::crdt_message_model::CrdtMessage;
use crate::structs::structs::Clock;
use crate::sync::hlc::hlc_service::HlcService;
use crate::sync::message_service;
use crate::sync::message_service::{compare_messages, create_messages};
use crate::sync::store::store_driver::apply;
use diesel::result::Error as DieselError;
use diesel::Connection;
use serde_json::Value;
use futures::{Stream, StreamExt};
use std::time::Duration;
use tokio::time::sleep;
use crate::sync::transactions::queue_service::QueueService;
use crate::sync::transport::transport_driver;
use crate::sync::transactions::transaction_service::TransactionService;

#[derive(Clone)]
pub struct Endpoint {
    pub url: String,
    pub username: String,
    pub password: String,
}

pub async fn insert(table: &String, row: Value) -> Result<(), DieselError> {
    let operation = "Insert".to_string();
    let mut conn = db::get_connection();

    // Create messages outside the transaction
    let messages: Vec<CrdtMessage> =
        conn.transaction(|mut tx| create_messages(&mut tx, &row, table, operation))?;

    // Then send messages asynchronously
    let mut tx = db::get_connection();
    send_messages(&mut tx, messages).await;

    Ok(())
}

pub async fn send_messages(
    mut tx: &mut DbPooledConnection,
    messages: Vec<CrdtMessage>,
) -> Result<(), Box<dyn std::error::Error>> {
    apply_messages(&mut tx, messages).await?;
    Ok(())
}

async fn apply_messages(
    mut tx: &mut DbPooledConnection,
    messages: Vec<CrdtMessage>,
) -> Result<(), Box<dyn std::error::Error>> {
    //use messageService.compareMessages here
    let existing_messages = compare_messages(&mut tx, messages.clone())?;

    for (msg, existing_msg) in existing_messages {
        // Check if there's no existing message or if the new message has a newer timestamp
        if existing_msg.is_none() || existing_msg.as_ref().unwrap().timestamp < msg.timestamp {
            // Apply the message (equivalent to storeService.apply)
            apply(&mut tx, &msg).await;
        }

        // Check if there's no existing message or if the timestamps are different
        if existing_msg.is_none() || existing_msg.as_ref().unwrap().timestamp != msg.timestamp {
            // Insert the timestamp into the clock (equivalent to clockService.insertTimestamp)
            let inserted_timestamp: Clock = HlcService::insert_timestamp(&mut tx, &msg.timestamp)?;
            let mut updated_msg = msg.clone();
            updated_msg.group_id =
                std::env::var("GROUP_ID").unwrap_or_else(|_| "my-group".to_string());
            updated_msg.client_id = inserted_timestamp.timestamp.node_id.clone();

            // Insert the message with additional fields (equivalent to messagesService.insertMessage)
            message_service::insert_message(&mut tx, updated_msg)?;
        }
    }

    Ok(())
}

pub async fn iterate_queue<'a>(
    endpoints: Vec<Endpoint>,
) -> impl Stream<Item = Vec<Value>> + 'a {
    async_stream::stream! {
        let sync_timer_ms = 1000;
        let mut conn = db::get_connection();
        
        loop {
            // Check queue size with current connection
            let size = match QueueService::size(&mut conn, "test") {
                Ok(s) => s,
                Err(_) => {
                    sleep(Duration::from_millis(sync_timer_ms)).await;
                    continue;
                }
            };
            
            if size == 0 {
                break;
            }
            
            // Dequeue with proper error handling
            let pack = match QueueService::dequeue(&mut conn, "test") {
                Ok(Some(value)) => value,
                Ok(None) => {
                    sleep(Duration::from_millis(100)).await;
                    continue;
                }
                Err(_) => {
                    sleep(Duration::from_millis(sync_timer_ms)).await;
                    continue;
                }
            };
            
            // Parse package once
            let messages = pack.get("messages")
                .and_then(|m| m.as_array())
                .cloned()
                .unwrap_or_default();
            
            let since = pack.get("since").cloned();
            let transaction_id = pack.get("transaction_id")
                .and_then(|t| t.as_str())
                .map(ToString::to_string);

            // Process all endpoints before acking
            let mut all_success = true;
            for endpoint in &endpoints {
                match sync(
                    messages.clone(),
                    since.clone(),
                    transaction_id.clone(),
                    endpoint.clone()
                ).await {
                    Ok(_) => (),
                    Err(_) => {
                        all_success = false;
                        break;
                    }
                }
            }

            if all_success {
                let _ = QueueService::ack(&mut conn, "test");
                yield messages;
            } else {
                sleep(Duration::from_millis(sync_timer_ms)).await;
            }
        }
    }
}


// ... existing code ...

async fn sync(
    initial_messages: Vec<Value>,
    since: Option<Value>,
    existing_transaction_id: Option<String>,
    endpoint: Endpoint,
) -> Result<(), Box<dyn std::error::Error>> {
    
    
    Ok(())
}

async fn receive_messages(
    conn: &mut DbPooledConnection,
    messages: Vec<Value>,
) -> Result<(), Box<dyn std::error::Error>> {
    // Implement message receiving logic here
    // This would apply the received messages to your local database
    
    for message in messages {
        // Convert JSON message to CrdtMessage
        let crdt_message: CrdtMessage = serde_json::from_value(message)?;
        
        // Apply the message
        apply(conn, &crdt_message).await;
    }
    
    Ok(())
}

// ... existing code ...