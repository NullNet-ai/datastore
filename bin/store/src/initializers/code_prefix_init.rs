use crate::controllers::store_controller::ApiError;
use crate::db;
use crate::models::counter_model::CounterModel;
use crate::schema::schema;
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodePrefixInitializer {
    pub prefixes: HashMap<String, CounterModel>,
}
impl CodePrefixInitializer {
    pub fn new() -> Self {
        let mut prefixes = HashMap::new();

        // Initialize with the example provided
        prefixes.insert(
            "devices".to_string(),
            CounterModel {
                default_code: 10000,
                prefix: "DV".to_string(),
                counter: 0,
                entity: "devices".to_string(),
                digits_number: 6,
            },
        );

        // Add more table configurations as needed
        // Example:
        // prefixes.insert(
        //     "devices".to_string(),
        //     CounterModel {
        //         default_code: 20000,
        //         prefix: "DV".to_string(),
        //         counter: 0,
        //         entity: "devices".to_string(),
        //     },
        // );

        CodePrefixInitializer { prefixes }
    }

    /// Inserts all prefix configurations into the counter table in the database
    /// If a record with the same entity already exists, it will update the prefix and default_code
    pub async fn initialize(&self) -> Result<(), ApiError> {
        let mut conn = db::get_async_connection().await;

        // Process each counter without using a transaction
        for (_, counter) in &self.prefixes {
            // Insert with on_conflict_do_update - if the entity already exists, update prefix and default_code
            diesel::insert_into(schema::counters::table)
                .values(counter)
                .on_conflict(schema::counters::entity)
                .do_update()
                .set((
                    schema::counters::prefix.eq(diesel::upsert::excluded(schema::counters::prefix)),
                    schema::counters::default_code
                        .eq(diesel::upsert::excluded(schema::counters::default_code)),
                ))
                .execute(&mut conn)
                .await
                .map_err(|e| {
                    log::error!(
                        "Error inserting/updating counter for entity {}: {}",
                        counter.entity,
                        e
                    );
                    // Convert DieselError to ApiError
                    ApiError::from(e)
                })?;

            log::info!("Initialized/updated counter for entity: {}", counter.entity);
        }

        Ok(())
    }
}

// Create a singleton instance that can be accessed throughout the application
lazy_static::lazy_static! {
    pub static ref CODE_PREFIX_INITIALIZER: CodePrefixInitializer = CodePrefixInitializer::new();
}

// Helper function to get the initializer instance
pub fn get_code_prefix_initializer() -> &'static CodePrefixInitializer {
    &CODE_PREFIX_INITIALIZER
}
