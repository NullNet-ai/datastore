use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use chrono::{NaiveDateTime};
#[allow(unused_imports)]
use serde_json::Value;

#[derive(
    Queryable, Selectable, Serialize, Default, Deserialize, Clone, AsChangeset, Insertable, Debug,
)]
#[diesel(table_name = crate::generated::schema::notifications)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[serde(default)]
pub struct NotificationModel {
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
    pub title: Option<String>,
    pub description: Option<String>,
    pub event_timestamp: Option<String>,
    pub link: Option<String>,
    pub icon: Option<String>,
    pub source: Option<String>,
    pub is_pinned: Option<bool>,
    pub recipient_id: Option<String>,
    pub actions: Option<serde_json::Value>,
    pub unread: Option<String>,
    pub low: Option<String>,
    pub priority_level: Option<i32>,
    pub expiry_date: Option<String>,
    pub metadata: Option<String>,
}
