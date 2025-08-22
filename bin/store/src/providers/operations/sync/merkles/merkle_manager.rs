use crate::database::db;
use crate::providers::operations::sync::merkles::merkle_service::MerkleService;
use lazy_static::lazy_static;
use log::info;
use merkle::MerkleTree;
use std::collections::HashMap;
use std::sync::{Arc, Once};
use tokio::sync::RwLock;

struct MerkleEntry {
    merkle: MerkleTree,
    timestamp: String,
}
pub struct MerkleManager {
    trees: Arc<RwLock<HashMap<String, MerkleEntry>>>,
    initialized: bool,
}

lazy_static! {
    static ref INSTANCE: Arc<MerkleManager> = Arc::new(MerkleManager::new_internal());
    static ref INIT: Once = Once::new();
}
#[allow(warnings)]
impl MerkleManager {
    fn new_internal() -> Self {
        let trees = HashMap::new();

        Self {
            trees: Arc::new(RwLock::new(trees)),
            initialized: false, // Add this line to initialize the field
        }
    }

    pub async fn set_tree(&self, group_id: String, tree: MerkleTree, timestamp: String) {
        let mut trees = self.trees.write().await;
        trees.insert(
            group_id,
            MerkleEntry {
                merkle: tree,
                timestamp,
            },
        );
    }

    pub fn instance() -> Arc<MerkleManager> {
        // Just return the instance without loading trees
        INSTANCE.clone()
    }
    pub async fn load_trees_from_db(&self) {
        let mut conn = db::get_async_connection().await;
        let merkle_service = MerkleService {};

        // Get all group IDs from database
        if let Ok(group_ids) = merkle_service.get_all_group_ids(&mut conn).await {
            let mut trees_map = self.trees.write().await;

            for group_id in group_ids {
                // Load each tree and store in HashMap
                if let Ok(Some(parsed_merkle)) = merkle_service
                    .get_merkles_by_group_id(group_id.clone(), &mut conn)
                    .await
                {
                    trees_map.insert(
                        group_id,
                        MerkleEntry {
                            merkle: parsed_merkle.merkle,
                            timestamp: parsed_merkle.timestamp,
                        },
                    );
                }
            }
        }
    }
    // Helper function to load initial trees
    #[allow(warnings)]
    pub fn initialize(&mut self) {
        if !self.initialized {
            // Perform any one-time initialization here
            self.initialized = true;
        }
    }
    // Get a clone of the Arc for sharing with other tasks
    pub fn clone(&self) -> Self {
        Self {
            trees: Arc::clone(&self.trees),
            initialized: self.initialized,
        }
    }

    // Get a tree by group ID
    pub async fn get_tree(&self, group_id: &str) -> Option<(MerkleTree, String)> {
        let trees = self.trees.read().await;
        trees
            .get(group_id)
            .map(|entry| (entry.merkle.clone(), entry.timestamp.clone()))
    }

    // Get a timestamp by group ID

    pub async fn get_timestamp(&self, group_id: &str) -> Option<String> {
        let trees = self.trees.read().await;
        trees.get(group_id).map(|entry| entry.timestamp.clone())
    }

    // Save all trees to database
    pub async fn save_to_db(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let mut conn = db::get_async_connection().await;
        let merkle_service = MerkleService {};
        let trees = self.trees.read().await;

        for (group_id, entry) in trees.iter() {
            log::debug!(
                "Saving tree for group {} with timestamp {}",
                group_id,
                entry.timestamp
            );
            // Save the tree with its timestamp
            merkle_service
                .set_merkles_by_group_id(
                    group_id.clone(),
                    entry.timestamp.clone(),
                    entry.merkle.clone(),
                    &mut conn,
                )
                .await?;

            info!(
                "Saved tree for group {} with timestamp {}",
                group_id, entry.timestamp
            );
        }

        Ok(())
    }

    // Start a background task to periodically save trees to database
    pub fn start_periodic_save(&self, interval_ms: u64) -> tokio::task::JoinHandle<()> {
        let manager = self.clone();

        tokio::spawn(async move {
            let interval = tokio::time::Duration::from_millis(interval_ms);
            let mut interval_timer = tokio::time::interval(interval);

            loop {
                interval_timer.tick().await;

                if let Err(e) = manager.save_to_db().await {
                    log::error!("Failed to save Merkle trees to database: {}", e);
                } else {
                    print!("Successfully saved Merkle trees to database");
                    log::debug!("Successfully saved Merkle trees to database");
                }
            }
        })
    }
}
