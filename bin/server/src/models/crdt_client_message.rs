use crate::schema::core::crdt_client_messages;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

/// Used when inserting rows — `position` is omitted because it is auto-assigned
/// by the `BIGSERIAL` sequence in PostgreSQL.
#[derive(Insertable, Debug, Clone)]
#[diesel(table_name = crdt_client_messages)]
pub struct NewCrdtClientMessage {
    pub record_id: String,
    pub client_id: String,
    pub message: String,
}

/// Used when querying rows — includes `position` which reflects the strict
/// insertion order and is used for stable, dependency-correct chunk pagination.
#[derive(Queryable, Serialize, Deserialize, Debug, Clone)]
#[diesel(table_name = crdt_client_messages)]
pub struct CrdtClientMessage {
    pub record_id: String,
    pub client_id: String,
    pub message: String,
    pub position: i64,
}
