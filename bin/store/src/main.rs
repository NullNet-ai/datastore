#![recursion_limit = "2056"]
use builders::templates::grpc_controller::grpc_controller_generator;
use builders::templates::proto_generator;
use builders::templates::table_enum::table_enum_generator;
use dotenv::dotenv;
use providers::operations::batch_sync::background_sync;
use std::env;
mod builders;
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
use crate::providers::operations::batch_sync::batch_sync::BatchSyncService;
use crate::providers::storage::cache::cache_factory::CacheType;
use crate::providers::storage::cache::{cache, CacheConfig};
// Add the cache function import
use crate::builders::generator::generator_service::GeneratorService;
use crate::constants::paths;
use crate::database::db;
use crate::database::schema::database_setup::DatabaseSetupFlags;
use crate::initializers::init::initialize;
use crate::initializers::structs::EInitializer;
use crate::middlewares::shutdown_handler;
use crate::providers::operations::message_stream::pg_listener_service::PgListenerService;
// use crate::providers::operations::sync::merkles::merkle_manager::MerkleManager;
use crate::providers::operations::sync::message_manager::{create_message_channel, SENDER};
use crate::providers::operations::sync::transactions::queue_service::QueueService;
use crate::providers::operations::sync::transactions::transaction_service::TransactionService;
use crate::lifecycle::{ manager::LifecycleManager, logging::{LogConfig, LogLevel}};
use env_logger::Env;
use log::{error, info};
use std::process;
use std::sync::Arc;
use std::time::Duration;

fn run_build_script() -> std::io::Result<()> {
    use std::process::Command;

    info!("Running build script manually...");

    let output = Command::new("cargo").arg("build").arg("--quiet").output()?;

    if output.status.success() {
        info!("Build script executed successfully");
    } else {
        error!(
            "Build script failed: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }

    Ok(())
}

/// Configuration structure for command-line arguments
struct CommandArgs {
    cleanup: bool,
    init_db: bool,
    generate_proto: bool,
    generate_grpc: bool,
    generate_table_enum: bool,
    create_schema: bool,
}

/// Configuration structure for environment variables
struct EnvConfig {
    cache_type: CacheType,
    redis_connection: Option<String>,
    ttl: Option<Duration>,
}

/// Parse command-line arguments
fn parse_command_args() -> CommandArgs {
    let args: Vec<String> = env::args().collect();

    CommandArgs {
        cleanup: args.contains(&"--cleanup".to_string()),
        init_db: args.contains(&"--init-db".to_string()),
        generate_proto: env::var("GENERATE_PROTO").unwrap_or_else(|_| "false".to_string())
            == "true",
        generate_grpc: env::var("GENERATE_GRPC").unwrap_or_else(|_| "false".to_string()) == "true",
        generate_table_enum: env::var("GENERATE_TABLE_ENUM")
            .unwrap_or_else(|_| "false".to_string())
            == "true",
        create_schema: env::var("CREATE_SCHEMA").unwrap_or_else(|_| "false".to_string()) == "true",
    }
}

/// Parse environment configuration
fn parse_env_config() -> EnvConfig {
    let cache_type_str = env::var("CACHE_TYPE").unwrap_or_else(|_| "inmemory".to_string());
    let cache_type = CacheType::from_str(&cache_type_str).unwrap_or(CacheType::InMemory);
    let redis_connection = env::var("REDIS_CONNECTION").ok();
    let ttl = env::var("CACHE_TTL")
        .ok()
        .and_then(|ttl_str| ttl_str.parse::<u64>().ok())
        .map(Duration::from_secs);

    EnvConfig {
        host: env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string()),
        port: env::var("PORT").unwrap_or_else(|_| "5000".to_string()),
        grpc_port: env::var("GRPC_PORT").unwrap_or_else(|_| "6000".to_string()),
        grpc_url: env::var("GRPC_URL").unwrap_or_else(|_| "127.0.0.1".to_string()),
        socket_host: env::var("SOCKET_HOST").unwrap_or_else(|_| "0.0.0.0".to_string()),
        socket_port: env::var("SOCKET_PORT").unwrap_or_else(|_| "3001".to_string()),
        cache_type,
        redis_connection,
        ttl,
    }
}

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

/// Handle code generation tasks
async fn handle_code_generation(args: &CommandArgs) {
    if args.generate_proto || args.generate_grpc || args.generate_table_enum || args.create_schema {
        info!("Starting code generation...");

        // Proto generation
        if args.generate_proto {
            info!("Generating proto files");
            proto_generator::generate_protos(
                paths::database::SCHEMA_FILE,
                paths::proto::OUTPUT_DIR,
            );

            if let Err(e) = run_build_script() {
                error!("Failed to run build script: {}", e);
            }
        }

        // gRPC controller generation
        if args.generate_grpc {
            info!("Generating gRPC controllers");
            if let Err(e) = grpc_controller_generator::run_generator() {
                error!("Error: {}", e);
                process::exit(1);
            }
        }

        // Table enum generation
        if args.generate_table_enum {
            info!("Generating table enums");
            if let Err(e) = table_enum_generator::run_generator() {
                error!("Failed to generate table enum: {}", e);
            }
        }

        // Schema generation
        if args.create_schema {
            info!("Running schema generator");
            if let Err(e) = GeneratorService::run() {
                error!("Failed to generate schema: {}", e);
                process::exit(1);
            }
        }

        info!("Code generation completed successfully!");
        process::exit(0);
    }
}

/// Handle database operations based on command arguments
async fn handle_database_operations(args: &CommandArgs) {
    if args.cleanup {
        info!("Running cleanup operation only...");
        match crate::database::schema::database_setup::setup_database(DatabaseSetupFlags {
            run_cleanup: true,
            run_migrations: true,
            initialize_services: false,
            run_init_sql: false,
        })
        .await
        {
            Ok(_) => {
                info!("Database cleanup completed successfully!");
            }
            Err(e) => {
                error!("Error during database cleanup: {}", e);
            }
        }
    }

    if args.init_db {
        info!("Running database initialization...");
        match crate::database::schema::database_setup::setup_database(DatabaseSetupFlags {
            run_cleanup: false,
            run_migrations: false,
            initialize_services: true,
            run_init_sql: true,
        })
        .await
        {
            Ok(_) => {
                info!("Database initialization completed successfully!");
            }
            Err(e) => {
                error!("Error during database initialization: {}", e);
            }
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    // Parse configuration
    let args = parse_command_args();
    let env_config = parse_env_config();

    // Initialize basic logging first
    initialize_logging_and_cache(&env_config);

    // Handle code generation (exits if any generation flags are set)
    handle_code_generation(&args).await;

    // Handle database operations
    handle_database_operations(&args).await;

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
    let mut lifecycle_manager = LifecycleManager::with_config(log_config);


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
