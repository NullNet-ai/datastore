use diesel::prelude::*;
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use serde_json::Value;

#[derive(
    Queryable, Selectable, Serialize, Default, Deserialize, Clone, AsChangeset, Insertable, Debug,
)]
#[diesel(table_name = crate::generated::schema::system_concurrency_snapshots)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[serde(default)]
pub struct SystemConcurrencySnapshotModel {
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
    pub snapshot_date: Option<String>,
    pub snapshot_time: Option<String>,
    pub per_instance_concurrency: Option<serde_json::Value>,
    pub total_active_scans: Option<i32>,
    pub total_active_instances: Option<i32>,
    pub system_capacity: Option<i32>,
    pub average_scan_duration_ms: Option<i32>,
    pub system_load_factor: Option<String>,
    pub pending_scans_count: Option<i32>,
    pub queue_wait_time_ms: Option<i32>,
    pub memory_usage_mb: Option<i32>,
    pub cpu_usage_percent: Option<String>,
    pub error_rate_percent: Option<String>,
    pub collection_method: Option<String>,
    pub notes: Option<String>,
}
