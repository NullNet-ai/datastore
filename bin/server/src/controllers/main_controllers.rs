use crate::controllers::gateway;
use crate::db;
use crate::models::crdt_client_message::{CrdtClientMessage, NewCrdtClientMessage};
use crate::models::crdt_messages::CrdtMessage;
use crate::schema::core::crdt_client_messages::client_id;
use crate::schema::core::crdt_client_messages::dsl::crdt_client_messages;
use crate::schema::core::crdt_client_messages::position;
use crate::schema::core::crdt_client_messages::record_id;
use diesel::dsl::count;

use crate::structs::core::{QueryParams, SyncRequestBody};
use crate::sync::crdt::crdt_service::{self, deserialize_value, get_all_messages_from_timestamp};
use actix_web::error::BlockingError;
use actix_web::Responder;
use actix_web::{http, web, ResponseError};
use diesel::prelude::*;
use diesel::result::Error as DieselError;
use merkle::MerkleTree;
use serde::Serialize;
use std::sync::mpsc::{sync_channel, Receiver, SyncSender};
use std::sync::OnceLock;
use ulid::Ulid;

#[derive(Serialize)]
struct ApiError {
    message: String,
    status: u16,
}
impl From<BlockingError> for ApiError {
    fn from(error: BlockingError) -> Self {
        ApiError {
            status: error.status_code().as_u16(),
            message: format!("Internal server error: {:?}", error),
        }
    }
}
#[allow(warnings)]
impl ApiError {
    fn new(status: http::StatusCode, message: impl Into<String>) -> Self {
        Self {
            status: status.as_u16(),
            message: message.into(),
        }
    }
}
impl From<DieselError> for ApiError {
    fn from(error: DieselError) -> Self {
        let status_code = match error {
            DieselError::NotFound => http::StatusCode::NOT_FOUND,
            DieselError::DatabaseError(_, _) => http::StatusCode::BAD_REQUEST,
            DieselError::DeserializationError(_) => http::StatusCode::UNPROCESSABLE_ENTITY,
            DieselError::SerializationError(_) => http::StatusCode::INTERNAL_SERVER_ERROR,
            DieselError::RollbackTransaction => http::StatusCode::INTERNAL_SERVER_ERROR,
            DieselError::AlreadyInTransaction => http::StatusCode::INTERNAL_SERVER_ERROR,
            _ => http::StatusCode::INTERNAL_SERVER_ERROR,
        };
        ApiError {
            status: status_code.as_u16(),
            message: format!("Database error: {}", error),
        }
    }
}

struct BgInsertTask {
    messages: Vec<NewCrdtClientMessage>,
}

static BG_INSERT_SENDER: OnceLock<SyncSender<BgInsertTask>> = OnceLock::new();

pub fn init_bg_insert_worker() {
    BG_INSERT_SENDER.get_or_init(|| {
        let capacity = std::env::var("INSERT_QUEUE_CAPACITY")
            .ok()
            .and_then(|v| v.parse::<usize>().ok())
            .unwrap_or(100);
        let (tx, rx): (SyncSender<BgInsertTask>, Receiver<BgInsertTask>) = sync_channel(capacity);
        std::thread::spawn(move || {
            let mut bg_conn = db::db::get_connection();
            loop {
                match rx.recv() {
                    Ok(task) => {
                        const BATCH_SIZE: usize = 10_000;
                        for chunk in task.messages.chunks(BATCH_SIZE) {
                            let _ = diesel::insert_into(crdt_client_messages)
                                .values(chunk)
                                .on_conflict(record_id)
                                .do_nothing()
                                .execute(&mut bg_conn);
                        }
                    }
                    Err(_) => break,
                }
            }
        });
        tx
    });
}

fn try_enqueue_bg_insert(cid: String, messages: Vec<NewCrdtClientMessage>) -> Result<(), ()> {
    if let Some(sender) = BG_INSERT_SENDER.get() {
        let _ = cid;
        sender.try_send(BgInsertTask { messages }).map_err(|_| ())
    } else {
        Err(())
    }
}

