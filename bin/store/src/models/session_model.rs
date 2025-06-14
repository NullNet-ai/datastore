use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

#[derive(
    Queryable, Selectable, Serialize, Default, Deserialize, Clone, AsChangeset, Insertable, Debug,
)]
#[diesel(table_name = crate::schema::schema::sessions)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[serde(default)]
pub struct SessionModel {
    pub sid: String,
    pub sess: JsonValue,
    pub expire: NaiveDateTime,
}
