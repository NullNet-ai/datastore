use crate::message_stream::token_bucket::TokenBucket;
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use log::{info, warn};
use serde::{Deserialize, Serialize};
use socketioxide::extract::{Data, SocketRef};
use socketioxide::SocketIo;
use std::collections::HashMap;
use std::env;
use std::sync::{Arc, Mutex};

// JWT Claims structure
#[derive(Debug, Serialize, Deserialize)]
struct Account {
    organization_id: String,
    account_id: String,
    organization_account_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    account: Account,
    exp: usize,
    iat: usize,
}

// Client data structure for socket extensions
#[derive(Debug, Clone)]
pub struct ClientData {
    pub client_id: String,
    pub organization_id: String,
}

// Organization client data structure
#[derive(Debug, Clone)]
#[allow(warnings)]
pub struct OrganizationClients {
    pub client_ids: Vec<String>,
    pub channels: Vec<String>,
}

// Global authenticated clients registry - organized by organization_id (user_id)
lazy_static::lazy_static! {
    static ref AUTHENTICATED_CLIENTS: Arc<Mutex<HashMap<String, OrganizationClients>>> = Arc::new(Mutex::new(HashMap::new()));
    static ref TOKEN_BUCKETS: Arc<Mutex<HashMap<String, Arc<TokenBucket>>>> = Arc::new(Mutex::new(HashMap::new()));
}

// Register a client for an organization
fn register_client(organization_id: String, client_id: String) {
    let mut clients = AUTHENTICATED_CLIENTS.lock().unwrap();

    let org_clients =
        clients
            .entry(organization_id.clone())
            .or_insert_with(|| OrganizationClients {
                client_ids: Vec::new(),
                channels: Vec::new(),
            });

    // Add client_id if not already present
    if !org_clients.client_ids.contains(&client_id) {
        org_clients.client_ids.push(client_id.clone());
    }

    info!(
        "Registered client {} for organization {}",
        client_id, organization_id
    );
}
#[allow(warnings)]
pub fn register_token_bucket(channel_name: &str, capacity: usize) -> Arc<TokenBucket> {
    let mut buckets = TOKEN_BUCKETS.lock().unwrap();

    if let Some(existing_bucket) = buckets.get(channel_name) {
        return existing_bucket.clone();
    }

    let new_bucket = TokenBucket::new(channel_name, capacity);
    buckets.insert(channel_name.to_string(), new_bucket.clone());

    info!("Registered TokenBucket: {}", channel_name);
    new_bucket
}

// Get a TokenBucket from the global registry
pub fn get_token_bucket(channel_name: &str) -> Option<Arc<TokenBucket>> {
    let buckets = TOKEN_BUCKETS.lock().unwrap();
    buckets.get(channel_name).cloned()
}
#[allow(warnings)]
// Remove a TokenBucket from the global registry
pub fn unregister_token_bucket(channel_name: &str) -> bool {
    let mut buckets = TOKEN_BUCKETS.lock().unwrap();
    let removed = buckets.remove(channel_name).is_some();

    if removed {
        info!("Unregistered TokenBucket: {}", channel_name);
    }

    removed
}
#[allow(warnings)]
// Get all TokenBucket IDs
pub fn get_all_token_bucket_ids() -> Vec<String> {
    let buckets = TOKEN_BUCKETS.lock().unwrap();
    buckets.keys().cloned().collect()
}
#[allow(warnings)]
// Get count of registered TokenBuckets
pub fn get_token_bucket_count() -> usize {
    let buckets = TOKEN_BUCKETS.lock().unwrap();
    buckets.len()
}