pub async fn delete_chunk(
    pool: web::Data<db::db::DbPool>,
    query: web::Query<QueryParams>,
) -> impl Responder {
    let pool = pool.clone();
    let client_id_param = query.client_id.clone();

    let result = web::block(move || {
        let mut conn = pool.get().map_err(|e| ApiError {
            status: http::StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
            message: format!("Failed to get DB connection: {}", e),
        })?;

        // Delete all messages for the specified client_id
        diesel::delete(crdt_client_messages)
            .filter(client_id.eq(client_id_param))
            .execute(&mut conn)
            .map_err(|e| ApiError::from(e))?;

        Ok::<_, ApiError>(())
    })
    .await;

    match result {
        Ok(Ok(())) => {
            let response = serde_json::json!({
                "status": "ok"
            });

            actix_web::HttpResponse::Ok().json(response)
        }
        Ok(Err(err)) => {
            // Handle database or processing errors
            let response = serde_json::json!({
                "status": "error",
                "message": err.message
            });

            actix_web::HttpResponse::InternalServerError().json(response)
        }
        Err(err) => {
            // Handle blocking operation errors
            let api_err = ApiError::from(err);
            let response = serde_json::json!({
                "status": "error",
                "message": api_err.message
            });

            actix_web::HttpResponse::InternalServerError().json(response)
        }
    }
}

/// Returns how many rows are currently stored in `crdt_client_messages` for a given client.
/// The client polls this until `count >= total` (the total sent in the `incomplete` response)
/// before starting to fetch chunks.
pub async fn get_chunk_status(
    pool: web::Data<db::db::DbPool>,
    query: web::Query<QueryParams>,
) -> impl Responder {
    let pool = pool.clone();
    let client_id_param = query.client_id.clone();

    let result = web::block(move || {
        let mut conn = pool.get().map_err(|e| ApiError {
            status: http::StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
            message: format!("Failed to get DB connection: {}", e),
        })?;

        let row_count: i64 = crdt_client_messages
            .filter(client_id.eq(&client_id_param))
            .select(count(record_id))
            .first(&mut conn)
            .map_err(ApiError::from)?;

        Ok::<_, ApiError>(row_count as usize)
    })
    .await;

    match result {
        Ok(Ok(row_count)) => actix_web::HttpResponse::Ok().json(serde_json::json!({
            "status": "ok",
            "data": { "count": row_count }
        })),
        Ok(Err(err)) => actix_web::HttpResponse::InternalServerError().json(serde_json::json!({
            "status": "error",
            "message": err.message
        })),
        Err(err) => {
            let api_err = ApiError::from(err);
            actix_web::HttpResponse::InternalServerError().json(serde_json::json!({
                "status": "error",
                "message": api_err.message
            }))
        }
    }
}

