use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

fn is_default<T: Default + PartialEq>(t: &T) -> bool {
    t == &T::default()
}
#[derive(
    Queryable, Selectable, Serialize, Default, Deserialize, Clone, AsChangeset, Insertable
)]
#[diesel(table_name = crate::schema::schema::packets)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[serde(default)]
pub struct Packet {
    pub tombstone: i32,
    pub status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_status: Option<String>,
    pub version: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_by: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted_by: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested_by: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<String>,

    pub id: Uuid,
    pub timestamp: chrono::NaiveDateTime,
    #[serde(skip_serializing_if = "is_default")]
    pub hypertable_timestamp: String,
    pub interface_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_length: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_id: Option<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_mac: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_mac: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ether_type: Option<String>,
    #[serde(skip_serializing_if = "is_default")]
    pub ip_header_length: i32,
    #[serde(skip_serializing_if = "is_default")]
    pub payload_length: i32,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub protocol: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub source_ip: String,
    #[serde(skip_serializing_if = "is_default")]
    pub destination_ip: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_port: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_port: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tcp_header_length: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tcp_sequence_number: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tcp_acknowledgment_number: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tcp_data_offset: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tcp_flags: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tcp_window_size: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tcp_urgent_pointer: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icmp_type: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icmp_code: Option<i32>,
}
