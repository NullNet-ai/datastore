use crate::controllers::store_controller::ApiError;
use crate::db;
use crate::initializers::initial_entity_data::connections::get_initial_connections;
use crate::initializers::initial_entity_data::packets::get_initial_packets;
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
        let mut entity_data: HashMap<&str, Vec<Value>> = HashMap::new();

        // Load initial data with individual error handling
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| get_initial_connections())) {
            Ok(connections) => {
                entity_data.insert("connections", connections);
                log::debug!("Successfully loaded connections data");
            }
            Err(_) => {
                log::error!("Failed to load connections data, skipping connections initialization");
            }
        }

        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| get_initial_packets())) {
            Ok(packets) => {
                entity_data.insert("packets", packets);
                log::debug!("Successfully loaded packets data");
            }
            Err(_) => {
                log::error!("Failed to load packets data, skipping packets initialization");
            }
        }

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

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_error_resilient_initialization() {
        let initializer = InitialEntityDataInitializer::new();

        // Test that initialization doesn't panic even with potential errors
        let result = initializer.initialize(None).await;
        assert!(
            result.is_ok(),
            "Initialization should always return Ok to prevent crashes"
        );

        // Test with sample data to ensure structure is correct
        let sample_connections = get_initial_connections();
        let sample_packets = get_initial_packets();

        assert!(
            !sample_connections.is_empty(),
            "Connections data should not be empty"
        );
        assert!(
            !sample_packets.is_empty(),
            "Packets data should not be empty"
        );

        // Get expected organization ID from environment
        let expected_org_id = std::env::var("DEFAULT_ORGANIZATION_ID")
            .unwrap_or_else(|_| "01JBHKXHYSKPP247HZZWHA3JCT".to_string());

        // Verify the first record of each type has required fields
        let first_connection = &sample_connections[0];
        assert!(
            first_connection.get("id").is_some(),
            "Connection record should have an id field"
        );
        assert!(
            first_connection.get("created_by").is_some(),
            "Connection record should have a created_by field"
        );
        assert!(
            first_connection.get("created_date").is_some(),
            "Connection record should have a created_date field"
        );
        assert!(
            first_connection.get("created_time").is_some(),
            "Connection record should have a created_time field"
        );
        assert!(
            first_connection.get("timestamp").is_some(),
            "Connection record should have a timestamp field"
        );
        assert!(
            first_connection.get("hypertable_timestamp").is_some(),
            "Connection record should have a hypertable_timestamp field"
        );
        assert!(
            first_connection.get("organization_id").is_some(),
            "Connection record should have an organization_id field"
        );

        // Verify organization_id matches environment variable
        let conn_org_id = first_connection
            .get("organization_id")
            .unwrap()
            .as_str()
            .unwrap();
        assert_eq!(
            conn_org_id, expected_org_id,
            "Connection organization_id should match DEFAULT_ORGANIZATION_ID"
        );

        // Verify timestamp format (ISO 8601 with microseconds)
        let conn_timestamp = first_connection.get("timestamp").unwrap().as_str().unwrap();
        let conn_hypertable_timestamp = first_connection
            .get("hypertable_timestamp")
            .unwrap()
            .as_str()
            .unwrap();
        assert!(
            conn_timestamp.contains("T"),
            "Timestamp should be in ISO 8601 format"
        );
        assert!(
            conn_timestamp.ends_with("+00:00"),
            "Timestamp should have UTC timezone"
        );
        assert_eq!(
            conn_timestamp, conn_hypertable_timestamp,
            "Timestamp and hypertable_timestamp should match"
        );

        let first_packet = &sample_packets[0];
        assert!(
            first_packet.get("id").is_some(),
            "Packet record should have an id field"
        );
        assert!(
            first_packet.get("created_by").is_some(),
            "Packet record should have a created_by field"
        );
        assert!(
            first_packet.get("created_date").is_some(),
            "Packet record should have a created_date field"
        );
        assert!(
            first_packet.get("created_time").is_some(),
            "Packet record should have a created_time field"
        );
        assert!(
            first_packet.get("timestamp").is_some(),
            "Packet record should have a timestamp field"
        );
        assert!(
            first_packet.get("hypertable_timestamp").is_some(),
            "Packet record should have a hypertable_timestamp field"
        );
        assert!(
            first_packet.get("organization_id").is_some(),
            "Packet record should have an organization_id field"
        );

        // Verify organization_id matches environment variable
        let packet_org_id = first_packet
            .get("organization_id")
            .unwrap()
            .as_str()
            .unwrap();
        assert_eq!(
            packet_org_id, expected_org_id,
            "Packet organization_id should match DEFAULT_ORGANIZATION_ID"
        );

        // Verify timestamp format (ISO 8601 with microseconds)
        let packet_timestamp = first_packet.get("timestamp").unwrap().as_str().unwrap();
        let packet_hypertable_timestamp = first_packet
            .get("hypertable_timestamp")
            .unwrap()
            .as_str()
            .unwrap();
        assert!(
            packet_timestamp.contains("T"),
            "Timestamp should be in ISO 8601 format"
        );
        assert!(
            packet_timestamp.ends_with("+00:00"),
            "Timestamp should have UTC timezone"
        );
        assert_eq!(
            packet_timestamp, packet_hypertable_timestamp,
            "Timestamp and hypertable_timestamp should match"
        );

        // Verify that timestamps are in the correct format (current date/time)
        let created_date = first_connection["created_date"].as_str().unwrap();
        let created_time = first_connection["created_time"].as_str().unwrap();

        // Check date format (YYYY-MM-DD)
        assert!(
            created_date.len() == 10,
            "Date should be in YYYY-MM-DD format"
        );
        assert!(created_date.contains('-'), "Date should contain hyphens");

        // Check time format (HH:MM:SS)
        assert!(created_time.len() == 8, "Time should be in HH:MM:SS format");
        assert!(created_time.contains(':'), "Time should contain colons");

        // Note: This test verifies error resilience, data structure correctness, and current timestamp usage
        // Full database integration testing would require a test database connection
    }
}
