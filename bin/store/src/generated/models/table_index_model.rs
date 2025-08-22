use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

#[derive(
    Queryable, Selectable, Serialize, Default, Deserialize, Clone, AsChangeset, Insertable, Debug,
)]
#[diesel(table_name = crate::database::schema::schema::table_indexes)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[serde(default)]
pub struct TableIndexModel {
    pub id: Option<String>,
    pub entity_id: Option<String>,
    pub secondary_index: Option<String>,
    pub compound_index: Option<JsonValue>,
    pub created_by: Option<String>,
    pub updated_by: Option<String>,
    pub deleted_by: Option<String>,
    pub timestamp: Option<chrono::NaiveDateTime>,
    pub tombstone: Option<i32>,
}
