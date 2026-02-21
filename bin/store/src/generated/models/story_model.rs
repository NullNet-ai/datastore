use diesel::prelude::*;
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use serde_json::Value;

#[derive(
    Queryable, Selectable, Serialize, Default, Deserialize, Clone, AsChangeset, Insertable, Debug,
)]
#[diesel(table_name = crate::generated::schema::stories)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[serde(default)]
pub struct StoryModel {
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
    pub name: Option<String>,
    pub course_id: Option<String>,
    pub order: Option<i32>,
    pub description: Option<String>,
    pub story_identifier: Option<String>,
    pub bundle_file_name: Option<String>,
    pub allowed_grades: Option<serde_json::Value>,
    pub birthdate_cutoff: Option<String>,
    pub must_be_born_on_or_after_birthdate_cutoff: Option<String>,
    pub must_be_born_before_birthdate_cutoff: Option<String>,
    pub allowed_ages: Option<serde_json::Value>,
}
