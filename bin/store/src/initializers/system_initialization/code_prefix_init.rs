use crate::controllers::store_controller::ApiError;
use crate::generated::models::counter_model::CounterModel;
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
            "accounts".to_string(),
            CounterModel {
                default_code: 100000,
                prefix: "A".to_string(),
                counter: 0,
                entity: "accounts".to_string(),
                digits_number: 6,
            },
        );
        prefixes.insert(
            "account_organizations".to_string(),
            CounterModel {
                default_code: 100000,
                prefix: "AO".to_string(),
                counter: 0,
                entity: "account_organizations".to_string(),
                digits_number: 6,
            },
        );
        prefixes.insert(
            "organizations".to_string(),
            CounterModel {
                default_code: 100000,
                prefix: "O".to_string(),
                counter: 0,
                entity: "organizations".to_string(),
                digits_number: 6,
            },
        );
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

        prefixes.insert(
            "sessions".to_string(),
            CounterModel {
                default_code: 10000,
                prefix: "SE".to_string(),
                counter: 0,
                entity: "sessions".to_string(),
                digits_number: 6,
            },
        );

        prefixes.insert(
            "signed_in_activities".to_string(),
            CounterModel {
                default_code: 10000,
                prefix: "SIA".to_string(),
                counter: 0,
                entity: "signed_in_activities".to_string(),
                digits_number: 6,
            },
        );

        prefixes.insert(
            "samples".to_string(),
            CounterModel {
                default_code: 10000,
                prefix: "SA".to_string(),
                counter: 0,
                entity: "samples".to_string(),
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

    /// Initializes counter config in the code service (counter-service). Requires CODE_SERVICE_GRPC_URL.
    pub async fn initialize(&self) -> Result<(), ApiError> {
        let counters: Vec<(String, String, i32, i32)> = self
            .prefixes
            .values()
            .map(|c| {
                (
                    c.entity.clone(),
                    c.prefix.clone(),
                    c.default_code,
                    c.digits_number,
                )
            })
            .collect();
        crate::utils::code_generator::init_counters(
            &crate::utils::code_generator::database_name_from_env(),
            &counters,
        )
        .await
        .map_err(|e| {
            ApiError::new(
                actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
                e.to_string(),
            )
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
