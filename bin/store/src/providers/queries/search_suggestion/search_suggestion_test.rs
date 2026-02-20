#[cfg(test)]
mod tests {
    use crate::providers::queries::search_suggestion::structs::{
        AliasedJoinedEntity, ConcatenatedExpressions, FieldExpression, FieldFiltersResult,
        FormatFilterResponse, SearchSuggestionCache,
    };
    use crate::providers::queries::search_suggestion::utils::{
        format_filters, generate_concatenated_expressions, get_field_filters,
    };
    use crate::providers::queries::search_suggestion::sql_constructor::SQLConstructor as SearchSuggestionSQLConstructor;
    use crate::structs::core::{
        ConcatenateField, FilterCriteria, FilterOperator, MatchPattern, SearchSuggestionParams,
    };
    use serde_json::json;
    use std::collections::HashMap;
    use std::env;

    // Helper function to create a default FilterCriteria for testing
    fn create_default_filter_criteria() -> FilterCriteria {
        FilterCriteria::Criteria {
            field: "name".to_string(),
            entity: Some("users".to_string()),
            operator: FilterOperator::Equal,
            values: vec![serde_json::Value::String("test_value".to_string())],
            case_sensitive: Some(false),
            parse_as: "string".to_string(),
            match_pattern: Some(MatchPattern::Contains),
            is_search: Some(true),
            has_group_count: Some(false),
        }
    }

    // Helper function to create AliasedJoinedEntity for testing
    fn create_aliased_joined_entity() -> AliasedJoinedEntity {
        AliasedJoinedEntity {
            to_entity: "profiles".to_string(),
            alias: "p".to_string(),
        }
    }

    // Helper function to create ConcatenateField for testing
    fn create_concatenate_field() -> ConcatenateField {
        ConcatenateField {
            entity: "users".to_string(),
            field_name: "full_name".to_string(),
            fields: vec!["first_name".to_string(), "last_name".to_string()],
            separator: " ".to_string(),
            aliased_entity: None,
        }
    }

    /// Tests that AliasedJoinedEntity creates correctly with entity and alias
    /// This ensures proper entity aliasing for joined tables
    #[test]
    fn should_create_aliased_joined_entity_with_valid_parameters() {
        println!("Testing AliasedJoinedEntity creation with entity and alias");

        let entity = create_aliased_joined_entity();

        println!("Created entity with to_entity: {}", entity.to_entity);
        assert_eq!(entity.to_entity, "profiles");

        println!("Created entity with alias: {}", entity.alias);
        assert_eq!(entity.alias, "p");

        println!("AliasedJoinedEntity creation test passed");
    }

    /// Tests that AliasedJoinedEntity clones correctly preserving all fields
    /// This ensures proper clone implementation for entity aliasing
    #[test]
    fn should_clone_aliased_joined_entity_preserving_all_fields() {
        println!("Testing AliasedJoinedEntity clone functionality");

        let entity = create_aliased_joined_entity();
        println!(
            "Original entity - to_entity: {}, alias: {}",
            entity.to_entity, entity.alias
        );

        let cloned_entity = entity.clone();
        println!(
            "Cloned entity - to_entity: {}, alias: {}",
            cloned_entity.to_entity, cloned_entity.alias
        );

        assert_eq!(entity.to_entity, cloned_entity.to_entity);
        assert_eq!(entity.alias, cloned_entity.alias);

        println!("AliasedJoinedEntity clone test passed");
    }

    /// Tests that FieldExpression creates correctly with expression and fields
    /// This ensures proper field expression initialization for search suggestions
    #[test]
    fn should_create_field_expression_with_valid_parameters() {
        println!("Testing FieldExpression creation with expression and fields");

        let field_expr = FieldExpression {
            expression: "CONCAT(first_name, ' ', last_name)".to_string(),
            fields: vec!["first_name".to_string(), "last_name".to_string()],
        };

        println!(
            "Created field expression - expression: {}",
            field_expr.expression
        );
        assert_eq!(field_expr.expression, "CONCAT(first_name, ' ', last_name)");

        println!(
            "Created field expression - fields count: {}",
            field_expr.fields.len()
        );
        assert_eq!(field_expr.fields.len(), 2);

        println!("Verifying field expression contains expected fields");
        assert!(field_expr.fields.contains(&"first_name".to_string()));
        assert!(field_expr.fields.contains(&"last_name".to_string()));

        println!("FieldExpression creation test passed");
    }

