use crate::schema::schema::transactions;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(
    Queryable, Selectable, Serialize, Debug, Default, Deserialize, Clone, AsChangeset, Insertable,
)]
#[diesel(table_name = transactions)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[serde(default)]
pub struct Transaction {
    pub id: String,
    pub timestamp: String,
    #[serde(default = "default_status")]
    pub status: String,
    pub expiry: i64,
}

fn default_status() -> String {
    "Active".to_string()
}
