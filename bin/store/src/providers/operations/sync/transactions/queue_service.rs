use crate::database::db;
use crate::generated::schema::{queue_items, queues};
use crate::generated::models::queue_item_model::QueueItemModel;
use crate::generated::models::queue_model::QueueModel;
use diesel::prelude::*;
use diesel::result::Error as DieselError;
use diesel::OptionalExtension;
use diesel_async::AsyncConnection;
use diesel_async::AsyncPgConnection;
use diesel_async::RunQueryDsl;
use serde_json::Value;
use ulid::Ulid;

pub struct QueueService;

impl QueueService {
    pub async fn init() -> Result<(), DieselError> {
        let mut conn = db::get_async_connection().await;

        conn.transaction::<_, DieselError, _>(|conn| {
            Box::pin(async move {
                diesel::insert_into(queues::table)
                    .values((
                        queues::id.eq("1"),
                        queues::name.eq("test"),
                        queues::size.eq(0),
                        queues::count.eq(0),
                    ))
                    .on_conflict_do_nothing()
                    .execute(conn)
                    .await?;

                Ok(())
            })
        })
        .await
    }

    pub async fn size(queue_name: &str) -> Result<i32, DieselError> {
        let mut conn = db::get_async_connection().await;
        let queue = queues::table
            .filter(queues::name.eq(queue_name))
            .first::<QueueModel>(&mut conn)
            .await
            .optional()?;

        match queue {
            Some(q) => Ok(q.size - q.count),
            None => Err(DieselError::NotFound),
        }
    }

    pub async fn enqueue(
        conn: &mut AsyncPgConnection,
        item: Value,
        queue_name: &str,
    ) -> Result<i32, DieselError> {
        conn.transaction::<_, DieselError, _>(|conn| {
            Box::pin(async move {
                let queue = queues::table
                    .filter(queues::name.eq(queue_name))
                    .first::<QueueModel>(conn)
                    .await
                    .optional()?;

                let queue = match queue {
                    Some(q) => q,
                    None => return Err(DieselError::NotFound),
                };

                let new_order = queue.size + 1;

                let queue_item = QueueItemModel {
                    id: Ulid::new().to_string(),
                    order: new_order,
                    queue_id: queue.id.clone(),
                    value: serde_json::to_string(&item).unwrap_or_else(|_| "{}".to_string()),
                };

                diesel::insert_into(queue_items::table)
                    .values(&queue_item)
                    .execute(conn)
                    .await
                    .map_err(|e| {
                        log::error!("Failed to insert queue item: {}", e);
                        e
                    })?;

                diesel::update(queues::table)
                    .filter(queues::id.eq(&queue.id))
                    .set(queues::size.eq(new_order))
                    .execute(conn)
                    .await
                    .map_err(|e| {
                        log::error!("Failed to update queue size: {}", e);
                        e
                    })?;

                Ok(new_order)

                // ... existing code ...
            })
        })
        .await
    }

    pub async fn dequeue(
        conn: &mut AsyncPgConnection,
        queue_name: &str,
    ) -> Result<Option<Value>, DieselError> {
        conn.transaction::<_, DieselError, _>(|conn| {
            Box::pin(async move {
                let queue = queues::table
                    .filter(queues::name.eq(queue_name))
                    .first::<QueueModel>(conn)
                    .await
                    .optional()?;

                let queue = match queue {
                    Some(q) => q,
                    None => return Err(DieselError::NotFound),
                };

                if queue.count == queue.size {
                    return Ok(None);
                }

                let queue_item = queue_items::table
                    .filter(
                        queue_items::queue_id
                            .eq(queue.id)
                            .and(queue_items::order.eq(queue.count + 1)),
                    )
                    .order(queue_items::order.asc())
                    .limit(1)
                    .first::<QueueItemModel>(conn)
                    .await
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
        })
        .await
    }

    pub async fn ack(conn: &mut AsyncPgConnection, queue_name: &str) -> Result<(), DieselError> {
        conn.transaction::<_, DieselError, _>(|conn| {
            Box::pin(async move {
                let rows = diesel::update(queues::table)
                    .filter(queues::name.eq(queue_name))
                    .set(queues::count.eq(queues::count + 1))
                    .execute(conn)
                    .await?;
                if rows == 0 {
                    Err(DieselError::NotFound)
                } else {
                    Ok(())
                }
            })
        })
        .await
    }
}
