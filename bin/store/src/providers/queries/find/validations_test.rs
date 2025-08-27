#[cfg(test)]
mod tests {
    use super::super::validations::Validation;
    use crate::structs::structs::{
        ConcatenateField, FilterCriteria, FilterOperator, GetByFilter, GroupAdvanceFilter, LogicalOperator, MatchPattern
    };
    use std::collections::HashMap;

    // Helper function to create a default GetByFilter for testing
    fn create_default_get_by_filter() -> GetByFilter {
        GetByFilter {
            pluck: vec!["id".to_string(), "name".to_string()],
            pluck_object: HashMap::new(),
            pluck_group_object: HashMap::new(),
            advance_filters: vec![],
            group_advance_filters: vec![],
            joins: vec![],
            group_by: None,
            concatenate_fields: vec![],
            multiple_sort: vec![],
            date_format: "YYYY-MM-DD".to_string(),
            order_by: "id".to_string(),
            order_direction: "asc".to_string(),
            is_case_sensitive_sorting: Some(false),
            offset: 0,
            limit: 10,
            distinct_by: None,
            timezone: None,
        }
    }

    /// Tests that table validation succeeds with a valid table name
    /// This ensures proper validation of table names in query requests
    #[test]
    fn should_validate_table_successfully_with_valid_name() {
        println!("Testing table validation with valid table name");

        let request_body = create_default_get_by_filter();
        let table = "users".to_string();

        println!("Creating validation for table: {}", table);
        let validation = Validation::new(&request_body, &table);

        let result = validation.validate_table();

        println!("Validation result - Success: {}", result.success);
        println!("Validation message: {}", result.message);

        assert!(result.success);
        assert_eq!(result.message, "Successfully validated table field");

        println!("Table validation test passed");
    }

    /// Tests that table validation fails with an empty table name
    /// This ensures proper error handling for invalid table names
    #[test]
    fn should_fail_table_validation_with_empty_name() {
        println!("Testing table validation with empty table name");

        let request_body = create_default_get_by_filter();
        let table = "".to_string();

        println!("Creating validation for empty table name");
        let validation = Validation::new(&request_body, &table);

        let result = validation.validate_table();

        println!("Validation result - Success: {}", result.success);
        println!("Validation message: {}", result.message);

        assert!(!result.success);
        assert_eq!(result.message, "table is required");

        println!("Empty table validation test passed");
    }

    /// Tests that pluck validation succeeds with valid fields
    /// This ensures proper validation of pluck fields in query requests
    #[test]
    fn should_validate_pluck_successfully_with_valid_fields() {
        println!("Testing pluck validation with valid fields");

        let request_body = create_default_get_by_filter();
        let table = "users".to_string();

        println!("Creating validation for table: {}", table);
        let validation = Validation::new(&request_body, &table);

        let result = validation.validate_pluck();

        println!("Validation result - Success: {}", result.success);
        println!("Validation message: {}", result.message);

        assert!(result.success);
        assert_eq!(result.message, "Successfully validated pluck field");

        println!("Pluck validation success test passed");
    }

    /// Tests that pluck validation fails with empty fields
    /// This ensures proper error handling for empty pluck arrays
    #[test]
    fn should_fail_pluck_validation_with_empty_fields() {
        println!("Testing pluck validation with empty fields");

        let mut request_body = create_default_get_by_filter();
        request_body.pluck = vec![];
        let table = "users".to_string();

        println!("Creating validation for table: {} with empty pluck", table);
        let validation = Validation::new(&request_body, &table);

        let result = validation.validate_pluck();

        println!("Validation result - Success: {}", result.success);
        println!("Validation message: {}", result.message);

        assert!(!result.success);
        assert_eq!(result.message, "pluck is required");

        println!("Empty pluck validation test passed");
    }