    /// Tests that FieldExpression clones correctly preserving all fields
    /// This ensures proper clone implementation for field expressions
    #[test]
    fn should_clone_field_expression_preserving_all_fields() {
        println!("Testing FieldExpression clone functionality");

        let field_expr = FieldExpression {
            expression: "test_expression".to_string(),
            fields: vec!["field1".to_string(), "field2".to_string()],
        };

        println!(
            "Original field expression - expression: {}",
            field_expr.expression
        );
        let cloned_expr = field_expr.clone();
        println!(
            "Cloned field expression - expression: {}",
            cloned_expr.expression
        );

        assert_eq!(field_expr.expression, cloned_expr.expression);
        assert_eq!(field_expr.fields, cloned_expr.fields);

        println!("FieldExpression clone test passed");
    }

    /// Tests that FieldExpression serializes correctly to JSON
    /// This ensures proper serialization for field expressions
    #[test]
    fn should_serialize_field_expression_to_json() {
        println!("Testing FieldExpression serialization to JSON");

        let field_expr = FieldExpression {
            expression: "test_expression".to_string(),
            fields: vec!["field1".to_string()],
        };

        println!(
            "Serializing field expression with expression: {}",
            field_expr.expression
        );
        let serialized = serde_json::to_string(&field_expr).unwrap();
        println!("Serialized JSON: {}", serialized);

        assert!(serialized.contains("test_expression"));
        assert!(serialized.contains("field1"));

        println!("FieldExpression serialization test passed");
    }

    /// Tests that ConcatenatedExpressions type works correctly with HashMap operations
    /// This ensures proper type definition and usage for concatenated expressions
    #[test]
    fn should_handle_concatenated_expressions_type_operations() {
        println!("Testing ConcatenatedExpressions type operations");

        let mut concatenated_expressions: ConcatenatedExpressions = HashMap::new();
        let mut entity_map = HashMap::new();

        let field_expr = FieldExpression {
            expression: "CONCAT(first_name, ' ', last_name)".to_string(),
            fields: vec!["first_name".to_string(), "last_name".to_string()],
        };

        println!("Adding field expression to entity map");
        entity_map.insert("full_name".to_string(), field_expr);
        concatenated_expressions.insert("users".to_string(), entity_map);

        println!("Verifying concatenated expressions contains users entity");
        assert!(concatenated_expressions.contains_key("users"));
        let users_map = concatenated_expressions.get("users").unwrap();
        assert!(users_map.contains_key("full_name"));

        println!("ConcatenatedExpressions type operations test passed");
    }

    /// Tests that FormatFilterResponse creates correctly with all fields
    /// This ensures proper initialization of filter response structures
    #[test]
    fn should_create_format_filter_response_with_valid_data() {
        println!("Testing FormatFilterResponse creation with valid data");

        let response = FormatFilterResponse {
            formatted_filters: vec![json!({"field": "name", "value": "test"})],
            search_term: "test_search".to_string(),
            filtered_fields: json!({"users": ["name", "email"]}),
        };

        println!(
            "Created response with {} formatted filters",
            response.formatted_filters.len()
        );
        assert_eq!(response.formatted_filters.len(), 1);

        println!(
            "Created response with search term: {}",
            response.search_term
        );
        assert_eq!(response.search_term, "test_search");

        println!("Verifying filtered fields is an object");
        assert!(response.filtered_fields.is_object());

        println!("FormatFilterResponse creation test passed");
    }

    /// Tests that FieldFiltersResult creates correctly with filters
    /// This ensures proper initialization of field filter results
    #[test]
    fn should_create_field_filters_result_with_valid_filters() {
        println!("Testing FieldFiltersResult creation with valid filters");

        let filter = create_default_filter_criteria();
        let result = FieldFiltersResult {
            all_field_filters: vec![filter.clone()],
            field_filter: Some(filter),
        };

        println!(
            "Created result with {} all field filters",
            result.all_field_filters.len()
        );
        assert_eq!(result.all_field_filters.len(), 1);

        println!("Verifying field filter is present");
        assert!(result.field_filter.is_some());

        println!("FieldFiltersResult creation test passed");
    }

