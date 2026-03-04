#![recursion_limit = "2056"]
use dotenv::dotenv;
mod code_service;
mod config;
mod constants;
mod controllers;
mod database;
mod generated;
mod initializers;
mod lifecycle;
mod macros;
mod middlewares;
mod providers;
mod routers;
mod structs;
mod utils;

#[cfg(test)]
mod test_init;

// Re-export database module for use in other modules
use crate::lifecycle::bootstrap;
use crate::lifecycle::code_generation;
use crate::lifecycle::{
    logging::{LogConfig, LogLevel},
    manager::LifecycleManager,
};
use crate::providers::storage::cache::CacheConfig;
use crate::utils::helpers::{parse_command_args, parse_env_config};
use config::core::EnvConfig;
pub use database::db;
use env_logger::Env;
use log::{error, info};
use std::env;

/// Initialize logging and cache configuration
fn initialize_logging_and_cache(env_config: &EnvConfig) {
    env_logger::Builder::from_env(Env::default().default_filter_or("info"))
        .filter_module("tokio_postgres", log::LevelFilter::Info)
        .init();

    CacheConfig::init(
        env_config.cache_type.clone(),
        env_config.redis_connection.clone(),
        env_config.ttl,
    );

    log::info!(
        "Initialized cache with type: {:?}, TTL: {:?}",
        env_config.cache_type,
        env_config.ttl
    );

    // Use the cache singleton to get cache type
    use crate::providers::storage::cache::cache_singleton::Cache;
    let _ = Cache::global().cache_type();
}

// Bootstrap
async fn bootstrap_with_lifecycle() -> std::io::Result<()> {
    // Create lifecycle configuration
    let log_config = LogConfig {
        level: LogLevel::Info,
        enable_console: true,
        enable_file: true,
        file_path: Some("lifecycle.log".to_string()),
        enable_structured: true,
        max_entries: 10000,
    };

    // Initialize lifecycle manager
    let mut lifecycle_manager =
        LifecycleManager::with_config(log_config, std::sync::Arc::new(parse_env_config()));

    // Execute the application lifecycle
    if let Err(e) = lifecycle_manager.execute().await {
        error!("Application lifecycle failed: {}", e);
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Application execution failed: {}", e),
        ));
    }

    info!("Application shutdown completed successfully");
    Ok(())
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    // Parse configuration
    let args = parse_command_args();
    let _args: Vec<String> = env::args().collect();

    // Initialize path configuration based on command line arguments
    crate::constants::paths::init_path_config(&_args);

    if _args.contains(&"--init-db".to_string()) {
        bootstrap::exec().await?;
        return Ok(());
    } else {
        // Initialize basic logging first
        initialize_logging_and_cache(&parse_env_config());

        // Handle code generation (exits if any generation flags are set)
        code_generation::handle_code_generation(&args).await;

        bootstrap_with_lifecycle().await?;
    }
    Ok(())
}
