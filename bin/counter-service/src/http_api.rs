//! HTTP API for counter-service: migration and query endpoints for Redis counters.

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use deadpool_redis::Pool;
use serde::Deserialize;
use std::sync::Arc;

use crate::redis_code;

#[derive(Debug, Deserialize)]
pub struct MigrateCounterBody {
    pub database: String,
    pub entity: String,
    pub prefix: String,
    pub default_code: i32,
    pub digits_number: i32,
    pub counter: i64,
}

async fn migrate_counter(
    State(pool): State<Arc<Pool>>,
    Json(body): Json<MigrateCounterBody>,
) -> impl IntoResponse {
    match redis_code::replace_counter_record(
        pool.as_ref(),
        &body.database,
        &body.entity,
        &body.prefix,
        body.default_code,
        body.digits_number,
        body.counter,
    )
    .await
    {
        Ok(()) => (
            StatusCode::OK,
            Json(serde_json::json!({
                "success": true,
                "message": "Counter record replaced",
                "database": body.database,
                "entity": body.entity,
            })),
        )
            .into_response(),
        Err(e) => {
            tracing::error!("migrate_counter error: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "success": false,
                    "message": e.to_string(),
                })),
            )
                .into_response()
        }
    }
}

/// GET /counters — list all counters with full details (database, entity, prefix, default_code, digits_number, counter).
async fn list_counters(State(pool): State<Arc<Pool>>) -> impl IntoResponse {
    match redis_code::list_counter_records(pool.as_ref()).await {
        Ok(records) => {
            let list = serde_json::to_value(&records).unwrap_or(serde_json::json!([]));
            (
                StatusCode::OK,
                Json(serde_json::json!({ "counters": list })),
            )
        }
        Err(e) => {
            tracing::error!("list_counters error: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({ "error": e.to_string() })),
            )
        }
    }
}

/// GET /counters/:database/:entity — get one counter record (config + current value) from Redis.
async fn get_counter(
    State(pool): State<Arc<Pool>>,
    Path((database, entity)): Path<(String, String)>,
) -> impl IntoResponse {
    match redis_code::get_counter_record(pool.as_ref(), &database, &entity).await {
        Ok(Some(record)) => (StatusCode::OK, Json(serde_json::to_value(&record).unwrap())),
        Ok(None) => (
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({
                "error": "Counter not found",
                "database": database,
                "entity": entity
            })),
        ),
        Err(e) => {
            tracing::error!("get_counter error: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({ "error": e.to_string() })),
            )
        }
    }
}

pub fn router(pool: Pool) -> Router {
    Router::new()
        .route("/migrate", post(migrate_counter))
        .route("/counters", get(list_counters))
        .route("/counters/:database/:entity", get(get_counter))
        .with_state(Arc::new(pool))
}
