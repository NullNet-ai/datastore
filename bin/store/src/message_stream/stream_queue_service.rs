use crate::db::AsyncDbPooledConnection;
use crate::models::stream_queue_item_model::{StreamQueueItemModel, NewStreamQueueItem};
use crate::schema::schema::stream_queue_items;

use diesel::result::Error as DieselError;
use diesel::{ExpressionMethods, QueryDsl, SelectableHelper};
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
        conn: &mut AsyncDbPooledConnection,
        queue_name: &str,
        content: Value,
    ) -> Result<StreamQueueItemModel, DieselError> {
        
        let new_item = NewStreamQueueItem::new(
            Uuid::new_v4().to_string(),
            queue_name.to_string(),
            content,
        );
        
        let item_model = diesel::insert_into(stream_queue_items::table)
            .values(&new_item)
            .get_result::<StreamQueueItemModel>(conn)
            .await?;
        
        Ok(item_model)
    }
    
    pub async fn dequeue_batch_from_channel(
        &self,
        conn: &mut AsyncDbPooledConnection,
        queue_name: &str,
        batch_size: usize,
    ) -> Result<Vec<StreamQueueItemModel>, DieselError> {

        let items: Vec<StreamQueueItemModel> = stream_queue_items::table
            .filter(stream_queue_items::queue_name.eq(queue_name))
            .order(stream_queue_items::timestamp.asc())
            .limit(batch_size as i64)
            .select(StreamQueueItemModel::as_select())
            .load(conn)
            .await?;
        
        Ok(items)
    }

    
    pub async fn delete_processed_items(
        &self,
        conn: &mut AsyncDbPooledConnection,
        item_ids: &[String],
    ) -> Result<usize, DieselError> {
        let deleted_count = diesel::delete(
            stream_queue_items::table.filter(stream_queue_items::id.eq_any(item_ids))
        )
        .execute(conn)
        .await?;
        
        Ok(deleted_count)
    }
    
    pub async fn has_queued_messages(
        &self,
        conn: &mut AsyncDbPooledConnection,
        queue_name: &str,
    ) -> Result<bool, DieselError> {
        let count: i64 = stream_queue_items::table
            .filter(stream_queue_items::queue_name.eq(queue_name))
            .count()
            .get_result(conn)
            .await?;
        
        Ok(count > 0)
    }

    #[allow(warnings)]
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
                if let Some(i) = n.as_i64() {
                    Value::Number(serde_json::Number::from(i))
                } else if let Some(u) = n.as_u64() {
                    if u <= i64::MAX as u64 {
                        Value::Number(serde_json::Number::from(u as i64))
                    } else {
                        Value::Number(serde_json::Number::from(u))
                    }
                } else {
                    Value::Number(n)
                }
            }
            _ => value,
        }
    }




}