    /// Tests that conflicting filters validation succeeds with no conflicts
    /// This ensures proper validation when no conflicting filters are present
    #[test]
    fn should_validate_conflicting_filters_successfully_with_no_conflicts() {
        println!("Testing conflicting filters validation with no conflicts");

        let request_body = create_default_get_by_filter();
        let table = "users".to_string();

        println!("Creating validation for table: {}", table);
        let validation = Validation::new(&request_body, &table);

        let result = validation.validate_conflicting_filters();

        println!("Validation result - Success: {}", result.success);
        println!("Validation message: {}", result.message);

        assert!(result.success);
        assert_eq!(
            result.message,
            "Successfully validated conflicting properties"
        );

        println!("Conflicting filters validation success test passed");
    }

    /// Tests that conflicting filters validation fails when both filter types are present
    /// This ensures proper error handling when conflicting filter types are used
    #[test]
    fn should_fail_conflicting_filters_validation_when_both_present() {
        println!("Testing conflicting filters validation with both filter types present");

        let mut request_body = create_default_get_by_filter();

        println!("Adding both advance_filters and group_advance_filters");
        request_body.advance_filters = vec![FilterCriteria::Criteria {
            field: "name".to_string(),
            entity: Some("users".to_string()),
            operator: FilterOperator::Equal,
            values: vec![serde_json::Value::String("test".to_string())],
            case_sensitive: Some(false),
            parse_as: String::new(),
            match_pattern: Some(MatchPattern::Exact),
            is_search: Some(false),
            has_group_count: None,
        }];
        request_body.group_advance_filters = vec![GroupAdvanceFilter::Criteria {
            operator: LogicalOperator::And,
            filters: vec![FilterCriteria::Criteria {
                field: "name".to_string(),
                entity: Some("users".to_string()),
                operator: FilterOperator::Equal,
                values: vec![serde_json::Value::String("test".to_string())],
                case_sensitive: Some(false),
                parse_as: String::new(),
                match_pattern: None,
                is_search: None,
                has_group_count: None,
            }],
        }];
        let table = "users".to_string();

        println!(
            "Creating validation for table: {} with conflicting filters",
            table
        );
        let validation = Validation::new(&request_body, &table);

        let result = validation.validate_conflicting_filters();

        println!("Validation result - Success: {}", result.success);
        println!("Validation message: {}", result.message);

        assert!(!result.success);
        assert_eq!(
            result.message,
            "Both advance_filters and group_advance_filters cannot be provided at the same time"
        );

        println!("Conflicting filters validation failure test passed");
    }