pub async fn get_chunk(
    pool: web::Data<db::db::DbPool>,
    query: web::Query<QueryParams>,
) -> impl Responder {
    let pool = pool.clone();
    let client_id_param = query.client_id.clone();
    let start = query.start;
    // Use requested limit, or default when 0 (client often omits it)
    let limit = if query.limit == 0 {
        std::env::var("CHUNK_LIMIT")
            .unwrap_or_else(|_| "100".to_string())
            .parse::<usize>()
            .unwrap_or(100)
    } else {
        query.limit
    };
    let result = web::block(move || {
        let mut conn = pool.get().map_err(|e| ApiError {
            status: http::StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
            message: format!("Failed to get DB connection: {}", e),
        })?;

        // Total count for this client (for size in response)
        let total_size: i64 = crdt_client_messages
            .filter(client_id.eq(&client_id_param))
            .select(count(record_id))
            .first(&mut conn)
            .map_err(ApiError::from)?;

        // Fetch only the requested slice (start..start+limit), ordered by position
        // (BIGSERIAL) which preserves strict insertion order = original timestamp.asc() order.
        let chunk_rows = crdt_client_messages
            .filter(client_id.eq(&client_id_param))
            .order(position.asc())
            .offset(start as i64)
            .limit(limit as i64)
            .load::<CrdtClientMessage>(&mut conn)?;

        // Debug: chunk slice vs total (DEBUG to avoid flooding when client fetches many chunks)
        let first_record_id = chunk_rows.first().map(|r| r.record_id.as_str()).unwrap_or("(none)");
        let last_record_id = chunk_rows.last().map(|r| r.record_id.as_str()).unwrap_or("(none)");
        log::debug!(
            "[GET_CHUNK] client_id={} start={} limit={} total_size={} chunk_len={} first_record_id={} last_record_id={}",
            client_id_param,
            start,
            limit,
            total_size,
            chunk_rows.len(),
            first_record_id,
            last_record_id
        );

        let parsed_messages: Vec<serde_json::Value> = chunk_rows
            .into_iter()
            .filter_map(|msg| {
                match serde_json::from_str::<serde_json::Value>(&msg.message) {
                    Ok(parsed_json) => {
                        let result = serde_json::json!({
                            "record_id": msg.record_id,
                            "client_id": msg.client_id,
                            "message": parsed_json
                        });
                        Some(result)
                    }
                    Err(err) => {
                        eprintln!("Error parsing message: {}", err);
                        None
                    }
                }
            })
            .collect();

        Ok::<_, ApiError>((parsed_messages, total_size as usize))
    })
    .await;

    match result {
        Ok(Ok((messages, size))) => {
            let is_last_chunk = messages.len() < limit as usize || messages.is_empty();
            if is_last_chunk {
                log::info!(
                    "Chunk catch-up done: client_id={} returned {} in final chunk (total buffered: {})",
                    query.client_id,
                    messages.len(),
                    size
                );
            } else {
                log::debug!(
                    "Chunk response: client_id={} returning {} messages in this chunk (total buffered: {})",
                    query.client_id,
                    messages.len(),
                    size
                );
            }
            let response = serde_json::json!({
                "status": "ok",
                "data": {
                    "messages": messages,
                    "start": start,
                    "size": size
                }
            });

            actix_web::HttpResponse::Ok().json(response)
        }
        Ok(Err(err)) => {
            // Handle database or processing errors
            let response = serde_json::json!({
                "status": "error",
                "message": err.message,
                "data": {
                    "messages": [],
                    "start": start,
                    "size": 0
                }
            });

            actix_web::HttpResponse::InternalServerError().json(response)
        }
        Err(err) => {
            // Handle blocking operation errors
            let api_err = ApiError::from(err);
            let response = serde_json::json!({
                "status": "error",
                "message": api_err.message,
                "data": {
                    "messages": [],
                    "start": start,
                    "size": 0
                }
            });

            actix_web::HttpResponse::InternalServerError().json(response)
        }
    }
}
#[allow(warnings)]
pub async fn sync(request: web::Json<SyncRequestBody>) -> impl Responder {
    log::debug!(
        "Received sync request: {}",
        serde_json::to_string(&request).unwrap_or_default()
    );
    let request_data = request.into_inner();
    let req_group_id = request_data.group_id.clone();
    let req_client_id = request_data.client_id.clone();
    let req_messages = request_data.messages.clone();
    let req_client_merkle = request_data.merkle.clone();
    let outgoing_limit = std::env::var("OUTGOING_LIMIT")
        .unwrap_or_else(|_| "1".to_string())
        .parse::<usize>()
        .unwrap_or(1);

    log::info!(
        "[NEW_CLIENT_CATCHUP] Step 0: Sync request received group_id={} client_id={} outgoing_messages={} has_merkle={} merkle_empty_or_placeholder={} outgoing_limit={}",
        req_group_id,
        req_client_id,
        req_messages.len(),
        req_client_merkle.is_some(),
        req_client_merkle.as_ref().map(|m| m.trim().is_empty() || m.trim() == "{}").unwrap_or(true),
        outgoing_limit
    );

    // Log sync attempt
    log::info!(
        "Sync Attempt from {} - {} - {}",
        req_group_id,
        req_client_id,
        req_messages.len()
    );
    let req_group_id_db = req_group_id.clone();
    let req_client_id_db = req_client_id.clone();
    let req_messages_db = req_messages.clone();
    let req_client_merkle_db = req_client_merkle.clone();
    let db_result = web::block(
        move || -> Result<(MerkleTree, Vec<CrdtMessage>), ApiError> {
            let mut conn = db::db::get_connection();
            let trie_result: Result<MerkleTree, DieselError> = conn.transaction(|tx| {
                Ok(crdt_service::add_messages(
                    tx,
                    req_group_id_db.clone(),
                    req_client_id_db.clone(),
                    req_messages_db.clone(),
                ))
            });
            let trie = trie_result.map_err(ApiError::from)?;
            let mut new_messages: Vec<CrdtMessage> = vec![];
            let client_merkle_empty_or_missing = req_client_merkle_db
                .as_ref()
                .map(|m| m.trim().is_empty() || m.trim() == "{}")
                .unwrap_or(true);
            if client_merkle_empty_or_missing {
                match get_all_messages_from_timestamp(
                    &mut conn,
                    "",
                    &req_group_id_db,
                    &req_client_id_db,
                ) {
                    Ok(messages) => {
                        new_messages = messages;
                    }
                    Err(_) => {}
                }
            } else if let Some(merkle) = req_client_merkle_db.clone() {
                if !merkle.trim().is_empty() && merkle.trim() != "{}" {
                    let parsed_client_merkle = match MerkleTree::deserialize(&merkle) {
                        Ok(tree) => tree,
                        Err(_) => {
                            return Err(ApiError::new(
                                http::StatusCode::BAD_REQUEST,
                                "invalid merkle",
                            ))
                        }
                    };
                    let diff_time = trie.find_differences(&parsed_client_merkle);
                    if !diff_time.is_empty() {
                        let (_, server_node, client_node) = &diff_time[0];
                        let min_timestamp_str = if server_node.value <= client_node.value {
                            &server_node.value
                        } else {
                            &client_node.value
                        };
                        if server_node.value != client_node.value {
                            match get_all_messages_from_timestamp(
                                &mut conn,
                                min_timestamp_str,
                                &req_group_id_db,
                                &req_client_id_db,
                            ) {
                                Ok(messages) => {
                                    new_messages = messages;
                                }
                                Err(_) => {}
                            }
                        }
                    }
                }
            }
            Ok((trie, new_messages))
        },
    )
    .await;
    let (trie, mut new_messages) = match db_result {
        Ok(Ok(pair)) => pair,
        Ok(Err(err)) => {
            let response = serde_json::json!({
                "status": "error",
                "message": err.message
            });
            return actix_web::HttpResponse::InternalServerError().json(response);
        }
        Err(err) => {
            let api_err = ApiError::from(err);
            let response = serde_json::json!({
                "status": "error",
                "message": api_err.message
            });
            return actix_web::HttpResponse::InternalServerError().json(response);
        }
    };
    log::info!("[NEW_CLIENT_CATCHUP] Step 1: Server merkle updated (add_messages done)");
    let mut incomplete = 0;

    log::info!(
        "[NEW_CLIENT_CATCHUP] Step 3: Decision new_messages.len()={} outgoing_limit={} will_store_and_return_incomplete={}",
        new_messages.len(),
        outgoing_limit,
        new_messages.len() >= outgoing_limit
    );

    if new_messages.len() >= outgoing_limit {
        log::info!(
            "[NEW_CLIENT_CATCHUP] Step 4: Storing {} messages in crdt_client_messages for client_id={} (then returning incomplete=1)",
            new_messages.len(),
            req_client_id
        );
        log::info!(
            "Sync response: client_id={} has {} messages (outgoing_limit={}), storing in crdt_client_messages and returning incomplete=1",
            req_client_id,
            new_messages.len(),
            outgoing_limit
        );
        // Serialize all messages to CrdtClientMessage structs in memory first (fast — no DB yet).
        // Deserialize value on the server (using existing deserialize_value) so the client
        // receives clean JSON (e.g. value: 0, value: "2026-02-22") instead of serialized form.
        let mut client_messages: Vec<NewCrdtClientMessage> = Vec::with_capacity(new_messages.len());
        for message in &new_messages {
            let mut message_json: serde_json::Value =
                serde_json::to_value(message).unwrap_or(serde_json::json!({}));
            match deserialize_value(&message.value) {
                Ok(deserialized_value) => {
                    if let Some(obj) = message_json.as_object_mut() {
                        obj["value"] = deserialized_value;
                    }
                }
                Err(err) => {
                    log::error!(
                        "Error deserializing value for message with timestamp {}: {}",
                        message.timestamp,
                        err
                    );
                }
            };
            client_messages.push(NewCrdtClientMessage {
                record_id: Ulid::new().to_string(),
                client_id: req_client_id.clone(),
                message: serde_json::to_string(&message_json).unwrap_or_else(|e| {
                    log::error!("Error serializing message: {}", e);
                    "{}".to_string()
                }),
            });
        }

        let stored_total = client_messages.len();

        init_bg_insert_worker();
        match try_enqueue_bg_insert(req_client_id.clone(), client_messages) {
            Ok(()) => {}
            Err(()) => {
                let response = serde_json::json!({
                    "status": "error",
                    "message": "server_busy"
                });
                return actix_web::HttpResponse::ServiceUnavailable().json(response);
            }
        }

        incomplete = 1;
        log::info!(
            "Sync response: client_id={} returning incomplete=1, total={} immediately (BG inserts in progress)",
            req_client_id,
            stored_total
        );
        let response = serde_json::json!({
            "status": "ok",
            "data": {
                "incomplete": incomplete,
                "total": stored_total,
                "messages": [],
                "merkle": trie
            }
        });

        return actix_web::HttpResponse::Ok().json(response);
    }

    log::info!(
        "Sync response: client_id={} returning {} messages in body (below outgoing_limit), incomplete={}",
        req_client_id,
        new_messages.len(),
        incomplete
    );
    if !req_messages.is_empty() {
        gateway::broadcast_notification(serde_json::json!({
            "type": "notice",
            "timestamp": chrono::Utc::now().to_rfc3339(),
            "group_id": req_group_id,
            "client_id": req_client_id
        }));
    }

    let response = serde_json::json!({
        "status": "ok",
        "data": {
            "messages": new_messages,
            "merkle": trie
        }
    });
    return actix_web::HttpResponse::Ok().json(response);
}
