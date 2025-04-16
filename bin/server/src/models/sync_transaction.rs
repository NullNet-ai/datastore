use crate::schema::schema::sync_transactions;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Insertable, Queryable, Serialize, Deserialize, Debug, Clone)]
#[diesel(table_name = sync_transactions)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct SyncTransaction {
    pub id: String,
    pub timestamp: String,
    pub group_id: String,
    pub sync_endpoint_id: i32,
    pub status: String,
    pub expiry: Option<i64>,
}
