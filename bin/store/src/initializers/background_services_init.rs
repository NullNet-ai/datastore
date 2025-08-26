use crate::controllers::store_controller::ApiError;
use crate::initializers::code_prefix_init::get_code_prefix_initializer;
use crate::initializers::initial_entity_data::init::get_initial_entity_data_initializer;
use crate::middlewares::session_middleware;
use crate::providers::operations::sync::merkles::merkle_manager::MerkleManager;
use std::env;
use tokio::time::{interval, Duration};

pub struct BackgroundServicesInitializer;

impl BackgroundServicesInitializer {
    pub fn new() -> Self {
        BackgroundServicesInitializer
    }

    pub async fn initialize(
        &self,
        _params: Option<crate::initializers::structs::InitializerParams>,
    ) -> Result<(), ApiError> {
        log::info!("Initializing background services...");

        // Initialize code prefix first to ensure counters table is properly populated
        if let Err(e) = get_code_prefix_initializer().initialize().await {
            log::error!("Failed to initialize code prefix: {}", e);
        } else {
            log::info!("Code prefix initialized successfully");
        }

        // Initialize entity data if environment variable is set
        let initialize_entity_data =
            env::var("INITIALIZE_ENTITY_DATA").unwrap_or_else(|_| "false".to_string()) == "true";

        if initialize_entity_data {
            log::info!("Initializing entity data...");
            if let Err(e) = get_initial_entity_data_initializer().initialize(None).await {
                log::error!("Failed to initialize entity data: {}", e);
            } else {
                log::info!("Entity data initialized successfully");
            }
        }

        // Start session pruning task
        self.start_session_pruning();
        log::info!("Session pruning service started");

        // Start merkle tree periodic save
        self.start_merkle_periodic_save().await;
        log::info!("Merkle tree periodic save service started");

        log::info!("All background services initialized successfully");
        Ok(())
    }

    // Start a background task to periodically prune expired sessions
    fn start_session_pruning(&self) {
        tokio::spawn(async move {
            log::info!("Starting session pruning background task");
            // Set the interval for pruning (e.g., every 6 hours)
            let prune_interval = std::env::var("SESSION_PRUNE_INTERVAL")
                .ok()
                .and_then(|s| s.parse::<u64>().ok())
                .unwrap_or(6 * 60 * 60); // Default to 6 hours in seconds

            log::info!("Session pruning interval set to {} seconds", prune_interval);
            let mut interval_timer = interval(Duration::from_secs(prune_interval));

            loop {
                interval_timer.tick().await;
                log::info!("Running session pruning task");
                match session_middleware::prune_expired_sessions().await {
                    Ok(count) => {
                        log::info!("Pruned {} expired sessions", count);
                    }
                    Err(e) => {
                        log::error!("Failed to prune expired sessions: {}", e);
                    }
                }
            }
        });
    }

    // Start periodic save for merkle trees
    async fn start_merkle_periodic_save(&self) {
        let merkle_manager = MerkleManager::instance();

        // First load existing trees from database
        log::info!("Loading Merkle trees from database...");
        merkle_manager.load_trees_from_db().await;
        log::info!("Merkle trees loaded from database successfully");

        // Save to database every 5 minutes (300000 milliseconds) or use environment variable
        let save_interval = std::env::var("MERKLE_SAVE_INTERVAL")
            .ok()
            .and_then(|s| s.parse::<u64>().ok())
            .unwrap_or(300000); // Default to 5 minutes in milliseconds

        log::info!(
            "Merkle tree save interval set to {} milliseconds",
            save_interval
        );
        merkle_manager.start_periodic_save(save_interval);
        log::info!("Merkle tree periodic save task started");
    }
}

pub fn get_background_services_initializer() -> BackgroundServicesInitializer {
    BackgroundServicesInitializer::new()
}
