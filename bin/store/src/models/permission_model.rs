use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(
    Queryable, Selectable, Serialize, Default, Deserialize, Clone, AsChangeset, Insertable, Debug,
)]
#[diesel(table_name = crate::schema::schema::permissions)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[serde(default)]
pub struct PermissionModel {
    pub id: Option<String>,
   
    #[serde(default = "default_read")]
    pub read: Option<bool>,
    #[serde(default = "default_write")]
    pub write: Option<bool>,
    #[serde(default = "default_encrypt")]
    pub encrypt: Option<bool>,
    #[serde(default = "default_decrypt")]
    pub decrypt: Option<bool>,
    #[serde(default = "default_required")]
    pub required: Option<bool>,
    #[serde(default = "default_sensitive")]
    pub sensitive: Option<bool>,
    #[serde(default = "default_archive")]
    pub archive: Option<bool>,
    #[serde(default = "default_delete")]
    pub delete: Option<bool>,
    pub version: Option<i32>,
    pub created_by: Option<String>,
    pub updated_by: Option<String>,
    pub deleted_by: Option<String>,
    pub timestamp: Option<String>,
    #[serde(default = "default_tombstone")]
    pub tombstone: Option<i32>,
}

// Default function implementations
fn default_read() -> Option<bool> {
    Some(true)
}

fn default_write() -> Option<bool> {
    Some(false)
}

fn default_encrypt() -> Option<bool> {
    Some(false)
}

fn default_decrypt() -> Option<bool> {
    Some(false)
}

fn default_required() -> Option<bool> {
    Some(false)
}

fn default_sensitive() -> Option<bool> {
    Some(false)
}

fn default_archive() -> Option<bool> {
    Some(false)
}

fn default_delete() -> Option<bool> {
    Some(false)
}

fn default_tombstone() -> Option<i32> {
    Some(0)
}
