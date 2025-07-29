use crate::controllers::store_controller::ApiError;
use crate::db;
// use crate::initializers::initial_entity_data::connections::get_initial_connections;
use crate::initializers::structs::InitializerParams;
use crate::schema::verify::{field_exists_in_table, get_table_fields};
use crate::sync::sync_service;
use crate::table_enum::{generate_code, Table};
use actix_web::http::StatusCode;
use serde_json::Value;
use std::collections::{HashMap, HashSet};

pub struct InitialEntityDataInitializer;

impl InitialEntityDataInitializer {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn initialize(&self, _params: Option<InitializerParams>) -> Result<(), ApiError> {
        log::info!("Starting initial entity data initialization");

        // Define entity data mappings with error handling
        let entity_data: HashMap<&str, Vec<Value>> = HashMap::new();

        // Load initial data with individual error handling
        // match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| get_initial_connections())) {
        //     Ok(connections) => {
        //         entity_data.insert("connections", connections);
        //         log::debug!("Successfully loaded connections data");
        //     }
        //     Err(_) => {
        //         log::error!("Failed to load connections data, skipping connections initialization");
        //     }
        // }


        if entity_data.is_empty() {
            log::warn!("No entity data could be loaded, initialization completed with no changes");
            return Ok(());
        }

        let mut successful_entities = 0;
        let total_entities = entity_data.len();

        // Process each entity with comprehensive error handling
        for (table_name, data_records) in entity_data {
            match self.process_entity_safe(table_name, data_records).await {
                Ok(_) => {
                    successful_entities += 1;
                    log::info!("Successfully processed entity: {}", table_name);
                }
                Err(e) => {
                    log::error!(
                        "Failed to process entity {}: {}. Continuing with next entity.",
                        table_name,
                        e
                    );
                    // Continue with other entities instead of failing completely
                }
            }
        }

        log::info!(
            "Initial entity data initialization completed. Successfully processed {}/{} entities",
            successful_entities,
            total_entities
        );

        // Always return Ok to prevent application crash, even if some entities failed
        Ok(())
    }

    async fn process_entity_safe(
        &self,
        table_name: &str,
        data_records: Vec<Value>,
    ) -> Result<(), ApiError> {
        // Wrap entity processing in comprehensive error handling
        match self.process_entity(table_name, data_records).await {
            Ok(_) => Ok(()),
            Err(e) => {
                log::error!(
                    "Entity processing failed for {}: {}. This will not crash the application.",
                    table_name,
                    e
                );
                Err(e)
            }
        }
    }

    async fn process_entity(
        &self,
        table_name: &str,
        data_records: Vec<Value>,
    ) -> Result<(), ApiError> {
        log::info!(
            "Processing entity: {} with {} records",
            table_name,
            data_records.len()
        );

        // Verify that the table exists in schema
        if !self.verify_table_exists(table_name) {
            return Err(ApiError::new(
                StatusCode::BAD_REQUEST,
                format!("Table '{}' does not exist in schema", table_name),
            ));
        }

        let mut successful_records = 0;
        let total_records = data_records.len();

        // Process each record with individual error handling
        for (index, record) in data_records.iter().enumerate() {
            // The process_record method now handles its own errors internally
            match self.process_record(table_name, record).await {
                Ok(_) => {
                    successful_records += 1;
                    log::debug!(
                        "Successfully processed record {} for table {}",
                        index,
                        table_name
                    );
                }
                Err(e) => {
                    // This should rarely happen since process_record handles errors internally
                    log::error!(
                        "Unexpected error processing record {} for table {}: {}",
                        index,
                        table_name,
                        e
                    );
                }
            }
        }

        log::info!(
            "Completed processing entity {}: {}/{} records successful",
            table_name,
            successful_records,
            total_records
        );

        // Always return Ok to prevent entity-level failures from stopping the process
        Ok(())
    }

    async fn process_record(&self, table_name: &str, record: &Value) -> Result<(), ApiError> {
        // Wrap the entire record processing in a comprehensive error handler
        let result = self.process_record_internal(table_name, record).await;

        match result {
            Ok(_) => {
                log::debug!("Successfully processed record for table {}", table_name);
                Ok(())
            }
            Err(e) => {
                // Log the error but don't propagate it to prevent application crash
                log::error!(
                    "Error processing record for table {}: {}. Continuing with next record.",
                    table_name,
                    e
                );
                // Return Ok to continue processing other records
                Ok(())
            }
        }
    }

