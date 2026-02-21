use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(
    Queryable, Selectable, Serialize, Default, Deserialize, Clone, AsChangeset, Insertable, Debug,
)]
#[diesel(table_name = crate::generated::schema::common_sessions)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[serde(default)]
pub struct CommonSessionModel {
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
    pub user_id: Option<String>,
    pub session_token: Option<String>,
    pub route: Option<String>,
    pub session_status: Option<String>,
    pub activity_status: Option<String>,
    pub last_activity_at: Option<String>,
    pub expires_at: Option<String>,
    pub device_details: Option<String>,
    pub ip_address: Option<String>,
    pub portal_application_type: Option<String>,
}
