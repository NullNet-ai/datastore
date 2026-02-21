use diesel::prelude::*;
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use serde_json::Value;

#[derive(
    Queryable, Selectable, Serialize, Default, Deserialize, Clone, AsChangeset, Insertable, Debug,
)]
#[diesel(table_name = crate::generated::schema::conversations)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[serde(default)]
pub struct ConversationModel {
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
    pub conversation_id: Option<String>,
    pub conversation_replica_name: Option<String>,
    pub last_message_activity: Option<chrono::DateTime<chrono::Utc>>,
    pub tavus_conversation_id: Option<String>,
    pub tavus_conversation_replica_name: Option<String>,
    pub tavus_conversation_start_date: Option<String>,
    pub tavus_conversation_end_date: Option<String>,
    pub tavus_conversation_date_start: Option<chrono::DateTime<chrono::Utc>>,
    pub tavus_conversation_date_end: Option<chrono::DateTime<chrono::Utc>>,
    pub hume_ai_job_id: Option<String>,
    pub hume_ai_sync_status: Option<String>,
    pub user_id: Option<String>,
    pub contact_id: Option<String>,
    pub conversation_topic_id: Option<String>,
    pub session_recording_url: Option<String>,
    pub method: Option<String>,
    pub perception: Option<serde_json::Value>,
    pub duration: Option<String>,
    pub summary: Option<String>,
    pub transcript: Option<serde_json::Value>,
    pub source: Option<String>,
    pub app_version: Option<String>,
    pub ip_address: Option<String>,
    pub is_skip_select_topic: Option<bool>,
    pub cybertipline_report_report_annotations: Option<String>,
    pub topic_name: Option<String>,
}
