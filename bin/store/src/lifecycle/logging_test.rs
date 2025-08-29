#[cfg(test)]
mod tests {
    use crate::lifecycle::logging::{LifecycleLogger, LogCategory, LogConfig, LogLevel};
    /// Test correlation counter functionality
    #[tokio::test]
    async fn should_generate_unique_correlation_ids() {
        println!("Testing correlation ID generation and uniqueness");

        let logger = LifecycleLogger::default();

        // Generate multiple correlation IDs
        let id1 = logger.generate_correlation_id().await;
        let id2 = logger.generate_correlation_id().await;
        let id3 = logger.generate_correlation_id().await;

        println!("Generated correlation IDs:");
        println!("  ID1: {}", id1);
        println!("  ID2: {}", id2);
        println!("  ID3: {}", id3);

        // Verify IDs are unique
        assert_ne!(id1, id2, "Correlation IDs should be unique");
        assert_ne!(id2, id3, "Correlation IDs should be unique");
        assert_ne!(id1, id3, "Correlation IDs should be unique");

        // Verify format (should start with "lc-")
        assert!(
            id1.starts_with("lc-"),
            "Correlation ID should start with 'lc-'"
        );
        assert!(
            id2.starts_with("lc-"),
            "Correlation ID should start with 'lc-'"
        );
        assert!(
            id3.starts_with("lc-"),
            "Correlation ID should start with 'lc-'"
        );

        // Verify format structure (lc-{timestamp}-{counter})
        let id1_parts: Vec<&str> = id1.split('-').collect();
        let id2_parts: Vec<&str> = id2.split('-').collect();
        let id3_parts: Vec<&str> = id3.split('-').collect();

        assert_eq!(
            id1_parts.len(),
            3,
            "Correlation ID should have 3 parts separated by '-'"
        );
        assert_eq!(
            id2_parts.len(),
            3,
            "Correlation ID should have 3 parts separated by '-'"
        );
        assert_eq!(
            id3_parts.len(),
            3,
            "Correlation ID should have 3 parts separated by '-'"
        );

        // Verify timestamp and counter are hexadecimal
        assert!(
            id1_parts[1].chars().all(|c| c.is_ascii_hexdigit()),
            "Timestamp should be hexadecimal"
        );
        assert!(
            id1_parts[2].chars().all(|c| c.is_ascii_hexdigit()),
            "Counter should be hexadecimal"
        );

        // Verify counter increments
        let counter1 = u64::from_str_radix(id1_parts[2], 16).expect("Counter should be valid hex");
        let counter2 = u64::from_str_radix(id2_parts[2], 16).expect("Counter should be valid hex");
        let counter3 = u64::from_str_radix(id3_parts[2], 16).expect("Counter should be valid hex");

        assert_eq!(counter2, counter1 + 1, "Counter should increment by 1");
        assert_eq!(counter3, counter2 + 1, "Counter should increment by 1");

        println!("✓ All correlation IDs are unique and properly formatted");
    }

    /// Test automatic correlation ID assignment in logging
    #[tokio::test]
    async fn should_automatically_assign_correlation_ids() {
        println!("Testing automatic correlation ID assignment in logging");

        let config = LogConfig {
            enable_structured: true,
            enable_console: false,
            enable_file: false,
            ..Default::default()
        };

        let logger = LifecycleLogger::new(config);

        // Log some entries without providing correlation IDs
        logger
            .log(
                LogLevel::Info,
                LogCategory::Lifecycle,
                "TestComponent",
                "Test message 1",
            )
            .await;

        logger
            .log(
                LogLevel::Warn,
                LogCategory::Runtime,
                "TestComponent",
                "Test message 2",
            )
            .await;

        // Get recent entries
        let entries = logger.get_recent_entries(Some(2)).await;

        println!("Retrieved {} log entries", entries.len());

        assert_eq!(entries.len(), 2, "Should have 2 log entries");

        // Verify both entries have correlation IDs
        for (i, entry) in entries.iter().enumerate() {
            println!(
                "Entry {}: correlation_id = {:?}",
                i + 1,
                entry.correlation_id
            );
            assert!(
                entry.correlation_id.is_some(),
                "Entry should have a correlation ID"
            );

            let correlation_id = entry.correlation_id.as_ref().unwrap();
            assert!(
                correlation_id.starts_with("lc-"),
                "Correlation ID should start with 'lc-'"
            );

            // Verify log entry structure
            assert_eq!(entry.component, "TestComponent", "Component should match");
            assert!(
                entry.message.starts_with("Test message"),
                "Message should start with 'Test message'"
            );

            // Verify timestamp is recent (within last 10 seconds)
            let now = std::time::SystemTime::now();
            let entry_time = entry.timestamp;
            let duration = now.duration_since(entry_time).unwrap_or_default();
            assert!(duration.as_secs() < 10, "Timestamp should be recent");
        }

        // Verify specific log levels and categories
        assert_eq!(
            entries[1].level,
            LogLevel::Info,
            "First entry should be Info level"
        );
        assert_eq!(
            entries[1].category,
            LogCategory::Lifecycle,
            "First entry should be Lifecycle category"
        );
        assert_eq!(
            entries[0].level,
            LogLevel::Warn,
            "Second entry should be Warn level"
        );
        assert_eq!(
            entries[0].category,
            LogCategory::Runtime,
            "Second entry should be Runtime category"
        );

        // Verify correlation IDs are different
        let id1 = entries[0].correlation_id.as_ref().unwrap();
        let id2 = entries[1].correlation_id.as_ref().unwrap();
        assert_ne!(id1, id2, "Correlation IDs should be unique");

        // Verify correlation ID format for both entries
        let id1_parts: Vec<&str> = id1.split('-').collect();
        let id2_parts: Vec<&str> = id2.split('-').collect();
        assert_eq!(
            id1_parts.len(),
            3,
            "Correlation ID should have proper format"
        );
        assert_eq!(
            id2_parts.len(),
            3,
            "Correlation ID should have proper format"
        );

        println!("✓ All log entries have unique correlation IDs and valid structure");
    }

