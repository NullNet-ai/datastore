#![recursion_limit = "2056"]
use actix_web::{ web, App, HttpServer};
use builders::templates::grpc_controller::grpc_controller_generator;
use builders::templates::proto_generator;
use builders::templates::table_enum::table_enum_generator;
use dotenv::dotenv;
use providers::operations::batch_sync::background_sync;
use providers::operations::message_stream::gateway::{create_socket_io, set_streaming_service};
use std::env;
mod builders;
mod constants;
mod controllers;
mod database;
mod generated;
mod initializers;
mod middlewares;
mod providers;
mod routers;
mod structs;
// table_enum is now in generated module
mod utils;
use crate::routers::{
    configure_organizations_routes, configure_token_routes, configure_store_routes,
    configure_root_store_routes, configure_listener_routes, configure_file_routes,
    configure_sync_routes,
};
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
use crate::providers::operations::message_stream::streaming_service::MessageStreamingService;
use crate::providers::storage::AppState;
// use crate::providers::operations::sync::merkles::merkle_manager::MerkleManager;
use crate::providers::operations::sync::message_manager::{create_message_channel, SENDER};
use crate::providers::operations::sync::sync_service::bg_sync;
use crate::providers::operations::sync::transactions::queue_service::QueueService;
use crate::providers::operations::sync::transactions::transaction_service::TransactionService;

use crate::generated::grpc_controller::GrpcController;
use env_logger::Env;
use log::{error, info};
use std::process;
use std::sync::Arc;
use std::time::Duration;
use tokio::signal::unix::{signal, SignalKind};

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

/// Spawns all background services including gRPC, background sync, and Socket.IO server
async fn spawn_background_services(grpc_addr: String, socket_host: String, socket_port: String) {
    // Start gRPC server
    tokio::spawn(async move {
        match GrpcController::init(&grpc_addr).await {
            Ok(_) => info!("gRPC server started successfully"),
            Err(e) => error!("Failed to start gRPC server: {}", e),
        }
    });

    // Start background sync service
    tokio::spawn(async {
        if let Err(e) = bg_sync().await {
            log::error!("Error starting background sync: {}", e);
        }
    });

    // Start Socket.IO server
    tokio::spawn(async move {
        use axum::Router;

        // Use your gateway function that includes all the handlers
        let (layer, io) = create_socket_io();

        // Initialize the MessageStreamingService
        let streaming_service = MessageStreamingService::new(io);

        // Set the streaming service reference in gateway
        set_streaming_service(streaming_service.clone());

        // Initialize the streaming service (starts broker and routing)
        if let Err(e) = streaming_service.initialize().await {
            log::error!("Failed to initialize MessageStreamingService: {}", e);
        } else {
            log::info!("MessageStreamingService initialized successfully");
        }

        // Note: Message processing is handled by the routing task in initialize()
        // No need for additional message processing loop

        let app = Router::new().layer(layer);

        let listener = tokio::net::TcpListener::bind(format!("{}:{}", socket_host, socket_port))
            .await
            .expect(&format!("Failed to bind Socket.IO server to port {}", socket_port));

        info!("Socket.IO server running on http://{}:{}", socket_host, socket_port);

        axum::serve(listener, app)
            .await
            .expect("Socket.IO server failed");
    });
}

