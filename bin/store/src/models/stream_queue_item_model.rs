use crate::schema::schema::stream_queue_items;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Queryable, Selectable, Serialize, Default, Deserialize, Clone, AsChangeset, Insertable, Debug)]
#[diesel(table_name = stream_queue_items)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[serde(default)]
pub struct StreamQueueItemModel {
    pub id: String,
    pub queue_name: String,
    pub content: Value,
    pub timestamp: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = stream_queue_items)]
pub struct NewStreamQueueItem {
    pub id: String,
    pub queue_name: String,
    pub content: Value,
    pub timestamp: Option<chrono::DateTime<chrono::Utc>>,
}

impl NewStreamQueueItem {
    pub fn new(id: String, queue_name: String, content: Value) -> Self {
        Self {
            id,
            queue_name,
            content,
            timestamp: Some(chrono::Utc::now()),
        }
    }
}

impl StreamQueueItemModel {
    pub fn new(id: String, queue_name: String, content: Value) -> Self {
        Self {
            id,
            queue_name,
            content,
            timestamp: Some(chrono::Utc::now()),
        }
    }
}