// Remove a client from an organization
fn remove_client(organization_id: &str, client_id: &str) {
    let mut clients = AUTHENTICATED_CLIENTS.lock().unwrap();

    if let Some(org_clients) = clients.get_mut(organization_id) {
        org_clients.client_ids.retain(|id| id != client_id);

        // If no clients left, remove the organization entry
        if org_clients.client_ids.is_empty() {
            clients.remove(organization_id);
        }
    }

    info!(
        "Removed client {} from organization {}",
        client_id, organization_id
    );
}
#[allow(warnings)]
// Add channel to organization
fn add_channel_to_organization(organization_id: &str, channel: &str) {
    let mut clients = AUTHENTICATED_CLIENTS.lock().unwrap();

    if let Some(org_clients) = clients.get_mut(organization_id) {
        if !org_clients.channels.contains(&channel.to_string()) {
            org_clients.channels.push(channel.to_string());
        }
    }
}
#[allow(warnings)]
// Get organization clients data
pub fn get_organization_clients(organization_id: &str) -> Option<OrganizationClients> {
    let clients = AUTHENTICATED_CLIENTS.lock().unwrap();
    clients.get(organization_id).cloned()
}
#[allow(warnings)]
// Get all client IDs for an organization
pub fn get_organization_client_ids(organization_id: &str) -> Vec<String> {
    let clients = AUTHENTICATED_CLIENTS.lock().unwrap();
    if let Some(org_clients) = clients.get(organization_id) {
        return org_clients.client_ids.clone();
    }
    Vec::new()
}
#[allow(warnings)]
// Get all channels for an organization
pub fn get_organization_channels(organization_id: &str) -> Vec<String> {
    let clients = AUTHENTICATED_CLIENTS.lock().unwrap();
    if let Some(org_clients) = clients.get(organization_id) {
        return org_clients.channels.clone();
    }
    Vec::new()
}
#[allow(warnings)]
// Send notification using socketioxide's broadcast functionality
pub fn broadcast_to_organization(
    io: &SocketIo,
    organization_id: &str,
    notification: serde_json::Value,
) {
    // Use socketioxide's room functionality to broadcast to all clients in an organization
    io.to(format!("org_{}", organization_id))
        .emit("notification", notification)
        .ok();
}
#[allow(warnings)]
// Send notification to specific channel in organization
pub fn broadcast_to_channel(
    io: &SocketIo,
    organization_id: &str,
    channel: &str,
    notification: serde_json::Value,
) {
    // Use socketioxide's room functionality to broadcast to specific channel
    io.to(format!("org_{}_{}", organization_id, channel))
        .emit("notification", notification)
        .ok();
}

// Verify JWT token and extract organization ID
fn verify_token(token: &str) -> Result<Claims, String> {
    let secret = env::var("JWT_SECRET").unwrap_or_else(|_| "default_secret".to_string());
    let key = DecodingKey::from_secret(secret.as_bytes());
    let mut validation = Validation::new(Algorithm::HS256);
    validation.validate_exp = false;

    match decode::<Claims>(token, &key, &validation) {
        Ok(token_data) => Ok(token_data.claims),
        Err(e) => Err(format!("Token validation error: {}", e)),
    }
}

// Socket.io handler
pub fn create_socket_io() -> (socketioxide::layer::SocketIoLayer, SocketIo) {
    let (layer, io) = SocketIo::new_layer();

    // Register handlers for the default namespace with connection-time auth
    io.ns(
        "/",
        |socket: SocketRef, Data(auth_data): Data<serde_json::Value>| {
            info!("Socket.IO connection attempt: {}", socket.id);

            // Extract and validate token during connection
            let token = auth_data.get("token").and_then(|t| t.as_str());

            if let Some(token) = token {
                match verify_token(token) {
                    Ok(claims) => {
                        let organization_id = claims.account.organization_id;

                        // Generate a client ID if not provided in auth
                        let client_id = auth_data
                            .get("client_id")
                            .and_then(|c| c.as_str())
                            .map(|s| s.to_string())
                            .unwrap_or_else(|| socket.id.to_string());

                        // Store client data in socket extensions immediately
                        socket.extensions.insert(ClientData {
                            client_id: client_id.clone(),
                            organization_id: organization_id.clone(),
                        });

                        // Join organization room immediately upon connection
                        if let Err(e) = socket.join(format!("org_{}", organization_id)) {
                            warn!("Failed to join organization room: {}", e);
                            socket.disconnect().ok();
                            return;
                        }

                        // Register client in our tracking system
                        register_client(organization_id.clone(), client_id.clone());

                        info!(
                            "Client {} authenticated and connected for organization {}",
                            client_id, organization_id
                        );

                        // Send authentication success response
                        let response = serde_json::json!({
                            "status": "ok",
                            "event": "connect",
                            "client_id": client_id
                        });
                        socket.emit("auth_success", response).ok();

                        // Set up event handlers for authenticated socket
                        setup_authenticated_handlers(socket);
                    }
                    Err(e) => {
                        warn!("Authentication failed during connection: {}", e);
                        let response = serde_json::json!({
                            "status": "error",
                            "event": "connect",
                            "message": "Invalid token"
                        });
                        socket.emit("auth_error", response).ok();
                        socket.disconnect().ok();
                        return;
                    }
                }
            } else {
                warn!("Connection rejected: No token provided");
                let response = serde_json::json!({
                    "status": "error",
                    "event": "connect",
                    "message": "No token provided"
                });
                socket.emit("auth_error", response).ok();
                socket.disconnect().ok();
                return;
            }
        },
    );

    (layer, io)
}

