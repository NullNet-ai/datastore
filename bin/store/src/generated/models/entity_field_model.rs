use crate::database::schema::common_defaults::default_sensitivity_level;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(
    Queryable, Selectable, Serialize, Default, Deserialize, Clone, AsChangeset, Insertable, Debug,
)]
#[diesel(table_name = crate::generated::schema::entity_fields)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[serde(default)]
pub struct EntityFieldModel {
    pub id: Option<String>,
    pub entity_id: Option<String>,
    pub field_id: Option<String>,
    pub version: Option<i32>,
    pub schema_version: Option<i32>,
    pub created_by: Option<String>,
    pub updated_by: Option<String>,
    pub deleted_by: Option<String>,
    pub timestamp: Option<String>,
    pub tombstone: Option<i32>,
    #[serde(default = "default_sensitivity_level")]
    pub sensitivity_level: Option<i32>,
    pub is_encryptable: Option<bool>,
}
