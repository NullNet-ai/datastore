use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;

#[derive(
    Queryable, Selectable, Serialize, Default, Deserialize, Clone, AsChangeset, Insertable, Debug,
)]
#[diesel(table_name = crate::schema::schema::record_permissions)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[serde(default)]
pub struct RecordPermissionModel {
    pub id: Option<String>,
    pub record_id: Option<String>,
    pub record_entity: Option<String>,
    pub permission_id: Option<String>,
    pub created_by: Option<String>,
    pub updated_by: Option<String>,
    pub deleted_by: Option<String>,
    pub timestamp: Option<NaiveDateTime>,
    pub tombstone: Option<i32>,
}