/// Creates and configures the main HTTP server
fn create_http_server(
    pool: crate::database::db::AsyncDbPool,
    s3_client: aws_sdk_s3::Client,
    bucket_name: String,
    bind_address: String,
) -> std::io::Result<actix_web::dev::Server> {
    info!("Store is running on {}", bind_address);
    
    let server = HttpServer::new(move || {
        let app_state = AppState {
            s3_client: s3_client.clone(),
            bucket_name: bucket_name.clone(),
        };
        
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .configure(configure_sync_routes)
            .configure(configure_organizations_routes)
            .configure(configure_token_routes)
            .configure(configure_root_store_routes)
            .configure(|cfg| configure_store_routes(cfg, app_state.clone()))
            .configure(configure_listener_routes)
            .configure(|cfg| configure_file_routes(cfg, app_state.clone()))
    })
    .disable_signals()
    .bind(bind_address)?
    .run();
    
    Ok(server)
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
    host: String,
    port: String,
    grpc_port: String,
    grpc_url: String,
    socket_host: String,
    socket_port: String,
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
        generate_proto: env::var("GENERATE_PROTO").unwrap_or_else(|_| "false".to_string()) == "true",
        generate_grpc: env::var("GENERATE_GRPC").unwrap_or_else(|_| "false".to_string()) == "true",
        generate_table_enum: env::var("GENERATE_TABLE_ENUM").unwrap_or_else(|_| "false".to_string()) == "true",
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

/// Initialize all services and dependencies
async fn initialize_services() -> std::io::Result<(crate::database::db::AsyncDbPool, aws_sdk_s3::Client, String)> {
    // Initialize S3 storage
    let (s3_client, bucket_name) = match providers::storage::initialize().await {
        Ok((client, bucket)) => (client, bucket),
        Err(e) => {
            log::error!("Failed to initialize S3 client: {}", e);
            std::process::exit(1);
        }
    };
    
    // Initialize background services
    if let Err(e) = initialize(EInitializer::BACKGROUND_SERVICES_CONFIG, None).await {
        log::error!("Failed to initialize background services: {}", e);
    } else {
        log::info!("Background services initialized successfully");
    }

    TransactionService::initialize().await;

    // Initialize background sync service
    let background_sync_service = match background_sync::BackgroundSyncService::new().await {
        Ok(service) => service,
        Err(e) => {
            log::error!("Failed to initialize BackgroundSyncService: {}", e);
            return Err(std::io::Error::new(std::io::ErrorKind::Other, e));
        }
    };

    // Spawn background sync service
    tokio::spawn(async move {
        if let Err(e) = background_sync_service.init().await {
            log::error!("Error in background sync service: {}", e);
        }
    });

    // Initialize PgListenerService
    info!("Starting PgListenerService...");
    if let Err(e) = PgListenerService::initialize().await {
        log::error!("Failed to initialize PgListenerService: {}", e);
    } else {
        log::info!("PgListenerService initialized successfully");
    }

    // Initialize message sender
    let sender = create_message_channel();
    let arc_sender = Arc::new(sender);
    SENDER.set(arc_sender).expect("Failed to initialize sender");

    // Initialize database pool
    let pool = db::establish_async_pool();
    info!("Database connected successfully.");

    // Initialize batch sync
    if let Err(e) = BatchSyncService::init().await {
        log::error!("Failed to initialize batch sync: {}", e);
    } else {
        log::info!("Batch sync initialized successfully");
    }

    // Initialize queue service
    if let Err(e) = QueueService::init().await {
        log::error!("Failed to initialize queue: {}", e);
    } else {
        info!("Queue initialized successfully");
    }
    
    Ok((pool, s3_client, bucket_name))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    
    // Parse configuration
    let args = parse_command_args();
    let env_config = parse_env_config();
    
    // Initialize logging and cache
    initialize_logging_and_cache(&env_config);

    // Handle code generation (exits if any generation flags are set)
    handle_code_generation(&args).await;
    
    // Handle database operations
    handle_database_operations(&args).await;
    
    // Initialize all services and dependencies
    let (pool, s3_client, bucket_name) = initialize_services().await?;
    
    // Configure server addresses
    let grpc_addr = format!("{}:{}", env_config.grpc_url, env_config.grpc_port);
    let store_server_url = format!("{}:{}", env_config.host, env_config.port);
    
    // Start background services
    spawn_background_services(grpc_addr, env_config.socket_host, env_config.socket_port).await;
    
    // Start main HTTP server
    let server = create_http_server(pool.clone(), s3_client.clone(), bucket_name.clone(), store_server_url.clone())?;

    let mut sigint = signal(SignalKind::interrupt())?;

    tokio::select! {
        _ = server => {},
        _ = sigint.recv() => {
            info!("SIGINT received, running custom shutdown...");

            // Set the shutdown flag
            shutdown_handler::request_shutdown();

            // Perform your async cleanup operations
            if let Err(e) = shutdown_handler::save_data_before_shutdown().await {
                log::error!("Error during shutdown process: {}", e);
            } else {
                log::info!("Successfully saved all data before shutdown");
            }

            // Wait for 5 seconds before proceeding with shutdown
            info!("Waiting 5 seconds before final shutdown...");
            // tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;

            info!("Shutdown delay complete, exiting now");
        },
    }

    Ok(())
}
