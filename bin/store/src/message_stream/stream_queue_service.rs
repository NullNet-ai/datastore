use crate::db;
use crate::models::stream_queue_item_model::{StreamQueueItemModel, NewStreamQueueItem};
use crate::schema::schema::stream_queue_items;
use diesel::prelude::*;
use diesel::result::Error as DieselError;
use diesel_async::RunQueryDsl;
use serde_json::Value;
use std::sync::Arc;
use uuid::Uuid;



pub struct StreamQueueService;

impl StreamQueueService {
    pub fn new() -> Arc<Self> {
        Arc::new(Self)
    }


    pub async fn insert_to_queue(
        &self,
        queue_name: &str,
        content: Value,
    ) -> Result<StreamQueueItemModel, DieselError> {
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
        
        Ok(item_model)
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
    ) -> Result<Vec<StreamQueueItemModel>, DieselError> {
        let mut conn = db::get_async_connection().await;
        
        let results = stream_queue_items::table
            .filter(stream_queue_items::queue_name.eq(queue_name))
            .order(stream_queue_items::timestamp.asc())
            .limit(limit as i64)
            .load::<StreamQueueItemModel>(&mut conn)
            .await?;
        
        Ok(results)
    }


}