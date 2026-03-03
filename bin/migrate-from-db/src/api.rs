//! HTTP API: POST /migrate, GET /status, GET /tables, POST /generate-tables-order.

use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::sync::Arc;

use crate::{
    format_rust_output, generate_table_order, run_migration, Config, ErrorLog, MigrationPhase,
    SharedMigrationState, StoreClient, MIGRATE_CIRCULAR_FK_COLS, MIGRATE_TABLES_ORDER,
    MIGRATE_UNIQUE_INDEXES,
};
use crate::metrics::MigrationMetrics;

/// Combined state for the API: migration state + optional Prometheus metrics.
pub struct AppState {
    pub migration_state: Arc<SharedMigrationState>,
    pub metrics: Option<Arc<MigrationMetrics>>,
}

#[derive(Debug, Deserialize)]
pub struct MigrateRequest {
    pub migrate_to_store_url: String,
    pub migrate_from_database_url: String,
    pub store_account_id: String,
    pub store_account_secret: String,
    #[serde(default)]
    pub store_root_account_id: Option<String>,
    #[serde(default)]
    pub store_root_account_secret: Option<String>,
    #[serde(default)]
    pub app_id: Option<String>,
    #[serde(default)]
    pub error_log_path: Option<String>,
    /// If set, use this table order instead of compiled-in MIGRATE_TABLES_ORDER (e.g. from /generate-tables-order).
    #[serde(default)]
    pub tables: Option<Vec<String>>,
    /// When `tables` is set, optionally pass circular FK columns (table -> list of column names). If omitted but `tables` is set, no circular columns are patched in pass 2.
    #[serde(default)]
    pub circular_fk_cols: Option<HashMap<String, Vec<String>>>,
    /// When set, rows from the "counters" table are sent to this service (POST /migrate) instead of the store.
    #[serde(default)]
    pub counter_service_url: Option<String>,
    /// When using counter_service_url, the database name (e.g. connectivo, skyll) sent as `database` to the counter service for Redis key prefixing. Required when counter_service_url is set.
    #[serde(default)]
    pub migrate_to_database_name: Option<String>,
    /// Direct Postgres URL to the destination (store) database. When set, unique indexes are dropped before migrating each table and recreated after (avoids partial unique index violations during insert).
    #[serde(default)]
    pub migrate_to_database_url: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct GenerateTablesOrderRequest {
    pub database_url: String,
    #[serde(default)]
    pub schema: Option<String>,
    /// When set, unique indexes are read from this DB (destination) instead of source. Use so GET /tables and migration see indexes that exist on the target.
    #[serde(default)]
    pub migrate_to_database_url: Option<String>,
    #[serde(default)]
    pub write_file: Option<bool>,
    #[serde(default)]
    pub output_path: Option<String>,
}

/// POST /migrate — start migration with the given URLs and credentials.
/// Returns 202 if started, 400 if connection/login failed, 409 if a migration is already in progress (phase Migration or Patch).
async fn post_migrate(
    State(state): State<Arc<AppState>>,
    Json(body): Json<MigrateRequest>,
) -> impl IntoResponse {
    let phase = state.migration_state.read().unwrap().phase.clone();
    if matches!(phase, MigrationPhase::Migration | MigrationPhase::Patch) {
        return (
            StatusCode::CONFLICT,
            Json(serde_json::json!({
                "started": false,
                "message": "Migration already running. Check GET /status.",
            })),
        )
            .into_response();
    }

    if body
        .counter_service_url
        .as_ref()
        .map(|u| !u.is_empty())
        .unwrap_or(false)
    {
        let db_name = body
            .migrate_to_database_name
            .as_deref()
            .unwrap_or("")
            .trim();
        if db_name.is_empty() {
            return (
                StatusCode::BAD_REQUEST,
                Json(serde_json::json!({
                    "started": false,
                    "message": "When using counter_service_url you must also provide migrate_to_database_name for counters (e.g. connectivo, skyll).",
                })),
            )
                .into_response();
        }
    }

    let tables: Vec<String> = body
        .tables
        .clone()
        .filter(|t| !t.is_empty())
        .unwrap_or_else(|| MIGRATE_TABLES_ORDER.iter().map(|s| s.to_string()).collect());
    let circular_fk_override = body.circular_fk_cols.clone();
    let config = match Config::from_request_validate(
        body.migrate_from_database_url.clone(),
        body.migrate_to_store_url.clone(),
        body.store_account_id.clone(),
        body.store_account_secret.clone(),
        body.store_root_account_id.clone(),
        body.store_root_account_secret.clone(),
        tables.clone(),
        body.app_id.clone(),
        body.error_log_path.clone(),
        body.counter_service_url.clone(),
        body.migrate_to_database_name.clone(),
        body.migrate_to_database_url.clone(),
    ) {
        Ok(c) => c,
        Err(e) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(serde_json::json!({
                    "started": false,
                    "message": e.to_string(),
                })),
            )
                .into_response();
        }
    };

    // Pre-flight: validate store login and DB connection before starting.
    let client = match StoreClient::new(&config) {
        Ok(c) => c,
        Err(e) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(serde_json::json!({
                    "started": false,
                    "message": format!("Store client init failed: {}", e),
                })),
            )
                .into_response();
        }
    };
    if let Err(e) = client.login().await {
        return (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({
                "started": false,
                "message": format!("Store login failed: {}", e),
            })),
        )
            .into_response();
    }
    if let Err(e) = client.login_root().await {
        return (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({
                "started": false,
                "message": format!("Store root login failed: {}", e),
            })),
        )
            .into_response();
    }
    if let Err(e) = config.connect_postgres().await {
        return (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({
                "started": false,
                "message": format!("Database connection failed: {}", e),
            })),
        )
            .into_response();
    }

    let error_log_path = config.error_log_path.clone();
    let error_log = match ErrorLog::open(&error_log_path) {
        Ok(l) => l,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "started": false,
                    "message": format!("Failed to open error log: {}", e),
                })),
            )
                .into_response();
        }
    };

    let state_clone = Arc::clone(&state.migration_state);
    let metrics_clone = state.metrics.clone();
    tokio::spawn(async move {
        if let Err(e) = run_migration(config, state_clone, error_log, circular_fk_override, metrics_clone).await {
            eprintln!("Migration task failed: {}", e);
        }
    });

    (
        StatusCode::ACCEPTED,
        Json(serde_json::json!({
            "started": true,
            "message": "Migration started. Poll GET /status for progress.",
        })),
    )
        .into_response()
}

