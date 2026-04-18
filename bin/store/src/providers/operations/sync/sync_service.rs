use crate::database::db;
use crate::generated::models::crdt_message_model::CrdtMessageModel;
use crate::providers::operations::sync::hlc::hlc_service::HlcService;
use crate::providers::operations::sync::message_manager::get_sender;
use crate::providers::operations::sync::message_service;
use crate::providers::operations::sync::message_service::{compare_messages, create_messages};
use crate::providers::operations::sync::store::store_driver::{apply, apply_batch};
use crate::providers::operations::sync::structs::Clock;
use crate::providers::operations::sync::sync_endpoints_service;
use crate::providers::operations::sync::transactions::queue_service::QueueService;
use crate::providers::operations::sync::transactions::transaction_service::TransactionService;
use crate::providers::operations::sync::transport::transport_driver::{self, HttpTransportDriver};
use diesel::result::Error as DieselError;
use diesel_async::AsyncConnection;
use diesel_async::AsyncPgConnection;
use futures::Stream;
use hlc;
use log::{debug, info};
use merkle::MerkleTree;
use serde_json::Value;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;
use tokio::time::sleep;

/// True after bootstrap_sync_once has been run once this process (when SYNC_BOOTSTRAP_URL is set).
static BOOTSTRAP_DONE: AtomicBool = AtomicBool::new(false);

use super::transport::transport_driver::PostOpts;

/// Returns bootstrap sync endpoint from env (SYNC_BOOTSTRAP_URL, SYNC_BOOTSTRAP_USERNAME, SYNC_BOOTSTRAP_PASSWORD).
/// When set, the store will pull all messages from this server first before doing normal sync.
pub fn get_bootstrap_opts_from_env() -> Option<PostOpts> {
    let url = std::env::var("SYNC_BOOTSTRAP_URL").ok()?;
    let url = url.trim();
    if url.is_empty() {
        return None;
    }
    let username = std::env::var("SYNC_BOOTSTRAP_USERNAME").unwrap_or_else(|_| String::new());
    let password = std::env::var("SYNC_BOOTSTRAP_PASSWORD").unwrap_or_else(|_| String::new());
    Some(PostOpts {
        url: url.to_string(),
        username,
        password,
    })
}

/// One-time bootstrap sync: send empty merkle and no messages so server returns all messages (and optionally chunks).
/// Applies received messages and updates local merkle from server response. Call before starting normal bg_sync when SYNC_BOOTSTRAP_URL is set.
pub async fn bootstrap_sync_once(
    conn: &mut AsyncPgConnection,
) -> Result<(), Box<dyn std::error::Error>> {
    let opts = match get_bootstrap_opts_from_env() {
        Some(o) => o,
        None => return Ok(()),
    };
    let group_id =
        std::env::var("GROUP_ID").unwrap_or_else(|_| "01JBHKXHYSKPP247HZZWHA3JBT".to_string());
    let clock = HlcService::get_clock(conn).await?;
    let empty_merkle =
        serde_json::to_string(&MerkleTree::new()).unwrap_or_else(|_| "{}".to_string());
    log::debug!(
        "Bootstrap sync: pulling all messages from {} (group_id={})",
        opts.url,
        group_id
    );
    let result = HttpTransportDriver
        .post(
            serde_json::json!({
                "group_id": group_id,
                "client_id": clock.timestamp.node_id,
                "messages": [],
                "merkle": empty_merkle,
            }),
            &opts,
        )
        .await
        .map_err(|e| {
            log::error!("Bootstrap sync network error: {}", e);
            e
        })?;
    if result.get("error").is_some() {
        log::error!("Bootstrap sync: server returned error");
        return Err("Bootstrap sync server error".into());
    }
    if let Some(received_messages) = result.get("messages").and_then(|m| m.as_array()) {
        if !received_messages.is_empty() {
            log::debug!(
                "Bootstrap sync: applying {} messages from server",
                received_messages.len()
            );
            receive_messages(conn, received_messages.clone()).await?;
        }
    }
    if let Some(merkle_val) = result.get("merkle") {
        let merkle_str = if let Some(s) = merkle_val.as_str() {
            s.to_string()
        } else {
            serde_json::to_string(merkle_val).unwrap_or_default()
        };
        if !merkle_str.is_empty() && merkle_str != "{}" {
            if let Ok(tree) = MerkleTree::deserialize(&merkle_str) {
                HlcService::commit_tree(conn, &tree).await?;
                log::debug!("Bootstrap sync: merkle updated from server");
            }
        }
    }
    log::debug!("Bootstrap sync complete");
    Ok(())
}

