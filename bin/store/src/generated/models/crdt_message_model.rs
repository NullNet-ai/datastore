use crate::generated::schema::crdt_messages;
use diesel::prelude::*;
use serde::{Deserialize, Deserializer, Serialize};
use serde_json::Value as JsonValue;

/// Deserialize a JSON value (string, number, null, bool, etc.) into a String.
/// Server sends `value` as e.g. 0.0 (tombstone), null, or "..." so we accept any type and coerce.
fn string_from_any<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let v = JsonValue::deserialize(deserializer)?;
    Ok(match v {
        JsonValue::String(s) => s,
        JsonValue::Number(n) => n.to_string(),
        JsonValue::Null => String::new(),
        JsonValue::Bool(b) => b.to_string(),
        JsonValue::Array(_) | JsonValue::Object(_) => v.to_string(),
    })
}

#[derive(
    Debug, Insertable, Hash, Eq, PartialEq, Default, Clone, Queryable, Serialize, Deserialize,
)]
#[diesel(table_name = crdt_messages)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[serde(default)]
pub struct CrdtMessageModel {
    pub database: Option<String>,
    pub dataset: String,
    pub group_id: String,
    pub timestamp: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub row: String,
    pub column: String,
    pub client_id: String,
    #[serde(deserialize_with = "string_from_any")]
    pub value: String,
    #[serde(deserialize_with = "string_from_any")]
    pub operation: String,
    pub hypertable_timestamp: Option<String>,
}

#[derive(Debug, AsChangeset, Deserialize)]
#[diesel(table_name = crdt_messages)]
pub struct UpdateCrdtMessage {
    pub database: Option<String>,
    pub dataset: Option<String>,
    pub value: Option<String>,
    pub operation: Option<String>,
    pub hypertable_timestamp: Option<String>,
}

//skip serializing when field is none
