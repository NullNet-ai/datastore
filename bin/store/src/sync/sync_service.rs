use crate::db;
use crate::db::DbPooledConnection;
use crate::models::crdt_message_model::CrdtMessage;
use crate::structs::structs::{Clock, Endpoint};
use crate::sync::hlc::hlc_service::{self, HlcService};
use crate::sync::message_service;
use crate::sync::message_service::{compare_messages, create_messages};
use crate::sync::store::store_driver::apply;
use actix_web::cookie::time;
use diesel::result::Error as DieselError;
use diesel::sql_types::Timestamp;
use diesel::Connection;
use serde_json::Value;
use futures::{Stream, StreamExt};
use std::time::Duration;
use tokio::time::sleep;
use crate::sync::transactions::queue_service::QueueService;
use crate::sync::transport::transport_driver::{self, HttpTransportDriver};
use crate::sync::transactions::transaction_service::TransactionService;
use hlc;
use merkle::MerkleTree;
use crate::sync::sync_endpoints_service;

use super::transport::transport_driver::PostOpts;

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
    endpoints: Vec<PostOpts>,
) -> impl Stream<Item = Vec<Value>> + 'a {
    async_stream::stream! {
        let sync_timer_ms = 1000;
        let mut conn = db::get_connection();
        
        loop {
            // Check queue size with current connection
            // ! default param passed as test
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
            // ! default param passed as test

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
            
                let since = pack.get("since")
    .and_then(|s| s.as_str())
    .map(|s_str| hlc::Timestamp::parse(s_str.to_string()))
    .map(|t| t.clone()); 

            let transaction_id = pack.get("transaction_id")
                .and_then(|t| t.as_str())
                .map(ToString::to_string);

            // Process all endpoints before acking
            let mut all_success = true;
            for endpoint in &endpoints {
                
                match sync(
                    messages.clone(),
                    since.as_ref(),
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

pub async fn process_queue(
    endpoints: Vec<PostOpts>,
    mut conn: &mut DbPooledConnection,
) -> Result<(), Box<dyn std::error::Error>> {
    let sync_timer_ms = 1000;
    
    loop {
        // Check queue size with current connection
        // ! default param passed as test
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
        // ! default param passed as test
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
        
        let since = pack.get("since")
            .and_then(|s| s.as_str())
            .map(|s_str| hlc::Timestamp::parse(s_str.to_string()))
            .map(|t| t.clone()); 

        let transaction_id = pack.get("transaction_id")
            .and_then(|t| t.as_str())
            .map(ToString::to_string);

        // Process all endpoints before acking
        let mut all_success = true;
        for endpoint in &endpoints {
            match sync(
                messages.clone(),
                since.as_ref(),
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
        } else {
            sleep(Duration::from_millis(sync_timer_ms)).await;
        }
    }
    
    Ok(())
}

pub async fn bg_sync(mut conn: &mut DbPooledConnection) -> Result<(), Box<dyn std::error::Error>> {
    let sync_enabled = std::env::var("SYNC_ENABLED").unwrap_or_else(|_| "false".to_string());
    if sync_enabled == "false" {
        return Ok(());
    }
    
    log::debug!("Sync Service Initialized");
    
    // Get endpoints from sync_endpoints_service
    let endpoints = match sync_endpoints_service::get_sync_endpoints(&mut conn) {
        Ok(endpoints) => endpoints,
        Err(e) => {
            log::error!("Failed to get sync endpoints: {}", e);
            return Ok(());
        }
    };
    
    log::debug!("endpoints {}", serde_json::to_string_pretty(&endpoints).unwrap_or_default());
    
    if !endpoints.is_empty() {
        // Check queue size
        let mut conn = db::get_connection();
        let queue_size = QueueService::size(&mut conn, "test").unwrap_or(0);
        
        if queue_size == 0 {
            // If queue is empty, sync with all endpoints
            for endpoint in &endpoints {
                match sync(Vec::new(), None, None, endpoint.clone()).await {
                    Ok(_) => (),
                    Err(e) => {
                        if e.to_string().contains("Existing Transaction") {
                            log::error!("Error in bg_sync: Existing Transaction Detected");
                        } else {
                            log::error!("Error in bg_sync: {}", e);
                        }
                    }
                }
            }
        } else {
            // Process queue if it's not empty - use the new function
            if let Err(e) = process_queue(endpoints, &mut conn).await {
                log::error!("Error processing queue: {}", e);
            }
        }
    }
    
    // Get sync timer from config
    let sync_timer_ms = std::env::var("SYNC_TIMER_MS")
        .ok()
        .and_then(|timer| timer.parse::<u64>().ok())
        .unwrap_or(60000); 
    
    // Use schedule_next_sync to handle the recursive call
    schedule_next_sync(sync_timer_ms);
    
    Ok(())
}
fn schedule_next_sync(delay_ms: u64) {
    tokio::spawn(async move {
        sleep(Duration::from_millis(delay_ms)).await;
        
        // Create a new connection inside the spawned task
        let mut conn = db::get_connection();
        
        // Call bg_sync with the new connection
        if let Err(e) = bg_sync(&mut conn).await {
            log::error!("Error in bg_sync: {}", e);
        }
    });
}

async fn sync(
    initial_messages: Vec<Value>,
    since: Option<&hlc::Timestamp>,
    existing_transaction_id: Option<String>,
    options: PostOpts,
) -> Result<(), Box<dyn std::error::Error>> {
    let sync_enabled = std::env::var("SYNC_ENABLED").unwrap_or_else(|_| "false".to_string());
    if sync_enabled != "true" {
        println!("Sync is disabled");
        return Ok(());
    }

    let group_id = std::env::var("GROUP_ID").unwrap_or_else(|_| "01JBHKXHYSKPP247HZZWHA3JBT".to_string());
    println!("Using group_id: {}", group_id);

    let mut conn = db::get_connection();
    
    let transaction_id = TransactionService::start_transaction(&mut conn, existing_transaction_id)?;
    let transaction_id_clone = transaction_id.clone();
    println!("Started transaction: {}", transaction_id);

    let clock= HlcService::get_clock(&mut conn)?;
    println!(
        "Sync Attempt at {} since:{} messages:{} transaction_id:{}",
        chrono::Utc::now().to_rfc3339(),
        since.as_ref().map_or("null".to_string(), |s| s.to_string()),
        initial_messages.len(),
        transaction_id
    );

    let mut messages = initial_messages;
    
    if let Some(since_val) = since.clone() {

        let timestamp =since_val.to_string();


        
        messages = message_service::get_messages_since(&mut conn, &timestamp)?;
        
        log::debug!(
            "Since:{} - {} messages:{}",
            timestamp,
            timestamp,
            messages.len()
        );
    }
    let result = match HttpTransportDriver.post(
        serde_json::json!({
            "group_id": group_id,
            "client_id": clock.timestamp.node_id,
            "messages": messages,
            "merkle": clock.merkle,
        }),
        &transport_driver::PostOpts {  // Convert Endpoint to PostOpts
            url: options.url.clone(),
            username: options.username.clone(),
            password: options.password.clone(),
        }
    ).await {
        Ok(response) => response,
        Err(e) => {
            log::error!("Network Failure - {}", e);
            return Ok(());
        }
    };

    if result.get("error").is_some() {
        log::error!("Error in syncing to server");
        TransactionService::stop_transaction(&mut conn, &transaction_id)?;
        return Ok(());
    }

    if let Some(received_messages) = result.get("messages").and_then(|m| m.as_array()) {
        if !received_messages.is_empty() {
            log::debug!("{} updates received.", received_messages.len());
            receive_messages(&mut conn, received_messages.clone()).await?;
            log::info!(
                "Synced {} at {}", 
                received_messages.len(),
                chrono::Utc::now().to_rfc3339()
            );
        } else {
            log::debug!("No new remote updates");
        }
    } else {
        log::debug!("No new remote updates");
    }
    let result_merkle=result.get("merkle").and_then(|m| m.as_str()).unwrap_or("").to_string();
    if result_merkle.is_empty() {
        log::debug!("No Merkle tree found in the response");
    }
    let clock = HlcService::get_clock(&mut conn)?;
        let merkle_str = serde_json::to_string(&clock.merkle)?;
        let clock_merkle = MerkleTree::deserialize(&merkle_str).unwrap();
        let parsed_merkle = MerkleTree::deserialize(&result_merkle).unwrap();
        let diff_time = parsed_merkle.find_differences(&clock_merkle);
        if !diff_time.is_empty() {

            // ! get reviewed by bon and also check first index manually, if the first index is smallest or not

            let (_, server_node, client_node) = &diff_time[0];

            let min_timestamp_str = if server_node.value <= client_node.value {
                &server_node.value
            } else {
                &client_node.value
            };
            log::debug!(
                "Timeline lag detected: since:{} diff:{}",
                since.as_ref().map_or("null".to_string(), |s| s.to_string()),
                min_timestamp_str
            );
    
            // Check if since matches diff_time, indicating potential clock drift
            if let Some(since_val) = since {
                if since_val.to_string() == *min_timestamp_str {
                    log::error!("Clock Drift Detected - Adjusting Clocks and Retrying Sync");
                    // Commit the server's Merkle tree to resolve drift
                    HlcService::commit_tree(&mut conn, &parsed_merkle)?;
                }
            }
            let parsed_timestamp = hlc::Timestamp::parse(min_timestamp_str.to_string());
    
            // Retry sync with diff_time
            
            Box::pin(sync(
                Vec::new(),
                Some(&parsed_timestamp),
                Some(transaction_id),
                options
            )).await?;
        }
        log::info!("Sync done - transaction_id:{}", transaction_id_clone);
        TransactionService::stop_transaction(&mut conn, &transaction_id_clone)?;
    Ok(())
}

async fn receive_messages(
    conn: &mut DbPooledConnection,
    messages: Vec<Value>,
) -> Result<(), Box<dyn std::error::Error>> {
    let inner_messages = conn.transaction::<Vec<CrdtMessage>, Box<dyn std::error::Error>, _>(|tx| {
        let mut processed_messages = Vec::new();
        
        for message in messages {
            let timestamp = message.get("message")
                .and_then(|m| m.get("timestamp"))
                .and_then(|t| t.as_str())
                .ok_or("Missing timestamp")?;

            HlcService::recv(tx, timestamp.to_string())?;
            
            let inner_message = message.get("message")
                .ok_or("Missing message content")?;
            
            // Convert Value to CrdtMessage
            let crdt_message: CrdtMessage = serde_json::from_value(inner_message.clone())?;
            processed_messages.push(crdt_message);
        }
        
        Ok(processed_messages)
    })?;

    // Apply processed messages
    apply_messages(conn, inner_messages).await?;
    
    Ok(())
}

// ... existing code ...