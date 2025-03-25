use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use crate::schema::schema::crdt_messages;

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
    pub operation: String,
    pub hypertable_timestamp: Option<String>,
}