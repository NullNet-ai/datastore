use crate::db;
use crate::models::crdt_message_model::CrdtMessage;
use crate::structs::structs::Clock;
use crate::sync::hlc::hlc_service::HlcService;
use crate::sync::message_manager::get_sender;
use crate::sync::message_service;
use crate::sync::message_service::{compare_messages, create_messages};
use crate::sync::store::store_driver::apply;
use crate::sync::sync_endpoints_service;
use crate::sync::transactions::queue_service::QueueService;
use crate::sync::transactions::transaction_service::TransactionService;
use crate::sync::transport::transport_driver::{self, HttpTransportDriver};
use diesel::result::Error as DieselError;
use diesel_async::AsyncConnection;
use diesel_async::AsyncPgConnection;
use futures::Stream;
use hlc;
use merkle::MerkleTree;
use serde_json::Value;
use std::time::Duration;
use tokio::time::sleep;

use super::transport::transport_driver::PostOpts;

pub async fn insert(table: &String, row: Value) -> Result<(), DieselError> {
    let operation = "Insert".to_string();
    let mut conn = db::get_async_connection().await;

    let messages: Vec<CrdtMessage> = conn
        .transaction::<_, DieselError, _>(|mut tx| {
            Box::pin(async move {
                let messages = create_messages(&mut tx, &row, table, operation)
                    .await
                    .map_err(|e| {
                        log::error!("Failed to create messages: {}", e);
                        DieselError::RollbackTransaction
                    })?;

                if messages.is_empty() {
                    log::warn!("create_messages returned empty vector");
                }

                if let Err(e) = send_messages(&mut tx, messages.clone()).await {
                    log::error!("Failed to send messages: {}", e);
                    return Err(DieselError::RollbackTransaction);
                }

                Ok(messages)
            })
        })
        .await?;

    if messages.is_empty() {
        log::warn!("No messages created for insert operation");
        return Ok(());
    }

    Ok(())
}

pub async fn send_messages(
    mut tx: &mut AsyncPgConnection,
    messages: Vec<CrdtMessage>,
) -> Result<(), Box<dyn std::error::Error>> {
    apply_messages(&mut tx, messages.clone()).await?;
    let messages_value: Vec<Value> = messages
        .iter()
        .map(|msg| {
            let serialized = serde_json::json!({
                "timestamp": msg.timestamp,
                "dataset": msg.dataset,
                "database":msg.database,
                "operation": msg.operation,
                "column": msg.column,
                "value": msg.value,
                "group_id": msg.group_id,
                "client_id": msg.client_id,
                "row":msg.row,
                "hypertable_timestamp": msg.hypertable_timestamp,
            });

            // Log each serialized message for debugging

            serialized
        })
        .collect();

    QueueService::enqueue(
        tx,
        serde_json::json!({
            "messages": messages_value,
            "since": null
        }),
        "test",
    )
    .await?;

    // Schedule next background sync with reduced timer
    // let sync_timer_ms = std::env::var("SYNC_TIMER_MS")
    //     .ok()
    //     .and_then(|timer| timer.parse::<u64>().ok())
    //     .unwrap_or(60000);

    // let reduced_timer = (sync_timer_ms as f64 * 0.25) as u64;
    // schedule_next_sync(reduced_timer);

    Ok(())
}

async fn apply_messages(
    mut tx: &mut AsyncPgConnection,
    messages: Vec<CrdtMessage>,
) -> Result<(), Box<dyn std::error::Error>> {
    let existing_messages = compare_messages(&mut tx, messages.clone()).await?;
    let sender = get_sender().cloned().unwrap_or_else(|| {
        log::error!("Failed to send message: sender not available");
        panic!("Message sender not available") // Or handle the error differently
    });

    for (msg, existing_msg) in existing_messages {
        if existing_msg.is_none() || existing_msg.as_ref().unwrap().timestamp < msg.timestamp {
            apply(&mut tx, &msg).await?;
        }

        if existing_msg.is_none() || existing_msg.as_ref().unwrap().timestamp != msg.timestamp {
            // ! bottleneck here
            let inserted_timestamp: Clock =
                HlcService::insert_timestamp(&mut tx, &msg.timestamp).await?;
            let mut updated_msg = msg; // Remove .clone()
            updated_msg.group_id =
                std::env::var("GROUP_ID").unwrap_or_else(|_| "my-group".to_string());
            updated_msg.client_id = inserted_timestamp.timestamp.node_id.clone();

            // println!("time till insert2 {:?}", time_till_insert2);

            sender.send(updated_msg).await?;
        }
    }

    Ok(())
}

