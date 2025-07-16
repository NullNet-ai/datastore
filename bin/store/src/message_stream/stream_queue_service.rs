use crate::db;
use crate::models::stream_queue_model::{StreamQueueModel, NewStreamQueue};
use crate::models::stream_queue_item_model::{StreamQueueItemModel, NewStreamQueueItem};
use crate::schema::schema::{stream_queue, stream_queue_items};
use diesel::prelude::*;
use diesel::result::Error as DieselError;
use diesel_async::RunQueryDsl;
use serde_json::Value;
use std::sync::Arc;
use uuid::Uuid;

// Stream queue item structure for backward compatibility
#[derive(Debug, Clone)]
pub struct StreamQueueItem {
    pub id: String,
    pub queue_name: String,
    pub timestamp: String,
    pub content: Value,
}

// Convert from model to legacy struct
impl From<StreamQueueItemModel> for StreamQueueItem {
    fn from(model: StreamQueueItemModel) -> Self {
        Self {
            id: model.id,
            queue_name: model.queue_name,
            timestamp: model.timestamp.map(|t| t.to_rfc3339()).unwrap_or_else(|| chrono::Utc::now().to_rfc3339()),
            content: model.content,
        }
    }
}

pub struct StreamQueueService;

impl StreamQueueService {
    pub fn new() -> Arc<Self> {
        Arc::new(Self)
    }

    pub async fn queue_exists(&self, queue_name: &str) -> Result<bool, DieselError> {
        let mut conn = db::get_async_connection().await;
        
        let result = stream_queue::table
            .filter(stream_queue::name.eq(queue_name))
            .first::<StreamQueueModel>(&mut conn)
            .await;
            
        match result {
            Ok(_) => Ok(true),
            Err(DieselError::NotFound) => Ok(false),
            Err(e) => Err(e),
        }
    }

    pub async fn get_queue_by_name(&self, queue_name: &str) -> Result<Option<StreamQueueModel>, DieselError> {
        let mut conn = db::get_async_connection().await;
        
        let result = stream_queue::table
            .filter(stream_queue::name.eq(queue_name))
            .first::<StreamQueueModel>(&mut conn)
            .await;
            
        match result {
            Ok(queue) => Ok(Some(queue)),
            Err(DieselError::NotFound) => Ok(None),
            Err(e) => Err(e),
        }
    }

    pub async fn create_queue(&self, queue_name: &str) -> Result<StreamQueueModel, DieselError> {
        let mut conn = db::get_async_connection().await;
        
        let new_queue = NewStreamQueue::new(
            Uuid::new_v4().to_string(),
            queue_name.to_string(),
        );
        
        let queue = diesel::insert_into(stream_queue::table)
            .values(&new_queue)
            .get_result::<StreamQueueModel>(&mut conn)
            .await?;
            
        Ok(queue)
    }

    pub async fn get_or_create_queue(&self, queue_name: &str) -> Result<StreamQueueModel, DieselError> {
        match self.get_queue_by_name(queue_name).await? {
            Some(queue) => {
                // Update last_accessed timestamp
                let updated_queue = diesel::update(stream_queue::table.filter(stream_queue::id.eq(&queue.id)))
                    .set(stream_queue::last_accessed.eq(Some(chrono::Utc::now())))
                    .get_result::<StreamQueueModel>(&mut db::get_async_connection().await)
                    .await?;
                Ok(updated_queue)
            },
            None => self.create_queue(queue_name).await,
        }
    }

    pub async fn delete_queue_by_name(&self, queue_name: &str) -> Result<(), DieselError> {
        let mut conn = db::get_async_connection().await;
        
        // First delete all items for this queue
        diesel::delete(stream_queue_items::table.filter(stream_queue_items::queue_name.eq(queue_name)))
            .execute(&mut conn)
            .await.ok(); // Ignore errors if no items exist
        
        // Delete the queue itself
        diesel::delete(stream_queue::table.filter(stream_queue::name.eq(queue_name)))
            .execute(&mut conn)
            .await?;
        
        Ok(())
    }

    pub async fn insert_to_queue(
        &self,
        queue_name: &str,
        content: Value,
    ) -> Result<StreamQueueItem, DieselError> {
        let mut conn = db::get_async_connection().await;
        
        // Normalize the JSON content to ensure all numbers are stored as regular integers
        let normalized_content = self.normalize_json_numbers(content);
        
        let new_item = NewStreamQueueItem::new(
            Uuid::new_v4().to_string(),
            queue_name.to_string(),
            normalized_content,
        );
        
        let item_model = diesel::insert_into(stream_queue_items::table)
            .values(&new_item)
            .get_result::<StreamQueueItemModel>(&mut conn)
            .await?;
        
        Ok(StreamQueueItem::from(item_model))
    }

    // Helper function to normalize JSON numbers to prevent BigInt issues
    fn normalize_json_numbers(&self, value: Value) -> Value {
        match value {
            Value::Object(mut map) => {
                for (_, v) in map.iter_mut() {
                    *v = self.normalize_json_numbers(v.clone());
                }
                Value::Object(map)
            }
            Value::Array(mut arr) => {
                for item in arr.iter_mut() {
                    *item = self.normalize_json_numbers(item.clone());
                }
                Value::Array(arr)
            }
            Value::Number(n) => {
                // Convert all numbers to i64 to prevent BigInt interpretation issues
                if let Some(i) = n.as_i64() {
                    Value::Number(serde_json::Number::from(i))
                } else if let Some(u) = n.as_u64() {
                    // Convert u64 to i64 if it fits, otherwise keep as u64
                    if u <= i64::MAX as u64 {
                        Value::Number(serde_json::Number::from(u as i64))
                    } else {
                        Value::Number(serde_json::Number::from(u))
                    }
                } else {
                    // Keep floating point numbers as is
                    Value::Number(n)
                }
            }
            _ => value,
        }
    }

    pub async fn delete_from_queue(&self, item_id: &str) -> Result<(), DieselError> {
        let mut conn = db::get_async_connection().await;
        
        diesel::delete(stream_queue_items::table.filter(stream_queue_items::id.eq(item_id)))
            .execute(&mut conn)
            .await?;
        
        Ok(())
    }

    pub async fn get_queue_items(
        &self,
        queue_name: &str,
        limit: i32,
    ) -> Result<Vec<StreamQueueItem>, DieselError> {
        let mut conn = db::get_async_connection().await;
        
        let results = stream_queue_items::table
            .filter(stream_queue_items::queue_name.eq(queue_name))
            .order(stream_queue_items::timestamp.asc())
            .limit(limit as i64)
            .load::<StreamQueueItemModel>(&mut conn)
            .await?;
        
        let items = results
            .into_iter()
            .map(StreamQueueItem::from)
            .collect();
        
        Ok(items)
    }

    pub async fn clear_queue(&self, queue_name: &str) -> Result<(), DieselError> {
        let mut conn = db::get_async_connection().await;
        
        diesel::delete(stream_queue_items::table.filter(stream_queue_items::queue_name.eq(queue_name)))
            .execute(&mut conn)
            .await?;
        
        Ok(())
    }

    pub async fn get_queue_size(&self, queue_name: &str) -> Result<i64, DieselError> {
        let mut conn = db::get_async_connection().await;
        
        let count = stream_queue_items::table
            .filter(stream_queue_items::queue_name.eq(queue_name))
            .count()
            .get_result::<i64>(&mut conn)
            .await?;
        
        Ok(count)
    }
}