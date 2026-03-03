//! Runs the migration (pass 1: insert, pass 2: patch) and updates shared state.

use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::Write;
use std::sync::{Arc, Mutex};
use std::time::Instant;

use serde_json::{Map, Value};
use tokio::sync::Semaphore;

use crate::config::Config;
use crate::metrics;
use crate::state::{MigrationPhase, PatchPhaseStats, SharedMigrationState, TableError, TableStats};
use crate::store_client::StoreClient;
use crate::tables_order;

/// Appends migration errors to a file. Safe to use from async (holds a Mutex).
pub struct ErrorLog(Mutex<std::fs::File>);

impl ErrorLog {
    pub fn open(path: &str) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        let f = OpenOptions::new()
            .create(true)
            .append(true)
            .write(true)
            .open(path)?;
        Ok(Self(Mutex::new(f)))
    }

    pub fn log(
        &self,
        table: &str,
        row_index: usize,
        row_id: Option<&str>,
        err: &str,
        data: Option<&Value>,
    ) {
        let ts = chrono::Utc::now().format("%Y-%m-%dT%H:%M:%S%.3fZ");
        let id_part = row_id
            .map(|id| format!(" id={} ", id))
            .unwrap_or_else(|| " ".to_string());
        let line = format!(
            "{} table={} row_index={}{}error={}\n",
            ts, table, row_index, id_part, err
        );
        if let Ok(mut f) = self.0.lock() {
            let _ = f.write_all(line.as_bytes());
            if let Some(d) = data {
                if let Ok(compact) = serde_json::to_string(d) {
                    let _ = f.write_all(b"  data: ");
                    let _ = f.write_all(compact.as_bytes());
                    let _ = f.write_all(b"\n");
                }
            }
            let _ = f.flush();
        }
    }
}

fn build_circular_fk_map(
    override_map: Option<&HashMap<String, Vec<String>>>,
) -> HashMap<String, Vec<String>> {
    if let Some(m) = override_map {
        return m.clone();
    }
    tables_order::MIGRATE_CIRCULAR_FK_COLS
        .iter()
        .map(|&(table, cols)| {
            (
                table.to_string(),
                cols.iter().map(|s| s.to_string()).collect(),
            )
        })
        .collect()
}

/// Build map table -> [(index_name, create_ddl)] from compiled MIGRATE_UNIQUE_INDEXES.
fn build_unique_indexes_map() -> HashMap<String, Vec<(String, String)>> {
    tables_order::MIGRATE_UNIQUE_INDEXES
        .iter()
        .map(|&(table, idxs)| {
            (
                table.to_string(),
                idxs.iter()
                    .map(|&(a, b)| (a.to_string(), b.to_string()))
                    .collect(),
            )
        })
        .collect()
}

/// Drop unique indexes for a table on the destination DB (before insert).
async fn drop_unique_indexes_for_table(
    dest_client: &tokio_postgres::Client,
    table: &str,
    indexes: &[(String, String)],
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    for (index_name, _ddl) in indexes {
        let sql = format!("DROP INDEX IF EXISTS \"{}\"", index_name.replace('"', "\"\""));
        if let Err(e) = dest_client.batch_execute(&sql).await {
            eprintln!("Warning: failed to drop index {} on {}: {}", index_name, table, e);
            // Continue with other indexes
        }
    }
    Ok(())
}

/// Recreate unique indexes for a table on the destination DB (after insert).
async fn recreate_unique_indexes_for_table(
    dest_client: &tokio_postgres::Client,
    table: &str,
    indexes: &[(String, String)],
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    for (index_name, ddl) in indexes {
        if let Err(e) = dest_client.batch_execute(ddl).await {
            eprintln!("Warning: failed to recreate index {} on {}: {}", index_name, table, e);
        }
    }
    Ok(())
}

fn strip_circular_cols(row: &Value, cols: &[String]) -> (Value, Map<String, Value>) {
    let mut stripped = row.clone();
    let mut saved: Map<String, Value> = Map::new();

    if let Some(obj) = stripped.as_object_mut() {
        for col in cols {
            if let Some(val) = obj.get(col) {
                if !val.is_null() {
                    saved.insert(col.clone(), val.clone());
                    obj.insert(col.clone(), Value::Null);
                }
            }
        }
    }

    (stripped, saved)
}

