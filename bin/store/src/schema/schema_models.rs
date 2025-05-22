use crate::schema::schema::crdt_merkles;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use crate::schema::schema::crdt_messages;
use uuid::Uuid;
use crate::schema::schema::queue_items;
use crate::schema::schema::queues;
use crate::schema::schema::sync_endpoints;
use crate::schema::schema::transactions;


#[derive(Insertable, Queryable, Serialize, Deserialize, Debug, Clone)]
#[diesel(table_name = crdt_merkles)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Merkle {
    pub group_id: String,
    pub timestamp: String,
    pub merkle: String,
}

#[derive(
    Debug, Insertable, Hash, Eq, PartialEq, Default, Clone, Queryable, Serialize, Deserialize,
)]
#[diesel(table_name = crdt_messages)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[serde(default)]
pub struct CrdtMessage {
    pub database: Option<String>,
    pub dataset: String,
    pub group_id: String,
    pub timestamp: String,
    #[serde(flatten, skip_serializing_if = "String::is_empty")]
    pub row: String,
    pub column: String,
    pub client_id: String,
    pub value: String,
    pub operation: String,
    pub hypertable_timestamp: Option<String>,
}


#[derive(Queryable, Selectable, Insertable, Serialize, Deserialize, Clone)]
#[diesel(table_name = crate::schema::schema::items)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Item {
    pub tombstone: i32,
    pub status: String,
    pub previous_status: Option<String>,
    pub version: i32,
    pub created_date: Option<String>,
    pub created_time: Option<String>,
    pub updated_date: Option<String>,
    pub updated_time: Option<String>,
    pub organization_id: Option<String>,
    pub created_by: Option<String>,
    pub updated_by: Option<String>,
    pub deleted_by: Option<String>,
    pub requested_by: Option<String>,
    pub tags: Vec<String>,

    pub id: i32,
    pub name: String,
    pub description: Option<String>,
}

#[derive(
    Queryable, Selectable, Serialize, Default, Deserialize, Clone, AsChangeset, Insertable,
)]
#[diesel(table_name = crate::schema::schema::packets)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[serde(default)]
pub struct Packet {
    pub tombstone: i32,
    pub status: String,
    pub previous_status: Option<String>,
    pub version: i32,
    pub created_date: Option<String>,
    pub created_time: Option<String>,
    pub updated_date: Option<String>,
    pub updated_time: Option<String>,
    pub organization_id: Option<String>,
    pub created_by: Option<String>,
    pub updated_by: Option<String>,
    pub deleted_by: Option<String>,
    pub requested_by: Option<String>,
    pub tags: Vec<String>,

    pub id: String,
    pub timestamp: chrono::NaiveDateTime,
    pub hypertable_timestamp: String,
    pub interface_name: String,
    pub total_length: Option<i32>,
    pub device_id: Option<String>,
    pub source_mac: Option<String>,
    pub destination_mac: Option<String>,
    pub ether_type: Option<String>,
    pub ip_header_length: i32,
    pub payload_length: i32,
    pub protocol: String,
    pub source_ip: String,
    pub destination_ip: String,
    pub source_port: Option<i32>,
    pub destination_port: Option<i32>,
    pub tcp_header_length: Option<i32>,
    pub tcp_sequence_number: Option<i64>,
    pub tcp_acknowledgment_number: Option<i64>,
    pub tcp_data_offset: Option<i32>,
    pub tcp_flags: Option<i32>,
    pub tcp_window_size: Option<i32>,
    pub tcp_urgent_pointer: Option<i32>,
    pub icmp_type: Option<i32>,
    pub icmp_code: Option<i32>,
}


#[derive(
    Queryable, Selectable, Serialize, Default, Deserialize, Clone, AsChangeset, Insertable, Debug,
)]
#[diesel(table_name = queue_items)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[serde(default)]
pub struct QueueItem {
    pub id: String,
    pub order: i32,
    pub queue_id: String,
    pub value: String,
}

#[derive(
    Queryable, Selectable, Serialize, Default, Deserialize, Clone, AsChangeset, Insertable, Debug,
)]
#[diesel(table_name = queues)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[serde(default)]
pub struct Queue {
    pub id: String,
    pub name: String,
    pub size: i32,
    pub count: i32,
}


#[derive(
    Queryable, Selectable, Serialize, Debug, Default, Deserialize, Clone, AsChangeset, Insertable,
)]
#[diesel(table_name = sync_endpoints)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[serde(default)]
pub struct SyncEndpoint {
    pub id: String,
    pub name: String,
    pub url: String,
    pub group_id: String,
    pub username: String,
    pub password: String,
    pub status: String,
}


#[derive(
    Queryable, Selectable, Serialize, Debug, Default, Deserialize, Clone, AsChangeset, Insertable,
)]
#[diesel(table_name = transactions)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[serde(default)]
pub struct Transaction {
    pub id: String,
    pub timestamp: String,
    #[serde(default = "default_status")]
    pub status: String,
    pub expiry: i64,
}

fn default_status() -> String {
    "Active".to_string()
}