    /// Test logging with specific correlation ID
    #[tokio::test]
    async fn should_use_provided_correlation_id() {
        println!("Testing logging with provided correlation ID");

        let config = LogConfig {
            enable_structured: true,
            enable_console: false,
            enable_file: false,
            ..Default::default()
        };

        let logger = LifecycleLogger::new(config);
        let custom_correlation_id = "custom-test-id-123".to_string();

        // Log with specific correlation ID
        logger
            .log_with_correlation(
                LogLevel::Error,
                LogCategory::Database,
                "TestComponent",
                "Test message with custom correlation ID",
                custom_correlation_id.clone(),
            )
            .await;

        // Get recent entries
        let entries = logger.get_recent_entries(Some(1)).await;

        assert_eq!(entries.len(), 1, "Should have 1 log entry");

        let entry = &entries[0];
        println!("Entry correlation_id: {:?}", entry.correlation_id);

        assert!(
            entry.correlation_id.is_some(),
            "Entry should have a correlation ID"
        );
        assert_eq!(
            entry.correlation_id.as_ref().unwrap(),
            &custom_correlation_id,
            "Should use the provided correlation ID"
        );

        // Verify all log entry properties
        assert_eq!(entry.level, LogLevel::Error, "Log level should be Error");
        assert_eq!(
            entry.category,
            LogCategory::Database,
            "Log category should be Database"
        );
        assert_eq!(entry.component, "TestComponent", "Component should match");
        assert_eq!(
            entry.message, "Test message with custom correlation ID",
            "Message should match"
        );

        // Verify timestamp is recent
        let now = std::time::SystemTime::now();
        let duration = now.duration_since(entry.timestamp).unwrap_or_default();
        assert!(duration.as_secs() < 10, "Timestamp should be recent");

        // Verify metadata is initialized
        assert!(
            entry.metadata.is_empty(),
            "Metadata should be empty by default"
        );

        println!("✓ Custom correlation ID was used correctly with valid log entry structure");
    }

    /// Test logging without correlation ID
    #[tokio::test]
    async fn should_log_without_correlation_id_when_requested() {
        println!("Testing logging without correlation ID");

        let config = LogConfig {
            enable_structured: true,
            enable_console: false,
            enable_file: false,
            ..Default::default()
        };

        let logger = LifecycleLogger::new(config);

        // Log without correlation ID
        logger
            .log_without_correlation(
                LogLevel::Debug,
                LogCategory::Performance,
                "TestComponent",
                "Test message without correlation ID",
            )
            .await;

        // Get recent entries
        let entries = logger.get_recent_entries(Some(1)).await;

        assert_eq!(entries.len(), 1, "Should have 1 log entry");

        let entry = &entries[0];
        println!("Entry correlation_id: {:?}", entry.correlation_id);

        assert!(
            entry.correlation_id.is_none(),
            "Entry should not have a correlation ID"
        );

        // Verify all other log entry properties are still valid
        assert_eq!(entry.level, LogLevel::Debug, "Log level should be Debug");
        assert_eq!(
            entry.category,
            LogCategory::Performance,
            "Log category should be Performance"
        );
        assert_eq!(entry.component, "TestComponent", "Component should match");
        assert_eq!(
            entry.message, "Test message without correlation ID",
            "Message should match"
        );

        // Verify timestamp is recent
        let now = std::time::SystemTime::now();
        let duration = now.duration_since(entry.timestamp).unwrap_or_default();
        assert!(duration.as_secs() < 10, "Timestamp should be recent");

        // Verify metadata is initialized
        assert!(
            entry.metadata.is_empty(),
            "Metadata should be empty by default"
        );

        println!("✓ Log entry correctly has no correlation ID but valid structure");
    }
}
