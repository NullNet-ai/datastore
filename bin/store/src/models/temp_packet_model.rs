use crate::schema::common_defaults::default_sensitivity_level;
use diesel::prelude::*;
use ipnetwork::IpNetwork;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[allow(warnings)]
fn is_default<T: Default + PartialEq>(t: &T) -> bool {
    t == &T::default()
}
#[derive(
    Queryable, Selectable, Serialize, Default, Deserialize, Clone, AsChangeset, Insertable, Debug,
)]
#[diesel(table_name = crate::schema::schema::temp_packets)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[serde(default)]
pub struct TempPacketModel {
    pub tombstone: Option<i32>,
    pub status: Option<String>,
    pub previous_status: Option<String>,
    pub version: Option<i32>,
    pub created_date: Option<String>,
    pub created_time: Option<String>,
    pub updated_date: Option<String>,
    pub updated_time: Option<String>,
    pub organization_id: Option<String>,
    pub created_by: Option<String>,
    pub updated_by: Option<String>,
    pub deleted_by: Option<String>,
    pub requested_by: Option<String>,
    pub tags: Option<Vec<String>>,
    pub categories: Option<Vec<String>>,
    pub code: Option<String>,
    pub id: Option<String>,
    pub timestamp: Option<chrono::NaiveDateTime>,
    #[serde(default = "default_sensitivity_level")]
    pub sensitivity_level: Option<i32>,

    pub hypertable_timestamp: Option<String>,
    pub interface_name: Option<String>,
    pub device_id: Option<Uuid>,
    pub source_mac: Option<String>,
    pub destination_mac: Option<String>,
    pub ether_type: Option<String>,
    pub protocol: Option<String>,
    pub total_length: Option<i32>,
    pub source_ip: Option<IpNetwork>,
    pub destination_ip: Option<IpNetwork>,
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
