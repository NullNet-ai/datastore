use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel::{Insertable, Queryable};
use ipnetwork::IpNetwork;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(
    Queryable, Selectable, Serialize, Default, Deserialize, Clone, AsChangeset, Insertable, Debug,
)]
#[diesel(table_name = crate::schema::schema::devices)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[serde(default)]
pub struct DeviceModel {
    // System fields
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

    pub id: String,
    pub timestamp: NaiveDateTime,

    pub model: Option<String>,
    pub address_id: Option<String>,
    pub instance_name: Option<String>,
    pub is_connection_established: Option<bool>,
    pub system_id: Option<String>,
    pub device_version: Option<String>,
    pub last_heartbeat: Option<String>,
    pub is_monitoring_enabled: Option<bool>,
    pub is_remote_access_enabled: Option<bool>,
    pub ip_address: Option<IpNetwork>,
    pub device_status: Option<String>,
    pub device_gui_protocol: Option<String>,
}
