use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

#[derive(
    Queryable, Selectable, Serialize, Default, Deserialize, Clone, AsChangeset, Insertable, Debug,
)]
#[diesel(table_name = crate::database::schema::schema::fields)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[serde(default)]
pub struct FieldModel {
    pub id: Option<String>,
    pub label: Option<String>,
    pub name: Option<String>,
    pub field_type: Option<String>,
    pub constraints: Option<JsonValue>,
    pub _default: Option<String>,
    pub reference_to: Option<String>,
    pub version: Option<i32>,
    pub created_by: Option<String>,
    pub updated_by: Option<String>,
    pub deleted_by: Option<String>,
    pub timestamp: Option<String>,
    pub tombstone: Option<i32>,
}
