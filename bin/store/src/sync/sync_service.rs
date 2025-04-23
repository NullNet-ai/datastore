use crate::db;
use crate::db::DbPooledConnection;
use crate::models::crdt_message_model::CrdtMessage;
use crate::structs::structs::Clock;
use crate::sync::hlc::hlc_service::HlcService;
use crate::sync::message_service;
use crate::sync::message_service::{compare_messages, create_messages};
use crate::sync::store::store_driver::apply;
use crate::sync::sync_endpoints_service;
use crate::sync::transactions::queue_service::QueueService;
use crate::sync::transactions::transaction_service::TransactionService;
use crate::sync::transport::transport_driver::{self, HttpTransportDriver};
use diesel::Connection;
use diesel::result::Error as DieselError;
use futures::Stream;
use hlc;
use merkle::MerkleTree;
use serde_json::Value;
use std::time::Duration;
use tokio::time::sleep;
use std::io::Write;

use super::transport::transport_driver::PostOpts;

pub async fn insert(table: &String, row: Value) -> Result<(), DieselError> {
    let operation = "Insert".to_string();
    let mut conn = db::get_connection();

    let messages: Vec<CrdtMessage> = conn.transaction(|mut tx| {
        let result = create_messages(&mut tx, &row, table, operation);
        match &result {
            Ok(msgs) => {
                if msgs.is_empty() {
                    log::warn!("create_messages returned empty vector");
                }
            }
            Err(e) => log::error!("Failed to create messages: {}", e),
        }
        result
    })?;

    if messages.is_empty() {
        log::warn!("No messages created for insert operation");
        return Ok(());
    }

    if let Err(e) = send_messages(&mut conn, messages).await {
        log::error!("Failed to send messages: {}", e);
        // You might want to return this error or handle it differently
        // depending on your application's requirements
    }

    Ok(())
}

pub async fn send_messages(
    mut tx: &mut DbPooledConnection,
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
    )?;

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
    mut tx: &mut DbPooledConnection,
    messages: Vec<CrdtMessage>,
) -> Result<(), Box<dyn std::error::Error>> {
    let existing_messages = compare_messages(&mut tx, messages.clone())?;

    for (msg, existing_msg) in existing_messages {
        if existing_msg.is_none() || existing_msg.as_ref().unwrap().timestamp < msg.timestamp {
            apply(&mut tx, &msg).await;
        }

        if existing_msg.is_none() || existing_msg.as_ref().unwrap().timestamp != msg.timestamp {
            let inserted_timestamp: Clock = HlcService::insert_timestamp(&mut tx, &msg.timestamp)?;
            let mut updated_msg = msg.clone();
            updated_msg.group_id =
                std::env::var("GROUP_ID").unwrap_or_else(|_| "my-group".to_string());
            updated_msg.client_id = inserted_timestamp.timestamp.node_id.clone();

            message_service::insert_message(&mut tx, updated_msg)?;
        }
    }

    Ok(())
}

