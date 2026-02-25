use crate::providers::operations::sync::hlc::mutable_timestamp::MutableTimestamp;
use crate::providers::operations::sync::merkles::merkle_manager::MerkleManager;
use crate::providers::operations::sync::structs::Clock;
use diesel_async::AsyncPgConnection;
use hlc::Timestamp;
use merkle::MerkleTree;
use std::env;
use ulid::Ulid;

#[allow(warnings)]
pub struct HlcService {
    pub timestamp: Timestamp,
    pub group_id: String,
}

/// CLOCK_MUTEX removed to allow sync and CRDT insertion to run in parallel.
/// MerkleManager maintains in-memory + persisted state; concurrent updates may interleave.
/// If duplicates or lost merkle leaves appear, consider merging in commit_tree (server merkle + local leaves).

static GROUP_ID: once_cell::sync::Lazy<String> =
    once_cell::sync::Lazy::new(|| env::var("GROUP_ID").unwrap_or_else(|_| "my-group".to_string()));

impl HlcService {
    // fn set_clock(tx: &mut DbPooledConnection, clock: Clock) {
    //     let merkle_manager=MerkleManager::instance();
    //     let group_id = env::var("GROUP_ID").unwrap_or_else(|_| "my-group".to_string());
    //     let merkle_service = MerkleService {};
    //     // Convert timestamp to string
    //     let timestamp_str = clock.timestamp.to_string();
    //     //convert merkle to string
    //     // Call the merkle service to set merkles by group id
    //     merkle_service
    //         .set_merkles_by_group_id(group_id, timestamp_str, clock.merkle, tx)
    //         .expect("Failed to set merkles");
    // }x

    async fn set_clock(clock: Clock) {
        let merkle_manager = MerkleManager::instance();
        let timestamp_str = clock.timestamp.to_string();

        // Use tokio runtime to execute async code in sync context

        merkle_manager
            .set_tree(GROUP_ID.clone(), clock.merkle, timestamp_str)
            .await;
    }

    fn make_clock(timestamp: Timestamp, merkle: MerkleTree) -> Clock {
        Clock {
            timestamp: MutableTimestamp::from(&timestamp),
            merkle,
        }
    }

    pub async fn commit_tree(
        tx: &mut AsyncPgConnection,
        tree: &MerkleTree,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let old_clock = Self::get_clock_internal(tx, true).await?;
        let clock = Self::make_clock(
            Timestamp::parse(old_clock.timestamp.to_string()),
            tree.clone(),
        );
        Ok(Self::set_clock(clock).await)
    }

    pub async fn recv(
        tx: &mut AsyncPgConnection,
        timestamp_str: String,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut clock = Self::get_clock_internal(tx, true).await?;
        let timestamp = Timestamp::parse(timestamp_str);
        let mut current_timestamp = Timestamp::parse(clock.timestamp.to_string());
        current_timestamp.recv(&timestamp);
        clock.timestamp = MutableTimestamp::from(&current_timestamp);
        Self::set_clock(clock).await;
        Ok(())
    }

    /// Returns current clock from MerkleManager; creates initial clock if none exists.
    async fn get_clock_internal(
        _tx: &mut AsyncPgConnection,
        _caller_holds_lock: bool,
    ) -> Result<Clock, Box<dyn std::error::Error>> {
        let group_id = env::var("GROUP_ID").unwrap_or_else(|_| "my-group".to_string());
        let merkle_manager = MerkleManager::instance();

        let tree_result = merkle_manager.get_tree(&group_id).await;
        match tree_result {
            Some((merkle, timestamp)) => {
                Ok(Self::make_clock(Timestamp::parse(timestamp), merkle))
            }
            None => {
                let tree_result = merkle_manager.get_tree(&group_id).await;
                if let Some((merkle, timestamp)) = tree_result {
                    return Ok(Self::make_clock(Timestamp::parse(timestamp), merkle));
                }
                let timestamp = Timestamp::new(0, 0, Self::make_client_id()?);
                let clock: Clock = Self::make_clock(timestamp, MerkleTree::new());
                Self::set_clock(clock.clone()).await;
                Ok(clock)
            }
        }
    }

    pub async fn get_clock(
        tx: &mut AsyncPgConnection,
    ) -> Result<Clock, Box<dyn std::error::Error>> {
        Self::get_clock_internal(tx, false).await
    }

    pub async fn send(tx: &mut AsyncPgConnection) -> Result<String, Box<dyn std::error::Error>> {
        let mut clock = Self::get_clock_internal(tx, true).await?;

        let timestamp_str = clock.timestamp.to_string();
        let mut timestamp = Timestamp::parse(timestamp_str);
        let other_timestamp = timestamp.clone();
        let timestamp_string = timestamp.send(&other_timestamp);
        clock.timestamp = MutableTimestamp::from(&timestamp);
        Self::set_clock(clock).await;
        Ok(timestamp_string)
    }

    pub async fn insert_timestamp(
        tx: &mut AsyncPgConnection,
        timestamp_str: &String,
    ) -> Result<Clock, Box<dyn std::error::Error>> {
        let mut clock = Self::get_clock_internal(tx, true).await?;
        clock.merkle.add_leaf(timestamp_str);
        clock.merkle.prune_to_level_4();
        Self::set_clock(clock.clone()).await;
        Ok(clock)
    }

    fn make_client_id() -> Result<String, &'static str> {
        let ulid = Ulid::new();
        let ulid_str = ulid.to_string();
        if ulid_str.len() >= 16 {
            let start_index = ulid_str.len() - 16;
            match ulid_str.get(start_index..) {
                Some(client_id) => Ok(client_id.to_string()),
                None => Err("Failed to extract client ID substring"),
            }
        } else {
            Err("Failed to generate client ID: ULID string too short")
        }
    }
}
