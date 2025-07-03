use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use crate::schema::common_defaults::default_sensitivity_level;


#[derive(
    Queryable, Selectable, Serialize, Default, Deserialize, Clone, AsChangeset, Insertable, Debug,
)]
#[diesel(table_name = crate::schema::schema::external_contacts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[serde(default)]
pub struct ExternalContactModel {
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

    // Specific fields
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>,
}
