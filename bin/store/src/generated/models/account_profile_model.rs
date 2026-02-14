use crate::database::schema::common_defaults::default_sensitivity_level;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(
    Queryable, Selectable, Serialize, Default, Deserialize, Clone, AsChangeset, Insertable, Debug,
)]
#[diesel(table_name = crate::generated::schema::account_profiles)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[serde(default)]
pub struct AccountProfileModel {
    // System fields
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
    pub tags: Option<Vec<String>>,
    pub categories: Option<Vec<String>>,
    pub code: Option<String>,
    pub id: Option<String>,
    pub timestamp: Option<chrono::NaiveDateTime>,
    #[serde(default = "default_sensitivity_level")]
    pub sensitivity_level: Option<i32>,
    pub sync_status: Option<String>,
    pub is_batch: Option<bool>,

    // Specific fields for account_profiles
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>,
    pub account_id: Option<String>,
    pub date_of_birth: Option<chrono::NaiveDateTime>,
    pub middle_name: Option<String>,
    pub auth_preference: Option<String>,
}