    /// Tests that FieldFiltersResult handles None field_filter correctly
    /// This ensures proper handling of optional field filters
    #[test]
    fn should_handle_field_filters_result_with_none_field_filter() {
        println!("Testing FieldFiltersResult with None field_filter");

        let filter = create_default_filter_criteria();
        let result = FieldFiltersResult {
            all_field_filters: vec![filter],
            field_filter: None,
        };

        println!(
            "Created result with {} all field filters",
            result.all_field_filters.len()
        );
        assert_eq!(result.all_field_filters.len(), 1);

        println!("Verifying field filter is None");
        assert!(result.field_filter.is_none());

        println!("FieldFiltersResult with None field_filter test passed");
    }

    /// Tests that SearchSuggestionCache hash_string produces consistent and valid hashes
    /// This ensures proper hash generation for cache keys
    #[test]
    fn should_generate_consistent_hash_for_search_suggestion_cache() {
        println!("Testing SearchSuggestionCache hash_string consistency");

        let input = "test_string";
        println!("Generating hash for input: {}", input);
        let hash1 = SearchSuggestionCache::hash_string(input);
        let hash2 = SearchSuggestionCache::hash_string(input);

        // Same input should produce same hash
        println!("Hash 1: {}, Hash 2: {}", hash1, hash2);
        assert_eq!(hash1, hash2);

        // Hash should be a valid hex string
        println!("Verifying hash is valid hex string");
        assert!(hash1.chars().all(|c| c.is_ascii_hexdigit()));

        // Different inputs should produce different hashes
        let different_hash = SearchSuggestionCache::hash_string("different_string");
        println!("Different input hash: {}", different_hash);
        assert_ne!(hash1, different_hash);

        println!("SearchSuggestionCache hash consistency test passed");
    }

    /// Tests that SearchSuggestionCache hash_string handles empty strings correctly
    /// This ensures proper hash generation for edge cases
    #[test]
    fn should_handle_empty_string_hash_for_search_suggestion_cache() {
        println!("Testing SearchSuggestionCache hash_string with empty string");

        let hash = SearchSuggestionCache::hash_string("");
        println!("Generated hash for empty string: {}", hash);

        println!("Verifying hash is not empty");
        assert!(!hash.is_empty());

        println!("Verifying hash is valid hex string");
        assert!(hash.chars().all(|c| c.is_ascii_hexdigit()));

        println!("SearchSuggestionCache empty string hash test passed");
    }

    /// Tests that get_field_filters works correctly with matching field criteria
    /// This ensures proper field filter matching logic
    #[test]
    fn should_get_field_filters_with_matching_field_criteria() {
        println!("Testing get_field_filters with matching field criteria");

        let filter = create_default_filter_criteria();
        let filters = vec![filter];

        println!("Getting field filters for field: name, entity: users");
        let result = get_field_filters(filters, "name", "users", "test_search");

        println!(
            "Result has {} all field filters",
            result.all_field_filters.len()
        );
        assert_eq!(result.all_field_filters.len(), 1);

        println!("Verifying field filter is present");
        assert!(result.field_filter.is_some());

        println!("get_field_filters matching field test passed");
    }

    /// Tests that get_field_filters handles non-matching field criteria correctly
    /// This ensures proper filtering when field names don't match
    #[test]
    fn should_handle_get_field_filters_with_non_matching_field() {
        println!("Testing get_field_filters with non-matching field criteria");

        let filter = create_default_filter_criteria();
        let filters = vec![filter];

        println!("Getting field filters for non-matching field: email, entity: users");
        let result = get_field_filters(filters, "email", "users", "test_search");

        // The filter is still added to all_field_filters but not as field_filter since field doesn't match
        println!(
            "Result has {} all field filters",
            result.all_field_filters.len()
        );
        assert_eq!(result.all_field_filters.len(), 0); // Non-search criteria are not added when values don't match search_term

        println!("Verifying field filter is None for non-matching field");
        assert!(result.field_filter.is_none());

        println!("get_field_filters non-matching field test passed");
    }

    /// Tests that get_field_filters handles non-matching entity criteria correctly
    /// This ensures proper filtering when entity names don't match
    #[test]
    fn should_handle_get_field_filters_with_non_matching_entity() {
        println!("Testing get_field_filters with non-matching entity criteria");

        let filter = create_default_filter_criteria();
        let filters = vec![filter];

        println!("Getting field filters for field: name, non-matching entity: profiles");
        let result = get_field_filters(filters, "name", "profiles", "test_search");

        // The filter is not added because entity doesn't match and values don't match search_term
        println!(
            "Result has {} all field filters",
            result.all_field_filters.len()
        );
        assert_eq!(result.all_field_filters.len(), 0);

        println!("Verifying field filter is None for non-matching entity");
        assert!(result.field_filter.is_none());

        println!("get_field_filters non-matching entity test passed");
    }