async fn fetch_table_count(
    client: &tokio_postgres::Client,
    table: &str,
) -> Result<i64, Box<dyn std::error::Error + Send + Sync>> {
    let quoted = format!("\"{}\"", table.replace('"', "\"\""));
    let sql = format!("SELECT count(*) FROM {} t", quoted);
    let row = client.query_one(&sql, &[]).await?;
    let count: i64 = row.get(0);
    Ok(count)
}

/// Fetch all rows from a table (used in patch-only mode).
async fn fetch_table_rows(
    client: &tokio_postgres::Client,
    table: &str,
) -> Result<Vec<Value>, Box<dyn std::error::Error + Send + Sync>> {
    let quoted = format!("\"{}\"", table.replace('"', "\"\""));
    let sql = format!("SELECT row_to_json(t) AS row FROM {} t", quoted);
    let rows = client.query(&sql, &[]).await?;
    let mut out = Vec::with_capacity(rows.len());
    for row in rows {
        let json: tokio_postgres::types::Json<serde_json::Value> = row.get("row");
        out.push(json.0);
    }
    Ok(out)
}

/// Fetch a batch of rows using LIMIT/OFFSET and ORDER BY ctid for stable paging.
async fn fetch_table_rows_batch(
    client: &tokio_postgres::Client,
    table: &str,
    limit: i64,
    offset: i64,
) -> Result<Vec<Value>, Box<dyn std::error::Error + Send + Sync>> {
    let quoted = format!("\"{}\"", table.replace('"', "\"\""));
    let sql = format!(
        "SELECT row_to_json(t) AS row FROM {} t ORDER BY ctid LIMIT $1 OFFSET $2",
        quoted
    );
    let rows = client.query(&sql, &[&limit, &offset]).await?;
    let mut out = Vec::with_capacity(rows.len());
    for row in rows {
        let json: tokio_postgres::types::Json<serde_json::Value> = row.get("row");
        out.push(json.0);
    }
    Ok(out)
}

/// POST one counter row to the counter service /migrate endpoint.
async fn migrate_counter_row(
    base_url: &str,
    database: &str,
    row: &Value,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let entity = row
        .get("entity")
        .and_then(|v| v.as_str())
        .ok_or("counters row missing 'entity'")?
        .to_string();
    let prefix = row
        .get("prefix")
        .and_then(|v| v.as_str())
        .unwrap_or("CTR")
        .to_string();
    let default_code = row
        .get("default_code")
        .and_then(|v| v.as_i64())
        .unwrap_or(0) as i32;
    let digits_number = row
        .get("digits_number")
        .and_then(|v| v.as_i64())
        .unwrap_or(0) as i32;
    let counter = row.get("counter").and_then(|v| v.as_i64()).unwrap_or(0);

    let url = format!("{}/migrate", base_url.trim_end_matches('/'));
    let body = serde_json::json!({
        "database": database,
        "entity": entity,
        "prefix": prefix,
        "default_code": default_code,
        "digits_number": digits_number,
        "counter": counter,
    });

    let client = reqwest::Client::new();
    let res = client.post(&url).json(&body).send().await?;
    if !res.status().is_success() {
        let status = res.status();
        let text = res.text().await.unwrap_or_default();
        return Err(format!("counter service {}: {}", status, text).into());
    }
    Ok(())
}