pub async fn iterate_queue<'a>(endpoints: Vec<PostOpts>) -> impl Stream<Item = Vec<Value>> + 'a {
    async_stream::stream! {
        let sync_timer_ms = 1000;
        let mut conn = db::get_connection();

        loop {
            // ! default param passed as test
            let size = match QueueService::size( "test") {
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
    mut conn: &mut DbPooledConnection,
) -> Result<(), Box<dyn std::error::Error>> {
    let sync_timer_ms = 20000;
    let start_time = std::time::Instant::now();
    let mut total_items_processed = 0;
    let mut total_messages_processed = 0;
    let benchmark_interval = 100000; // Every 100,000 queue items 


    let mut last_benchmark_time = start_time;
    let mut last_benchmark_count = 0;
    
    // Create a file to store benchmark data
    let file_path = "sync_benchmark_results.csv";
    let mut file = std::fs::File::create(file_path)?;
    
    // Write CSV header
    writeln!(file, "items_processed,messages_processed,elapsed_seconds,items_per_sec,messages_per_sec")?;

    println!("Starting queue processing benchmark...");
    loop {
        // ! default param passed as test
        let size = match QueueService::size("test") {
            Ok(s) => s,
            Err(_) => {
                continue;
            }
        };
        println!("queue size {}", size);

        if size == 0 {
            break;
        }

        // ! default param passed as test
        let pack = match QueueService::dequeue(&mut conn, "test") {
            Ok(Some(value)) => value,
            Ok(None) => {
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
        println!(
            "pack {}",
            serde_json::to_string_pretty(&pack).unwrap_or_default()
        );

        let messages = pack
            .get("messages")
            .and_then(|m| m.as_array())
            .cloned()
            .unwrap_or_default();
        let message_count = messages.len();

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
            let _ = QueueService::ack(&mut conn, "test");
            total_items_processed += 1;
            total_messages_processed += message_count;
            
            // Check if we've reached a benchmark interval for queue items
            if total_items_processed / benchmark_interval > last_benchmark_count / benchmark_interval {
                let current_time = std::time::Instant::now();
                let interval_time = current_time.duration_since(last_benchmark_time);
                let total_time = current_time.duration_since(start_time);
                
                // Calculate items processed in this interval
                let interval_items = total_items_processed - last_benchmark_count;
                
                // Calculate rates
                let interval_items_rate = interval_items as f64 / interval_time.as_secs_f64();
                let total_items_rate = total_items_processed as f64 / total_time.as_secs_f64();
                let total_messages_rate = total_messages_processed as f64 / total_time.as_secs_f64();
                
                // Write to CSV
                writeln!(
                    file, 
                    "{},{},{:.2},{:.2},{:.2}", 
                    total_items_processed,
                    total_messages_processed,
                    total_time.as_secs_f64(),
                    total_items_rate,
                    total_messages_rate
                )?;
                
                println!("===== BENCHMARK RESULTS =====");
                println!("Queue items processed: {}/{}", total_items_processed, 1_000_000);
                println!("Total messages processed: {}", total_messages_processed);
                println!("Interval time ({} items): {:?}", interval_items, interval_time);
                println!("Interval throughput: {:.2} items/sec", interval_items_rate);
                println!("Total time so far: {:?}", total_time);
                println!("Overall throughput: {:.2} items/sec, {:.2} msgs/sec", 
                    total_items_rate, total_messages_rate);
                println!("Estimated time remaining: {:?}", 
                    std::time::Duration::from_secs_f64(
                        (1_000_000 - total_items_processed) as f64 / total_items_rate
                    ));
                println!("=============================");
                
                // Update benchmark tracking variables
                last_benchmark_time = current_time;
                last_benchmark_count = total_items_processed;
            }

        } else {
            sleep(Duration::from_millis(sync_timer_ms)).await;
        }
        let total_time = start_time.elapsed();
        let total_items_rate = total_items_processed as f64 / total_time.as_secs_f64();
        let total_messages_rate = total_messages_processed as f64 / total_time.as_secs_f64();
        
        // Write final data point
        writeln!(
            file, 
            "{},{},{:.2},{:.2},{:.2}", 
            total_items_processed,
            total_messages_processed,
            total_time.as_secs_f64(),
            total_items_rate,
            total_messages_rate
        )?;
        
        println!("===== FINAL BENCHMARK RESULTS =====");
        println!("Total queue items processed: {}", total_items_processed);
        println!("Total messages processed: {}", total_messages_processed);
        println!("Total time: {:?}", total_time);
        println!("Overall throughput: {:.2} items/sec, {:.2} msgs/sec", 
            total_items_rate, total_messages_rate);
        println!("Benchmark data saved to {}", file_path);
        println!("==================================");
    
    }

    Ok(())
}

pub async fn bg_sync(conn: &mut DbPooledConnection) -> Result<(), Box<dyn std::error::Error>> {
    let sync_enabled = std::env::var("SYNC_ENABLED").unwrap_or_else(|_| "false".to_string());

    if sync_enabled == "false" {
        return Ok(());
    }

    log::debug!("Sync Service Initialized");

    // Get endpoints from sync_endpoints_service
    let endpoints = match sync_endpoints_service::get_sync_endpoints(conn) {
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
        let queue_size = QueueService::size("test").unwrap_or(0);

        if queue_size == 0 {
            for endpoint in &endpoints {
                match sync(Vec::new(), None, None, endpoint.clone(), conn).await {
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
            if let Err(e) = process_queue(endpoints, conn).await {
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
    let mut conn = db::get_connection();
    tokio::spawn(async move {
        sleep(Duration::from_millis(delay_ms)).await;

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
    conn: &mut DbPooledConnection,
) -> Result<(), Box<dyn std::error::Error>> {
    let sync_enabled = std::env::var("SYNC_ENABLED").unwrap_or_else(|_| "false".to_string());
    if sync_enabled != "true" {
        println!("Sync is disabled");
        return Ok(());
    }

    let group_id =
        std::env::var("GROUP_ID").unwrap_or_else(|_| "01JBHKXHYSKPP247HZZWHA3JBT".to_string());
    println!("Using group_id: {}", group_id);

    let transaction_id = TransactionService::start_transaction(conn, existing_transaction_id)?;
    let transaction_id_clone = transaction_id.clone();
    println!("Started transaction: {}", transaction_id);

    let clock = HlcService::get_clock(conn)?;
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

        messages = message_service::get_messages_since(conn, &timestamp)?;

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
        TransactionService::stop_transaction(conn, &transaction_id)?;
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
    let clock = HlcService::get_clock(conn)?;
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
                HlcService::commit_tree(conn, &parsed_merkle)?;
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
    TransactionService::stop_transaction(conn, &transaction_id_clone)?;
    Ok(())
}

async fn receive_messages(
    conn: &mut DbPooledConnection,
    messages: Vec<Value>,
) -> Result<(), Box<dyn std::error::Error>> {
    let inner_messages =
        conn.transaction::<Vec<CrdtMessage>, Box<dyn std::error::Error>, _>(|tx| {
            let mut processed_messages = Vec::new();

            for message in messages {
                let timestamp = message
                    .get("message")
                    .and_then(|m| m.get("timestamp"))
                    .and_then(|t| t.as_str())
                    .ok_or("Missing timestamp")?;

                HlcService::recv(tx, timestamp.to_string())?;

                let inner_message = message.get("message").ok_or("Missing message content")?;

                // Convert Value to CrdtMessage
                let crdt_message: CrdtMessage = serde_json::from_value(inner_message.clone())?;
                processed_messages.push(crdt_message);
            }

            Ok(processed_messages)
        })?;

    apply_messages(conn, inner_messages).await?;

    Ok(())
}
