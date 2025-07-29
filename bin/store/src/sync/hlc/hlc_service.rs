use crate::structs::structs::Clock;
use crate::sync::hlc::mutable_timestamp::MutableTimestamp;
use crate::sync::merkles::merkle_manager::MerkleManager;
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
        // Get the current clock
        let old_clock = Self::get_clock(tx).await?;

        // Create new clock with old timestamp and new tree
        let clock = Self::make_clock(
            Timestamp::parse(old_clock.timestamp.to_string()),
            tree.clone(),
        );

        // Save the updated clock and return the result
        Ok(Self::set_clock(clock).await)
    }

    pub async fn recv(
        tx: &mut AsyncPgConnection,
        timestamp_str: String,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut clock = Self::get_clock(tx).await?;

        let timestamp = Timestamp::parse(timestamp_str);

        let mut current_timestamp = Timestamp::parse(clock.timestamp.to_string());
        current_timestamp.recv(&timestamp);

        clock.timestamp = MutableTimestamp::from(&current_timestamp);

        Self::set_clock(clock).await;

        Ok(())
    }

    pub async fn get_clock(
        _tx: &mut AsyncPgConnection,
    ) -> Result<Clock, Box<dyn std::error::Error>> {
        let group_id = env::var("GROUP_ID").unwrap_or_else(|_| "my-group".to_string());
        let merkle_manager = MerkleManager::instance();

        let tree_result = merkle_manager.get_tree(&group_id).await;
        //print clock
        match tree_result {
            Some((merkle, timestamp)) => {
                // Found in memory
                Ok(Self::make_clock(Timestamp::parse(timestamp), merkle))
            }
            None => {
                // Not found in memory, create new
                let timestamp = Timestamp::new(0, 0, Self::make_client_id()?);
                let clock: Clock = Self::make_clock(timestamp, MerkleTree::new());

                // Save to memory
                Self::set_clock(clock.clone()).await;
                Ok(clock)
            }
        }
    }

    pub async fn send(tx: &mut AsyncPgConnection) -> Result<String, Box<dyn std::error::Error>> {
        let mut clock = Self::get_clock(tx).await?;

        // Parse the timestamp from the clock
        let timestamp_str = clock.timestamp.to_string();
        let mut timestamp = Timestamp::parse(timestamp_str);

        // Create a new timestamp for comparison (to avoid borrowing issues)
        let other_timestamp = timestamp.clone();

        // Call send on the timestamp
        let timestamp_string = timestamp.send(&other_timestamp);

        // Update the clock with the new timestamp
        clock.timestamp = MutableTimestamp::from(&timestamp);

        // Save the updated clock
        Self::set_clock(clock).await;

        // Return the timestamp string
        Ok(timestamp_string)
    }

    pub async fn insert_timestamp(
        tx: &mut AsyncPgConnection,
        timestamp_str: &String,
    ) -> Result<Clock, Box<dyn std::error::Error>> {
        let mut clock = Self::get_clock(tx).await?;

        // Modify the merkle tree directly without cloning
        clock.merkle.add_leaf(&timestamp_str);
        clock.merkle.prune_to_level_4();

        // Save the updated clock - only one clone needed here
        Self::set_clock(clock.clone()).await;

        // Return the updated clock
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
