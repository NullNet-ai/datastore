use crate::db::DbPooledConnection;
use crate::models::crdt_merkle_model::{GetMerkle, ParsedMerkle};
use crate::schema::schema::crdt_merkles;
use diesel::prelude::*;
use diesel::result::Error as DieselError;
use serde_json::Value;

pub struct MerkleService {}

impl MerkleService {
    pub fn get_merkles_by_group_id(
        &self,
        group_id: String,
        tx: &mut DbPooledConnection,
    ) -> Result<Option<ParsedMerkle>, DieselError> {
        let merkles = crdt_merkles::table
            .filter(crdt_merkles::group_id.eq(group_id))
            .load::<GetMerkle>(tx)?;
        if merkles.is_empty() {
            return Ok(None);
        }
        let merkle = merkles.get(0).ok_or(DieselError::NotFound)?;
        let parsed = ParsedMerkle {
            group_id: merkle.group_id.clone(),
            timestamp: merkle.timestamp.clone(),
            merkle: if merkle.merkle.is_empty() {
                serde_json::json!({})
            } else {
                serde_json::from_str(&merkle.merkle).unwrap_or_else(|_| serde_json::json!({}))
            },
        };

        Ok(Some(parsed))
    }

    pub fn set_merkles_by_group_id(
        &self,
        group_id: String,
        timestamp: String,
        merkle: Value,
        tx: &mut DbPooledConnection,
    ) -> Result<(), DieselError> {
        let merkle = serde_json::to_string(&merkle).unwrap_or_else(|_| "{}".to_string());
        let exists = crdt_merkles::table
            .filter(crdt_merkles::group_id.eq(&group_id))
            .first::<GetMerkle>(tx)
            .optional()?
            .is_some();

        if exists {
            // Update existing record
            diesel::update(crdt_merkles::table.filter(crdt_merkles::group_id.eq(&group_id)))
                .set((
                    crdt_merkles::timestamp.eq(&timestamp),
                    crdt_merkles::merkle.eq(&merkle),
                ))
                .execute(tx)?;
        } else {
            // Insert new record
            diesel::insert_into(crdt_merkles::table)
                .values((
                    crdt_merkles::group_id.eq(group_id),
                    crdt_merkles::timestamp.eq(timestamp),
                    crdt_merkles::merkle.eq(merkle),
                ))
                .execute(tx)?;
        }

        Ok(())
    }
}
