use diesel::prelude::*;
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use serde_json::Value;

#[derive(
    Queryable, Selectable, Serialize, Default, Deserialize, Clone, AsChangeset, Insertable, Debug,
)]
#[diesel(table_name = crate::generated::schema::grid_filters)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[serde(default)]
pub struct GridFilterModel {
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
    pub name: Option<String>,
    pub grid_id: Option<String>,
    pub link: Option<String>,
    pub is_current: Option<bool>,
    pub is_default: Option<bool>,
    pub contact_id: Option<String>,
    pub account_organization_id: Option<String>,
    pub entity: Option<String>,
    #[serde(rename = "columns")]
    pub columns_data: Option<serde_json::Value>,
    pub groups: Option<serde_json::Value>,
    pub sorts: Option<serde_json::Value>,
    pub default_sorts: Option<serde_json::Value>,
    pub advance_filters: Option<serde_json::Value>,
    pub group_advance_filters: Option<serde_json::Value>,
    pub filter_groups: Option<serde_json::Value>,
}
