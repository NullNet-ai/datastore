use crate::schema::schema::crdt_merkles;
use diesel::prelude::*;
use merkle::MerkleTree;
use serde::{Deserialize, Serialize};

#[derive(Insertable, Queryable, Serialize, Deserialize, Debug, Clone)]
#[diesel(table_name = crdt_merkles)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct CrdtMerkleModel {
    pub group_id: String,
    pub timestamp: String,
    pub merkle: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ParsedMerkle {
    pub group_id: String,
    pub timestamp: String,
    pub merkle: MerkleTree, // TODO: instead of String but parse it to protobuff and also store it as stringified protobuff
}
