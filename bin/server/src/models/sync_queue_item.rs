use crate::schema::core::sync_queue_items;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Insertable, Queryable, Serialize, Deserialize, Debug, Clone)]
#[diesel(table_name = sync_queue_items)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct SyncQueueItem {
    pub id: String,
    pub order: i32,
    pub group_id: String,
    pub value: String,
}
