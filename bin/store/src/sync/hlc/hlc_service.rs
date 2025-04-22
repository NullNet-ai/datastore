use crate::db::DbPooledConnection;
use crate::structs::structs::Clock;
use crate::sync::hlc::mutable_timestamp::MutableTimestamp;
use crate::sync::merkles::merkle_service::MerkleService;
use diesel::deserialize;
use hlc::Timestamp;
use merkle::MerkleTree;
use serde_json::Value;
use std::env;
use uuid::Uuid;

pub struct HlcService {
    pub timestamp: Timestamp,
    pub group_id: String,
}

impl HlcService {
    fn set_clock(tx: &mut DbPooledConnection, clock: Clock) {
        let group_id = env::var("GROUP_ID").unwrap_or_else(|_| "my-group".to_string());
        let merkle_service = MerkleService {};
        // Convert timestamp to string
        let timestamp_str = clock.timestamp.to_string();
        //convert merkle to string
        let merkle_str = serde_json::to_string(&clock.merkle).unwrap();
        //sending {} as merkle for now
        // Call the merkle service to set merkles by group id
        merkle_service
            .set_merkles_by_group_id(group_id, timestamp_str, clock.merkle, tx)
            .expect("Failed to set merkles");
    }

    fn make_clock(timestamp: Timestamp, merkle: MerkleTree) -> Clock {
        Clock {
            timestamp: MutableTimestamp::from(&timestamp),
            merkle,
        }
    }

    pub fn commit_tree(
        tx: &mut DbPooledConnection,
        tree: &MerkleTree,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // Get the current clock
        let old_clock = Self::get_clock(tx)?;

        // Create new clock with old timestamp and new tree
        let clock = Self::make_clock(
            Timestamp::parse(old_clock.timestamp.to_string()),
            tree.clone(),
        );

        // Save the updated clock
        Self::set_clock(tx, clock);

        Ok(())
    }

    pub fn recv(
        tx: &mut DbPooledConnection,
        timestamp_str: String,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut clock = Self::get_clock(tx)?;

        let timestamp = Timestamp::parse(timestamp_str);

        let mut current_timestamp = Timestamp::parse(clock.timestamp.to_string());
        current_timestamp.recv(&timestamp);

        clock.timestamp = MutableTimestamp::from(&current_timestamp);

        Self::set_clock(tx, clock);

        Ok(())
    }

    pub fn get_clock(tx: &mut DbPooledConnection) -> Result<Clock, Box<dyn std::error::Error>> {
        let group_id = env::var("GROUP_ID").unwrap_or_else(|_| "my-group".to_string());
        let merkle_service = MerkleService {};
        let clock = merkle_service.get_merkles_by_group_id(group_id, tx);
        //print clock
        match clock {
            Ok(Some(clock)) => {
                //destructure timestamp and merkle from clock
                let timestamp = clock.timestamp;
                let merkle = clock.merkle;

                //print merkle
                Ok(Self::make_clock(Timestamp::parse(timestamp), merkle))
            }
            Ok(None) => {
                let timestamp = Timestamp::new(0, 0, Self::make_client_id()?);
                let clock: Clock = Self::make_clock(timestamp, MerkleTree::new());
                Self::set_clock(tx, clock);
                Self::get_clock(tx)
            }
            Err(e) => Err(Box::new(e)),
        }
    }

    pub fn send(tx: &mut DbPooledConnection) -> Result<String, Box<dyn std::error::Error>> {
        let mut clock = Self::get_clock(tx)?;

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
        Self::set_clock(tx, clock);

        // Return the timestamp string
        Ok(timestamp_string)
    }

    pub fn insert_timestamp(
        tx: &mut DbPooledConnection,
        timestamp_str: &String,
    ) -> Result<Clock, Box<dyn std::error::Error>> {
        // Get the current clock
        let mut clock = Self::get_clock(tx)?;

        // Create a new MerkleTree and add the timestamp
        let mut merkle_tree = MerkleTree::new();
        merkle_tree.add_leaf(&timestamp_str);

        // Convert the merkle tree to a Value and update the clock's merkle
        clock.merkle = merkle_tree;

        // Save the updated clock
        Self::set_clock(tx, clock.clone());

        // Return the updated clock
        Ok(clock)
    }

    fn make_client_id() -> Result<String, &'static str> {
        let uuid = Uuid::new_v4();
        let uuid_str = uuid.to_string();
        let no_hyphens = uuid_str.replace("-", "");
        if no_hyphens.len() >= 16 {
            let start_index = no_hyphens.len() - 16;
            match no_hyphens.get(start_index..) {
                Some(client_id) => Ok(client_id.to_string()),
                None => Err("Failed to extract client ID substring"),
            }
        } else {
            Err("Failed to generate client ID: UUID string too short")
        }
    }
}
