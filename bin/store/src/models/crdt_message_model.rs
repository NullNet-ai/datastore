use crate::schema::schema::crdt_messages;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Queryable, Serialize, Deserialize)]
#[diesel(table_name = crdt_messages)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct GetCrdtMessage {
    pub database: Option<String>,
    pub dataset: String,
    pub group_id: String,
    pub timestamp: String,
    pub row: String,
    pub column: String,
    pub client_id: String,
    pub value: String,
    pub operation: Option<String>,
    pub hypertable_timestamp: Option<String>,
}

#[derive(Debug, Insertable, Deserialize)]
#[diesel(table_name = crdt_messages)]
pub struct InsertCrdtMessage {
    pub database: Option<String>,
    pub dataset: String,
    pub group_id: String,
    pub timestamp: String,
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
