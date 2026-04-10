use crate::database::db::{self, DatabaseTypeConverter};
use crate::providers::operations::sync::sync_service;
use dotenv::dotenv;
use log;
use serde_json::{json, Value};
use std::env;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::time::{sleep, Duration};
use tokio_postgres::Client;

pub struct BackgroundSyncService {
    db_client: Arc<Mutex<Client>>,
    batch_sync_size: usize,
    batch_sync_enabled: bool,
    batch_sync_type: String,
}

impl BackgroundSyncService {
    pub async fn new() -> Result<Self, String> {
        dotenv().ok();

        // Read environment variables with defaults
        let batch_sync_size = env::var("BATCH_SYNC_SIZE")
            .unwrap_or_else(|_| "100".to_string())
            .parse::<usize>()
            .unwrap_or(100);
        let batch_sync_enabled =
            env::var("BATCH_SYNC_ENABLED").unwrap_or_else(|_| "false".to_string()) == "true";
        let batch_sync_type =
            env::var("BATCH_SYNC_TYPE").unwrap_or_else(|_| "round-robin".to_string());

        // Get database connection
        let client = match db::create_connection().await {
            Ok(client) => client,
            Err(e) => {
                log::error!("Error creating database connection: {}", e);
                return Err(format!("Error creating database connection: {}", e));
            }
        };

        Ok(Self {
            db_client: Arc::new(Mutex::new(client)),
            batch_sync_size,
            batch_sync_enabled,
            batch_sync_type,
        })
    }

    pub async fn init(&self) -> Result<(), String> {
        log::info!("Initializing BackgroundSyncService...");

        if !self.batch_sync_enabled {
            log::info!("Batch sync is disabled; skipping background loop");
            return Ok(());
        }

        loop {
            match self.batch_sync().await {
                Ok(_) => {
                    log::debug!("Batch sync cycle completed, continuing...");
                    sleep(Duration::from_secs(1)).await;
                }
                Err(e) => {
                    log::error!("Error in background batch sync: {}", e);
                    sleep(Duration::from_secs(5)).await;
                }
            }
        }
    }
    pub async fn batch_sync(&self) -> Result<(), String> {
        let mut batch_number = 1;

        while self.batch_sync_enabled {
            log::debug!(
                "Starting batch {}'s sync with batch size: {}",
                batch_number,
                self.batch_sync_size
            );
            batch_number += 1;

            let table_list = match self.batch_sync_type.as_str() {
                "round-robin" => self.table_list().await?,
                "weighted-round-robin" => self.weighted_table_list().await?,
                "ordered" => self.ordered_table_list().await?,
                _ => {
                    log::error!("Invalid batch sync type: {}", self.batch_sync_type);
                    return Err(format!("Invalid batch sync type: {}", self.batch_sync_type));
                }
            };

            if table_list.is_empty() {
                log::debug!("No more tables to sync");
                sleep(Duration::from_secs(5)).await;
                continue;
            }

            for table_name in table_list {
                // For ordered mode, drain the entire table before moving to the next
                let drain_fully = self.batch_sync_type == "ordered";
                loop {
                    let client = self.db_client.lock().await;

                    // Query to get records with tombstone = 0
                    let query = format!(
                        "SELECT * FROM {} WHERE tombstone = 0 LIMIT {}",
                        table_name, self.batch_sync_size
                    );

                    let rows = match client.query(query.as_str(), &[]).await {
                        Ok(rows) => rows,
                        Err(e) => {
                            log::error!("Error querying table {}: {}", table_name, e);
                            break;
                        }
                    };

                    if rows.is_empty() {
                        log::debug!("No more records to sync for table {}", table_name);
                        break;
                    }

                    // Get the actual table name without temp_ prefix
                    let actual_table = table_name.replace("temp_", "");
                    let mut synced_in_batch = 0u64;

                    for row in &rows {
                        let mut row_value = self.row_to_value(row, &table_name);

                        // Format the row (remove null, undefined, empty arrays, empty objects)
                        self.format(&mut row_value);

                        // Insert the record
                        match sync_service::insert(&actual_table, row_value.clone()).await {
                            Ok(_) => {
                                // Update the record to mark as synced
                                let id = match row.try_get::<_, String>("id") {
                                    Ok(id) => id,
                                    Err(_) => {
                                        log::error!("[batch_sync] table={} Record missing ID field or ID is not a string", table_name);
                                        continue;
                                    }
                                };

                                let update_query = format!(
                                    "UPDATE {} SET tombstone = 1, status = 'Synced' WHERE id = $1",
                                    table_name
                                );

                                if let Err(e) = client.execute(&update_query, &[&id]).await {
                                    log::error!(
                                        "[batch_sync] table={} Error updating record status: {}",
                                        table_name,
                                        e
                                    );
                                } else {
                                    synced_in_batch += 1;
                                }
                            }
                            Err(e) => {
                                log::error!(
                                    "[batch_sync] table={} Error syncing record: {}",
                                    table_name,
                                    e
                                );
                            }
                        }
                    }

                    // If no records were synced in this batch, move to next table to avoid infinite loop
                    if synced_in_batch == 0 {
                        log::warn!("[batch_sync] table={} No records synced in batch, moving to next table", table_name);
                        break;
                    }

                    // If not draining fully, process one batch per table per cycle
                    if !drain_fully {
                        break;
                    }
                    // Small delay between batches within the same table
                    drop(client);
                    sleep(Duration::from_millis(50)).await;
                }
            }

            // Add a small delay between batches
            sleep(Duration::from_millis(100)).await;
        }

        Ok(())
    }

