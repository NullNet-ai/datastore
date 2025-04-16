use crate::schema::schema::crdt_messages_merkles;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Insertable, Queryable, Serialize, Deserialize, Debug, Clone)]
#[diesel(table_name = crdt_messages_merkles)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct CrdtMessagesMerkle {
    pub group_id: String,
    pub merkle: String,
}