    /// Tests that get_field_filters handles empty filter list correctly
    /// This ensures proper handling of edge cases with no filters
    #[test]
    fn should_handle_get_field_filters_with_empty_filter_list() {
        println!("Testing get_field_filters with empty filter list");

        let filters = vec![];

        println!("Getting field filters with empty filter list");
        let result = get_field_filters(filters, "name", "users", "test_search");

        println!(
            "Result has {} all field filters",
            result.all_field_filters.len()
        );
        assert_eq!(result.all_field_filters.len(), 0);

        println!("Verifying field filter is None for empty filters");
        assert!(result.field_filter.is_none());

        println!("get_field_filters empty filters test passed");
    }

    /// Tests that generate_concatenated_expressions creates proper SQL expressions
    /// This ensures correct concatenation expression generation for database queries
    #[test]
    fn should_generate_concatenated_expressions_with_valid_sql() {
        println!("Testing generate_concatenated_expressions with valid SQL generation");

        let concatenate_field = create_concatenate_field();
        let concatenate_fields = vec![concatenate_field];

        println!("Generating concatenated expressions with date format: YYYY-MM-DD");
        let result = generate_concatenated_expressions(
            concatenate_fields,
            Some("YYYY-MM-DD"),
            None,
            "HH24:MI",
        );

        println!("Verifying result contains users entity");
        assert!(result.contains_key("users"));
        let users_map = result.get("users").unwrap();
        assert!(users_map.contains_key("full_name"));

        let field_expr = users_map.get("full_name").unwrap();
        println!("Generated expression: {}", field_expr.expression);
        // The function uses PostgreSQL || operator, not CONCAT
        assert!(field_expr.expression.contains("COALESCE"));
        assert!(field_expr.expression.contains("||"));
        assert_eq!(field_expr.fields.len(), 2);

        println!("generate_concatenated_expressions test passed");
    }

    /// Tests that generate_concatenated_expressions handles empty input correctly
    /// This ensures proper handling of edge cases with no concatenate fields
    #[test]
    fn should_handle_generate_concatenated_expressions_with_empty_input() {
        println!("Testing generate_concatenated_expressions with empty input");

        let concatenate_fields = vec![];

        println!("Generating concatenated expressions with empty fields list");
        let result = generate_concatenated_expressions(concatenate_fields, None, None, "HH24:MI");

        println!("Verifying result is empty");
        assert!(result.is_empty());

        println!("generate_concatenated_expressions empty input test passed");
    }

    /// Tests that generate_concatenated_expressions handles custom separators correctly
    /// This ensures proper separator handling in concatenation expressions
    #[test]
    fn should_generate_concatenated_expressions_with_custom_separator() {
        println!("Testing generate_concatenated_expressions with custom separator");

        let mut concatenate_field = create_concatenate_field();
        concatenate_field.separator = " - ".to_string();
        let concatenate_fields = vec![concatenate_field];

        println!("Generating concatenated expressions with custom separator: ' - '");
        let result = generate_concatenated_expressions(concatenate_fields, None, None, "HH24:MI");

        assert!(result.contains_key("users"));
        let users_map = result.get("users").unwrap();
        let field_expr = users_map.get("full_name").unwrap();
        println!(
            "Generated expression with separator: {}",
            field_expr.expression
        );
        assert!(field_expr.expression.contains(" - "));

        println!("generate_concatenated_expressions custom separator test passed");
    }

    /// Tests that format_filters works correctly with basic filter criteria
    /// This ensures proper filter formatting with aliased entities
    #[test]
    fn should_format_filters_with_basic_criteria_and_aliases() {
        println!("Testing format_filters with basic criteria and aliases");

        let filter = create_default_filter_criteria();
        let filters = vec![filter];
        let aliased_entities = vec![create_aliased_joined_entity()];
        let filtered_fields = json!({"users": ["name"]});

        println!("Formatting filters with aliased entities");
        let result = format_filters(
            filters,
            Some(&aliased_entities),
            "users",
            filtered_fields.clone(),
            String::new(),
        );

        println!(
            "Result has {} formatted filters",
            result.formatted_filters.len()
        );
        assert!(!result.formatted_filters.is_empty());

        println!("Search term: {}", result.search_term);
        assert_eq!(result.search_term, "test_value");
        assert_eq!(result.filtered_fields, filtered_fields);

        println!("format_filters basic test passed");
    }

