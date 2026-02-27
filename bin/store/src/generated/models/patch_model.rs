use diesel::prelude::*;
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use serde_json::Value;

#[derive(
    Queryable, Selectable, Serialize, Default, Deserialize, Clone, AsChangeset, Insertable, Debug,
)]
#[diesel(table_name = crate::generated::schema::patches)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[serde(default)]
pub struct PatchModel {
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
    pub accessibility_report_id: Option<String>,
    pub patch_status: Option<String>,
    pub patch_start_date: Option<String>,
    pub patch_start_time: Option<String>,
    pub patch_end_date: Option<String>,
    pub patch_end_time: Option<String>,
    pub approval_id: Option<String>,
    pub selector: Option<String>,
    pub replacement: Option<String>,
    pub publication_id: Option<String>,
    pub fix_type: Option<String>,
    pub operation: Option<String>,
    pub parameters: Option<serde_json::Value>,
    pub fix_recipe_item_order: Option<i32>,
    pub wcag_rule_id: Option<String>,
}
