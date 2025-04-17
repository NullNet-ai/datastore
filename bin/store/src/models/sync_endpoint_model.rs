use crate::schema::schema::sync_endpoints;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Serialize, Debug, Default, Deserialize, Clone, AsChangeset, Insertable)]
#[diesel(table_name = sync_endpoints)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[serde(default)]
pub struct SyncEndpoint {
    pub id: String,
    pub name: String,
    pub url: String,
    pub group_id: String,
    pub username: String,
    pub password: String,
    pub status: String,
}