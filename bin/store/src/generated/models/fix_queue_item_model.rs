use diesel::prelude::*;
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use serde_json::Value;

#[derive(
    Queryable, Selectable, Serialize, Default, Deserialize, Clone, AsChangeset, Insertable, Debug,
)]
#[diesel(table_name = crate::generated::schema::fix_queue_items)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[serde(default)]
pub struct FixQueueItemModel {
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
    pub timestamp: Option<chrono::NaiveDateTime>,
    pub tags: Option<Vec<String>>,
    pub categories: Option<Vec<String>>,
    pub code: Option<String>,
    pub id: Option<String>,
    pub sensitivity_level: Option<i32>,
    pub sync_status: Option<String>,
    pub is_batch: Option<bool>,
    pub image_url: Option<String>,
    pub website_id: Option<String>,
    pub audit_id: Option<String>,
    pub page_id: Option<String>,
    pub fix_queue_id: Option<String>,
    pub url: Option<String>,
    pub fix_start_date: Option<String>,
    pub fix_start_time: Option<String>,
    pub fix_end_date: Option<String>,
    pub fix_end_time: Option<String>,
    pub fix_status: Option<String>,
    pub offset: Option<i32>,
    pub unresolved_issues: Option<serde_json::Value>,
    pub fix_processes_id: Option<String>,
    pub page_original_url: Option<String>,
    pub token: Option<String>,
}
