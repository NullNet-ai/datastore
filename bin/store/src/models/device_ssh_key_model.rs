use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel::{Insertable, Queryable};
use serde::{Deserialize, Serialize};

#[derive(
    Queryable, Selectable, Serialize, Default, Deserialize, Clone, AsChangeset, Insertable, Debug,
)]
#[diesel(table_name = crate::schema::schema::device_ssh_keys)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[serde(default)]
pub struct DeviceSshKeyModel {
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

    // SSH key specific fields
    pub id: Option<String>,
    pub timestamp: Option<chrono::NaiveDateTime>,
    pub public_key: Option<String>,
    pub private_key: Option<String>,
    pub passphrase: Option<String>,
    pub device_id: Option<String>,
}
