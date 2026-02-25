use crate::controllers::gateway;
use crate::db;
use crate::models::crdt_client_message::*;
use crate::models::crdt_messages::CrdtMessage;
use crate::schema::core::crdt_client_messages::client_id;
use crate::schema::core::crdt_client_messages::record_id;
use crate::schema::core::crdt_client_messages::dsl::crdt_client_messages;
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

// ... existing code ...

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

        // Fetch only the requested slice (start..start+limit), not all messages
        let chunk_rows = crdt_client_messages
            .filter(client_id.eq(&client_id_param))
            .order(record_id)
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
    let mut conn = db::db::get_connection();
    let result_trie: Result<MerkleTree, DieselError> = conn.transaction(|tx| {
        Ok(crdt_service::add_messages(
            tx,
            req_group_id.clone(),
            req_client_id.clone(),
            req_messages.clone(),
        ))
    });
    let trie = match result_trie {
        Ok(trie) => trie,
        Err(_) => {
            let response = serde_json::json!({
                "status": "error",
                "message": "Internal server error"
            });
            return actix_web::HttpResponse::InternalServerError().json(response);
        }
    };
    log::info!("[NEW_CLIENT_CATCHUP] Step 1: Server merkle updated (add_messages done)");
    // ! check for release the connection to the pool afterwards
    let mut incomplete = 0;
    let mut new_messages: Vec<CrdtMessage> = vec![];
    let client_merkle_empty_or_missing = req_client_merkle
        .as_ref()
        .map(|m| m.trim().is_empty() || m.trim() == "{}")
        .unwrap_or(true);
    if client_merkle_empty_or_missing {
        // Bootstrap: client has no / empty merkle — send all messages from server so client can catch up
        log::info!(
            "[NEW_CLIENT_CATCHUP] Step 2b: Path BOOTSTRAP (client merkle empty) client_id={}",
            req_client_id
        );
        match get_all_messages_from_timestamp(
            &mut conn,
            "",
            &req_group_id,
            &req_client_id,
        ) {
            Ok(messages) => {
                log::info!(
                    "Bootstrap: sending {} messages to client_id={}",
                    messages.len(),
                    req_client_id
                );
                new_messages = messages;
            }
            Err(err) => {
                log::error!("Bootstrap: error retrieving messages: {:?}", err);
            }
        }
    } else if let Some(merkle) = req_client_merkle.clone() {
        if !merkle.trim().is_empty() && merkle.trim() != "{}" {
            log::info!(
                "[NEW_CLIENT_CATCHUP] Step 2a: Path MERKLE_DIFF (client has non-empty merkle) client_id={}",
                req_client_id
            );
            log::info!(
                "Sync catch-up: client_id={} has non-empty merkle, computing diff",
                req_client_id
            );
            let client_merkle = match req_client_merkle {
                Some(merkle) => merkle,
                None => {
                    log::error!("Client merkle is None despite previous check");
                    let response = serde_json::json!({
                        "status": "error",
                        "message": "Invalid client merkle data"
                    });
                    return actix_web::HttpResponse::BadRequest().json(response);
                }
            };

            let parsed_client_merkle = match MerkleTree::deserialize(&client_merkle) {
                Ok(tree) => tree,
                Err(err) => {
                    log::error!("Failed to deserialize client merkle tree: {:?}", err);
                    let response = serde_json::json!({
                        "status": "error",
                        "message": "Failed to parse client merkle tree"
                    });
                    return actix_web::HttpResponse::BadRequest().json(response);
                }
            };

            // Log both trees and the diff for debugging
            let server_root = trie.root.as_ref().map(|n| format!("hash={} value={}", n.hash, n.value)).unwrap_or_else(|| "None".to_string());
            let client_root = parsed_client_merkle.root.as_ref().map(|n| format!("hash={} value={}", n.hash, n.value)).unwrap_or_else(|| "None".to_string());
            log::info!(
                "[MERKLE_TREES] client_id={} server_root=[{}] client_root=[{}]",
                req_client_id,
                server_root,
                client_root
            );
            let diff_time = trie.find_differences(&parsed_client_merkle);
            for (idx, (path, server_node, client_node)) in diff_time.iter().enumerate() {
                log::info!(
                    "[MERKLE_DIFF] client_id={} diff[{}] path={} server_value={} client_value={} server_hash={} client_hash={}",
                    req_client_id,
                    idx,
                    path,
                    server_node.value,
                    client_node.value,
                    server_node.hash,
                    client_node.hash
                );
            }
            log::info!(
                "Sync catch-up: client_id={} merkle diff size={}",
                req_client_id,
                diff_time.len()
            );

            if !diff_time.is_empty() {
                let (_, server_node, client_node) = &diff_time[0];

                let min_timestamp_str = if server_node.value <= client_node.value {
                    &server_node.value
                } else {
                    &client_node.value
                };
                // Check if both timestamps are equal
                if server_node.value != client_node.value {
                    log::debug!(
                        "Lag detected - Using timestamp: {}, client_id: {}",
                        min_timestamp_str,
                        req_client_id
                    );
                    // Parse the full timestamp for further use if needed
                    // let timestamp = Timestamp::parse(min_timestamp_str.to_string());
                    match get_all_messages_from_timestamp(
                        &mut conn,
                        min_timestamp_str,
                        &req_group_id,
                        &req_client_id,
                    ) {
                        Ok(messages) => {
                            log::debug!("Retrieved {} messages", messages.len());
                            new_messages = messages;
                        }
                        Err(err) => {
                            log::error!("Error retrieving messages: {:?}", err);
                            // Handle the error appropriately
                        }
                    }
                } else {
                    log::debug!("No lag detected for client_id: {}", req_client_id);
                }
                log::debug!(
                    "Server timestamp: {}, Client timestamp: {}, Using: {}",
                    server_node.value,
                    client_node.value,
                    min_timestamp_str
                );
            }
        }
    }

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
        // Store messages in the database instead of sending them.
        // Deserialize value on the server (using existing deserialize_value) so the client
        // receives clean JSON (e.g. value: 0, value: "2026-02-22") instead of serialized form.
        for message in &new_messages {
            //remove double quotes from the message
            let mut message_json: serde_json::Value =
                serde_json::to_value(message).unwrap_or(serde_json::json!({}));

            // Create a new client message record
            match deserialize_value(&message.value) {
                Ok(deserialized_value) => {
                    // Update the value field with the deserialized content
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
                    // Keep the original value
                }
            };

            let client_message = CrdtClientMessage {
                record_id: Ulid::new().to_string(),
                client_id: req_client_id.clone(),
                message: serde_json::to_string(&message_json).unwrap_or_else(|e| {
                    log::error!("Error serializing message: {}", e);
                    "{}".to_string()
                }),
            };

            //debug log whenever you are inserting a new message
            log::debug!(
                "Inserting message with timestamp {} into database",
                message.timestamp
            );

            // Insert into database, ignoring conflicts
            match diesel::insert_into(crdt_client_messages)
                .values(&client_message)
                .on_conflict_do_nothing()
                .execute(&mut conn)
            {
                Ok(_) => {}
                Err(err) => {
                    log::error!("Error storing message in database: {}", err);
                }
            }
        }

        // Set incomplete flag and clear messages to avoid sending them
        incomplete = 1;
        log::info!(
            "Sync response: client_id={} returning incomplete=1, messages=[] (client should fetch via /chunk)",
            req_client_id
        );
        let response = serde_json::json!({
            "status": "ok",
            "data": {
                "incomplete": incomplete,
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
