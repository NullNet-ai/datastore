use crate::controllers::store_controller::ApiError;
use crate::db;
use crate::models::counter_model::CounterModel;
use crate::schema::schema;
// use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodePrefixInitializer {
    pub prefixes: HashMap<String, CounterModel>,
}
#[allow(warnings)]
impl CodePrefixInitializer {
    pub fn new() -> Self {
        let mut prefixes = HashMap::new();

        // Initialize with the example provided
        prefixes.insert(
            "connections".to_string(),
            CounterModel {
                default_code: 10000,
                prefix: "CO".to_string(),
                counter: 0,
                entity: "connections".to_string(),
                digits_number: 6,
            },
        );

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

    pub fn get_config(&self, table_name: &str) -> Option<&CounterModel> {
        self.prefixes.get(table_name)
    }

    pub fn add_config(&mut self, table_name: String, config: CounterModel) {
        self.prefixes.insert(table_name, config);
    }

    /// Inserts all prefix configurations into the counter table in the database
    /// If a record with the same entity already exists, it will be skipped
    pub async fn initialize(&self) -> Result<(), ApiError> {
        let mut conn = db::get_async_connection().await;

        // Process each counter without using a transaction
        for (_, counter) in &self.prefixes {
            // Insert with on_conflict_do_nothing - if the entity already exists, skip it
            diesel::insert_into(schema::counters::table)
                .values(counter)
                .on_conflict_do_nothing()
                .execute(&mut conn)
                .await
                .map_err(|e| {
                    log::error!(
                        "Error inserting counter for entity {}: {}",
                        counter.entity,
                        e
                    );
                    // Convert DieselError to ApiError
                    ApiError::from(e)
                })?;

            log::info!("Initialized counter for entity: {}", counter.entity);
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