    /// Tests that format_filters works correctly without aliased entities
    /// This ensures proper filter formatting when no aliases are provided
    #[test]
    fn should_format_filters_without_aliased_entities() {
        println!("Testing format_filters without aliased entities");

        let filter = create_default_filter_criteria();
        let filters = vec![filter];
        let filtered_fields = json!({"users": ["name"]});

        println!("Formatting filters without aliased entities");
        let result = format_filters(
            filters,
            None,
            "users",
            filtered_fields.clone(),
            String::new(),
        );

        println!(
            "Result has {} formatted filters",
            result.formatted_filters.len()
        );
        assert!(!result.formatted_filters.is_empty());

        println!("Search term: {}", result.search_term);
        assert_eq!(result.search_term, "test_value");

        println!("format_filters without aliases test passed");
    }

    /// Tests that format_filters handles empty filter list correctly
    /// This ensures proper handling of edge cases with no filters
    #[test]
    fn should_handle_format_filters_with_empty_filter_list() {
        println!("Testing format_filters with empty filter list");

        let filters = vec![];
        let filtered_fields = json!({"users": ["name"]});

        println!("Formatting empty filters list");
        let result = format_filters(
            filters,
            None,
            "users",
            filtered_fields.clone(),
            String::new(),
        );

        println!("Verifying result has empty formatted filters");
        assert!(result.formatted_filters.is_empty());

        println!("Verifying search term is empty");
        assert!(result.search_term.is_empty());

        println!("format_filters empty filters test passed");
    }

    /// Tests that ConcatenateField creates correctly with all properties
    /// This ensures proper initialization of concatenate field structures
    #[test]
    fn should_create_concatenate_field_with_valid_properties() {
        println!("Testing ConcatenateField creation with valid properties");

        let concatenate_field = create_concatenate_field();

        println!(
            "Created concatenate field with entity: {}",
            concatenate_field.entity
        );
        assert_eq!(concatenate_field.entity, "users");

        println!(
            "Created concatenate field with field_name: {}",
            concatenate_field.field_name
        );
        assert_eq!(concatenate_field.field_name, "full_name");

        println!(
            "Created concatenate field with {} fields",
            concatenate_field.fields.len()
        );
        assert_eq!(concatenate_field.fields.len(), 2);

        println!(
            "Created concatenate field with separator: '{}'",
            concatenate_field.separator
        );
        assert_eq!(concatenate_field.separator, " ".to_string());

        println!("ConcatenateField creation test passed");
    }

    /// Tests that ConcatenateField handles different separators correctly
    /// This ensures proper separator customization for concatenate fields
    #[test]
    fn should_handle_concatenate_field_with_custom_separator() {
        println!("Testing ConcatenateField with custom separator");

        let mut concatenate_field = create_concatenate_field();
        concatenate_field.separator = "-".to_string();

        println!("Set custom separator: '{}'", concatenate_field.separator);
        assert_eq!(concatenate_field.separator, "-".to_string());

        println!("ConcatenateField custom separator test passed");
    }

    /// Tests that FilterCriteria debug format contains expected information
    /// This ensures proper debug representation for filter criteria
    #[test]
    fn should_format_filter_criteria_debug_representation() {
        println!("Testing FilterCriteria debug format representation");

        let filter = create_default_filter_criteria();
        let debug_str = format!("{:?}", filter);

        println!("Debug string: {}", debug_str);

        println!("Verifying debug string contains 'Criteria'");
        assert!(debug_str.contains("Criteria"));

        println!("Verifying debug string contains 'name'");
        assert!(debug_str.contains("name"));

        println!("Verifying debug string contains 'users'");
        assert!(debug_str.contains("users"));

        println!("FilterCriteria debug format test passed");
    }

