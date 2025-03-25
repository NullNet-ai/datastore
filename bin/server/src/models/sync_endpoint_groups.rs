use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use crate::schema::schema::sync_endpoint_groups;

#[derive(Insertable, Queryable, Serialize, Deserialize, Debug, Clone)]
#[diesel(table_name = sync_endpoint_groups)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct SyncEndpointGroup {
    pub sync_endpoint_id: i32,
    pub group_id: String,
    pub status: String,
}
