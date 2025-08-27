use super::validations::Validation;
use crate::structs::structs::{
    ConcatenateField, FilterCriteria, FilterOperator, GetByFilter, GroupAdvanceFilter,
    LogicalOperator, MatchPattern,
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

#[test]
fn test_validate_table_success() {
    let request_body = create_default_get_by_filter();
    let table = "users".to_string();
    let validation = Validation::new(&request_body, &table);

    let result = validation.validate_table();

    assert!(result.success);
    assert_eq!(result.message, "Successfully validated table field");
}

#[test]
fn test_validate_table_empty_table() {
    let request_body = create_default_get_by_filter();
    let table = "".to_string();
    let validation = Validation::new(&request_body, &table);

    let result = validation.validate_table();

    assert!(!result.success);
    assert_eq!(result.message, "table is required");
}

#[test]
fn test_validate_pluck_success() {
    let request_body = create_default_get_by_filter();
    let table = "users".to_string();
    let validation = Validation::new(&request_body, &table);

    let result = validation.validate_pluck();

    assert!(result.success);
    assert_eq!(result.message, "Successfully validated pluck field");
}

#[test]
fn test_validate_pluck_empty() {
    let mut request_body = create_default_get_by_filter();
    request_body.pluck = vec![];
    let table = "users".to_string();
    let validation = Validation::new(&request_body, &table);

    let result = validation.validate_pluck();

    assert!(!result.success);
    assert_eq!(result.message, "pluck is required");
}

#[test]
fn test_validate_conflicting_filters_success() {
    let request_body = create_default_get_by_filter();
    let table = "users".to_string();
    let validation = Validation::new(&request_body, &table);

    let result = validation.validate_conflicting_filters();

    assert!(result.success);
    assert_eq!(
        result.message,
        "Successfully validated conflicting properties"
    );
}

#[test]
fn test_validate_conflicting_filters_both_present() {
    let mut request_body = create_default_get_by_filter();
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
    let validation = Validation::new(&request_body, &table);

    let result = validation.validate_conflicting_filters();

    assert!(!result.success);
    assert_eq!(
        result.message,
        "Both advance_filters and group_advance_filters cannot be provided at the same time"
    );
}

#[test]
fn test_validate_concatenated_fields_success() {
    let request_body = create_default_get_by_filter();
    let table = "users".to_string();
    let validation = Validation::new(&request_body, &table);

    let result = validation.validate_concatenated_fields();

    assert!(result.success);
}

#[test]
fn test_validate_concatenated_fields_empty_fields() {
    let mut request_body = create_default_get_by_filter();
    request_body.concatenate_fields = vec![ConcatenateField {
        field_name: "full_name".to_string(),
        fields: vec![], // Empty fields array
        separator: " ".to_string(),
        entity: "users".to_string(),
        aliased_entity: None,
    }];
    let table = "users".to_string();
    let validation = Validation::new(&request_body, &table);

    let result = validation.validate_concatenated_fields();

    assert!(!result.success);
    assert!(result.message.contains("Fields array cannot be empty"));
}

#[test]
fn test_validate_concatenated_fields_empty_field_name() {
    let mut request_body = create_default_get_by_filter();
    request_body.concatenate_fields = vec![ConcatenateField {
        field_name: "".to_string(), // Empty field name
        fields: vec!["first_name".to_string(), "last_name".to_string()],
        separator: " ".to_string(),
        entity: "users".to_string(),
        aliased_entity: None,
    }];
    let table = "users".to_string();
    let validation = Validation::new(&request_body, &table);

    let result = validation.validate_concatenated_fields();

    assert!(!result.success);
    assert!(result.message.contains("Field name cannot be empty"));
}

#[test]
fn test_validate_distinct_by_success() {
    let mut request_body = create_default_get_by_filter();
    request_body.distinct_by = None; // Test with None to avoid field validation
    let table = "users".to_string();
    let validation = Validation::new(&request_body, &table);

    let result = validation.validate_distinct_by();

    assert!(result.success);
    assert_eq!(result.message, "Successfully validated distinct_by field");
}

#[test]
fn test_validate_distinct_by_empty() {
    let mut request_body = create_default_get_by_filter();
    request_body.distinct_by = Some("".to_string());
    let table = "users".to_string();
    let validation = Validation::new(&request_body, &table);

    let result = validation.validate_distinct_by();

    assert!(result.success);
    assert_eq!(result.message, "Successfully validated distinct_by field");
}

#[test]
fn test_validate_distinct_by_none() {
    let request_body = create_default_get_by_filter();
    let table = "users".to_string();
    let validation = Validation::new(&request_body, &table);

    let result = validation.validate_distinct_by();

    assert!(result.success);
    assert_eq!(result.message, "Successfully validated distinct_by field");
}

#[test]
fn test_normalize_entity_name() {
    let request_body = create_default_get_by_filter();
    let table = "users".to_string();
    let validation = Validation::new(&request_body, &table);

    // Test singular to plural conversion - the function adds 's' to singular forms
    assert_eq!(validation.normalize_entity_name("user"), "users");
    assert_eq!(validation.normalize_entity_name("product"), "products");

    // Test already plural forms - function returns as-is if already plural
    assert_eq!(validation.normalize_entity_name("users"), "users");
    assert_eq!(validation.normalize_entity_name("products"), "products");
}

#[test]
fn test_exec_all_validations_pass() {
    let mut request_body = create_default_get_by_filter();
    // Ensure no field validations that require database access
    request_body.pluck = vec!["id".to_string()]; // pluck is required and cannot be empty
    request_body.distinct_by = None;
    request_body.concatenate_fields = vec![];
    request_body.order_by = "".to_string(); // Clear order_by to avoid field validation
    request_body.date_format = "YYYY-mm-dd".to_string(); // Use valid date format
    let table = "users".to_string();
    let validation = Validation::new(&request_body, &table);

    let result = validation.exec();

    if !result.success {
        println!("Validation failed with message: {}", result.message);
    }
    assert!(
        result.success,
        "Expected validation to pass, but got: {}",
        result.message
    );
    assert_eq!(result.message, "All validations passed successfully");
}

#[test]
fn test_exec_validation_fails_on_empty_table() {
    let request_body = create_default_get_by_filter();
    let table = "".to_string();
    let validation = Validation::new(&request_body, &table);

    let result = validation.exec();

    assert!(!result.success);
    assert_eq!(result.message, "table is required");
}

#[test]
fn test_exec_validation_fails_on_empty_pluck() {
    let mut request_body = create_default_get_by_filter();
    request_body.pluck = vec![];
    let table = "users".to_string();
    let validation = Validation::new(&request_body, &table);

    let result = validation.exec();

    assert!(!result.success);
    assert_eq!(result.message, "pluck is required");
}

#[test]
fn test_exec_validation_fails_on_conflicting_filters() {
    let mut request_body = create_default_get_by_filter();
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
    let validation = Validation::new(&request_body, &table);

    let result = validation.exec();

    assert!(!result.success);
    assert_eq!(
        result.message,
        "Both advance_filters and group_advance_filters cannot be provided at the same time"
    );
}
