use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(
    Queryable, Selectable, Serialize, Default, Deserialize, Clone, AsChangeset, Insertable, Debug,
)]
#[diesel(table_name = crate::schema::schema::accounts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[serde(default)]
pub struct AccountModel {
    // System fields
    pub id: Option<String>,
    pub categories: Option<Vec<String>>,
    pub code: Option<String>,
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
    pub timestamp: Option<String>,
    pub tags: Option<Vec<String>>,

    // Specific fields
    pub account_id: Option<String>,
    pub account_secret: Option<String>,
    pub account_status: Option<String>,
    pub is_new_user: Option<bool>,
}
