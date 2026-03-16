use once_cell::sync::Lazy;
use regex::Regex;
use std::collections::HashMap;
use std::sync::Mutex;
use tokio_postgres::Row;

static CACHE: Lazy<Mutex<HashMap<String, bool>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

const SCHEMA_CONTENT: &str = include_str!("../generated/schema.rs");

pub fn is_partitioned_table(table: &str) -> bool {
    if let Some(v) = CACHE.lock().unwrap().get(table) {
        return *v;
    }
    let result = has_id_timestamp_pk_in_schema(table);
    CACHE.lock().unwrap().insert(table.to_string(), result);
    schedule_relkind_query(table.to_string());
    result
}

fn schedule_relkind_query(table: String) {
    if tokio::runtime::Handle::try_current().is_ok() {
        tokio::task::spawn_blocking(move || {
            if let Some(rt) = tokio::runtime::Runtime::new().ok() {
                rt.block_on(async move {
                    if let Ok(client) = crate::database::db::create_connection().await {
                        if let Some(relkind) = query_relkind(&client, &table).await {
                            let is_part = relkind.as_str() == "p";
                            let mut cache = CACHE.lock().unwrap();
                            cache.insert(table, is_part);
                        }
                    }
                });
            }
        });
    } else {
        std::thread::spawn(move || {
            if let Some(rt) = tokio::runtime::Runtime::new().ok() {
                rt.block_on(async move {
                    if let Ok(client) = crate::database::db::create_connection().await {
                        if let Some(relkind) = query_relkind(&client, &table).await {
                            let is_part = relkind.as_str() == "p";
                            let mut cache = CACHE.lock().unwrap();
                            cache.insert(table, is_part);
                        }
                    }
                });
            }
        });
    }
}

async fn query_relkind(client: &tokio_postgres::Client, table: &str) -> Option<String> {
    match client
        .query_opt(
            "SELECT relkind::text FROM pg_class WHERE oid = $1::regclass",
            &[&table],
        )
        .await
    {
        Ok(Some(row)) => row.try_get::<_, String>(0).ok(),
        _ => None,
    }
}

pub async fn is_partitioned_table_async(table: &str) -> bool {
    if let Ok(client) = crate::database::db::create_connection().await {
        if let Some(relkind) = query_relkind(&client, table).await {
            let is_part = relkind.as_str() == "p";
            let mut cache = CACHE.lock().unwrap();
            cache.insert(table.to_string(), is_part);
            return is_part;
        }
    }
    has_id_timestamp_pk_in_schema(table)
}

fn has_id_timestamp_pk_in_schema(table: &str) -> bool {
    let pattern = format!(
        r"(?s)table!\s*\{{\s*{}\s*\(([^)]*)\)\s*\{{",
        regex::escape(table)
    );
    let re = match Regex::new(&pattern) {
        Ok(re) => re,
        Err(_) => return false,
    };
    if let Some(caps) = re.captures(SCHEMA_CONTENT) {
        if let Some(pk) = caps.get(1) {
            let keys = pk.as_str().replace(' ', "").to_lowercase();
            return keys.contains("id") && keys.contains("timestamp");
        }
    }
    false
}
