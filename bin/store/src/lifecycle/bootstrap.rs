use crate::config::core::EnvConfig;
use crate::providers::operations::sync::message_manager::{create_message_channel, SENDER};
use crate::providers::storage::cache::CacheConfig;
use crate::{
    database::schema::{self, database_setup::DatabaseSetupFlags},
    initializers::system_initialization::{init::initialize, structs::EInitializer},
    providers::operations::{
        batch_sync::{background_sync, batch_sync::BatchSyncService},
        sync::transactions::{
            queue_service::QueueService, transaction_service::TransactionService,
        },
    },
};
use dotenv::dotenv;
use env_logger::Env;
use log::{error, info};
use std::sync::Arc;

pub async fn exec() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::Builder::from_env(Env::default().default_filter_or("info"))
        .filter_module("tokio_postgres", log::LevelFilter::Info)
        .init();

    let env_config = EnvConfig::default();
    CacheConfig::init(
        env_config.cache_type.clone(),
        env_config.redis_connection.clone(),
        env_config.ttl,
    );

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
            info!("Database cleanup and migrations completed successfully!");
        }
        Err(e) => {
            error!("Error during database cleanup and migrations: {}", e);
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

    let sender = create_message_channel();
    let arc_sender = Arc::new(sender);
    let _ = SENDER.set(arc_sender);

    info!("Running initialized data operation only...");
    match schema::database_setup::setup_database(DatabaseSetupFlags {
        run_cleanup: false,
        run_migrations: false,
        initialize_services: true,
        run_init_sql: true,
    })
    .await
    {
        Ok(_) => {
            info!("Database initialized data completed successfully!");
        }
        Err(e) => {
            error!("Error during database initialized data: {}", e);
        }
    }
    info!("Cleanup operation completed successfully!");
    Ok(())
}
