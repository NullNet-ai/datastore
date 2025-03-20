use crate::schema::schema::crdt_merkles;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Queryable, Serialize, Deserialize, Debug)]
#[diesel(table_name = crdt_merkles)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct GetMerkle {
    pub group_id: String,
    pub timestamp: String,
    pub merkle: String,
}

#[derive(Insertable, Deserialize, Debug, Clone)]
#[diesel(table_name = crdt_merkles)]
pub struct InsertMerkle {
    pub group_id: String,
    pub timestamp: String,
    pub merkle: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ParsedMerkle {
    pub group_id: String,
    pub timestamp: String,
    pub merkle: Value, // TODO: instead of String but parse it to protobuff and also store it as stringified protobuff
}