    /// Tests that concatenated fields validation succeeds with valid configuration
    /// This ensures proper validation of concatenated field configurations
    #[test]
    fn should_validate_concatenated_fields_successfully_with_valid_config() {
    println!("Testing concatenated fields validation with valid configuration");

    let request_body = create_default_get_by_filter();
    let table = "users".to_string();

    println!("Creating validation for table: {}", table);
    let validation = Validation::new(&request_body, &table);

    let result = validation.validate_concatenated_fields();

    println!("Validation result - Success: {}", result.success);
    println!("Validation message: {}", result.message);

    assert!(result.success);

    println!("Concatenated fields validation success test passed");
}

/// Tests that concatenated fields validation fails with empty fields array
/// This ensures proper error handling for invalid concatenated field configurations
#[test]
fn should_fail_concatenated_fields_validation_with_empty_fields() {
    println!("Testing concatenated fields validation with empty fields array");

    let mut request_body = create_default_get_by_filter();
    request_body.concatenate_fields = vec![ConcatenateField {
        field_name: "full_name".to_string(),
        fields: vec![], // Empty fields array
        separator: " ".to_string(),
        entity: "users".to_string(),
        aliased_entity: None,
    }];
    let table = "users".to_string();

    println!(
        "Creating validation for table: {} with empty concatenated fields",
        table
    );
    let validation = Validation::new(&request_body, &table);

    let result = validation.validate_concatenated_fields();

    println!("Validation result - Success: {}", result.success);
    println!("Validation message: {}", result.message);

    assert!(!result.success);
    assert!(result.message.contains("Fields array cannot be empty"));

    println!("Empty concatenated fields validation test passed");
}

/// Tests that concatenated fields validation fails with empty field name
/// This ensures proper error handling for invalid field name configurations
#[test]
fn should_fail_concatenated_fields_validation_with_empty_field_name() {
    println!("Testing concatenated fields validation with empty field name");

    let mut request_body = create_default_get_by_filter();
    request_body.concatenate_fields = vec![ConcatenateField {
        field_name: "".to_string(), // Empty field name
        fields: vec!["first_name".to_string(), "last_name".to_string()],
        separator: " ".to_string(),
        entity: "users".to_string(),
        aliased_entity: None,
    }];
    let table = "users".to_string();

    println!(
        "Creating validation for table: {} with empty field name",
        table
    );
    let validation = Validation::new(&request_body, &table);

    let result = validation.validate_concatenated_fields();

    println!("Validation result - Success: {}", result.success);
    println!("Validation message: {}", result.message);

    assert!(!result.success);
    assert!(result.message.contains("Field name cannot be empty"));

    println!("Empty field name validation test passed");
}

/// Tests that distinct_by validation succeeds with valid field
/// This ensures proper validation of distinct_by field configurations
#[test]
fn should_validate_distinct_by_successfully_with_valid_field() {
    println!("Testing distinct_by validation with valid field");

    let mut request_body = create_default_get_by_filter();
    request_body.distinct_by = Some("id".to_string());
    let table = "users".to_string();

    println!(
        "Creating validation for table: {} with distinct_by field",
        table
    );
    let validation = Validation::new(&request_body, &table);

    let result = validation.validate_distinct_by();

    println!("Validation result - Success: {}", result.success);
    println!("Validation message: {}", result.message);

    assert!(result.success);
    assert_eq!(result.message, "Successfully validated distinct_by field");

    println!("Distinct_by validation success test passed");
}

/// Tests that distinct_by validation fails with empty field
/// This ensures proper error handling for empty distinct_by values
#[test]
fn should_fail_distinct_by_validation_with_empty_field() {
    println!("Testing distinct_by validation with empty field");

    let mut request_body = create_default_get_by_filter();
    request_body.distinct_by = Some("".to_string());
    let table = "users".to_string();

    println!(
        "Creating validation for table: {} with empty distinct_by",
        table
    );
    let validation = Validation::new(&request_body, &table);

    let result = validation.validate_distinct_by();

    println!("Validation result - Success: {}", result.success);
    println!("Validation message: {}", result.message);

    assert!(!result.success);

    println!("Empty distinct_by validation test passed");
}

/// Tests that distinct_by validation succeeds with None value
/// This ensures proper handling when distinct_by is not specified
#[test]
fn should_validate_distinct_by_successfully_with_none_value() {
    println!("Testing distinct_by validation with None value");

    let request_body = create_default_get_by_filter();
    let table = "users".to_string();

    println!(
        "Creating validation for table: {} with None distinct_by",
        table
    );
    let validation = Validation::new(&request_body, &table);

    let result = validation.validate_distinct_by();

    println!("Validation result - Success: {}", result.success);
    println!("Validation message: {}", result.message);

    assert!(result.success);
    assert_eq!(result.message, "Successfully validated distinct_by field");

    println!("None distinct_by validation test passed");
}

/// Tests entity name normalization functionality
/// This ensures proper conversion between singular and plural entity names
#[test]
fn should_normalize_entity_names_correctly() {
    println!("Testing entity name normalization");

    let request_body = create_default_get_by_filter();
    let table = "users".to_string();

    println!("Creating validation for table: {}", table);
    let validation = Validation::new(&request_body, &table);

    println!("Testing singular to plural conversion");
    // Test singular to plural conversion - the function adds 's' to singular forms
    assert_eq!(validation.normalize_entity_name("user"), "users");
    assert_eq!(validation.normalize_entity_name("product"), "products");

    println!("Testing already plural forms");
    // Test already plural forms - function returns as-is if already plural
    assert_eq!(validation.normalize_entity_name("users"), "users");
    assert_eq!(validation.normalize_entity_name("products"), "products");

    println!("Entity name normalization test passed");
}

/// Tests that all validations pass with valid configuration
/// This ensures the complete validation pipeline works correctly
#[test]
fn should_pass_all_validations_with_valid_configuration() {
    println!("Testing complete validation pipeline with valid configuration");

    let mut request_body = create_default_get_by_filter();
    // Ensure no field validations that require database access
    request_body.pluck = vec!["id".to_string()]; // pluck is required and cannot be empty
    request_body.distinct_by = None;
    request_body.concatenate_fields = vec![];
    request_body.order_by = "".to_string(); // Clear order_by to avoid field validation
    request_body.date_format = "YYYY-mm-dd".to_string(); // Use valid date format
    let table = "users".to_string();

    println!(
        "Creating validation for table: {} with valid configuration",
        table
    );
    let validation = Validation::new(&request_body, &table);

    let result = validation.exec();

    println!("Validation result - Success: {}", result.success);
    println!("Validation message: {}", result.message);

    if !result.success {
        println!("Validation failed with message: {}", result.message);
    }
    assert!(
        result.success,
        "Expected validation to pass, but got: {}",
        result.message
    );
    assert_eq!(result.message, "All validations passed successfully");

    println!("Complete validation pipeline test passed");
}

/// Tests that validation fails with empty table name
/// This ensures proper error handling in the complete validation pipeline
#[test]
fn should_fail_validation_with_empty_table_name() {
    println!("Testing validation pipeline failure with empty table name");

    let request_body = create_default_get_by_filter();
    let table = "".to_string();

    println!("Creating validation with empty table name");
    let validation = Validation::new(&request_body, &table);

    let result = validation.exec();

    println!("Validation result - Success: {}", result.success);
    println!("Validation message: {}", result.message);

    assert!(!result.success);
    assert_eq!(result.message, "table is required");

    println!("Empty table validation pipeline test passed");
}

/// Tests that validation fails with empty pluck array
/// This ensures proper error handling for required pluck fields
#[test]
fn should_fail_validation_with_empty_pluck_array() {
    println!("Testing validation pipeline failure with empty pluck array");

    let mut request_body = create_default_get_by_filter();
    request_body.pluck = vec![];
    let table = "users".to_string();

    println!("Creating validation for table: {} with empty pluck", table);
    let validation = Validation::new(&request_body, &table);

    let result = validation.exec();

    println!("Validation result - Success: {}", result.success);
    println!("Validation message: {}", result.message);

    assert!(!result.success);
    assert_eq!(result.message, "pluck is required");

    println!("Empty pluck validation pipeline test passed");
}

/// Tests that validation fails with conflicting filter types
/// This ensures proper error handling when both filter types are present
#[test]
fn should_fail_validation_with_conflicting_filter_types() {
    println!("Testing validation pipeline failure with conflicting filter types");

    let mut request_body = create_default_get_by_filter();

    println!("Adding both advance_filters and group_advance_filters");
    request_body.advance_filters = vec![FilterCriteria::Criteria {
        field: "name".to_string(),
        entity: Some("users".to_string()),
        operator: FilterOperator::Equal,
        values: vec![serde_json::Value::String("test".to_string())],
        case_sensitive: Some(false),
        parse_as: String::new(),
        match_pattern: Some(MatchPattern::Exact),
        is_search: Some(false),
        has_group_count: None,
    }];
    request_body.group_advance_filters = vec![GroupAdvanceFilter::Criteria {
        operator: LogicalOperator::And,
        filters: vec![FilterCriteria::Criteria {
            field: "name".to_string(),
            entity: Some("users".to_string()),
            operator: FilterOperator::Equal,
            values: vec![serde_json::Value::String("test".to_string())],
            case_sensitive: Some(false),
            parse_as: String::new(),
            match_pattern: None,
            is_search: None,
            has_group_count: None,
        }],
    }];
    let table = "users".to_string();

    println!(
        "Creating validation for table: {} with conflicting filters",
        table
    );
    let validation = Validation::new(&request_body, &table);

    let result = validation.exec();

    println!("Validation result - Success: {}", result.success);
    println!("Validation message: {}", result.message);

    assert!(!result.success);
    assert_eq!(
        result.message,
        "Both advance_filters and group_advance_filters cannot be provided at the same time"
    );

    println!("Conflicting filters validation pipeline test passed");
    }
}
