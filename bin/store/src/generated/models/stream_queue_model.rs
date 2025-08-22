use crate::database::schema::schema::stream_queue;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(
    Queryable, Selectable, Serialize, Default, Deserialize, Clone, AsChangeset, Insertable, Debug,
)]
#[diesel(table_name = stream_queue)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[serde(default)]
pub struct StreamQueueModel {
    pub id: String,
    pub name: String,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub last_accessed: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = stream_queue)]
pub struct NewStreamQueue {
    pub id: String,
    pub name: String,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub last_accessed: Option<chrono::DateTime<chrono::Utc>>,
}

impl StreamQueueModel {
    pub fn new(id: String, name: String) -> Self {
        Self {
            id,
            name,
            created_at: Some(chrono::Utc::now()),
            last_accessed: None,
        }
    }
}

impl NewStreamQueue {
    pub fn new(id: String, name: String) -> Self {
        Self {
            id,
            name,
            created_at: Some(chrono::Utc::now()),
            last_accessed: None,
        }
    }
}
