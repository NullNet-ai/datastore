use serde_json::Value;
use sha1::{Digest, Sha1};
use std::env;
use std::time::Duration;

use crate::cache::cache;
use crate::providers::search_suggestion::structs::SearchSuggestionCache;

impl SearchSuggestionCache {
    pub fn get_cache_by_key(key: &str) -> Option<Value> {
        let cached_data = cache.get(&key);
        cached_data
    }

    pub fn set_cache(key: &str, value: Value) {
        let expiry_ms: u64 = env::var("SEARCH_SUGGESTION_CACHE_TTL_MS")
            .unwrap_or_else(|_| "30000".to_string()) // Default 30 seconds
            .parse()
            .unwrap_or(30000);
        cache.insert_with_ttl(key.to_string(), value, Duration::from_millis(expiry_ms));
    }
    
    pub fn hash_string(input: &str) -> String {
        let mut hasher = Sha1::new();
        hasher.update(input.as_bytes());
        format!("{:x}", hasher.finalize())
    }
}
