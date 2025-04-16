use crate::schema::schema::sync_queues;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Insertable, Queryable, Serialize, Deserialize, Debug, Clone)]
#[diesel(table_name = sync_queues)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct SyncQueue {
    pub group_id: String,
    pub count: i32,
    pub size: i32,
}