    /// Tests that AliasedJoinedEntity debug format contains expected information
    /// This ensures proper debug representation for aliased joined entities
    #[test]
    fn should_format_aliased_joined_entity_debug_representation() {
        println!("Testing AliasedJoinedEntity debug format representation");

        let entity = create_aliased_joined_entity();
        let debug_str = format!("{:?}", entity);

        println!("Debug string: {}", debug_str);

        println!("Verifying debug string contains 'AliasedJoinedEntity'");
        assert!(debug_str.contains("AliasedJoinedEntity"));

        println!("Verifying debug string contains 'profiles'");
        assert!(debug_str.contains("profiles"));

        println!("Verifying debug string contains 'p'");
        assert!(debug_str.contains("p"));

        println!("AliasedJoinedEntity debug format test passed");
    }

    /// Tests that FieldExpression debug format contains expected information
    /// This ensures proper debug representation for field expressions
    #[test]
    fn should_format_field_expression_debug_representation() {
        println!("Testing FieldExpression debug format representation");

        let field_expr = FieldExpression {
            expression: "test_expression".to_string(),
            fields: vec!["field1".to_string()],
        };

        let debug_str = format!("{:?}", field_expr);
        println!("Debug string: {}", debug_str);

        println!("Verifying debug string contains 'FieldExpression'");
        assert!(debug_str.contains("FieldExpression"));

        println!("Verifying debug string contains 'test_expression'");
        assert!(debug_str.contains("test_expression"));

        println!("FieldExpression debug format test passed");
    }

    /// Tests that FormatFilterResponse debug format contains expected information
    /// This ensures proper debug representation for format filter responses
    #[test]
    fn should_format_filter_response_debug_representation() {
        println!("Testing FormatFilterResponse debug format representation");

        let response = FormatFilterResponse {
            formatted_filters: vec![json!({"test": "value"})],
            search_term: "test".to_string(),
            filtered_fields: json!({"field": "value"}),
        };

        let debug_str = format!("{:?}", response);
        println!("Debug string: {}", debug_str);

        println!("Verifying debug string contains 'FormatFilterResponse'");
        assert!(debug_str.contains("FormatFilterResponse"));

        println!("FormatFilterResponse debug format test passed");
    }

    /// Tests that FieldFiltersResult debug format contains expected information
    /// This ensures proper debug representation for field filter results
    #[test]
    fn should_format_field_filters_result_debug_representation() {
        println!("Testing FieldFiltersResult debug format representation");

        let filter = create_default_filter_criteria();
        let result = FieldFiltersResult {
            all_field_filters: vec![filter.clone()],
            field_filter: Some(filter),
        };

        let debug_str = format!("{:?}", result);
        println!("Debug string: {}", debug_str);

        println!("Verifying debug string contains 'FieldFiltersResult'");
        assert!(debug_str.contains("FieldFiltersResult"));

        println!("FieldFiltersResult debug format test passed");
    }

    /// Tests that default search pattern fallback works correctly
    /// This ensures proper handling when environment variables are not set
    #[test]
    fn should_handle_default_search_pattern_fallback() {
        println!("Testing default search pattern fallback behavior");
        // Remove the environment variable if it exists
        env::remove_var("DEFAULT_SEARCH_PATTERN");

        // Create a filter to trigger the default pattern logic
        let mut filter = create_default_filter_criteria();
        if let FilterCriteria::Criteria { match_pattern, .. } = &mut filter {
            *match_pattern = None; // This should trigger the default pattern
        }

        let filters = vec![filter];
        let result = format_filters(
            filters,
            None,
            "users",
            json!({"users": ["name"]}),
            String::new(),
        );

        // Should not panic and should return a result
        println!("Verifying result has formatted filters");
        assert!(!result.formatted_filters.is_empty());

        println!("Default search pattern fallback test passed");
    }

    /// Tests that multiple concatenate fields are handled correctly
    /// This ensures proper generation of multiple concatenated expressions
    #[test]
    fn should_handle_multiple_concatenate_fields() {
        println!("Testing multiple concatenate fields handling");
        let concatenate_field1 = ConcatenateField {
            entity: "users".to_string(),
            field_name: "full_name".to_string(),
            fields: vec!["first_name".to_string(), "last_name".to_string()],
            separator: " ".to_string(),
            aliased_entity: None,
        };

        let concatenate_field2 = ConcatenateField {
            entity: "users".to_string(),
            field_name: "address".to_string(),
            fields: vec![
                "street".to_string(),
                "city".to_string(),
                "state".to_string(),
            ],
            separator: ", ".to_string(),
            aliased_entity: None,
        };

        let concatenate_fields = vec![concatenate_field1, concatenate_field2];
        let result = generate_concatenated_expressions(concatenate_fields, None, None, "HH24:MI");

        assert!(result.contains_key("users"));
        let users_map = result.get("users").unwrap();
        assert!(users_map.contains_key("full_name"));
        assert!(users_map.contains_key("address"));

        let address_expr = users_map.get("address").unwrap();
        println!(
            "Address expression fields count: {}",
            address_expr.fields.len()
        );
        assert_eq!(address_expr.fields.len(), 3);

        println!("Verifying address expression contains separator");
        assert!(address_expr.expression.contains(", "));

        println!("Multiple concatenate fields test passed");
    }

