use crate::database::schema::schema::queues;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(
    Queryable, Selectable, Serialize, Default, Deserialize, Clone, AsChangeset, Insertable, Debug,
)]
#[diesel(table_name = queues)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[serde(default)]
pub struct QueueModel {
    pub id: String,
    pub name: String,
    pub size: i32,
    pub count: i32,
}