pub async fn iterate_queue<'a>(endpoints: Vec<PostOpts>) -> impl Stream<Item = Vec<Value>> + 'a {
    async_stream::stream! {
        let sync_timer_ms = 1000;
        let mut conn = db::get_async_connection().await;

        loop {
            // ! default param passed as test
            let size = match QueueService::size( "test").await {
                Ok(s) => s,
                Err(_) => {
                    sleep(Duration::from_millis(sync_timer_ms)).await;
                    continue;
                }
            };

            if size == 0 {
                break;
            }

            // ! default param passed as test

            let pack = match QueueService::dequeue(&mut conn, "test").await {
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

            let mut all_success = true;
            for endpoint in &endpoints {

                match sync(
                    messages.clone(),
                    since.as_ref(),
                    transaction_id.clone(),
                    endpoint.clone(),
                   &mut conn
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
    mut conn: &mut AsyncPgConnection,
) -> Result<(), Box<dyn std::error::Error>> {
    let sync_timer_ms = 20000;

    // Create a file to store benchmark data

    // Write CSV header

    loop {
        // ! default param passed as test
        let size = match QueueService::size("test").await {
            Ok(s) => s,
            Err(_) => {
                continue;
            }
        };

        if size == 0 {
            break;
        }

        // ! default param passed as test
        let pack = match QueueService::dequeue(&mut conn, "test").await {
            Ok(Some(value)) => value,
            Ok(None) => {
                println!("Queue dequeue returned None, no items available");
                log::debug!("Queue dequeue returned None, no items available");
                sleep(Duration::from_millis(100)).await;
                continue;
            }
            Err(e) => {
                log::error!("Error dequeuing from queue: {}", e);
                sleep(Duration::from_millis(sync_timer_ms)).await;
                continue;
            }
        };
        // println!(
        //     "pack {}",
        //     serde_json::to_string_pretty(&pack).unwrap_or_default()
        // );

        let messages = pack
            .get("messages")
            .and_then(|m| m.as_array())
            .cloned()
            .unwrap_or_default();

        let since = pack
            .get("since")
            .and_then(|s| s.as_str())
            .map(|s_str| hlc::Timestamp::parse(s_str.to_string()))
            .map(|t| t.clone());

        let transaction_id = pack
            .get("transaction_id")
            .and_then(|t| t.as_str())
            .map(ToString::to_string);

        let mut all_success = true;
        for endpoint in &endpoints {
            match sync(
                messages.clone(),
                since.as_ref(),
                transaction_id.clone(),
                endpoint.clone(),
                &mut conn,
            )
            .await
            {
                Ok(_) => (),
                Err(_) => {
                    all_success = false;
                    break;
                }
            }
        }

        if all_success {
            log::debug!("All endpoints succeeded");
            if let Err(e) = QueueService::ack(&mut conn, "test").await {
                log::error!("Failed to acknowledge queue message: {}", e);
            }
            // Check if we've reached a benchmark interval for queue item
        } else {
            sleep(Duration::from_millis(sync_timer_ms)).await;
        }
    }

    Ok(())
}

pub async fn bg_sync() -> Result<(), Box<dyn std::error::Error>> {
    let sync_enabled = std::env::var("SYNC_ENABLED").unwrap_or_else(|_| "false".to_string());
    let mut conn = db::get_async_connection().await;

    if sync_enabled == "false" {
        return Ok(());
    }

    log::debug!("Sync Service Initialized");

    // Get endpoints from sync_endpoints_service
    let endpoints = match sync_endpoints_service::get_sync_endpoints(&mut conn).await {
        Ok(endpoints) => endpoints,
        Err(e) => {
            log::error!("Failed to get sync endpoints: {}", e);
            return Ok(());
        }
    };

    log::debug!(
        "endpoints {}",
        serde_json::to_string_pretty(&endpoints).unwrap_or_default()
    );

    if !endpoints.is_empty() {
        let queue_size = QueueService::size("test").await.unwrap_or(0);

        if queue_size == 0 {
            for endpoint in &endpoints {
                match sync(Vec::new(), None, None, endpoint.clone(), &mut conn).await {
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
            if let Err(e) = process_queue(endpoints, &mut conn).await {
                log::error!("Error processing queue: {}", e);
            }
        }
    }

    let sync_timer_ms = std::env::var("SYNC_TIMER_MS")
        .ok()
        .and_then(|timer| timer.parse::<u64>().ok())
        .unwrap_or(60000);

    schedule_next_sync(sync_timer_ms);

    Ok(())
}
fn schedule_next_sync(delay_ms: u64) {
    tokio::spawn(async move {
        sleep(Duration::from_millis(delay_ms)).await;

        // Create the connection inside the spawned task and handle the Result

        if let Err(e) = bg_sync().await {
            log::error!("Error in bg_sync: {}", e);
        }
    });
}

async fn sync(
    initial_messages: Vec<Value>,
    since: Option<&hlc::Timestamp>,
    existing_transaction_id: Option<String>,
    options: PostOpts,
    conn: &mut AsyncPgConnection,
) -> Result<(), Box<dyn std::error::Error>> {
    let sync_enabled = std::env::var("SYNC_ENABLED").unwrap_or_else(|_| "false".to_string());
    if sync_enabled != "true" {
        println!("Sync is disabled");
        return Ok(());
    }

    let group_id =
        std::env::var("GROUP_ID").unwrap_or_else(|_| "01JBHKXHYSKPP247HZZWHA3JBT".to_string());
    println!("Using group_id: {}", group_id);

    let transaction_id =
        TransactionService::start_transaction(conn, existing_transaction_id).await?;
    let transaction_id_clone = transaction_id.clone();
    println!("Started transaction: {}", transaction_id);

    let clock = HlcService::get_clock(conn).await?;
    println!(
        "Sync Attempt at {} since:{} messages:{} transaction_id:{}",
        chrono::Utc::now().to_rfc3339(),
        since.as_ref().map_or("null".to_string(), |s| s.to_string()),
        initial_messages.len(),
        transaction_id
    );

    let mut messages = initial_messages;

    if let Some(since_val) = since.clone() {
        let timestamp = since_val.to_string();

        messages = message_service::get_messages_since(conn, &timestamp).await?;

        log::debug!(
            "Since:{} - {} messages:{}",
            timestamp,
            timestamp,
            messages.len()
        );
    }
    let result = match HttpTransportDriver
        .post(
            serde_json::json!({
                "group_id": group_id,
                "client_id": clock.timestamp.node_id,
                "messages": messages,
                "merkle": clock.merkle,
            }),
            &transport_driver::PostOpts {
                url: options.url.clone(),
                username: options.username.clone(),
                password: options.password.clone(),
            },
        )
        .await
    {
        Ok(response) => response,
        Err(e) => {
            log::error!("Network Failure - {}", e);
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Network failure during sync: {}", e),
            )));
            //return error here
        }
    };

    if result.get("error").is_some() {
        log::error!("Error in syncing to server");
        TransactionService::stop_transaction(conn, &transaction_id).await?;
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Error in syncing to server",
        )));
    }

    if let Some(received_messages) = result.get("messages").and_then(|m| m.as_array()) {
        if !received_messages.is_empty() {
            log::debug!("{} updates received.", received_messages.len());
            receive_messages(conn, received_messages.clone()).await?;
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
    let result_merkle = result
        .get("merkle")
        .and_then(|m| m.as_str())
        .unwrap_or("")
        .to_string();
    if result_merkle.is_empty() {
        log::debug!("No Merkle tree found in the response");
    }
    let clock = HlcService::get_clock(conn).await?;
    let merkle_str = serde_json::to_string(&clock.merkle)?;
    let clock_merkle = MerkleTree::deserialize(&merkle_str).unwrap();
    let parsed_merkle = if result_merkle.is_empty() {
        // Create an empty merkle tree if the string is empty
        MerkleTree::new()
    } else {
        match MerkleTree::deserialize(&result_merkle) {
            Ok(tree) => tree,
            Err(e) => {
                log::error!("Failed to deserialize merkle tree: {}", e);
                return Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "Failed to deserialize merkle tree",
                )));
            }
        }
    };
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
                HlcService::commit_tree(conn, &parsed_merkle).await?;
            }
        }
        let parsed_timestamp = hlc::Timestamp::parse(min_timestamp_str.to_string());

        Box::pin(sync(
            Vec::new(),
            Some(&parsed_timestamp),
            Some(transaction_id),
            options,
            conn,
        ))
        .await?;
    }
    log::info!("Sync done - transaction_id:{}", transaction_id_clone);
    TransactionService::stop_transaction(conn, &transaction_id_clone).await?;
    Ok(())
}

