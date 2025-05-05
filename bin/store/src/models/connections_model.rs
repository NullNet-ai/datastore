use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use ipnetwork::IpNetwork;

#[derive(
    Queryable, Selectable, Serialize, Default, Deserialize, Clone, AsChangeset, Insertable, Debug
)]
#[diesel(table_name = crate::schema::schema::connections)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[serde(default)]
pub struct ConnectionModel {
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


    pub id: String,
    pub timestamp: chrono::NaiveDateTime,
    pub interface_name: Option<String>,
    pub hypertable_timestamp: Option<String>,
    pub total_packet: Option<i32>,
    pub total_bytes: Option<i32>,
    pub device_id: Option<String>,
    pub protocol: Option<String>,
    pub source_ip: Option<IpNetwork>,
    pub destination_ip: Option<IpNetwork>,
    pub remote_ip: Option<IpNetwork>,
    pub source_port: Option<i32>,
    pub destination_port: Option<i32>,
   
}
