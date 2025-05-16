use crate::models::crdt_merkle_model::{CrdtMerkleModel, ParsedMerkle};
use crate::schema::schema::crdt_merkles;
use diesel::prelude::*;
use diesel::result::Error as DieselError;
use diesel::OptionalExtension;
use diesel_async::AsyncPgConnection;
use diesel_async::RunQueryDsl;
use merkle::MerkleTree;

pub struct MerkleService {}

impl MerkleService {
    pub async fn get_merkles_by_group_id(
        &self,
        group_id: String,
        tx: &mut AsyncPgConnection,
    ) -> Result<Option<ParsedMerkle>, DieselError> {
        let merkles = crdt_merkles::table
            .filter(crdt_merkles::group_id.eq(group_id))
            .load::<CrdtMerkleModel>(tx)
            .await?;
        if merkles.is_empty() {
            return Ok(None);
        }
        let merkle = merkles.get(0).ok_or(DieselError::NotFound)?;
        let parsed = ParsedMerkle {
            group_id: merkle.group_id.clone(),
            timestamp: merkle.timestamp.clone(),
            merkle: if merkle.merkle.is_empty() {
                MerkleTree::new()
            } else {
                MerkleTree::deserialize(&merkle.merkle).map_err(|e| {
                    log::error!("Failed to deserialize merkle tree: {}", e);
                    DieselError::DeserializationError(Box::new(e))
                })?
            },
        };

        Ok(Some(parsed))
    }

    pub async fn set_merkles_by_group_id(
        &self,
        group_id: String,
        timestamp: String,
        merkle: MerkleTree,
        tx: &mut AsyncPgConnection,
    ) -> Result<(), DieselError> {
        let merkle = merkle.serialize().map_err(|e| {
            log::error!(
                "Failed to serialize merkle tree in set_merkles_by_group_id for group_id {}: {}",
                group_id,
                e
            );
            DieselError::RollbackTransaction
        })?;
        let exists = crdt_merkles::table
            .filter(crdt_merkles::group_id.eq(&group_id))
            .first::<CrdtMerkleModel>(tx)
            .await
            .optional()?
            .is_some();

        if exists {
            // Update existing record
            diesel::update(crdt_merkles::table.filter(crdt_merkles::group_id.eq(&group_id)))
                .set((
                    crdt_merkles::timestamp.eq(&timestamp),
                    crdt_merkles::merkle.eq(&merkle),
                ))
                .execute(tx)
                .await
                .map_err(|e| {
                    log::error!("Failed to update merkle tree: {}", e);
                    e
                })?;
        } else {
            // Insert new record
            diesel::insert_into(crdt_merkles::table)
                .values((
                    crdt_merkles::group_id.eq(group_id),
                    crdt_merkles::timestamp.eq(timestamp),
                    crdt_merkles::merkle.eq(merkle),
                ))
                .execute(tx)
                .await
                .map_err(|e| {
                    log::error!("Failed to insert merkle tree: {}", e);
                    e
                })?;
        }

        Ok(())
    }

    pub async fn get_all_group_ids(
        &self,
        conn: &mut AsyncPgConnection,
    ) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        // Use Diesel's query builder to get all distinct group_ids
        let group_ids = crdt_merkles::table
            .select(crdt_merkles::group_id)
            .distinct()
            .load::<String>(conn)
            .await?;

        Ok(group_ids)
    }
}
