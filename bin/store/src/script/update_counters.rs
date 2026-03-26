use dotenv::dotenv;
use futures_util::StreamExt;
use redis::{AsyncCommands, AsyncIter};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let args: Vec<String> = env::args().collect();

    let default_redis_url = env::var("REDIS_CONNECTION_COUNTER")
        .unwrap_or_else(|_| "redis://127.0.0.1:6379/0".to_string());

    let mut arg_index = 1usize;
    let redis_url = if let Some(first) = args.get(1) {
        if first.starts_with("redis://") {
            arg_index = 2;
            first.clone()
        } else {
            default_redis_url
        }
    } else {
        default_redis_url
    };

    log::info!("Using Redis URL: {}", redis_url);

    let pattern = args.get(arg_index).map(String::as_str).unwrap_or("*");

    let increment_env = env::var("REDIS_COUNTER_INCREMENT")
        .ok()
        .and_then(|v| v.parse::<i64>().ok());

    let increment: i64 = args
        .get(arg_index + 1)
        .and_then(|s| s.parse::<i64>().ok())
        .or(increment_env)
        .unwrap_or(1000);

    let client = redis::Client::open(redis_url)?;
    let mut scan_conn = client.get_async_connection().await?;

    let mut iter: AsyncIter<String> = scan_conn.scan_match(pattern).await?;

    let mut keys: Vec<String> = Vec::new();
    while let Some(key) = iter.next().await {
        keys.push(key);
    }

    drop(iter);
    drop(scan_conn);

    let mut conn = client.get_async_connection().await?;

    let mut processed: usize = 0;
    let mut updated: usize = 0;

    for key in keys {
        processed += 1;
        let result: redis::RedisResult<i64> = conn.incr(&key, increment).await;
        match result {
            Ok(new_value) => {
                println!("Key {} updated to {}", key, new_value);
                updated += 1;
            }
            Err(e) => {
                eprintln!("Failed to increment key {}: {}", key, e);
            }
        }
    }

    println!(
        "Finished updating counters. Processed {} keys, successfully updated {} keys.",
        processed, updated
    );

    Ok(())
}
