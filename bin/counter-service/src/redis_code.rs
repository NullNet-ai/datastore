//! Redis-backed code generation: atomic INCR and config (prefix, default_code, digits_number).

use deadpool_redis::Pool;
use redis::AsyncCommands;
use thiserror::Error;

const KEY_PREFIX_COUNTER: &str = "code:counter";
const KEY_PREFIX_CONFIG: &str = "code:config";

fn counter_key(database: &str, entity: &str) -> String {
    format!("{}:{}:{}", KEY_PREFIX_COUNTER, database, entity)
}

fn config_key(database: &str, entity: &str) -> String {
    format!("{}:{}:{}", KEY_PREFIX_CONFIG, database, entity)
}

#[derive(Debug, Error)]
pub enum CodeError {
    #[error("Redis error: {0}")]
    Redis(#[from] redis::RedisError),
    #[error("Config missing for {database}:{entity} — run InitCounters first")]
    ConfigMissing { database: String, entity: String },
    #[error("Pool error: {0}")]
    Pool(#[from] deadpool_redis::PoolError),
}

/// Config for one entity (prefix, default_code, digits_number).
#[derive(Debug, Clone)]
pub struct EntityConfig {
    pub prefix: String,
    pub default_code: i32,
    pub digits_number: i32,
}

/// Get the next code for (database, entity). Atomic: INCR then format using stored config.
pub async fn get_next_code(pool: &Pool, database: &str, entity: &str) -> Result<String, CodeError> {
    let mut conn = pool.get().await?;
    let ck = counter_key(database, entity);
    let count: i64 = conn.incr(&ck, 1).await?;

    let config_key = config_key(database, entity);
    let raw: Vec<Option<String>> = redis::cmd("HMGET")
        .arg(&config_key)
        .arg("prefix")
        .arg("default_code")
        .arg("digits_number")
        .query_async(&mut conn)
        .await?;
    if raw.len() < 3 || raw[0].is_none() || raw[1].is_none() || raw[2].is_none() {
        return Err(CodeError::ConfigMissing {
            database: database.to_string(),
            entity: entity.to_string(),
        });
    }
    let prefix = raw[0].as_ref().unwrap().clone();
    let default_code = raw[1].as_ref().unwrap().parse::<i32>().unwrap_or(0);
    let digits_number = raw[2].as_ref().unwrap().parse::<i32>().unwrap_or(0);

    let code = format_code(&prefix, default_code, count as i32, digits_number);
    Ok(code)
}

fn format_code(prefix: &str, default_code: i32, counter: i32, digits_number: i32) -> String {
    if digits_number > 0 {
        let counter_str = counter.to_string();
        let zero_digits = (digits_number as i32) - (counter_str.len() as i32);
        let zeros = if zero_digits > 0 {
            "0".repeat(zero_digits as usize)
        } else {
            String::new()
        };
        format!("{}{}{}", prefix, zeros, counter)
    } else {
        format!("{}{}", prefix, default_code + counter)
    }
}

/// Initialize or update config for entities. Sets Redis hash and optionally initial counter.
pub async fn init_counters(
    pool: &Pool,
    database: &str,
    entities: &[(String, String, i32, i32)], // (entity, prefix, default_code, digits_number)
) -> Result<(), CodeError> {
    let mut conn = pool.get().await?;
    for (entity, prefix, default_code, digits_number) in entities {
        let ck = config_key(database, entity);
        let _: () = redis::cmd("HSET")
            .arg(&ck)
            .arg("prefix")
            .arg(prefix.as_str())
            .arg("default_code")
            .arg(*default_code)
            .arg("digits_number")
            .arg(*digits_number)
            .query_async(&mut conn)
            .await?;
        // If counter key does not exist, set to 0 so first INCR gives 1
        let counter_k = counter_key(database, entity);
        let exists: bool = redis::cmd("EXISTS")
            .arg(&counter_k)
            .query_async(&mut conn)
            .await?;
        if !exists {
            let _: () = redis::cmd("SET")
                .arg(&counter_k)
                .arg(0_i32)
                .query_async(&mut conn)
                .await?;
        }
    }
    Ok(())
}

/// Get one counter record from Redis (config + current value). Returns None if config or counter key is missing.
pub async fn get_counter_record(
    pool: &Pool,
    database: &str,
    entity: &str,
) -> Result<Option<CounterRecord>, CodeError> {
    let mut conn = pool.get().await?;
    let ck = config_key(database, entity);
    let raw: Vec<Option<String>> = redis::cmd("HMGET")
        .arg(&ck)
        .arg("prefix")
        .arg("default_code")
        .arg("digits_number")
        .query_async(&mut conn)
        .await?;
    if raw.len() < 3 || raw[0].is_none() || raw[1].is_none() || raw[2].is_none() {
        return Ok(None);
    }
    let prefix = raw[0].as_ref().unwrap().clone();
    let default_code = raw[1].as_ref().unwrap().parse::<i32>().unwrap_or(0);
    let digits_number = raw[2].as_ref().unwrap().parse::<i32>().unwrap_or(0);
    let counter_k = counter_key(database, entity);
    let counter: i64 = redis::cmd("GET")
        .arg(&counter_k)
        .query_async(&mut conn)
        .await
        .unwrap_or(0);
    Ok(Some(CounterRecord {
        database: database.to_string(),
        entity: entity.to_string(),
        prefix,
        default_code,
        digits_number,
        counter,
    }))
}

/// Counter record as returned by get_counter_record / list.
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub struct CounterRecord {
    pub database: String,
    pub entity: String,
    pub prefix: String,
    pub default_code: i32,
    pub digits_number: i32,
    pub counter: i64,
}

/// List all counter (database, entity) pairs by scanning Redis keys code:config:*.
/// Key format: code:config:<database>:<entity> (database and entity must not contain ':').
pub async fn list_counter_keys(pool: &Pool) -> Result<Vec<(String, String)>, CodeError> {
    let mut conn = pool.get().await?;
    let keys: Vec<String> = redis::cmd("KEYS")
        .arg("code:config:*")
        .query_async(&mut conn)
        .await?;
    let mut out = Vec::new();
    let prefix = "code:config:";
    for key in keys {
        if let Some(suffix) = key.strip_prefix(prefix) {
            let parts: Vec<&str> = suffix.splitn(2, ':').collect();
            if parts.len() == 2 {
                out.push((parts[0].to_string(), parts[1].to_string()));
            }
        }
    }
    out.sort_by(|a, b| a.0.cmp(&b.0).then_with(|| a.1.cmp(&b.1)));
    Ok(out)
}

/// List all counter records with full details (config + current count).
pub async fn list_counter_records(pool: &Pool) -> Result<Vec<CounterRecord>, CodeError> {
    let pairs = list_counter_keys(pool).await?;
    let mut records = Vec::with_capacity(pairs.len());
    for (database, entity) in pairs {
        if let Some(record) = get_counter_record(pool, &database, &entity).await? {
            records.push(record);
        }
    }
    Ok(records)
}

/// Migration: replace the whole counter record in Redis with the given values.
/// Overwrites both the config hash (prefix, default_code, digits_number) and the counter value.
pub async fn replace_counter_record(
    pool: &Pool,
    database: &str,
    entity: &str,
    prefix: &str,
    default_code: i32,
    digits_number: i32,
    counter: i64,
) -> Result<(), CodeError> {
    let mut conn = pool.get().await?;
    let ck = config_key(database, entity);
    let _: () = redis::cmd("HSET")
        .arg(&ck)
        .arg("prefix")
        .arg(prefix)
        .arg("default_code")
        .arg(default_code)
        .arg("digits_number")
        .arg(digits_number)
        .query_async(&mut conn)
        .await?;
    let counter_k = counter_key(database, entity);
    let _: () = redis::cmd("SET")
        .arg(&counter_k)
        .arg(counter)
        .query_async(&mut conn)
        .await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn format_code_with_digits() {
        assert_eq!(format_code("DV", 10000, 1, 6), "DV000001");
        assert_eq!(format_code("SE", 1000, 42, 6), "SE000042");
        assert_eq!(format_code("SIA", 10000, 0, 6), "SIA000000");
    }

    #[test]
    fn format_code_no_digits() {
        assert_eq!(format_code("", 100000, 1, 0), "100001");
        assert_eq!(format_code("P", 100, 5, 0), "P105");
    }

    #[test]
    fn format_code_empty_prefix() {
        assert_eq!(format_code("", 0, 1, 3), "001");
    }
}
