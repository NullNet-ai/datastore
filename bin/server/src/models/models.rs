use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct SyncEndpoint {
    pub id: i32,
    pub url: String,
    pub auth_username: String,
    pub auth_password: String,
    pub sync_interval: i32,
}

#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct SyncEndpointGroup {
    pub sync_endpoint_id: i32,
    pub group_id: String,
    pub status: String,
}

#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct SyncQueue {
    pub group_id: String,
    pub count: i32,
    pub size: i32,
}

#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct SyncQueueItem {
    pub id: String,
    pub order: i32,
    pub group_id: String,
    pub value: String,
}

#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct SyncTransaction {
    pub id: String,
    pub timestamp: String,
    pub group_id: String,
    pub sync_endpoint_id: i32,
    pub status: String,
    pub expiry: Option<i64>,
}

#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct ClientMessage {
    pub record_id: String,
    pub client_id: String,
    pub message: String,
}

#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct CrdtMessage {
    pub database: Option<String>,
    pub dataset: String,
    pub group_id: String,
    pub timestamp: String,
    pub row: String,
    pub column: String,
    pub client_id: String,
    pub value: String,
    pub operation: String,
    pub hypertable_timestamp: Option<String>,
}

#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct MessagesMerkle {
    pub group_id: String,
    pub merkle: String,
}