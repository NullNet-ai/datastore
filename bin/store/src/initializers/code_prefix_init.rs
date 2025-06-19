use crate::db;
use crate::models::counter_model::CounterModel;
use crate::schema::schema;
use diesel::associations::HasTable;
use diesel::prelude::*;
use diesel::result::Error as DieselError;
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
            "connections".to_string(),
            CounterModel {
                default_code: 10000,
                prefix: "PA".to_string(),
                counter: 0,
                entity: "packets".to_string(),
                digits_number: 1,
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
    pub fn initialize_counter_table(&self) -> Result<(), DieselError> {
        let mut conn = db::get_sync_connection();

        // Start a transaction to ensure all inserts succeed or fail together
        conn.transaction::<_, DieselError, _>(|conn| {
            for (_, counter) in &self.prefixes {
                // Insert with on_conflict_do_nothing - if the entity already exists, skip it
                diesel::insert_into(schema::counters::dsl::counters::table())
                    .values(counter)
                    .on_conflict_do_nothing()
                    .execute(conn)
                    .map_err(|e| {
                        log::error!(
                            "Error inserting counter for entity {}: {}",
                            counter.entity,
                            e
                        );
                        e
                    })?;

                log::info!("Initialized counter for entity: {}", counter.entity);
            }

            Ok(())
        })
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
