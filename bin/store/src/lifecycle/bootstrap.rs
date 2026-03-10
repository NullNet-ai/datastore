use crate::utils::helpers::parse_command_args;
use crate::{
    database::{
        schema::{self, database_setup::DatabaseSetupFlags},
    },
    initializers::system_initialization::{init::initialize, structs::EInitializer},
    providers::{
        operations::{
            batch_sync::{background_sync, batch_sync::BatchSyncService},
            sync::{
                message_manager::{create_message_channel, SENDER},
                transactions::{
                    queue_service::QueueService, transaction_service::TransactionService,
                },
            },
        },
        storage::{
            cache::{cache, cache_factory::CacheType, CacheConfig},
        },
    },
};
use dotenv::dotenv;
use env_logger::Env;
use log::{error, info};
use std::{env, sync::Arc, time::Duration};

pub async fn exec() -> std::io::Result<()> {
    dotenv().ok();

    let command_args = parse_command_args();

    env_logger::Builder::from_env(Env::default().default_filter_or("info"))
        .filter_module("tokio_postgres", log::LevelFilter::Info)
        .init();
    let cache_type_str = env::var("CACHE_TYPE").unwrap_or_else(|_| "inmemory".to_string());
    let cache_type = match cache_type_str.as_str() {
        "redis" => CacheType::Redis,
        _ => CacheType::InMemory,
    };
    let redis_connection = env::var("REDIS_CONNECTION").ok();
    let ttl = env::var("CACHE_TTL")
        .ok()
        .and_then(|ttl_str| ttl_str.parse::<u64>().ok())
        .map(Duration::from_secs);

    CacheConfig::init(cache_type, redis_connection, ttl);
    log::info!(
        "Initialized cache with type: {:?}, TTL: {:?}",
        cache_type,
        ttl
    );

    let _ = cache.cache_type();

    let cleanup = command_args.cleanup;
    let init_db = command_args.init_db;
    if cleanup {
        info!("Running cleanup operation only...");
        match schema::database_setup::setup_database(DatabaseSetupFlags {
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
    if let Err(e) = initialize(EInitializer::BACKGROUND_SERVICES_CONFIG, None).await {
        log::error!("Failed to initialize background services: {}", e);
    } else {
        log::info!("Background services initialized successfully");
    }

    TransactionService::initialize().await;

    let background_sync_service = match background_sync::BackgroundSyncService::new().await {
        Ok(service) => service,
        Err(e) => {
            log::error!("Failed to initialize BackgroundSyncService: {}", e);
            return Ok(());
        }
    };

    // Spawn it in a background task
    tokio::spawn(async move {
        if let Err(e) = background_sync_service.init().await {
            log::error!("Error in background sync service: {}", e);
        }
    });

    // Initialize the message sender
    let sender = create_message_channel();
    let arc_sender = Arc::new(sender);
    SENDER.set(arc_sender).expect("Failed to initialize sender");


    // init batch sync
    if let Err(e) = BatchSyncService::init().await {
        log::error!("Failed to initialize queue: {}", e);
    } else {
        log::info!("Queue initialized successfully");
    }

    if let Err(e) = QueueService::init().await {
        log::error!("Failed to initialize queue: {}", e);
    } else {
        info!("Queue initialized successfully");
    }

    if init_db {
        info!("Running cleanup operation only...");
        match schema::database_setup::setup_database(DatabaseSetupFlags {
            run_cleanup: false,
            run_migrations: false,
            initialize_services: true,
            run_init_sql: true,
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

    Ok(())
}
