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
