use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(
    Queryable, Selectable, Serialize, Default, Deserialize, Clone, AsChangeset, Insertable, Debug,
)]
#[diesel(table_name = crate::database::schema::schema::system_config_fields)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[serde(default)]
pub struct SystemConfigFieldModel {
    pub field_id: Option<String>,
    pub is_searchable: Option<bool>,
    pub is_system_field: Option<bool>,
    pub is_encryptable: Option<bool>,
    pub is_allowed_to_return: Option<bool>,
    pub created_by: Option<String>,
    pub updated_by: Option<String>,
    pub deleted_by: Option<String>,
    pub timestamp: Option<NaiveDateTime>,
    pub tombstone: Option<i32>,
}
