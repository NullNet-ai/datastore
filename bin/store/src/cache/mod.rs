// Private modules (only accessible within the cache directory)
mod cache_interface;
mod in_memory_cache;
mod redis_cache;

// Public modules
pub mod cache_config;
pub mod cache_factory;
pub mod cache_singleton;

// Public re-exports
pub use cache_config::CacheConfig;
pub use cache_singleton::CACHE as cache;
