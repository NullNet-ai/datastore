use crate::controllers::store_controller::ApiError;
use crate::initializers::generate_schema::{GenerateSchemaService, GenerateSchemaOptions};
use actix_web::http::StatusCode;
use redis::Client;
use std::env;

#[derive(Debug, Clone)]
pub struct GenerateSchemaInitializer {
    pub options: GenerateSchemaOptions,
}

impl GenerateSchemaInitializer {
    pub fn new() -> Self {
        // Default options for schema generation
        let mut options = GenerateSchemaOptions::default();
        
        // Configure CRDT tables based on environment
        if env::var("INCLUDE_CRDT_TABLES")
            .unwrap_or_else(|_| "true".to_string())
            .to_lowercase()
            == "true" 
        {
            // Add common CRDT table names
            options.include_crdt_tables = vec![
                "crdt_entries".to_string(),
                "crdt_states".to_string(),
                "crdt_operations".to_string(),
            ];
        }

        // Configure formatting field exclusions
        if env::var("EXCLUDE_FORMATTING_FIELDS")
            .unwrap_or_else(|_| "true".to_string())
            .to_lowercase()
            == "true" 
        {
            // Add common formatting field names to exclude
            options.exclude_formatting_fields = vec![
                "created_at".to_string(),
                "updated_at".to_string(),
                "deleted_at".to_string(),
            ];
        }

        GenerateSchemaInitializer { options }
    }

    /// Generates the application schema and stores configuration
    pub async fn initialize(&self, _params: Option<crate::initializers::system_initialization::structs::InitializerParams>) -> Result<(), ApiError> {
        // Create Redis client for the service
        let redis_url = env::var("REDIS_URL").unwrap_or_else(|_| "redis://127.0.0.1:6379".to_string());
        let redis_client = Client::open(redis_url)
            .map_err(|e| {
                log::error!("Failed to create Redis client: {}", e);
                ApiError::new(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Failed to create Redis client"
                )
            })?;

        // Create the schema service
        let schema_service = GenerateSchemaService::new(redis_client);

        // Generate the schema
        schema_service.generate_schema(self.options.clone()).await
            .map_err(|e| {
                log::error!("Failed to generate schema: {}", e);
                ApiError::new(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Failed to generate application schema"
                )
            })?;

        log::info!("Successfully generated application schema");
        Ok(())
    }
}

// Create a singleton instance that can be accessed throughout the application
lazy_static::lazy_static! {
    pub static ref GENERATE_SCHEMA_INITIALIZER: GenerateSchemaInitializer = GenerateSchemaInitializer::new();
}

// Helper function to get the initializer instance
pub fn get_generate_schema_initializer() -> &'static GenerateSchemaInitializer {
    &GENERATE_SCHEMA_INITIALIZER
}