    async fn process_record_internal(
        &self,
        table_name: &str,
        record: &Value,
    ) -> Result<(), ApiError> {
        // Clone the record so we can modify it
        let mut record_clone = record.clone();

        // Extract the record ID for existence check
        let record_id = record_clone
            .get("id")
            .and_then(|v| v.as_str())
            .ok_or_else(|| {
                ApiError::new(
                    StatusCode::BAD_REQUEST,
                    format!("Record missing 'id' field for table {}", table_name),
                )
            })?;

        // Create table enum and database connection
        let table = Table::from_str(table_name).ok_or_else(|| {
            ApiError::new(
                StatusCode::BAD_REQUEST,
                format!("Unknown table: {}", table_name),
            )
        })?;

        let mut conn = db::get_async_connection().await;

        // Check if record already exists using get_by_id
        let existing_record = table
            .get_by_id(&mut conn, record_id, true, None)
            .await
            .map_err(|e| {
                ApiError::new(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Failed to check if record exists in {}: {}", table_name, e),
                )
            })?;

        // If record already exists, skip insertion
        if existing_record.is_some() {
            log::info!(
                "Record with ID '{}' already exists in table '{}', skipping insertion",
                record_id,
                table_name
            );
            return Ok(());
        }

        // Generate and assign code to the record
        if let Some(obj) = record_clone.as_object_mut() {
            // Generate code using the table name, empty prefix, and default code 100000
            let code = generate_code(table_name, "", 100000).await.map_err(|e| {
                ApiError::new(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Failed to generate code for {}: {}", table_name, e),
                )
            })?;

            // Assign the generated code to the record
            obj.insert("code".to_string(), Value::String(code));

            log::debug!(
                "Generated and assigned code to record for table {}",
                table_name
            );
        }

        // Validate record structure against model
        if let Err(e) = self.validate_record_structure(table_name, &record_clone) {
            return Err(e);
        }

        // Insert record using sync service
        let table_string = table_name.to_string();
        if let Err(e) = sync_service::insert(&table_string, record_clone).await {
            return Err(ApiError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to insert record into {}: {}", table_name, e),
            ));
        }

        log::debug!("Successfully inserted new record into {}", table_name);
        Ok(())
    }

    fn verify_table_exists(&self, table_name: &str) -> bool {
        // Check if the table has at least one common field (like 'id')
        // This is a simple way to verify table existence
        field_exists_in_table(table_name, "id")
    }

    fn validate_record_structure(&self, table_name: &str, record: &Value) -> Result<(), ApiError> {
        if let Some(obj) = record.as_object() {
            // Get all valid fields for the table at once
            if let Some(valid_fields) = get_table_fields(table_name) {
                let valid_fields_set: HashSet<String> = valid_fields.into_iter().collect();

                // Validate that record fields exist in the schema
                for field_name in obj.keys() {
                    if !valid_fields_set.contains(field_name) {
                        log::warn!(
                            "Field '{}' does not exist in table '{}' schema, but continuing",
                            field_name,
                            table_name
                        );
                        // Don't fail for unknown fields, just warn
                    }
                }
            } else {
                log::warn!(
                    "Could not retrieve field list for table '{}', skipping field validation",
                    table_name
                );
            }

            // Ensure 'id' field exists as it's typically required
            if !obj.contains_key("id") {
                return Err(ApiError::new(
                    StatusCode::BAD_REQUEST,
                    format!(
                        "Record for table '{}' missing required 'id' field",
                        table_name
                    ),
                ));
            }

            Ok(())
        } else {
            Err(ApiError::new(
                StatusCode::BAD_REQUEST,
                format!(
                    "Invalid record format for table '{}': expected JSON object",
                    table_name
                ),
            ))
        }
    }
}

pub fn get_initial_entity_data_initializer() -> InitialEntityDataInitializer {
    InitialEntityDataInitializer::new()
}
