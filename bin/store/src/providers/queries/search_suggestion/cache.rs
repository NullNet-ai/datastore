use serde_json::Value;
use sha1::{Digest, Sha1};
use std::env;
use std::time::Duration;

use crate::providers::queries::search_suggestion::structs::SearchSuggestionCache;
use crate::providers::storage::cache::cache;

impl SearchSuggestionCache {
    #[allow(dead_code)] // Not used as Search Suggestion will be using Materialized View instead of caching the query results
    pub fn get_cache_by_key(key: &str) -> Option<Value> {
        cache.get(&key)
    }

    #[allow(dead_code)] // Not used as Search Suggestion will be using Materialized View instead of caching the query results
    pub fn set_cache(key: &str, value: Value) {
        let expiry_ms: u64 = env::var("SEARCH_SUGGESTION_CACHE_TTL_MS")
            .unwrap_or_else(|_| "30000".to_string()) // Default 30 seconds
            .parse()
            .unwrap_or(30000);
        cache.insert_with_ttl(key.to_string(), value, Duration::from_millis(expiry_ms));
    }

    pub fn mv_results_key(mv_hash: &str) -> String {
        format!("mv_results_{}", mv_hash)
    }

    pub fn mv_refresh_trigger_key(mv_hash: &str) -> String {
        format!("mv_refresh_trigger_{}_ttl", mv_hash)
    }

    fn mv_cache_ttls() -> (Duration, Duration) {
        let default_results_secs: u64 = 300;
        let default_trigger_secs: u64 = 180;

        let results_secs: u64 = env::var("SEARCH_SUGGESTION_MV_RESULTS_TTL_SECS")
            .unwrap_or_else(|_| default_results_secs.to_string())
            .parse()
            .unwrap_or(default_results_secs);

        let trigger_secs: u64 = env::var("SEARCH_SUGGESTION_MV_REFRESH_TRIGGER_TTL_SECS")
            .unwrap_or_else(|_| default_trigger_secs.to_string())
            .parse()
            .unwrap_or(default_trigger_secs);

        let results_secs = results_secs.max(trigger_secs);
        let trigger_secs = trigger_secs.min(results_secs);

        (
            Duration::from_secs(results_secs),
            Duration::from_secs(trigger_secs),
        )
    }

    pub fn get_mv_results(mv_hash: &str) -> Option<Value> {
        let key = Self::mv_results_key(mv_hash);
        cache.get(&key)
    }

    pub fn set_mv_results(mv_hash: &str, value: Value) {
        let (results_ttl, _) = Self::mv_cache_ttls();
        let key = Self::mv_results_key(mv_hash);
        cache.insert_with_ttl(key, value, results_ttl);
    }

    pub fn set_mv_refresh_trigger_if_absent(mv_hash: &str) -> bool {
        let (_, trigger_ttl) = Self::mv_cache_ttls();
        let trigger_key = Self::mv_refresh_trigger_key(mv_hash);
        if cache.contains_key(&trigger_key) {
            return false;
        }

        let results_key = Self::mv_results_key(mv_hash);
        cache.insert_with_ttl(trigger_key, Value::String(results_key), trigger_ttl);
        true
    }

    pub fn hash_string(input: &str) -> String {
        let mut hasher = Sha1::new();
        hasher.update(input.as_bytes());
        format!("{:x}", hasher.finalize())
    }
}