/// Consumes the error (so it is not held across an await) and returns its message for use in a Send future.
fn network_error_message(e: Box<dyn std::error::Error>) -> String {
    log::error!("Network Failure - {}", e);
    e.to_string()
}

/// Consumes the error and returns its message so the future stays Send.
fn error_to_message(e: Box<dyn std::error::Error>) -> String {
    e.to_string()
}

/// Consumes an error by value so it is not held across an await (keeps future Send).
fn consume_error<E: std::fmt::Display>(e: E) {
    log::error!("{}", e);
}

pub async fn insert(table: &String, row: Value) -> Result<(), DieselError> {
    let operation = "Insert".to_string();

    let mut conn = db::get_async_connection().await;

    let messages: Vec<CrdtMessageModel> = conn
        .transaction::<_, DieselError, _>(|mut tx| {
            Box::pin(async move {
                let messages = create_messages(&mut tx, &row, table, operation)
                    .await
                    .map_err(|e| {
                        log::error!("Failed to create messages: {}", e);
                        DieselError::DatabaseError(
                            diesel::result::DatabaseErrorKind::Unknown,
                            Box::new(format!("Failed to create messages: {}", e)),
                        )
                    })?;

                if messages.is_empty() {
                    log::warn!("create_messages returned empty vector");
                }

                if let Err(e) = send_messages(&mut tx, messages.clone()).await {
                    log::error!("Failed to send messages: {}", e);
                    return Err(DieselError::DatabaseError(
                        diesel::result::DatabaseErrorKind::Unknown,
                        Box::new(format!("Failed to send messages: {}", e)),
                    ));
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

pub async fn update(table: &String, row: Value, id: &String) -> Result<(), DieselError> {
    let operation = "Update".to_string();

    //insert id into the row

    let mut conn = db::get_async_connection().await;
    let mut modified_row = row.clone();
    if let Some(obj) = modified_row.as_object_mut() {
        obj.insert("id".to_string(), Value::String(id.clone()));
    }

    let messages: Vec<CrdtMessageModel> = conn
        .transaction::<_, DieselError, _>(|mut tx| {
            Box::pin(async move {
                let messages = create_messages(&mut tx, &modified_row, table, operation)
                    .await
                    .map_err(|e| {
                        log::error!("Failed to create messages: {}", e);
                        DieselError::DatabaseError(
                            diesel::result::DatabaseErrorKind::Unknown,
                            Box::new(format!("Failed to create messages: {}", e)),
                        )
                    })?;

                if messages.is_empty() {
                    log::warn!("create_messages returned empty vector");
                }

                if let Err(e) = send_messages(&mut tx, messages.clone()).await {
                    log::error!("Failed to send messages: {}", e);
                    return Err(DieselError::DatabaseError(
                        diesel::result::DatabaseErrorKind::Unknown,
                        Box::new(format!("Failed to send messages: {}", e)),
                    ));
                }

                Ok(messages)
            })
        })
        .await?;

    if messages.is_empty() {
        log::warn!("No messages created for update operation");
        return Ok(());
    }

    Ok(())
}

pub async fn send_messages(
    mut tx: &mut AsyncPgConnection,
    messages: Vec<CrdtMessageModel>,
) -> Result<(), Box<dyn std::error::Error>> {
    apply_messages(&mut tx, messages.clone(), true).await?;
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
/// Apply messages to the dataset table and to crdt_messages (channel).
/// - When `from_local_insert` is true: apply all and send all (no compare). Fast path.
/// - When false (from server): compare first, only apply/send when we're the winner.
async fn apply_messages(
    mut tx: &mut AsyncPgConnection,
    messages: Vec<CrdtMessageModel>,
    from_local_insert: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut to_send: Vec<CrdtMessageModel> = Vec::new();

    if from_local_insert {
        // Batch all field applies into a single upsert query instead of one per field.
        // CRDT messages, timestamps, and merkle updates are unaffected — only the local
        // DB write is batched. The remote sync path (from_local_insert=false) is unchanged.
        apply_batch(&mut tx, &messages).await?;
        for msg in &messages {
            let inserted_timestamp: Clock =
                HlcService::insert_timestamp(&mut tx, &msg.timestamp).await?;
            let mut updated_msg = msg.clone();
            updated_msg.group_id =
                std::env::var("GROUP_ID").unwrap_or_else(|_| "my-group".to_string());
            updated_msg.client_id = inserted_timestamp.timestamp.node_id.clone();
            to_send.push(updated_msg);
        }
    } else {
        let existing_messages = compare_messages(&mut tx, messages.clone()).await?;
        for (msg, existing_msg) in &existing_messages {
            let should_apply = match existing_msg {
                None => true,
                Some(existing) => existing.timestamp < msg.timestamp,
            };
            if should_apply {
                apply(&mut tx, msg).await?;
                let inserted_timestamp: Clock =
                    HlcService::insert_timestamp(&mut tx, &msg.timestamp).await?;
                let mut updated_msg = msg.clone();
                updated_msg.group_id =
                    std::env::var("GROUP_ID").unwrap_or_else(|_| "my-group".to_string());
                updated_msg.client_id = inserted_timestamp.timestamp.node_id.clone();
                to_send.push(updated_msg);
            }
        }
    }

    // Phase 2: only after all applies succeeded, send to channel (crdt_messages).
    let sender = get_sender().cloned().ok_or_else(|| {
        log::error!("Failed to send message: sender not available");
        std::io::Error::new(std::io::ErrorKind::Other, "sender not available")
    })?;
    for msg in to_send {
        sender.send(msg)?;
    }

    Ok(())
}

#[allow(warnings)]
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

/// Max queue items to batch into one sync request (reduces round-trips).
const SYNC_QUEUE_BATCH_SIZE: i32 = 20;

pub async fn process_queue(
    endpoints: Vec<PostOpts>,
    mut conn: &mut AsyncPgConnection,
) -> Result<(), Box<dyn std::error::Error>> {
    let sync_timer_ms = 20000;
    let batch_size = std::env::var("SYNC_QUEUE_BATCH_SIZE")
        .ok()
        .and_then(|s| s.parse::<i32>().ok())
        .unwrap_or(SYNC_QUEUE_BATCH_SIZE)
        .max(1);

    loop {
        let size = match QueueService::size("test").await {
            Ok(s) => s,
            Err(_) => continue,
        };
        if size == 0 {
            break;
        }

        let packs = match QueueService::dequeue_batch(&mut conn, "test", batch_size.min(size)).await
        {
            Ok(p) if p.is_empty() => {
                sleep(Duration::from_millis(100)).await;
                continue;
            }
            Ok(p) => p,
            Err(e) => {
                log::error!("Error dequeuing batch from queue: {}", e);
                sleep(Duration::from_millis(sync_timer_ms)).await;
                continue;
            }
        };

        let mut messages: Vec<Value> = Vec::new();
        let mut since: Option<hlc::Timestamp> = None;
        let mut transaction_id: Option<String> = None;
        for (i, pack) in packs.iter().enumerate() {
            let pack_messages = pack
                .get("messages")
                .and_then(|m| m.as_array())
                .cloned()
                .unwrap_or_default();
            messages.extend(pack_messages);
            if i == 0 {
                since = pack
                    .get("since")
                    .and_then(|s| s.as_str())
                    .map(|s_str| hlc::Timestamp::parse(s_str.to_string()));
                transaction_id = pack
                    .get("transaction_id")
                    .and_then(|t| t.as_str())
                    .map(ToString::to_string);
            }
        }

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
            let n = packs.len() as i32;
            log::debug!(
                "Synced batch of {} queue items ({} messages) to all endpoints",
                n,
                messages.len()
            );
            if let Err(e) = QueueService::ack_batch(&mut conn, "test", n).await {
                log::error!("Failed to ack batch: {}", e);
            }
        } else {
            sleep(Duration::from_millis(sync_timer_ms)).await;
        }
    }

    Ok(())
}

pub async fn bg_sync() -> Result<(), Box<dyn std::error::Error>> {
    bg_sync_with_shutdown_check(|| async { false }).await
}

pub async fn bg_sync_with_shutdown_check<F, Fut>(
    shutdown_check: F,
) -> Result<(), Box<dyn std::error::Error>>
where
    F: Fn() -> Fut + Send + Sync,
    Fut: std::future::Future<Output = bool> + Send,
{
    let sync_enabled = std::env::var("SYNC_ENABLED").unwrap_or_else(|_| "false".to_string());
    let mut conn = db::get_async_connection().await;

    if sync_enabled == "false" {
        return Ok(());
    }

    if shutdown_check().await {
        return Ok(());
    }
    log::debug!("Sync Service Initialized");

    // When SYNC_BOOTSTRAP_URL is set, run bootstrap sync once per process (pull all messages from server first).
    if get_bootstrap_opts_from_env().is_some() && !BOOTSTRAP_DONE.load(Ordering::Relaxed) {
        if let Err(e) = bootstrap_sync_once(&mut conn).await {
            log::error!("Bootstrap sync failed: {}", e);
        } else {
            BOOTSTRAP_DONE.store(true, Ordering::Relaxed);
        }
    }

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

    let mut queue_size_after = 0i32;
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
                            log::error!("Error in bg_sync1: {}", e);
                        }
                    }
                }
            }
        } else {
            if let Err(e) = process_queue(endpoints, &mut conn).await {
                log::error!("Error processing queue: {}", e);
            }
        }
        queue_size_after = QueueService::size("test").await.unwrap_or(0);
    }

    // When queue still has work, schedule next run soon so we keep draining (no 60s wait).
    // When queue is empty, use the normal idle interval.
    let sync_timer_ms = std::env::var("SYNC_TIMER_MS")
        .ok()
        .and_then(|timer| timer.parse::<u64>().ok())
        .unwrap_or(60000);
    let sync_busy_interval_ms = std::env::var("SYNC_BUSY_INTERVAL_MS")
        .ok()
        .and_then(|t| t.parse::<u64>().ok())
        .unwrap_or(2000);

    let delay_ms = if queue_size_after > 0 {
        sync_busy_interval_ms
    } else {
        sync_timer_ms
    };
    if queue_size_after > 0 {
        log::debug!(
            "Queue still has {} items, scheduling next sync in {} ms",
            queue_size_after,
            delay_ms
        );
    }
    schedule_next_sync(delay_ms);

    Ok(())
}
fn schedule_next_sync(delay_ms: u64) {
    tokio::spawn(async move {
        sleep(Duration::from_millis(delay_ms)).await;

        // Create the connection inside the spawned task and handle the Result

        if let Err(e) = bg_sync().await {
            log::error!("Error in bg_sync reshedule: {}", e);
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
        info!("Sync is disabled");
        return Ok(());
    }

    let group_id =
        std::env::var("GROUP_ID").unwrap_or_else(|_| "01JBHKXHYSKPP247HZZWHA3JBT".to_string());
    debug!("Using group_id: {}", group_id);

    let transaction_id =
        TransactionService::start_transaction(conn, existing_transaction_id).await?;
    let transaction_id_clone = transaction_id.clone();
    debug!("Started transaction: {}", transaction_id);

    let clock = HlcService::get_clock(conn).await?;
    debug!(
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

    // Capture before clock fields are moved into the json! macro below.
    let client_id = clock.timestamp.node_id.clone();

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
        .map_err(network_error_message)
    {
        Ok(response) => response,
        Err(err_msg) => {
            if let Err(stop_err) = TransactionService::stop_transaction(conn, &transaction_id).await
            {
                log::warn!(
                    "Failed to stop transaction after network error: {}",
                    stop_err
                );
            }
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Network failure during sync: {}", err_msg),
            )));
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

    let is_incomplete = result
        .get("incomplete")
        .map(|v| {
            v.as_bool().unwrap_or(false)
                || v.as_u64().map(|n| n != 0).unwrap_or(false)
                || v.as_i64().map(|n| n != 0).unwrap_or(false)
        })
        .unwrap_or(false);

    if is_incomplete {
        // Server has more messages than fit in the inline response.
        // Fetch chunks oldest-first and apply in rolling batches of APPLY_BATCH_SIZE so
        // we never load the full message set into memory.

        // How many rows the server wrote to crdt_client_messages (included in the response
        // so we can poll readiness before fetching chunks).
        let expected_total = result
            .get("total")
            .and_then(|t| t.as_u64())
            .map(|t| t as usize)
            .unwrap_or(0);

        let chunk_opts = transport_driver::PostOpts {
            url: options.url.clone(),
            username: options.username.clone(),
            password: options.password.clone(),
        };

        if expected_total > 0 {
            log::debug!(
                "Polling server readiness: waiting for {} rows in crdt_client_messages for client_id={}",
                expected_total,
                client_id
            );
            // Convert error to String immediately so no Box<dyn Error> (which is !Send)
            // is held across any subsequent .await points.
            if let Err(err_msg) = HttpTransportDriver
                .poll_chunk_ready(&client_id, expected_total, &chunk_opts)
                .await
                .map_err(|e| e.to_string())
            {
                let _ = HttpTransportDriver
                    .delete_chunks(&client_id, &chunk_opts)
                    .await;
                if let Err(stop_err) =
                    TransactionService::stop_transaction(conn, &transaction_id).await
                {
                    log::warn!(
                        "Failed to stop transaction after readiness poll failure: {}",
                        stop_err
                    );
                }
                return Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!("Server readiness poll failed: {}", err_msg),
                )));
            }
        }

        let chunk_limit: usize = std::env::var("CHUNK_LIMIT")
            .unwrap_or_else(|_| "100".to_string())
            .parse()
            .unwrap_or(100);
        const APPLY_BATCH_SIZE: usize = 2000;
        // How many fetched chunk pages the producer can buffer ahead of the consumer.
        // Bounded channel provides backpressure: producer blocks when the consumer is slow.
        const PREFETCH_BUFFER: usize = 30;
        // Flush pending messages after this many milliseconds even if the batch isn't full yet,
        // so slow or small chunks don't stall the consumer indefinitely.
        const FLUSH_TIMEOUT_MS: u64 = 5;

        let mut total_applied = 0usize;

        // Producer task: fetches chunk pages sequentially and sends them into the channel.
        // Runs concurrently with the consumer so the next HTTP fetch overlaps with the
        // current receive_messages DB write.
        let (chunk_tx, mut chunk_rx) =
            tokio::sync::mpsc::channel::<Result<Vec<Value>, String>>(PREFETCH_BUFFER);
        let producer_client_id = client_id.clone();
        let producer_opts = chunk_opts.clone();
        let producer = tokio::spawn(async move {
            let mut start = 0usize;
            loop {
                let rows = match HttpTransportDriver
                    .fetch_chunk(&producer_client_id, start, chunk_limit, &producer_opts)
                    .await
                    .map_err(|e| e.to_string())
                {
                    Ok(r) => r,
                    Err(e) => {
                        // Signal error to consumer then stop.
                        let _ = chunk_tx.send(Err(e)).await;
                        return;
                    }
                };

                if rows.is_empty() {
                    break; // chunk_tx dropped here → channel closes → consumer loop ends
                }

                let fetched = rows.len();
                let transformed: Vec<Value> = rows
                    .into_iter()
                    .filter_map(|r| r.get("message").cloned())
                    .collect();
                start += fetched;

                // Blocks when channel is full (backpressure): won't fetch ahead more than
                // PREFETCH_BUFFER pages beyond what the consumer has processed.
                if chunk_tx.send(Ok(transformed)).await.is_err() {
                    break; // consumer dropped its receiver (error path), stop fetching
                }
            }
        });

        // Consumer: drains the channel, accumulates into pending.
        // Flushes when EITHER pending reaches APPLY_BATCH_SIZE OR FLUSH_TIMEOUT_MS elapses
        // with at least one message waiting — whichever comes first.
        let mut pending: Vec<Value> = Vec::with_capacity(APPLY_BATCH_SIZE);
        let mut consumer_error: Option<String> = None;
        let flush_timeout = std::time::Duration::from_millis(FLUSH_TIMEOUT_MS);

        'consumer: loop {
            let recv_result = tokio::time::timeout(flush_timeout, chunk_rx.recv()).await;

            match recv_result {
                Ok(Some(result)) => {
                    // Received a chunk page from the producer.
                    let chunk = match result {
                        Ok(c) => c,
                        Err(e) => {
                            consumer_error = Some(format!("Chunk fetch error: {}", e));
                            break 'consumer;
                        }
                    };
                    pending.extend(chunk);

                    // Flush every full APPLY_BATCH_SIZE batch immediately.
                    while pending.len() >= APPLY_BATCH_SIZE {
                        let batch: Vec<Value> = pending.drain(..APPLY_BATCH_SIZE).collect();
                        total_applied += batch.len();
                        log::debug!(
                            "Applying batch of {} messages ({} total so far)",
                            batch.len(),
                            total_applied
                        );
                        if let Err(err_msg) = receive_messages(conn, batch)
                            .await
                            .map_err(error_to_message)
                        {
                            consumer_error = Some(err_msg);
                            break 'consumer;
                        }
                    }
                }
                Ok(None) => {
                    // Channel closed: producer finished (or was dropped on error path).
                    break 'consumer;
                }
                Err(_timeout) => {
                    // FLUSH_TIMEOUT_MS elapsed with no new chunk — flush partial batch now
                    // so in-flight messages aren't held up waiting for the buffer to fill.
                    if !pending.is_empty() {
                        let batch: Vec<Value> = pending.drain(..).collect();
                        total_applied += batch.len();
                        log::debug!(
                            "Timeout flush: applying {} messages ({} total so far)",
                            batch.len(),
                            total_applied
                        );
                        if let Err(err_msg) = receive_messages(conn, batch)
                            .await
                            .map_err(error_to_message)
                        {
                            consumer_error = Some(err_msg);
                            break 'consumer;
                        }
                    }
                    // Loop back and wait for next chunk (or channel close).
                }
            }
        }

        // Wait for producer to finish (it may still be running if consumer broke out early).
        let _ = producer.await;

        if let Some(err_msg) = consumer_error {
            let _ = HttpTransportDriver
                .delete_chunks(&client_id, &chunk_opts)
                .await;
            if let Err(stop_err) = TransactionService::stop_transaction(conn, &transaction_id).await
            {
                log::warn!("Failed to stop transaction after chunk error: {}", stop_err);
            }
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                err_msg,
            )));
        }

        // Flush any tail that did not fill a full APPLY_BATCH_SIZE batch.
        if !pending.is_empty() {
            total_applied += pending.len();
            log::debug!(
                "Applying final batch of {} messages ({} total)",
                pending.len(),
                total_applied
            );
            if let Err(err_msg) = receive_messages(conn, pending)
                .await
                .map_err(error_to_message)
            {
                let _ = HttpTransportDriver
                    .delete_chunks(&client_id, &chunk_opts)
                    .await;
                if let Err(stop_err) =
                    TransactionService::stop_transaction(conn, &transaction_id).await
                {
                    log::warn!(
                        "Failed to stop transaction after receive_messages error: {}",
                        stop_err
                    );
                }
                return Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    err_msg,
                )));
            }
        }

        log::debug!(
            "Chunk transfer done: {} messages applied for client_id={}",
            total_applied,
            client_id
        );
        let _ = HttpTransportDriver
            .delete_chunks(&client_id, &chunk_opts)
            .await;
    } else {
        // Normal path: messages are inline in the initial response
        if let Some(received_messages) = result.get("messages").and_then(|m| m.as_array()) {
            if !received_messages.is_empty() {
                log::debug!("{} updates received.", received_messages.len());
                let receive_result = receive_messages(conn, received_messages.clone())
                    .await
                    .map_err(error_to_message);
                if let Err(err_msg) = receive_result {
                    if let Err(stop_err) =
                        TransactionService::stop_transaction(conn, &transaction_id).await
                    {
                        log::warn!(
                            "Failed to stop transaction after receive_messages error: {}",
                            stop_err
                        );
                    }
                    return Err(Box::new(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        err_msg,
                    )));
                }
                log::debug!(
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
    }
    log::debug!("Result: {:?}", result);

    let result_merkle = result
        .get("merkle")
        .map(|m| {
            if let Some(s) = m.as_str() {
                s.to_string()
            } else {
                serde_json::to_string(m).unwrap_or_default()
            }
        })
        .unwrap_or_default();
    if result_merkle.is_empty() {
        log::debug!("No Merkle tree found in the response");
    }
    let clock = match HlcService::get_clock(conn).await.map_err(error_to_message) {
        Ok(c) => c,
        Err(err_msg) => {
            if let Err(stop_err) = TransactionService::stop_transaction(conn, &transaction_id).await
            {
                log::warn!(
                    "Failed to stop transaction after get_clock error: {}",
                    stop_err
                );
            }
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                err_msg,
            )));
        }
    };
    let merkle_str = serde_json::to_string(&clock.merkle)?;
    let clock_merkle = MerkleTree::deserialize(&merkle_str).unwrap();
    let parsed_merkle = if result_merkle.is_empty() {
        // Create an empty merkle tree if the string is empty
        MerkleTree::new()
    } else {
        match MerkleTree::deserialize(&result_merkle).map_err(|e| {
            consume_error(e);
            "Failed to deserialize merkle tree".to_string()
        }) {
            Ok(tree) => tree,
            Err(_) => {
                if let Err(stop_err) =
                    TransactionService::stop_transaction(conn, &transaction_id).await
                {
                    log::warn!(
                        "Failed to stop transaction after merkle deserialize error: {}",
                        stop_err
                    );
                }
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

        // Do NOT recurse on timeline lag: it repeatedly caused duplicate sends (149 → 270/297)
        // because the recursive sync would refetch and resend the same or overlapping messages.
        // Catch-up is handled by the next bg_sync cycle or by the server's response messages.
        log::debug!(
            "sync_lag_retry: diff present but skipping recursive sync to prevent duplicates (sent {} messages this request)",
            messages.len()
        );
    }
    log::debug!("Sync done - transaction_id:{}", transaction_id_clone);
    TransactionService::stop_transaction(conn, &transaction_id_clone).await?;
    Ok(())
}

/// Normalize a message from either server format (flat: { timestamp, dataset, ... })
/// or wrapped format ({ "message": { timestamp, ... } }) into the inner message value and timestamp.
fn extract_message_and_timestamp(message: &Value) -> Option<(&Value, &str)> {
    let inner = message.get("message").unwrap_or(message);
    let timestamp = inner.get("timestamp").and_then(|t| t.as_str())?;
    Some((inner, timestamp))
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
                    let (inner_message, timestamp_str) = extract_message_and_timestamp(&message)
                        .ok_or_else(|| DieselError::RollbackTransaction)?;

                    HlcService::recv(conn, timestamp_str.to_string())
                        .await
                        .map_err(|e| {
                            log::error!("Failed to receive HLC: {}", e);
                            DieselError::RollbackTransaction
                        })?;

                    let crdt_message: CrdtMessageModel =
                        serde_json::from_value(inner_message.clone())
                            .map_err(|_| DieselError::RollbackTransaction)?;

                    processed_messages.push(crdt_message);
                }

                Ok(processed_messages)
            })
        })
        .await?; // Critical: Must await the transaction

    apply_messages(conn, inner_messages, false).await?;
    Ok(())
}