/// POST /generate-tables-order — generate table order and circular FK cols from a live DB.
/// Body: database_url, optional schema (default "public"), optional write_file (default false), optional output_path.
/// Returns { tables, circular_fk_cols } and optionally writes migrate_tables_order.rs to output_path.
async fn post_generate_tables_order(
    Json(body): Json<GenerateTablesOrderRequest>,
) -> impl IntoResponse {
    let schema = body.schema.as_deref().unwrap_or("public");
    match generate_table_order(&body.database_url, schema, body.migrate_to_database_url.as_deref()).await {
        Ok((tables, circular_fk_cols, unique_indexes)) => {
            let mut circular_vec: Vec<(String, Vec<String>)> = circular_fk_cols
                .iter()
                .map(|(k, v)| (k.clone(), v.clone()))
                .collect();
            circular_vec.sort_by(|a, b| a.0.cmp(&b.0));
            let unique_indexes_vec: Vec<(String, Vec<(String, String)>)> = unique_indexes
                .iter()
                .map(|(k, v)| (k.clone(), v.clone()))
                .collect();
            let mut response = serde_json::json!({
                "tables": tables,
                "circular_fk_cols": circular_vec,
                "unique_indexes": unique_indexes_vec,
            });
            if body.write_file == Some(true) {
                let path = body
                    .output_path
                    .as_deref()
                    .unwrap_or("migrate_tables_order.rs");
                let rust_content = format_rust_output(&tables, &circular_fk_cols, &unique_indexes);
                match fs::write(path, rust_content) {
                    Ok(()) => {
                        response["written"] = serde_json::json!(path);
                    }
                    Err(e) => {
                        return (
                            StatusCode::INTERNAL_SERVER_ERROR,
                            Json(serde_json::json!({
                                "error": format!("Failed to write file: {}", e),
                                "tables": tables,
                                "circular_fk_cols": circular_vec,
                            })),
                        )
                            .into_response();
                    }
                }
            }
            (StatusCode::OK, Json(response)).into_response()
        }
        Err(e) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({
                "error": e.to_string(),
            })),
        )
            .into_response(),
    }
}

/// GET /status — current migration state (phase, tables, records, errors, patch stats).
async fn get_status(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let snapshot = state.migration_state.read().unwrap().snapshot();
    (StatusCode::OK, Json(snapshot))
}

/// GET /tables — table order, circular FK columns, and unique indexes (same as migrate_tables_order.rs).
async fn get_tables() -> impl IntoResponse {
    let tables: Vec<&str> = MIGRATE_TABLES_ORDER.to_vec();
    let circular: Vec<(String, Vec<String>)> = MIGRATE_CIRCULAR_FK_COLS
        .iter()
        .map(|(t, cols)| (t.to_string(), cols.iter().map(|s| s.to_string()).collect()))
        .collect();
    let unique_indexes: Vec<(String, Vec<(String, String)>)> = MIGRATE_UNIQUE_INDEXES
        .iter()
        .map(|(t, idxs)| (t.to_string(), idxs.iter().map(|(a, b)| (a.to_string(), b.to_string())).collect()))
        .collect();
    (
        StatusCode::OK,
        Json(serde_json::json!({
            "tables": tables,
            "circular_fk_cols": circular,
            "unique_indexes": unique_indexes,
        })),
    )
}

pub fn router(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/migrate", post(post_migrate))
        .route("/generate-tables-order", post(post_generate_tables_order))
        .route("/status", get(get_status))
        .route("/tables", get(get_tables))
        .with_state(state)
}
