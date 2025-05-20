use crate::sync::message_manager;
use lazy_static::lazy_static;
use std::sync::atomic::{AtomicBool, Ordering};
use tokio::signal::unix::{signal, SignalKind};
use crate::sync::merkles::merkle_manager::MerkleManager;

// Global flag to track shutdown status
lazy_static! {
    pub static ref SHUTDOWN_REQUESTED: AtomicBool = AtomicBool::new(false);
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

    if let Err(e) = message_manager::save_pending_messages().await {
        log::error!("Error waiting for message queue: {}", e);
    }

    let merkle_manager = MerkleManager::instance();
    if let Err(e) = merkle_manager.save_to_db().await {
        log::error!("Error saving Merkle trees to database: {}", e);
    } else {
        log::info!("Successfully saved Merkle trees to database");
    }

    log::info!("All data saved successfully, ready for shutdown");
    Ok(())
}

// Function to set up the SIGINT handler
pub async fn setup_shutdown_handler() {
    tokio::spawn(async {
        let mut sigint = signal(SignalKind::interrupt()).expect("Failed to set up SIGINT handler");

        sigint.recv().await;
        log::warn!("SIGINT received, initiating graceful shutdown");

        request_shutdown();

        if let Err(e) = save_data_before_shutdown().await {
            log::error!("Error during shutdown process: {}", e);
        }

        std::process::exit(0);
    });

    log::info!("SIGINT handler set up successfully");
}
