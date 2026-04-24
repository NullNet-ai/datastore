use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use chrono::{NaiveDateTime};

#[derive(
    Queryable, Selectable, Serialize, Default, Deserialize, Clone, AsChangeset, Insertable, Debug,
)]
#[diesel(table_name = crate::generated::schema::temp_connections)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[serde(default)]
pub struct TempConnectionModel {
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
    pub timestamp: Option<NaiveDateTime>,
    pub tags: Option<Vec<String>>,
    pub categories: Option<Vec<String>>,
    pub code: Option<String>,
    pub id: Option<String>,
    pub sensitivity_level: Option<i32>,
    pub sync_status: Option<String>,
    pub is_batch: Option<bool>,
    pub image_url: Option<String>,
    pub hypertable_timestamp: Option<String>,
    pub interface_name: Option<String>,
    pub total_packet: Option<i32>,
    pub total_byte: Option<i32>,
    pub device_id: Option<String>,
    pub protocol: Option<String>,
    pub source_ip: Option<String>,
    pub destination_ip: Option<String>,
    pub source_port: Option<i32>,
    pub destination_port: Option<i32>,
    pub remote_ip: Option<String>,
}