    /// Returns temp table names in the order specified by BATCH_SYNC_TABLE_ORDER env var.
    /// Tables not present in the DB are skipped.
    async fn ordered_table_list(&self) -> Result<Vec<String>, String> {
        let order_str = env::var("BATCH_SYNC_TABLE_ORDER").map_err(|_| {
            "BATCH_SYNC_TABLE_ORDER env var is required when BATCH_SYNC_TYPE=ordered".to_string()
        })?;

        // Get existing temp tables from DB for validation
        let existing = {
            let client = self.db_client.lock().await;
            let query = "SELECT table_name FROM information_schema.tables WHERE table_schema = 'public' AND table_name LIKE 'temp_%'";
            let rows = client
                .query(query, &[])
                .await
                .map_err(|e| format!("Error querying table list: {}", e))?;
            rows.iter()
                .map(|row| row.get::<_, String>(0))
                .collect::<std::collections::HashSet<String>>()
        };

        let ordered: Vec<String> = order_str
            .split(',')
            .map(|s| format!("temp_{}", s.trim()))
            .filter(|t| existing.contains(t))
            .collect();

        Ok(ordered)
    }

    async fn table_list(&self) -> Result<Vec<String>, String> {
        let client = self.db_client.lock().await;

        let query = "
            SELECT table_name 
            FROM information_schema.tables 
            WHERE table_schema = 'public' AND table_name LIKE 'temp!_%' ESCAPE '!'
        ";

        let rows = match client.query(query, &[]).await {
            Ok(rows) => rows,
            Err(e) => {
                log::error!("Error querying table list: {}", e);
                return Err(format!("Error querying table list: {}", e));
            }
        };

        let table_names = rows.iter().map(|row| row.get::<_, String>(0)).collect();

        Ok(table_names)
    }

    async fn weighted_table_list(&self) -> Result<Vec<String>, String> {
        let client = self.db_client.lock().await;

        // Step 1: Fetch all table names with 'temp_' prefix
        let query = "
            SELECT table_name
            FROM information_schema.tables
            WHERE table_schema = 'public' AND table_name LIKE 'temp!_%' ESCAPE '!'
        ";

        let rows = match client.query(query, &[]).await {
            Ok(rows) => rows,
            Err(e) => {
                log::error!("Error querying table list: {}", e);
                return Err(format!("Error querying table list: {}", e));
            }
        };
        let mut table_weights = Vec::new();

        // Step 2: Fetch the record count for each table
        for row in rows {
            let table_name: String = row.get(0);

            let count_query = format!(
                "SELECT COUNT(*) as total FROM {} WHERE tombstone = 0",
                table_name
            );

            match client.query_one(count_query.as_str(), &[]).await {
                Ok(count_row) => {
                    let count: i64 = count_row.get(0);
                    table_weights.push((table_name, count));
                }
                Err(e) => {
                    log::error!("Error counting records for table {}: {}", table_name, e);
                    continue;
                }
            }
        }

        // Step 3: Sort the tables by record count in descending order
        table_weights.sort_by(|a, b| b.1.cmp(&a.1));

        // Step 4: Generate the sorted table names array
        Ok(table_weights.into_iter().map(|(name, _)| name).collect())
    }
    /// Convert PostgreSQL row to JSON value using the centralized DatabaseTypeConverter
    /// This method now uses the shared type conversion logic for consistency
    fn row_to_value(&self, row: &tokio_postgres::Row, table_name: &str) -> Value {
        match DatabaseTypeConverter::row_to_json(row) {
            Ok(json_value) => json_value,
            Err(e) => {
                log::error!(
                    "[batch_sync] table={} Failed to convert row to JSON: {}",
                    table_name,
                    e
                );
                // Return empty object as fallback
                json!({})
            }
        }
    }

    fn format(&self, data: &mut Value) {
        if let Some(obj) = data.as_object_mut() {
            // Collect keys to remove to avoid borrowing issues
            //remove empty objects arrays and values
            let keys_to_remove: Vec<String> = obj
                .iter()
                .filter_map(|(key, value)| {
                    if value.is_null()
                        || (value.is_array()
                            && value.as_array().map_or(false, |arr| arr.is_empty()))
                        || (value.is_object()
                            && value.as_object().map_or(false, |obj| obj.is_empty()))
                    {
                        Some(key.clone())
                    } else {
                        None
                    }
                })
                .collect();

            // Remove the keys
            for key in keys_to_remove {
                obj.remove(&key);
            }
        }
    }
}
