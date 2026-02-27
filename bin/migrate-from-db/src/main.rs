//! Migrate data from a PostgreSQL database (migrate_from) into the store API (migrate_to_store_url).
//!
//! 1. Logs in once to the store to get a token.
//! 2. Reads each configured table from the source DB (raw SQL, row_to_json).
//! 3. For circular-FK tables: inserts with those columns nulled out (first pass).
//! 4. Inserts all other tables normally.
//! 5. Second pass: PATCHes the nulled columns back with real values.
//!
//! Env:
//! - MIGRATE_FROM_DATABASE_URL       - Postgres connection string (source)
//! - MIGRATE_TO_STORE_URL            - Store base URL (e.g. http://localhost:3002)
//! - MIGRATE_STORE_ACCOUNT_ID        - Store login account_id
//! - MIGRATE_STORE_ACCOUNT_SECRET    - Store login account_secret
//! - MIGRATE_TABLES_FILE             - Optional; default migrate_tables_order.rs (from order-tables script)
//! - MIGRATE_APP_ID                  - Optional, default "portal"
//! - MIGRATE_ERROR_LOG               - Optional; file to append every error (default migrate_errors.log)
//! - MIGRATE_ONLY_PATCH_SECOND_PASS  - Optional; if true/1, skip inserts and only run the patch phase

use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::Write;
use std::sync::Mutex;

use serde_json::{Map, Value};

mod config;
mod store_client;

use config::Config;
use store_client::StoreClient;

// Include the generated order file (provides MIGRATE_CIRCULAR_FK_COLS).
// Regenerate with `make order-tables` so this stays in sync.
include!("../migrate_tables_order.rs");

/// Appends migration errors to a file. Safe to use from async (holds a Mutex).
struct ErrorLog(Mutex<std::fs::File>);

