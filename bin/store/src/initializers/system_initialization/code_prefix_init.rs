use crate::controllers::store_controller::ApiError;
use crate::database::db;
use crate::generated::models::counter_model::CounterModel;
use crate::generated::schema::counters;
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

        prefixes.insert(
            "contacts".to_string(),
            CounterModel {
                default_code: 100000,
                prefix: "CO".to_string(),
                counter: 0,
                entity: "contacts".to_string(),
                digits_number: 6,
            },
        );
        prefixes.insert(
            "user_roles".to_string(),
            CounterModel {
                default_code: 100000,
                prefix: "RO".to_string(),
                counter: 0,
                entity: "user_roles".to_string(),
                digits_number: 6,
            },
        );
        prefixes.insert(
            "organizations".to_string(),
            CounterModel {
                default_code: 100000,
                prefix: "OR".to_string(),
                counter: 0,
                entity: "organizations".to_string(),
                digits_number: 6,
            },
        );
        prefixes.insert(
            "notifications".to_string(),
            CounterModel {
                default_code: 100000,
                prefix: "NO".to_string(),
                counter: 0,
                entity: "notifications".to_string(),
                digits_number: 6,
            },
        );
        prefixes.insert(
            "communication_templates".to_string(),
            CounterModel {
                default_code: 100000,
                prefix: "CT".to_string(),
                counter: 0,
                entity: "communication_templates".to_string(),
                digits_number: 6,
            },
        );
        prefixes.insert(
            "account_organizations".to_string(),
            CounterModel {
                default_code: 100000,
                prefix: "AC".to_string(),
                counter: 0,
                entity: "account_organizations".to_string(),
                digits_number: 6,
            },
        );
        prefixes.insert(
            "devices".to_string(),
            CounterModel {
                default_code: 100000,
                prefix: "DV".to_string(),
                counter: 0,
                entity: "devices".to_string(),
                digits_number: 6,
            },
        );
        prefixes.insert(
            "courses".to_string(),
            CounterModel {
                default_code: 100000,
                prefix: "CS".to_string(),
                counter: 0,
                entity: "courses".to_string(),
                digits_number: 6,
            },
        );
        prefixes.insert(
            "classrooms".to_string(),
            CounterModel {
                default_code: 100000,
                prefix: "CL".to_string(),
                counter: 0,
                entity: "classrooms".to_string(),
                digits_number: 6,
            },
        );
        prefixes.insert(
            "faqs".to_string(),
            CounterModel {
                default_code: 100000,
                prefix: "FAQ".to_string(),
                counter: 0,
                entity: "faqs".to_string(),
                digits_number: 6,
            },
        );
        prefixes.insert(
            "conversation_topics".to_string(),
            CounterModel {
                default_code: 100000,
                prefix: "CT".to_string(),
                counter: 0,
                entity: "conversation_topics".to_string(),
                digits_number: 6,
            },
        );
        prefixes.insert(
            "conversations".to_string(),
            CounterModel {
                default_code: 100000,
                prefix: "INT".to_string(),
                counter: 0,
                entity: "conversations".to_string(),
                digits_number: 6,
            },
        );
        prefixes.insert(
            "smtp_payloads".to_string(),
            CounterModel {
                default_code: 100000,
                prefix: "CM".to_string(),
                counter: 0,
                entity: "smtp_payloads".to_string(),
                digits_number: 6,
            },
        );
        prefixes.insert(
            "smtp_transactions".to_string(),
            CounterModel {
                default_code: 100000,
                prefix: "TR".to_string(),
                counter: 0,
                entity: "smtp_transactions".to_string(),
                digits_number: 6,
            },
        );
        prefixes.insert(
            "sponsorships".to_string(),
            CounterModel {
                default_code: 100000,
                prefix: "SP".to_string(),
                counter: 0,
                entity: "sponsorships".to_string(),
                digits_number: 6,
            },
        );
        prefixes.insert(
            "game_stats".to_string(),
            CounterModel {
                default_code: 100000,
                prefix: "GS".to_string(),
                counter: 0,
                entity: "game_stats".to_string(),
                digits_number: 6,
            },
        );
        prefixes.insert(
            "stories".to_string(),
            CounterModel {
                default_code: 100000,
                prefix: "ST".to_string(),
                counter: 0,
                entity: "stories".to_string(),
                digits_number: 6,
            },
        );
        prefixes.insert(
            "episodes".to_string(),
            CounterModel {
                default_code: 100000,
                prefix: "EP".to_string(),
                counter: 0,
                entity: "episodes".to_string(),
                digits_number: 6,
            },
        );
        prefixes.insert(
            "reports".to_string(),
            CounterModel {
                default_code: 100000,
                prefix: "RP".to_string(),
                counter: 0,
                entity: "reports".to_string(),
                digits_number: 6,
            },
        );
        prefixes.insert(
            "game_questions".to_string(),
            CounterModel {
                default_code: 100000,
                prefix: "QU".to_string(),
                counter: 0,
                entity: "game_questions".to_string(),
                digits_number: 6,
            },
        );
        prefixes.insert(
            "game_choices".to_string(),
            CounterModel {
                default_code: 100000,
                prefix: "CH".to_string(),
                counter: 0,
                entity: "game_choices".to_string(),
                digits_number: 6,
            },
        );

        CodePrefixInitializer { prefixes }
    }

    /// Inserts all prefix configurations into the counter table in the database
    /// If a record with the same entity already exists, it will update the prefix and default_code
    pub async fn initialize(&self) -> Result<(), ApiError> {
        let mut conn = db::get_async_connection().await;

        // Process each counter without using a transaction
        for (_, counter) in &self.prefixes {
            // Insert with on_conflict_do_update - if the entity already exists, update prefix and default_code
            diesel::insert_into(counters::table)
                .values(counter)
                .on_conflict(counters::entity)
                .do_update()
                .set((
                    counters::prefix.eq(diesel::upsert::excluded(counters::prefix)),
                    counters::default_code.eq(diesel::upsert::excluded(counters::default_code)),
                    counters::digits_number
                        .eq(diesel::upsert::excluded(counters::digits_number)),
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