async fn receive_messages(
    conn: &mut AsyncPgConnection, // Must use async connection type
    messages: Vec<Value>,
) -> Result<(), Box<dyn std::error::Error>> {
    // Transaction with explicit error handling
    let inner_messages = conn
        .transaction::<_, DieselError, _>(|conn| {
            Box::pin(async move {
                let mut processed_messages = Vec::new();

                for message in messages {
                    // Error handling with context and better error messages
                    let timestamp = message
                        .get("message")
                        .and_then(|m| m.get("timestamp"))
                        .and_then(|t| t.as_str())
                        .ok_or_else(|| DieselError::RollbackTransaction)?;

                    // Async operation within transaction
                    HlcService::recv(conn, timestamp.to_string())
                        .await
                        .map_err(|e| {
                            log::error!("Failed to receive HLC: {}", e);
                            DieselError::RollbackTransaction
                        })?;

                    let inner_message = message
                        .get("message")
                        .ok_or_else(|| DieselError::RollbackTransaction)?;

                    let crdt_message: CrdtMessage =
                        serde_json::from_value(inner_message.clone())
                            .map_err(|_| DieselError::RollbackTransaction)?;

                    processed_messages.push(crdt_message);
                }

                Ok(processed_messages)
            })
        })
        .await?; // Critical: Must await the transaction

    apply_messages(conn, inner_messages).await?;
    Ok(())
}
