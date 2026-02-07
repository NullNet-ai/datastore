use std::time::Duration;
use serde::{Deserialize, Serialize};
use crate::providers::storage::cache::redis_cache::RedisCache;
use crate::providers::storage::cache::cache_interface::CacheInterface;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct TestValue {
    id: i32,
    name: String,
    data: Vec<String>,
}

fn main() {
    println!("Testing Redis connection...");
    
    match RedisCache::<String, TestValue>::new("redis://127.0.0.1:6379".to_string(), Some(Duration::from_secs(60))) {
        Ok(cache) => {
            println!("Redis cache created successfully!");
            
            let test_value = TestValue {
                id: 1,
                name: "Test Item".to_string(),
                data: vec!["item1".to_string(), "item2".to_string()],
            };
            
            println!("Inserting test value...");
            cache.insert("test_key".to_string(), test_value.clone());
            
            // Give Redis a moment to process
            std::thread::sleep(Duration::from_millis(100));
            
            println!("Retrieving test value...");
            let retrieved = cache.get(&"test_key".to_string());
            println!("Retrieved: {:?}", retrieved);
            
            if retrieved == Some(test_value) {
                println!("SUCCESS: Redis is working correctly!");
            } else {
                println!("FAILURE: Redis operations are not working as expected");
            }
        },
        Err(e) => {
            println!("Failed to create Redis cache: {}", e);
        }
    }
}