    /// Tests timezone handling in search suggestions: generate_concatenated_expressions
    /// applies timezone to date/time fields when building concatenated expressions.
    #[test]
    fn should_apply_timezone_to_date_time_fields_in_concatenated_expressions() {
        println!("Testing timezone in concatenated expressions for search suggestions");

        let concatenate_field = ConcatenateField {
            entity: "contacts".to_string(),
            field_name: "created_date_time".to_string(),
            fields: vec!["created_date".to_string(), "created_time".to_string()],
            separator: " ".to_string(),
            aliased_entity: None,
        };

        let result = generate_concatenated_expressions(
            vec![concatenate_field],
            Some("mm/dd/YYYY"),
            Some("Europe/Berlin"),
            "HH24:MI",
        );

        assert!(result.contains_key("contacts"));
        let expr = result
            .get("contacts")
            .unwrap()
            .get("created_date_time")
            .unwrap();
        assert!(
            expr.expression.contains("AT TIME ZONE 'Europe/Berlin'"),
            "Concatenated expression should contain AT TIME ZONE. Got: {}",
            expr.expression
        );

        println!("Timezone in concatenated expressions test passed");
    }

    /// Tests timezone handling in search suggestions: when timezone is None,
    /// date/time fields are still formatted but without AT TIME ZONE.
    #[test]
    fn should_handle_search_suggestions_without_timezone() {
        println!("Testing search suggestion concatenated expressions without timezone");

        let concatenate_field = ConcatenateField {
            entity: "contacts".to_string(),
            field_name: "created_date_time".to_string(),
            fields: vec!["created_date".to_string(), "created_time".to_string()],
            separator: " ".to_string(),
            aliased_entity: None,
        };

        let result = generate_concatenated_expressions(
            vec![concatenate_field],
            Some("mm/dd/YYYY"),
            None, // No timezone
            "HH24:MI",
        );

        assert!(result.contains_key("contacts"));
        let expr = result
            .get("contacts")
            .unwrap()
            .get("created_date_time")
            .unwrap();
        assert!(
            expr.expression.contains("TO_CHAR"),
            "Expression should still contain TO_CHAR for date/time formatting"
        );

        println!("Search suggestion without timezone test passed");
    }

    /// Tests that multiple entities with concatenated expressions work correctly
    /// This ensures proper handling of concatenated expressions across different entities
    #[test]
    fn should_handle_multiple_entities_concatenated_expressions() {
        println!("Testing multiple entities with concatenated expressions");
        let concatenate_field1 = ConcatenateField {
            entity: "users".to_string(),
            field_name: "full_name".to_string(),
            fields: vec!["first_name".to_string(), "last_name".to_string()],
            separator: " ".to_string(),
            aliased_entity: None,
        };

        let concatenate_field2 = ConcatenateField {
            entity: "profiles".to_string(),
            field_name: "display_name".to_string(),
            fields: vec!["title".to_string(), "name".to_string()],
            separator: ": ".to_string(),
            aliased_entity: None,
        };

        let concatenate_fields = vec![concatenate_field1, concatenate_field2];
        let result = generate_concatenated_expressions(concatenate_fields, None, None, "HH24:MI");

        assert!(result.contains_key("users"));
        assert!(result.contains_key("profiles"));

        let users_map = result.get("users").unwrap();
        let profiles_map = result.get("profiles").unwrap();

        println!("Verifying users entity has full_name field");
        assert!(users_map.contains_key("full_name"));

        println!("Verifying profiles entity has display_name field");
        assert!(profiles_map.contains_key("display_name"));

        println!("Multiple entities concatenated expressions test passed");
    }

