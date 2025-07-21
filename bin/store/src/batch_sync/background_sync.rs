use crate::db;
use crate::sync::sync_service;
use dotenv::dotenv;
use log;
use serde_json::{json, Value};
use std::env;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::time::{sleep, Duration};
use tokio_postgres::Client;
use std::collections::HashMap;
use tokio_postgres::types::Type;
use base64::prelude::*;


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

        // Run in an infinite loop to keep the service running
        loop {
            match self.batch_sync().await {
                Ok(_) => {
                    log::debug!("Batch sync cycle completed, continuing...");
                    // Add a small delay before starting next cycle
                    sleep(Duration::from_secs(1)).await;
                }
                Err(e) => {
                    log::error!("Error in background batch sync: {}", e);
                    // Add a delay before retry on error
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
                        continue;
                    }
                };

                if rows.is_empty() {
                    log::debug!("No more records to sync for table {}", table_name);
                    continue;
                }

                // Get the actual table name without temp_ prefix
                let actual_table = table_name.replace("temp_", "");

                for row in rows {
                    let mut row_value = self.row_to_value(&row);

                    // Format the row (remove null, undefined, empty arrays, empty objects)
                    self.format(&mut row_value);

                    // Insert the record
                    match sync_service::insert(&actual_table, row_value.clone()).await {
                        Ok(_) => {
                            // Update the record to mark as synced
                            let id = match row.try_get::<_, String>("id") {
                                Ok(id) => id,
                                Err(_) => {
                                    log::error!("Record missing ID field or ID is not a string");
                                    continue;
                                }
                            };

                            let update_query = format!(
                                "UPDATE {} SET tombstone = 1, status = 'Synced' WHERE id = $1",
                                table_name
                            );

                            if let Err(e) = client.execute(&update_query, &[&id]).await {
                                log::error!("Error updating record status: {}", e);
                            }
                        }
                        Err(e) => {
                            log::error!("Error syncing record: {}", e);
                        }
                    }
                }
            }

            // Add a small delay between batches
            sleep(Duration::from_millis(100)).await;
        }

        Ok(())
    }

    async fn table_list(&self) -> Result<Vec<String>, String> {
        let client = self.db_client.lock().await;

        let query = "
            SELECT table_name 
            FROM information_schema.tables 
            WHERE table_schema = 'public' AND table_name LIKE '%temp%'
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

        // Step 1: Fetch all table names with 'temp' in the name
        let query = "
            SELECT table_name 
            FROM information_schema.tables 
            WHERE table_schema = 'public' AND table_name LIKE '%temp%'
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
fn row_to_value(&self, row: &tokio_postgres::Row) -> Value {
    let mut obj = serde_json::Map::new();

    for i in 0..row.len() {
        let column_name = row.columns()[i].name();
        let column_type = row.columns()[i].type_();
        
        let value = match column_type {
            // String types
            &Type::VARCHAR | &Type::TEXT | &Type::BPCHAR | &Type::NAME | &Type::CHAR => {
                row.try_get::<_, Option<String>>(i).ok().flatten().map(|v| json!(v))
            },
            
            // Integer types
            &Type::INT2 => {
                row.try_get::<_, Option<i16>>(i).ok().flatten().map(|v| json!(v))
            },
            &Type::INT4 => {
                row.try_get::<_, Option<i32>>(i).ok().flatten().map(|v| json!(v))
            },
            &Type::INT8 => {
                row.try_get::<_, Option<i64>>(i).ok().flatten().map(|v| json!(v))
            },
            
            // Floating point types
            &Type::FLOAT4 => {
                row.try_get::<_, Option<f32>>(i).ok().flatten().map(|v| json!(v))
            },
            &Type::FLOAT8 => {
                row.try_get::<_, Option<f64>>(i).ok().flatten().map(|v| json!(v))
            },
            
            // Numeric/Decimal
            &Type::NUMERIC => {
                // Handle as string to preserve precision
                row.try_get::<_, Option<String>>(i).ok().flatten().map(|v| json!(v))
            },
            
            // Boolean
            &Type::BOOL => {
                row.try_get::<_, Option<bool>>(i).ok().flatten().map(|v| json!(v))
            },
            
            // Date and time types
            &Type::DATE => {
                row.try_get::<_, Option<chrono::NaiveDate>>(i).ok().flatten()
                    .map(|v| json!(v.to_string()))
            },
            &Type::TIME => {
                row.try_get::<_, Option<chrono::NaiveTime>>(i).ok().flatten()
                    .map(|v| json!(v.to_string()))
            },
            &Type::TIMESTAMP => {
                row.try_get::<_, Option<chrono::NaiveDateTime>>(i).ok().flatten()
                    .map(|v| json!(v.to_string()))
            },
            &Type::TIMESTAMPTZ => {
                row.try_get::<_, Option<chrono::DateTime<chrono::Utc>>>(i).ok().flatten()
                    .map(|v| json!(v.to_rfc3339()))
            },
            &Type::INTERVAL => {
                // Handle as string representation
                row.try_get::<_, Option<String>>(i).ok().flatten().map(|v| json!(v))
            },
            
            // Network types
            &Type::INET => {
                row.try_get::<_, Option<std::net::IpAddr>>(i).ok().flatten()
                    .map(|v| json!(v.to_string()))
            },
            &Type::CIDR => {
                // Handle as string
                row.try_get::<_, Option<String>>(i).ok().flatten().map(|v| json!(v))
            },
            &Type::MACADDR => {
                row.try_get::<_, Option<String>>(i).ok().flatten().map(|v| json!(v))
            },
            
            // JSON types
            &Type::JSON | &Type::JSONB => {
                if let Ok(Some(val)) = row.try_get::<_, Option<serde_json::Value>>(i) {
                    Some(val)
                } else {
                    // Fallback to string parsing
                    row.try_get::<_, Option<String>>(i).ok().flatten()
                        .and_then(|s| serde_json::from_str(&s).ok())
                }
            },
            
            // UUID
            &Type::UUID => {
                row.try_get::<_, Option<uuid::Uuid>>(i).ok().flatten()
                    .map(|v| json!(v.to_string()))
            },
            
            // Binary data
            &Type::BYTEA => {
                row.try_get::<_, Option<Vec<u8>>>(i).ok().flatten()
                    .map(|v| json!(base64::prelude::BASE64_STANDARD.encode(v)))
            },
            
            // Array types
            &Type::TEXT_ARRAY => {
                row.try_get::<_, Option<Vec<String>>>(i).ok().flatten()
                    .map(|v| json!(v))
            },
            &Type::INT4_ARRAY => {
                row.try_get::<_, Option<Vec<i32>>>(i).ok().flatten()
                    .map(|v| json!(v))
            },
            &Type::INT8_ARRAY => {
                row.try_get::<_, Option<Vec<i64>>>(i).ok().flatten()
                    .map(|v| json!(v))
            },
            &Type::FLOAT4_ARRAY => {
                row.try_get::<_, Option<Vec<f32>>>(i).ok().flatten()
                    .map(|v| json!(v))
            },
            &Type::FLOAT8_ARRAY => {
                row.try_get::<_, Option<Vec<f64>>>(i).ok().flatten()
                    .map(|v| json!(v))
            },
            &Type::BOOL_ARRAY => {
                row.try_get::<_, Option<Vec<bool>>>(i).ok().flatten()
                    .map(|v| json!(v))
            },
            
            // Geometric types
            &Type::POINT => {
                row.try_get::<_, Option<String>>(i).ok().flatten().map(|v| json!(v))
            },
            &Type::LINE => {
                row.try_get::<_, Option<String>>(i).ok().flatten().map(|v| json!(v))
            },
            &Type::LSEG => {
                row.try_get::<_, Option<String>>(i).ok().flatten().map(|v| json!(v))
            },
            &Type::BOX => {
                row.try_get::<_, Option<String>>(i).ok().flatten().map(|v| json!(v))
            },
            &Type::PATH => {
                row.try_get::<_, Option<String>>(i).ok().flatten().map(|v| json!(v))
            },
            &Type::POLYGON => {
                row.try_get::<_, Option<String>>(i).ok().flatten().map(|v| json!(v))
            },
            &Type::CIRCLE => {
                row.try_get::<_, Option<String>>(i).ok().flatten().map(|v| json!(v))
            },
            
            // Range types
            &Type::INT4_RANGE => {
                row.try_get::<_, Option<String>>(i).ok().flatten().map(|v| json!(v))
            },
            &Type::INT8_RANGE => {
                row.try_get::<_, Option<String>>(i).ok().flatten().map(|v| json!(v))
            },
            &Type::NUM_RANGE => {
                row.try_get::<_, Option<String>>(i).ok().flatten().map(|v| json!(v))
            },
            &Type::TS_RANGE => {
                row.try_get::<_, Option<String>>(i).ok().flatten().map(|v| json!(v))
            },
            &Type::TSTZ_RANGE => {
                row.try_get::<_, Option<String>>(i).ok().flatten().map(|v| json!(v))
            },
            &Type::DATE_RANGE => {
                row.try_get::<_, Option<String>>(i).ok().flatten().map(|v| json!(v))
            },
            
            // Bit string types
            &Type::BIT | &Type::VARBIT => {
                row.try_get::<_, Option<String>>(i).ok().flatten().map(|v| json!(v))
            },
            
            // Money type
            &Type::MONEY => {
                row.try_get::<_, Option<String>>(i).ok().flatten().map(|v| json!(v))
            },
            
            // OID types
            &Type::OID => {
                row.try_get::<_, Option<u32>>(i).ok().flatten().map(|v| json!(v))
            },
            
            // Text search types
            &Type::TS_VECTOR => {
                row.try_get::<_, Option<String>>(i).ok().flatten().map(|v| json!(v))
            },
            &Type::TSQUERY => {
                row.try_get::<_, Option<String>>(i).ok().flatten().map(|v| json!(v))
            },
            
            // XML type
            &Type::XML => {
                row.try_get::<_, Option<String>>(i).ok().flatten().map(|v| json!(v))
            },
            
            // Handle custom types and unknown types by trying string conversion
            _ => {
                // Try common type name patterns for custom/extension types
                match column_type.name() {
                    // PostGIS geometry types
                    "geometry" | "geography" => {
                        row.try_get::<_, Option<String>>(i).ok().flatten().map(|v| json!(v))
                    },
                    // LTREE extension
                    "ltree" => {
                        row.try_get::<_, Option<String>>(i).ok().flatten().map(|v| json!(v))
                    },
                    // HSTORE extension
                    "hstore" => {
                        row.try_get::<_, Option<HashMap<String, Option<String>>>>(i).ok().flatten()
                            .map(|v| json!(v))
                    },
                    // Enum types and other custom types - fallback to string
                    _ => {
                        row.try_get::<_, Option<String>>(i).ok().flatten().map(|v| json!(v))
                    }
                }
            }
        };
        
        // Only insert non-null values
        if let Some(val) = value {
            obj.insert(column_name.to_string(), val);
        }
    }

    Value::Object(obj)
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
