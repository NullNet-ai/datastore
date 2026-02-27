use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(
    Queryable, Selectable, Serialize, Default, Deserialize, Clone, AsChangeset, Insertable, Debug,
)]
#[diesel(table_name = crate::generated::schema::page_accessibility_outcomes)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[serde(default)]
pub struct PageAccessibilityOutcomeModel {
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
    pub page_id: Option<String>,
    pub audit_id_initial: Option<String>,
    pub audit_id_latest: Option<String>,
    pub upstream_accessibility_report_id: Option<String>,
    pub latest_accessibility_report_id: Option<String>,
    pub wcag_rule_id: Option<String>,
    pub wcag_snippet_path: Option<String>,
    pub position: Option<String>,
    pub css_selector: Option<String>,
    pub original_result: Option<String>,
    pub remediated_result: Option<String>,
    pub patch_type: Option<String>,
    pub patch_id: Option<String>,
    pub patch_success: Option<bool>,
    pub fix_type: Option<String>,
    pub wcag_requirement_id: Option<String>,
    pub title: Option<String>,
    pub level: Option<String>,
    pub wcag_snippet: Option<String>,
    pub is_new_issue: Option<bool>,
}