impl ErrorLog {
    fn open(path: &str) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        let f = OpenOptions::new()
            .create(true)
            .append(true)
            .write(true)
            .open(path)?;
        Ok(Self(Mutex::new(f)))
    }

    fn log(
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

/// Build a lookup: table -> list of columns to null on first pass (from included const).
fn build_circular_fk_map() -> HashMap<String, Vec<String>> {
    MIGRATE_CIRCULAR_FK_COLS
        .iter()
        .map(|&(table, cols)| {
            (
                table.to_string(),
                cols.iter().map(|s| s.to_string()).collect(),
            )
        })
        .collect()
}

/// Strip circular FK columns from a row for first-pass insert.
/// Returns (stripped_row, saved_values) where saved_values only contains non-null originals.
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

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    dotenv::dotenv().ok();
    env_logger::try_init_from_env(env_logger::Env::default().default_filter_or("info")).ok();

    let config = Config::from_env()?;
    let error_log = ErrorLog::open(&config.error_log_path)?;
    println!("Errors will be appended to: {}", config.error_log_path);

    let client = StoreClient::new(&config)?;
    let token = client.login().await?;
    let root_token = client.login_root().await?;

    let mut pg_client = config.connect_postgres().await?;
    let mut error_count: u64 = 0;

    let circular_fk_map = build_circular_fk_map();

    // Pending patches for second pass: (table, row_id, patch_body)
    let mut pending_patches: Vec<(String, String, Map<String, Value>)> = Vec::new();

    // Allow running only the patch phase by setting MIGRATE_ONLY_PATCH_SECOND_PASS=true/1.
    let only_patch_second_pass = std::env::var("MIGRATE_ONLY_PATCH_SECOND_PASS")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(false);

    if !only_patch_second_pass {
        // ---------------------------------------------------------------------
        // FIRST PASS: insert all tables. For circular-FK tables, null out cols.
        // ---------------------------------------------------------------------
        println!("\n=== PASS 1: Inserting all tables ===\n");

        for table in &config.tables {
            let table = table.trim();
            if table.is_empty() {
                continue;
            }

            let circular_cols = circular_fk_map.get(table).cloned().unwrap_or_default();
            let has_circular = !circular_cols.is_empty();

            if has_circular {
                println!(
                    "Migrating table (circular-FK, nulling: {}): {}",
                    circular_cols.join(", "),
                    table
                );
            } else {
                println!("Migrating table: {}", table);
            }

            let rows = match fetch_table_rows(&mut pg_client, table).await {
                Ok(r) => r,
                Err(e) => {
                    let msg = e.to_string();
                    eprintln!("  error fetching rows: {}", msg);
                    error_log.log(table, 0, None, &msg, None);
                    error_count += 1;
                    continue;
                }
            };

            println!("  rows: {}", rows.len());
            let total = rows.len();

            for (i, row_json) in rows.into_iter().enumerate() {
                let row_id = row_json
                    .get("id")
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_string());

                let (insert_row, saved_cols) = if has_circular {
                    strip_circular_cols(&row_json, &circular_cols)
                } else {
                    (row_json.clone(), Map::new())
                };

                if !saved_cols.is_empty() {
                    if let Some(ref id) = row_id {
                        pending_patches.push((table.to_string(), id.clone(), saved_cols));
                    } else {
                        eprintln!(
                            "  warning: row {} in {} has no 'id', cannot patch circular FKs later",
                            i + 1,
                            table
                        );
                    }
                }

                match client.create_record(table, &insert_row, &token).await {
                    Ok(_) => {
                        if (i + 1) % 100 == 0 || total <= 10 {
                            println!("  inserted {} / {}", i + 1, total);
                        }
                    }
                    Err(e) => {
                        let msg = e.to_string();
                        eprintln!("  error inserting row {}: {}", i + 1, msg);
                        error_log.log(table, i + 1, row_id.as_deref(), &msg, Some(&row_json));
                        error_count += 1;
                    }
                }
            }

            println!("  done: {}", table);
        }
    } else {
        // ---------------------------------------------------------------------
        // PATCH-ONLY MODE: rebuild pending_patches from source DB without inserts.
        // ---------------------------------------------------------------------
        println!("\n=== PATCH-ONLY MODE: collecting circular FK patches from source DB ===\n");

        for table in &config.tables {
            let table = table.trim();
            if table.is_empty() {
                continue;
            }

            let circular_cols = circular_fk_map.get(table).cloned().unwrap_or_default();
            if circular_cols.is_empty() {
                continue;
            }

            println!(
                "Collecting patches for circular-FK table (cols: {}): {}",
                circular_cols.join(", "),
                table
            );

            let rows = match fetch_table_rows(&mut pg_client, table).await {
                Ok(r) => r,
                Err(e) => {
                    let msg = e.to_string();
                    eprintln!("  error fetching rows: {}", msg);
                    error_log.log(table, 0, None, &msg, None);
                    error_count += 1;
                    continue;
                }
            };

            let before = pending_patches.len();

            for (i, row_json) in rows.into_iter().enumerate() {
                let row_id = row_json
                    .get("id")
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_string());

                let (_ignored_insert_row, saved_cols) =
                    strip_circular_cols(&row_json, &circular_cols);

                if !saved_cols.is_empty() {
                    if let Some(ref id) = row_id {
                        pending_patches.push((table.to_string(), id.clone(), saved_cols));
                    } else {
                        eprintln!(
                            "  warning: row {} in {} has no 'id', cannot patch circular FKs later",
                            i + 1,
                            table
                        );
                    }
                }
            }

            let added = pending_patches.len().saturating_sub(before);
            println!("  collected {} patch record(s) for {}", added, table);
        }
    }

    // -------------------------------------------------------------------------
    // SECOND PASS: PATCH circular FK columns back with real values.
    // -------------------------------------------------------------------------
    println!(
        "\n=== PASS 2: Patching circular FK columns ({} records) ===\n",
        pending_patches.len()
    );

    for (table, row_id, patch_body) in &pending_patches {
        let patch_value = Value::Object(patch_body.clone());
        if let Err(e) = client
            .patch_record(table, row_id, &patch_value, &root_token)
            .await
        {
            let msg = e.to_string();
            eprintln!("  error patching {}/{}: {}", table, row_id, msg);
            error_log.log(table, 0, Some(row_id), &msg, Some(&patch_value));
            error_count += 1;
        }
    }

    println!("  patch pass done.");

    if error_count > 0 {
        println!(
            "\nMigration finished with {} error(s). See {}",
            error_count, config.error_log_path
        );
        return Err(format!(
            "{} error(s) logged to {}",
            error_count, config.error_log_path
        )
        .into());
    }

    println!("\nMigration completed with no errors.");
    Ok(())
}

/// Fetch all rows from `table` as JSON. Read-only (SELECT only); source connection is read-only.
async fn fetch_table_rows(
    client: &mut tokio_postgres::Client,
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
