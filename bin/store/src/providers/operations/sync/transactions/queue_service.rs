use crate::database::db;
use crate::generated::models::queue_item_model::QueueItemModel;
use crate::generated::models::queue_model::QueueModel;
use crate::generated::schema::{queue_items, queues};
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

    /// Enqueue one item. Uses UPDATE ... RETURNING to get next order in one step (shorter critical section than SELECT for_update + insert + update).
    pub async fn enqueue(
        conn: &mut AsyncPgConnection,
        item: Value,
        queue_name: &str,
    ) -> Result<i32, DieselError> {
        use diesel::ExpressionMethods;
        conn.transaction::<_, DieselError, _>(|conn| {
            Box::pin(async move {
                let (queue_id, new_order): (String, i32) = diesel::update(queues::table)
                    .set(queues::size.eq(queues::size + 1))
                    .filter(queues::name.eq(queue_name))
                    .returning((queues::id, queues::size))
                    .get_result(conn)
                    .await
                    .map_err(|e| {
                        log::error!("Failed to increment queue size: {}", e);
                        e
                    })?;

                let queue_item = QueueItemModel {
                    id: Ulid::new().to_string(),
                    order: new_order,
                    queue_id,
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

                Ok(new_order)
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

    /// Dequeue up to `limit` items (by order) without acking. Call ack_batch after sync.
    pub async fn dequeue_batch(
        conn: &mut AsyncPgConnection,
        queue_name: &str,
        limit: i32,
    ) -> Result<Vec<Value>, DieselError> {
        use diesel::ExpressionMethods;
        if limit <= 0 {
            return Ok(vec![]);
        }
        let items: Vec<Value> = conn
            .transaction::<_, DieselError, _>(|conn| {
                Box::pin(async move {
                    let queue = queues::table
                        .filter(queues::name.eq(queue_name))
                        .first::<QueueModel>(conn)
                        .await
                        .optional()?;
                    let queue = match queue {
                        Some(q) => q,
                        None => return Ok(vec![]),
                    };
                    if queue.count == queue.size {
                        return Ok(vec![]);
                    }
                    let max_order = (queue.count + limit).min(queue.size);
                    let queue_items_result = queue_items::table
                        .filter(queue_items::queue_id.eq(&queue.id))
                        .filter(queue_items::order.ge(queue.count + 1))
                        .filter(queue_items::order.le(max_order))
                        .order(queue_items::order.asc())
                        .load::<QueueItemModel>(conn)
                        .await?;
                    let mut out = Vec::with_capacity(queue_items_result.len());
                    for item in queue_items_result {
                        if let Ok(v) = serde_json::from_str(&item.value) {
                            out.push(v);
                        }
                    }
                    Ok(out)
                })
            })
            .await?;
        Ok(items)
    }

    /// Ack the last N items (advance count by n).
    pub async fn ack_batch(
        conn: &mut AsyncPgConnection,
        queue_name: &str,
        n: i32,
    ) -> Result<(), DieselError> {
        if n <= 0 {
            return Ok(());
        }
        conn.transaction::<_, DieselError, _>(|conn| {
            Box::pin(async move {
                let rows = diesel::update(queues::table)
                    .filter(queues::name.eq(queue_name))
                    .set(queues::count.eq(queues::count + n))
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