/// Run the full migration (pass 1 + pass 2), updating shared state for status.
/// If circular_fk_override is Some, it is used instead of the compiled-in MIGRATE_CIRCULAR_FK_COLS.
/// If metrics is Some, records Prometheus metrics (rows, duration, progress, second pass).
pub async fn run_migration(
    config: Config,
    state: Arc<SharedMigrationState>,
    error_log: ErrorLog,
    circular_fk_override: Option<HashMap<String, Vec<String>>>,
    metrics: Option<Arc<crate::metrics::MigrationMetrics>>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Mark started
    {
        let mut s = state.write().unwrap();
        s.phase = MigrationPhase::Migration;
        s.started_at = Some(chrono::Utc::now());
        s.tables = config.tables.clone();
        s.table_stats = config
            .tables
            .iter()
            .map(|t| (t.clone(), TableStats::default()))
            .collect();
        s.patch_phase = PatchPhaseStats::default();
    }

    let client = StoreClient::new(&config)?;
    let token = match client.login().await {
        Ok(t) => t,
        Err(e) => {
            let msg = e.to_string();
            let _ = state.write().unwrap().start_error.insert(msg.clone());
            let _ = state.write().unwrap().phase = MigrationPhase::Error;
            return Err(msg.into());
        }
    };
    let root_token = match client.login_root().await {
        Ok(t) => t,
        Err(e) => {
            let msg = e.to_string();
            let _ = state.write().unwrap().start_error.insert(msg.clone());
            let _ = state.write().unwrap().phase = MigrationPhase::Error;
            return Err(msg.into());
        }
    };

    let pg_client = match config.connect_postgres().await {
        Ok(c) => c,
        Err(e) => {
            let msg = e.to_string();
            let _ = state.write().unwrap().start_error.insert(msg.clone());
            let _ = state.write().unwrap().phase = MigrationPhase::Error;
            return Err(msg.into());
        }
    };

    let dest_client = match config.connect_postgres_destination().await {
        Ok(opt) => opt,
        Err(e) => {
            let msg = e.to_string();
            let _ = state.write().unwrap().start_error.insert(msg.clone());
            let _ = state.write().unwrap().phase = MigrationPhase::Error;
            return Err(msg.into());
        }
    };

    let unique_indexes_map = build_unique_indexes_map();
    let circular_fk_map = build_circular_fk_map(circular_fk_override.as_ref());
    let mut pending_patches: Vec<(String, String, Map<String, Value>)> = Vec::new();

    let only_patch_second_pass = std::env::var("MIGRATE_ONLY_PATCH_SECOND_PASS")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(false);

    if !only_patch_second_pass {
        for table in &config.tables {
            let table = table.trim();
            if table.is_empty() {
                continue;
            }

            let circular_cols = circular_fk_map.get(table).cloned().unwrap_or_default();
            let has_circular = !circular_cols.is_empty();

            let total = match fetch_table_count(&pg_client, table).await {
                Ok(n) => n as u64,
                Err(e) => {
                    let msg = e.to_string();
                    error_log.log(table, 0, None, &msg, None);
                    {
                        let mut s = state.write().unwrap();
                        let st = s.table_stats.entry(table.to_string()).or_default();
                        st.records_total = 0;
                        st.errors_count += 1;
                        st.errors.push(TableError {
                            row_index: Some(0),
                            row_id: None,
                            message: msg.clone(),
                        });
                    }
                    continue;
                }
            };
            {
                let mut s = state.write().unwrap();
                if let Some(st) = s.table_stats.get_mut(table) {
                    st.records_total = total;
                }
            }

            let use_counter_service = table == "counters"
                && config
                    .counter_service_url
                    .as_ref()
                    .map(|u| !u.is_empty())
                    .unwrap_or(false);
            let counter_service_url = config.counter_service_url.as_deref();
            let counter_database = use_counter_service
                .then(|| {
                    config
                        .migrate_to_database_name
                        .as_deref()
                        .map(str::trim)
                        .filter(|s| !s.is_empty())
                        .map(String::from)
                })
                .flatten();

            if use_counter_service && counter_database.is_none() {
                let msg = "counter_service_url is set but migrate_to_database_name is missing. Provide migrate_to_database_name for counters (e.g. connectivo, skyll).";
                error_log.log(table, 0, None, msg, None);
                if let Ok(mut s) = state.write() {
                    let st = s.table_stats.entry(table.to_string()).or_default();
                    st.records_total = total;
                    st.errors_count += 1;
                    st.errors.push(TableError {
                        row_index: Some(0),
                        row_id: None,
                        message: msg.to_string(),
                    });
                }
                continue;
            }

            eprintln!("[migrate] table={} total_rows={}", table, total);

            // Drop unique indexes on destination before insert (if destination URL is set).
            if let Some(ref dest) = dest_client {
                if let Some(indexes) = unique_indexes_map.get(table) {
                    eprintln!("[migrate] table={} dropping {} unique index(es)", table, indexes.len());
                    if let Err(e) = drop_unique_indexes_for_table(dest, table, indexes).await {
                        eprintln!("Warning: drop unique indexes for {}: {}", table, e);
                    }
                }
            }

            let table_start = Instant::now();
            if let Some(ref m) = metrics {
                m.migration_progress_percent.with_label_values(&[table]).set(0);
            }

            let batch_size = config.batch_size as i64;
            let semaphore = Arc::new(Semaphore::new(config.concurrent));
            let mut offset = 0i64;

            loop {
                let batch =
                    match fetch_table_rows_batch(&pg_client, table, batch_size, offset).await {
                        Ok(b) => b,
                        Err(e) => {
                            let msg = e.to_string();
                            error_log.log(table, offset as usize + 1, None, &msg, None);
                            let mut s = state.write().unwrap();
                            if let Some(st) = s.table_stats.get_mut(table) {
                                st.errors_count += 1;
                                st.errors.push(TableError {
                                    row_index: Some(offset as usize + 1),
                                    row_id: None,
                                    message: msg,
                                });
                            }
                            break;
                        }
                    };
                if batch.is_empty() {
                    break;
                }
                let batch_len = batch.len();

                let table_ = table.to_string();
                let token = token.clone();
                let client = client.clone();
                let mut tasks = Vec::with_capacity(batch.len());
                for (i, row_json) in batch.into_iter().enumerate() {
                    let row_index = offset as usize + i + 1;
                    let row_id = row_json
                        .get("id")
                        .or_else(|| row_json.get("entity"))
                        .and_then(|v| v.as_str())
                        .map(|s| s.to_string());
                    let (insert_row, saved_cols) = if has_circular {
                        strip_circular_cols(&row_json, &circular_cols)
                    } else {
                        (row_json.clone(), Map::new())
                    };
                    let patch_data = if saved_cols.is_empty() {
                        None
                    } else {
                        row_id.as_ref().map(|id| (id.clone(), saved_cols))
                    };

                    let permit = semaphore.clone().acquire_owned().await.unwrap();
                    let client = client.clone();
                    let token = token.clone();
                    let table_ = table_.clone();
                    let row_json = row_json.clone();
                    let insert_row = insert_row.clone();
                    let counter_url = counter_service_url.map(String::from);
                    let counter_db = counter_database.as_deref().map(String::from);
                    let use_counter = use_counter_service;

                    tasks.push(tokio::spawn(async move {
                        let _permit = permit;
                        let result = if use_counter {
                            let url = counter_url.as_deref().unwrap();
                            let db = counter_db.as_deref().unwrap();
                            migrate_counter_row(url, db, &row_json).await
                        } else {
                            client.create_record(&table_, &insert_row, &token).await
                        };
                        (row_index, row_id, result, patch_data)
                    }));
                }

                for join in tasks {
                    let (row_index, row_id, result, patch_data) = join
                        .await
                        .unwrap_or_else(|e| (0, None, Err(e.to_string().into()), None));
                    if let Some((id, saved_cols)) = patch_data {
                        pending_patches.push((table.to_string(), id, saved_cols));
                    }
                    match result {
                        Ok(_) => {
                            let mut s = state.write().unwrap();
                            let inserted = if let Some(st) = s.table_stats.get_mut(table) {
                                st.records_inserted += 1;
                                st.records_inserted
                            } else {
                                0
                            };
                            if let Some(ref m) = metrics {
                                m.migration_rows_total
                                    .with_label_values(&[table, "success"])
                                    .inc();
                                m.inc_row_count(table);
                                if total > 0 {
                                    let pct = ((inserted as u64) * 100) / total;
                                    m.migration_progress_percent
                                        .with_label_values(&[table])
                                        .set(pct as i64);
                                }
                            }
                        }
                        Err(e) => {
                            let msg = e.to_string();
                            eprintln!("[migrate] ERROR table={} row={} id={:?} err={}", table, row_index, row_id, msg);
                            error_log.log(table, row_index, row_id.as_deref(), &msg, None);
                            let mut s = state.write().unwrap();
                            if let Some(st) = s.table_stats.get_mut(table) {
                                st.errors_count += 1;
                                st.errors.push(TableError {
                                    row_index: Some(row_index),
                                    row_id: row_id.clone(),
                                    message: msg.clone(),
                                });
                            }
                            if let Some(ref m) = metrics {
                                m.migration_rows_total
                                    .with_label_values(&[table, "error"])
                                    .inc();
                                if let Some(constraint) = metrics::extract_fk_constraint_from_error(&msg) {
                                    m.migration_fk_violations_total
                                        .with_label_values(&[table, &constraint])
                                        .inc();
                                }
                            }
                        }
                    }
                }

                offset += batch_len as i64;
            }

            if let Some(ref m) = metrics {
                m.migration_duration_seconds
                    .with_label_values(&[table])
                    .observe(table_start.elapsed().as_secs_f64());
                m.migration_progress_percent
                    .with_label_values(&[table])
                    .set(100);
            }

            let inserted = state.read().unwrap().table_stats.get(table).map(|s| s.records_inserted).unwrap_or(0);
            let errors = state.read().unwrap().table_stats.get(table).map(|s| s.errors_count).unwrap_or(0);
            eprintln!(
                "[migrate] table={} done inserted={} errors={} elapsed={:.1}s",
                table, inserted, errors, table_start.elapsed().as_secs_f64()
            );

            // Recreate unique indexes on destination after insert (if destination URL is set).
            if let Some(ref dest) = dest_client {
                if let Some(indexes) = unique_indexes_map.get(table) {
                    eprintln!("[migrate] table={} recreating {} unique index(es)", table, indexes.len());
                    if let Err(e) = recreate_unique_indexes_for_table(dest, table, indexes).await {
                        eprintln!("Warning: recreate unique indexes for {}: {}", table, e);
                    }
                }
            }
        }
    } else {
        for table in &config.tables {
            let table = table.trim();
            if table.is_empty() {
                continue;
            }
            let circular_cols = circular_fk_map.get(table).cloned().unwrap_or_default();
            if circular_cols.is_empty() {
                continue;
            }
            let rows = match fetch_table_rows(&pg_client, table).await {
                Ok(r) => r,
                Err(e) => {
                    let msg = e.to_string();
                    error_log.log(table, 0, None, &msg, None);
                    continue;
                }
            };
            for (_i, row_json) in rows.into_iter().enumerate() {
                let row_id = row_json
                    .get("id")
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_string());
                let (_ignored, saved_cols) = strip_circular_cols(&row_json, &circular_cols);
                if !saved_cols.is_empty() {
                    if let Some(ref id) = row_id {
                        pending_patches.push((table.to_string(), id.clone(), saved_cols));
                    }
                }
            }
        }
    }

    // Pass 2: patch
    {
        let mut s = state.write().unwrap();
        s.phase = MigrationPhase::Patch;
        s.patch_phase.total_patches = pending_patches.len() as u64;
        for (table, _id, _) in &pending_patches {
            s.patch_phase.by_table.entry(table.clone()).or_default();
        }
    }

    for (table, row_id, patch_body) in &pending_patches {
        let patch_value = Value::Object(patch_body.clone());
        if let Err(e) = client
            .patch_record(table, row_id, &patch_value, &root_token)
            .await
        {
            let msg = e.to_string();
            error_log.log(table, 0, Some(row_id), &msg, Some(&patch_value));
            let mut s = state.write().unwrap();
            s.patch_phase.errors_count += 1;
            if let Some(bt) = s.patch_phase.by_table.get_mut(table) {
                bt.errors_count += 1;
                bt.errors.push(TableError {
                    row_index: None,
                    row_id: Some(row_id.clone()),
                    message: msg.clone(),
                });
            }
            if let Some(ref m) = metrics {
                m.migration_second_pass_total
                    .with_label_values(&[table, "error"])
                    .inc();
                if let Some(constraint) = metrics::extract_fk_constraint_from_error(&msg) {
                    m.migration_fk_violations_total
                        .with_label_values(&[table, &constraint])
                        .inc();
                }
            }
        } else {
            let mut s = state.write().unwrap();
            s.patch_phase.patched_count += 1;
            if let Some(bt) = s.patch_phase.by_table.get_mut(table) {
                bt.patched += 1;
            }
            if let Some(ref m) = metrics {
                m.migration_second_pass_total
                    .with_label_values(&[table, "success"])
                    .inc();
            }
        }
    }

    {
        let mut s = state.write().unwrap();
        s.phase = if s.patch_phase.errors_count > 0
            || s.table_stats.values().any(|t| t.errors_count > 0)
        {
            MigrationPhase::Error
        } else {
            MigrationPhase::Done
        };
    }

    Ok(())
}
