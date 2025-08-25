use crate::generated::schema::queue_items;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(
    Queryable, Selectable, Serialize, Default, Deserialize, Clone, AsChangeset, Insertable, Debug,
)]
#[diesel(table_name = queue_items)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[serde(default)]
pub struct QueueItemModel {
    pub id: String,
    pub order: i32,
    pub queue_id: String,
    pub value: String,
}
