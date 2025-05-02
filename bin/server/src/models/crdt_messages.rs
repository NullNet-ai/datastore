use crate::schema::schema::crdt_messages;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Insertable, Queryable, Serialize, Deserialize, Debug, Clone)]
#[diesel(table_name = crdt_messages)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct CrdtMessage {
    pub database: Option<String>,
    pub dataset: String,
    pub group_id: String,
    pub timestamp: String,
    pub row: String,
    pub column: String,
    pub client_id: String,
    pub value: String,
    pub hypertable_timestamp: Option<String>,
}

use serde_json::Value as JsonValue;

impl CrdtMessage {
    /// Get the value as serde_json::Value
    pub fn value_as_json(&self) -> serde_json::Result<JsonValue> {
        serde_json::from_str(&self.value)
    }

    /// Set the value from serde_json::Value
    pub fn set_value_from_json(&mut self, val: &JsonValue) -> serde_json::Result<()> {
        self.value = serde_json::to_string(val)?;
        Ok(())
    }
}