    #[test]
    fn should_cast_advance_filter_field_to_text_when_parse_as_text() {
        let mut params = SearchSuggestionParams::default();
        params.date_format = "YYYY-MM-DD".to_string();
        params.time_format = "HH24:MI".to_string();
        params.timezone = None;

        let table = "users".to_string();
        let mut sql_constructor =
            SearchSuggestionSQLConstructor::new(params, table, true, None);

        let filtered_fields = json!({ "users": ["name"] });

        let filter = FilterCriteria::Criteria {
            field: "name".to_string(),
            entity: Some("users".to_string()),
            operator: FilterOperator::Equal,
            values: vec![serde_json::Value::String("test_value".to_string())],
            case_sensitive: Some(false),
            parse_as: "text".to_string(),
            match_pattern: Some(MatchPattern::Contains),
            is_search: Some(true),
            has_group_count: Some(false),
        };

        let advance_filters = vec![serde_json::to_value(&filter).expect("Failed to serialize FilterCriteria to JSON for advance_filters")];
        let group_advance_filters: Vec<serde_json::Value> = Vec::new();

        let concatenated_expressions: ConcatenatedExpressions = HashMap::new();

        let sql = sql_constructor
            .construct(
                &filtered_fields,
                &advance_filters,
                &group_advance_filters,
                "test_value",
                &concatenated_expressions,
            )
            .expect("SQL should be constructed");

        assert!(
            sql.contains("::text"),
            "SQL query should contain ::text cast when parse_as is text. Got: {}",
            sql
        );
    }

    #[test]
    fn should_have_at_least_one_search_advance_filter() {
        let payload = SearchSuggestionParams {
            advance_filters: vec![
                FilterCriteria::Criteria {
                    field: "name".to_string(),
                    entity: Some("users".to_string()),
                    operator: FilterOperator::Equal,
                    values: vec![serde_json::Value::String("test_value".to_string())],
                    case_sensitive: Some(false),
                    parse_as: "string".to_string(),
                    match_pattern: Some(MatchPattern::Contains),
                    is_search: Some(true),
                    has_group_count: Some(false),
                },
                FilterCriteria::Criteria {
                    field: "status".to_string(),
                    entity: Some("users".to_string()),
                    operator: FilterOperator::Equal,
                    values: vec![serde_json::Value::String("Active".to_string())],
                    case_sensitive: Some(false),
                    parse_as: "string".to_string(),
                    match_pattern: Some(MatchPattern::Exact),
                    is_search: Some(false),
                    has_group_count: Some(false),
                },
            ],
            ..SearchSuggestionParams::default()
        };

        let search_filters: Vec<&FilterCriteria> = payload
            .advance_filters
            .iter()
            .filter(|f| match f {
                FilterCriteria::Criteria { is_search, .. } => is_search.unwrap_or(false),
                _ => false,
            })
            .collect();

        assert!(
            !search_filters.is_empty(),
            "Should have at least one advance_filter with is_search = true"
        );
    }

    #[test]
    fn should_cast_value_to_text_when_parse_as_text_in_advance_filter() {
        let mut params = SearchSuggestionParams::default();
        params.date_format = "YYYY-MM-DD".to_string();
        params.time_format = "HH24:MI".to_string();
        params.timezone = None;

        let table = "users".to_string();
        let mut sql_constructor =
            SearchSuggestionSQLConstructor::new(params, table, true, None);

        let filtered_fields = json!({ "users": ["name"] });

        let filter = FilterCriteria::Criteria {
            field: "name".to_string(),
            entity: Some("users".to_string()),
            operator: FilterOperator::Equal,
            values: vec![serde_json::Value::String("test_value".to_string())],
            case_sensitive: Some(false),
            parse_as: "text".to_string(),
            match_pattern: Some(MatchPattern::Contains),
            is_search: Some(true),
            has_group_count: Some(false),
        };

        let advance_filters = vec![serde_json::to_value(&filter).expect("Failed to serialize FilterCriteria to JSON for advance_filters")];
        let group_advance_filters: Vec<serde_json::Value> = Vec::new();

        let concatenated_expressions: ConcatenatedExpressions = HashMap::new();

        let sql = sql_constructor
            .construct(
                &filtered_fields,
                &advance_filters,
                &group_advance_filters,
                "test_value",
                &concatenated_expressions,
            )
            .expect("SQL should be constructed");

        assert!(
            sql.contains("::text AS value"),
            "SQL query should contain '::text AS value' when parse_as is text. Got: {}",
            sql
        );
    }
}
