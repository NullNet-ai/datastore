use crate::providers::operations::sync::merkles::merkle_manager::MerkleManager;
use crate::providers::operations::sync::message_manager;
use actix_web::dev::ServerHandle;
use lazy_static::lazy_static;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use tokio::signal::unix::{signal, SignalKind};
use tokio::sync::Notify;

// Global flag to track shutdown status
lazy_static! {
    pub static ref SHUTDOWN_REQUESTED: AtomicBool = AtomicBool::new(false);
    pub static ref SHUTDOWN_NOTIFY: Arc<Notify> = Arc::new(Notify::new());
}

pub fn is_shutdown_requested() -> bool {
    SHUTDOWN_REQUESTED.load(Ordering::SeqCst)
}

// Function to set the shutdown flag
pub fn request_shutdown() {
    SHUTDOWN_REQUESTED.store(true, Ordering::SeqCst);
    log::info!("Shutdown requested, preparing for graceful shutdown");
}

pub async fn save_data_before_shutdown() -> Result<(), String> {
    log::info!("Saving data before shutdown");

    // Save message queue - wait for completion without timeout
    log::info!("Saving pending messages...");
    if let Err(e) = message_manager::save_pending_messages().await {
        log::error!("Error saving message queue: {}", e);
    } else {
        log::info!("Successfully saved all pending messages");
    }

    // Save Merkle trees - wait for completion without timeout
    log::info!("Saving Merkle trees to database...");
    let merkle_manager = MerkleManager::instance();
    if let Err(e) = merkle_manager.save_to_db().await {
        log::error!("Error saving Merkle trees to database: {}", e);
        return Err(format!("Failed to save Merkle trees: {}", e));
    } else {
        log::info!("Successfully saved all Merkle trees to database");
    }

    log::info!("All data saved successfully, ready for shutdown");
    Ok(())
}
#[allow(warnings)]
pub async fn setup_shutdown_handler(server_handle: ServerHandle) {
    tokio::spawn(async move {
        let mut sigint = signal(SignalKind::interrupt()).expect("Failed to set up SIGINT handler");
        sigint.recv().await;
        log::warn!("SIGINT received, initiating graceful shutdown");

        request_shutdown();

        if let Err(e) = save_data_before_shutdown().await {
            log::error!("Error during shutdown process: {}", e);
        }

        log::info!("Graceful shutdown complete, stopping Actix server...");
        server_handle.stop(true).await;
        // No need to call process::exit(0), Actix will exit after .await
    });

    log::info!("SIGINT handler set up successfully");
}
