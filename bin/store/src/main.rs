#![recursion_limit = "2056"]
use dotenv::dotenv;
mod builders;
mod config;
mod constants;
mod controllers;
mod database;
mod generated;
mod initializers;
mod lifecycle;
mod middlewares;
mod providers;
mod routers;
mod structs;
mod utils;
use crate::builders::generator::generator_service;
use crate::lifecycle::old_main;
use crate::providers::storage::cache::{cache, CacheConfig};
// Add the cache function import
use crate::database::db;
use crate::lifecycle::{
    logging::{LogConfig, LogLevel},
    manager::LifecycleManager,
};
use crate::utils::helpers::{parse_command_args, parse_env_config};
use config::core::EnvConfig;
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

    let _ = cache.cache_type();
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    // Parse configuration
    let args = parse_command_args();
    // TODO: Old bootstrap must be depracated after fixing the issues in initializers with lifecycle
    let _args: Vec<String> = env::args().collect();
    if _args.contains(&"--init-db".to_string()) {
        old_main::bootstrap().await?;
    } else {
        // Initialize basic logging first
        initialize_logging_and_cache(&parse_env_config());

        // Handle code generation (exits if any generation flags are set)
        generator_service::handle_code_generation(&args).await;

        // Handle database operations
        // db::handle_database_operations(&args).await;
        bootstrap().await?;
    }
    Ok(())
}

// Bootstrap
async fn bootstrap() -> std::io::Result<()> {
    // Create lifecycle configuration
    let log_config = LogConfig {
        level: LogLevel::Info,
        enable_console: true,
        enable_file: true,
        file_path: Some("logs/lifecycle.log".to_string()),
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
