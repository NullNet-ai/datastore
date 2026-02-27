use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(
    Queryable, Selectable, Serialize, Default, Deserialize, Clone, AsChangeset, Insertable, Debug,
)]
#[diesel(table_name = crate::generated::schema::website_accessibility_reports)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[serde(default)]
pub struct WebsiteAccessibilityReportModel {
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
    pub page_id: Option<String>,
    pub audit_id: Option<String>,
    pub website_id: Option<String>,
    pub published_date: Option<String>,
    pub published_time: Option<String>,
    pub number_of_issues_count: Option<i32>,
    pub fixed_issues_count: Option<i32>,
    pub upstream_issues_count: Option<i32>,
    pub published_issues_count: Option<i32>,
    pub patch_applied_count: Option<i32>,
    pub unresolved_issues_count: Option<i32>,
    pub original_page_url: Option<String>,
    pub accessible_page_url: Option<String>,
    pub published_audit_code: Option<String>,
    pub remaining_issues_count: Option<i32>,
    pub for_review_issues_count: Option<i32>,
    pub website_accessibility_fix_status: Option<String>,
}
