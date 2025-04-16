use crate::schema::schema::crdt_client_messages;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Insertable, Queryable, Serialize, Deserialize, Debug, Clone)]
#[diesel(table_name = crdt_client_messages)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct CrdtClientMessage {
    pub record_id: String,
    pub client_id: String,
    pub message: String,
}
