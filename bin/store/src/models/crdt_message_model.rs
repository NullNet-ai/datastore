use crate::schema::schema::crdt_messages;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

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
    #[serde(flatten, skip_serializing_if = "String::is_empty")]
    pub row: String,
    pub column: String,
    pub client_id: String,
    pub value: String,
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
