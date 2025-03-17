use crate::db::DbPooledConnection;
use crate::structs::structs::Clock;
use crate::sync::hlc::mutable_timestamp::MutableTimestamp;
use crate::sync::merkles::merkle_service::MerkleService;
use hlc::Timestamp;
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

    fn make_clock(timestamp: Timestamp, merkle: Value) -> Clock {
        Clock {
            timestamp: MutableTimestamp::from(&timestamp),
            merkle,
        }
    }

    pub fn get_clock(tx: &mut DbPooledConnection) -> Result<Clock, Box<dyn std::error::Error>> {
        let group_id = env::var("GROUP_ID").unwrap_or_else(|_| "my-group".to_string());
        let merkle_service = MerkleService {};
        let clock = merkle_service.get_merkles_by_group_id(group_id, tx);
        match clock {
            Ok(Some(clock)) => {
                //destructure timestamp and merkle from clock
                let timestamp = clock.timestamp;
                let merkle = clock.merkle;
                Ok(Self::make_clock(Timestamp::parse(timestamp), merkle))
            }
            Ok(None) => {
                let timestamp = Timestamp::new(0, 0, Self::make_client_id());
                let clock: Clock = Self::make_clock(timestamp, serde_json::json!({}));
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

    fn make_client_id() -> String {
        let uuid = Uuid::new_v4();
        let uuid_str = uuid.to_string();
        let no_hyphens = uuid_str.replace("-", "");
        no_hyphens[no_hyphens.len() - 16..].to_string()
    }
}
