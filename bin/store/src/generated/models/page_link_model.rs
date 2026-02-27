use diesel::prelude::*;
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use serde_json::Value;

#[derive(
    Queryable, Selectable, Serialize, Default, Deserialize, Clone, AsChangeset, Insertable, Debug,
)]
#[diesel(table_name = crate::generated::schema::page_links)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[serde(default)]
pub struct PageLinkModel {
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
    pub link: Option<String>,
    pub link_type: Option<String>,
    pub link_status: Option<String>,
    pub has_redirect: Option<bool>,
    pub redirect_count: Option<i32>,
    pub original_url: Option<String>,
    pub final_url: Option<String>,
    pub redirect_chain: Option<serde_json::Value>,
    pub is_document: Option<bool>,
    pub document_type: Option<String>,
    pub content_type: Option<String>,
    pub status_code: Option<i32>,
    pub response_headers: Option<serde_json::Value>,
    pub error_category: Option<String>,
    pub error_message: Option<String>,
}
