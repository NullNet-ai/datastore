use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Insertable, AsChangeset, Debug, Clone, Serialize, Deserialize)]
#[diesel(table_name = crate::generated::schema::signed_in_activities)]
pub struct SignedInActivityModel {
    pub id: Option<String>,
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
    pub sensitivity_level: Option<i32>,
    pub sync_status: Option<String>,
    pub is_batch: Option<bool>,
    pub account_organization_id: Option<String>,
    pub device_name: Option<String>,
    pub browser_name: Option<String>,
    pub operating_system: Option<String>,
    pub authentication_method: Option<String>,
    pub location: Option<String>,
    pub ip_address: Option<String>,
    pub session_started: Option<NaiveDateTime>,
    pub remark: Option<String>,
    pub session_id: Option<String>,
    pub hypertable_timestamp: Option<String>,
}