fn setup_authenticated_handlers(socket: SocketRef) {
    // Handle updateHighWaterMark event
    socket.on(
        "updateHighWaterMark",
        |socket: SocketRef, Data(data): Data<serde_json::Value>| async move {
            // Client is guaranteed to be authenticated at this point
            let channel_name = data.get("channel_name").and_then(|c| c.as_str());
            let highwatermark = data.get("highwatermark");

            if let (Some(channel), Some(mark)) = (channel_name, highwatermark) {
                // Add channel to organization's channels
                //get the channel with the channel_name
                let bucket = get_token_bucket(channel);
                if let Some(bucket) = bucket {
                    bucket.set_tokens(mark.as_u64().unwrap() as usize).await;
                }

                info!(
                    "Updated high water mark for channel {} to {}",
                    channel, mark
                );

                let response = serde_json::json!({
                    "status": "ok",
                    "event": "updateHighWaterMark",
                    "channel": channel
                });
                socket.emit("updateHighWaterMark", response).ok();
            } else {
                let response = serde_json::json!({
                    "status": "error",
                    "event": "updateHighWaterMark",
                    "message": "Missing channel_name or highwatermark"
                });
                socket.emit("updateHighWaterMark", response).ok();
            }
        },
    );
    // Handle getCurrentHighWaterMark event
    socket.on(
        "getCurrentHighWaterMark",
        |socket: SocketRef, Data(data): Data<serde_json::Value>| async move {
            let channel_name = data.get("channel_name").and_then(|c| c.as_str());

            if let Some(channel) = channel_name {
                // Retrieve the current high water mark for this channel from token bucket
                let current_highwatermark = if let Some(bucket) = get_token_bucket(channel) {
                    bucket.get_high_watermark().await
                } else {
                    0 // Default if bucket doesn't exist
                };
                let response;

                if current_highwatermark == 0 {
                    response = serde_json::json!({
                        "status": "error",
                        "event": "getCurrentHighWaterMark",
                        "message": "Channel not found"
                    });
                } else {
                    response = serde_json::json!({
                        "status": "ok",
                        "event": "getCurrentHighWaterMark",
                        "channel": channel,
                        "highwatermark": current_highwatermark
                    });
                }

                socket.emit("currentHighWaterMark", response).ok();
            } else {
                let response = serde_json::json!({
                    "status": "error",
                    "event": "getCurrentHighWaterMark",
                    "message": "Missing channel_name"
                });
                socket.emit("getCurrentHighWaterMark", response).ok();
            }
        },
    );

    // Handle explicit disconnect event
    socket.on("disconnect", |socket: SocketRef| {
        handle_client_disconnect(&socket);
    });

    // Handle automatic disconnection
    socket.on_disconnect(|socket: SocketRef| {
        handle_client_disconnect(&socket);
    });
}

// Helper function to handle client disconnection cleanup
fn handle_client_disconnect(socket: &SocketRef) {
    let client_data = socket.extensions.get::<ClientData>();

    if let Some(client_data) = client_data {
        remove_client(&client_data.organization_id, &client_data.client_id);
        info!(
            "Client {} disconnected from organization {}",
            client_data.client_id, client_data.organization_id
        );
    } else {
        info!("Unauthenticated socket {} disconnected", socket.id);
    }
}
