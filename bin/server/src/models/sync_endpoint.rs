use crate::schema::core::sync_endpoints;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Insertable, Queryable, Serialize, Deserialize, Debug, Clone)]
#[diesel(table_name = sync_endpoints)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct SyncEndpoint {
    pub id: i32,
    pub url: String,
    pub auth_username: String,
    pub auth_password: String,
    pub sync_interval: i32,
}
