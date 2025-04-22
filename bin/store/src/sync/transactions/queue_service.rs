use crate::db;
use crate::db::DbPooledConnection;
use crate::models::queue_item_model::QueueItem;
use crate::models::queue_model::Queue;

use crate::schema::schema::{queue_items, queues};
use diesel::prelude::*;
use diesel::result::Error as DieselError;
use serde_json::{Value, json};
use uuid::Uuid;

pub struct QueueService;

impl QueueService {
    pub fn init() -> Result<(), DieselError> {
        let mut conn = db::get_connection();

        conn.transaction(|conn| {
            diesel::insert_into(queues::table)
                .values((
                    queues::id.eq("1"),
                    queues::name.eq("test"),
                    queues::size.eq(0),
                    queues::count.eq(0),
                ))
                .on_conflict_do_nothing()
                .execute(conn)
                .map(|_| ())
        })
    }

    pub fn size(queue_name: &str) -> Result<i32, DieselError> {
        let mut conn = db::get_connection();
        let queue = queues::table
            .filter(queues::name.eq(queue_name))
            .first::<Queue>(&mut conn)
            .optional()?;

        match queue {
            Some(q) => Ok(q.size - q.count),
            None => Err(DieselError::NotFound),
        }
    }

    pub fn enqueue(
        conn: &mut DbPooledConnection,
        item: Value,
        queue_name: &str,
    ) -> Result<i32, DieselError> {
        conn.transaction(|conn| {
            let queue = queues::table
                .filter(queues::name.eq(queue_name))
                .first::<Queue>(conn)
                .optional()?;

            let queue = match queue {
                Some(q) => q,
                None => return Err(DieselError::NotFound),
            };

            let new_order = queue.size + 1;

            let queue_item = QueueItem {
                id: Uuid::new_v4().to_string(),
                order: new_order,
                queue_id: queue.id.clone(),
                value: serde_json::to_string(&item).unwrap_or_else(|_| "{}".to_string()),
            };

            diesel::insert_into(queue_items::table)
                .values(&queue_item)
                .execute(conn)?;

            diesel::update(queues::table)
                .filter(queues::id.eq(&queue.id))
                .set(queues::size.eq(new_order))
                .execute(conn)?;

            Ok(new_order)
        })
    }

    pub fn dequeue(
        conn: &mut DbPooledConnection,
        queue_name: &str,
    ) -> Result<Option<Value>, DieselError> {
        conn.transaction(|conn| {
            let queue = queues::table
                .filter(queues::name.eq(queue_name))
                .first::<Queue>(conn)
                .optional()?;

            let queue = match queue {
                Some(q) => q,
                None => return Err(DieselError::NotFound),
            };

            if queue.count == queue.size {
                return Ok(None);
            }
            //print queue

            let queue_item = queue_items::table
                .filter(
                    queue_items::queue_id
                        .eq(queue.id)
                        .and(queue_items::order.eq(queue.count+1)),
                )
                .order(queue_items::order.asc())
                .limit(1)
                .first::<QueueItem>(conn)
                .optional()?;

            match queue_item {
                Some(item) => match serde_json::from_str(&item.value) {
                    Ok(value) => Ok(Some(value)),
                    Err(e) => {
                        log::error!("Failed to parse queue item value: {}", e);
                        Ok(None)
                    }
                },
                None => Ok(None),
            }
        })
    }

    pub fn ack(conn: &mut DbPooledConnection, queue_name: &str) -> Result<(), DieselError> {
        conn.transaction(|conn| {
            diesel::update(queues::table)
                .filter(queues::name.eq(queue_name))
                .filter(queues::count.lt(queues::size)) // Prevent over-counting
                .set(queues::count.eq(queues::count + 1))
                .execute(conn)
                .and_then(|rows| {
                    if rows == 0 {
                        Err(DieselError::NotFound)
                    } else {
                        Ok(())
                    }
                })?;

            Ok(())
        })
    }
}
