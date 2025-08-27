#[cfg(test)]
mod tests {
    use crate::providers::queries::search_suggestion::structs::{
        AliasedJoinedEntity, ConcatenatedExpressions, FieldExpression, FieldFiltersResult,
        FormatFilterResponse, SearchSuggestionCache,
    };
    use crate::providers::queries::search_suggestion::utils::{
        format_filters, generate_concatenated_expressions, get_field_filters,
    };
    use crate::structs::structs::{
        ConcatenateField, FilterCriteria, FilterOperator, MatchPattern,
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

    #[tokio::test]
    async fn test_aliased_joined_entity_creation() {
        let entity = create_aliased_joined_entity();
        
        assert_eq!(entity.to_entity, "profiles");
        assert_eq!(entity.alias, "p");
    }

    #[tokio::test]
    async fn test_aliased_joined_entity_clone() {
        let entity = create_aliased_joined_entity();
        let cloned_entity = entity.clone();
        
        assert_eq!(entity.to_entity, cloned_entity.to_entity);
        assert_eq!(entity.alias, cloned_entity.alias);
    }

    #[tokio::test]
    async fn test_field_expression_creation() {
        let field_expr = FieldExpression {
            expression: "CONCAT(first_name, ' ', last_name)".to_string(),
            fields: vec!["first_name".to_string(), "last_name".to_string()],
        };
        
        assert_eq!(field_expr.expression, "CONCAT(first_name, ' ', last_name)");
        assert_eq!(field_expr.fields.len(), 2);
        assert!(field_expr.fields.contains(&"first_name".to_string()));
        assert!(field_expr.fields.contains(&"last_name".to_string()));
    }

    #[tokio::test]
    async fn test_field_expression_clone() {
        let field_expr = FieldExpression {
            expression: "test_expression".to_string(),
            fields: vec!["field1".to_string(), "field2".to_string()],
        };
        
        let cloned_expr = field_expr.clone();
        assert_eq!(field_expr.expression, cloned_expr.expression);
        assert_eq!(field_expr.fields, cloned_expr.fields);
    }

    #[tokio::test]
    async fn test_field_expression_serialization() {
        let field_expr = FieldExpression {
            expression: "test_expression".to_string(),
            fields: vec!["field1".to_string()],
        };
        
        let serialized = serde_json::to_string(&field_expr).unwrap();
        assert!(serialized.contains("test_expression"));
        assert!(serialized.contains("field1"));
    }

    #[tokio::test]
    async fn test_concatenated_expressions_type() {
        let mut concatenated_expressions: ConcatenatedExpressions = HashMap::new();
        let mut entity_map = HashMap::new();
        
        let field_expr = FieldExpression {
            expression: "CONCAT(first_name, ' ', last_name)".to_string(),
            fields: vec!["first_name".to_string(), "last_name".to_string()],
        };
        
        entity_map.insert("full_name".to_string(), field_expr);
        concatenated_expressions.insert("users".to_string(), entity_map);
        
        assert!(concatenated_expressions.contains_key("users"));
        let users_map = concatenated_expressions.get("users").unwrap();
        assert!(users_map.contains_key("full_name"));
    }

    #[tokio::test]
    async fn test_format_filter_response_creation() {
        let response = FormatFilterResponse {
            formatted_filters: vec![json!({"field": "name", "value": "test"})],
            search_term: "test_search".to_string(),
            filtered_fields: json!({"users": ["name", "email"]}),
        };
        
        assert_eq!(response.formatted_filters.len(), 1);
        assert_eq!(response.search_term, "test_search");
        assert!(response.filtered_fields.is_object());
    }

    #[tokio::test]
    async fn test_field_filters_result_creation() {
        let filter = create_default_filter_criteria();
        let result = FieldFiltersResult {
            all_field_filters: vec![filter.clone()],
            field_filter: Some(filter),
        };
        
        assert_eq!(result.all_field_filters.len(), 1);
        assert!(result.field_filter.is_some());
    }

    #[tokio::test]
    async fn test_field_filters_result_with_none() {
        let filter = create_default_filter_criteria();
        let result = FieldFiltersResult {
            all_field_filters: vec![filter],
            field_filter: None,
        };
        
        assert_eq!(result.all_field_filters.len(), 1);
        assert!(result.field_filter.is_none());
    }

    #[tokio::test]
    async fn test_search_suggestion_cache_hash_string() {
        let input = "test_string";
        let hash1 = SearchSuggestionCache::hash_string(input);
        let hash2 = SearchSuggestionCache::hash_string(input);
        
        // Same input should produce same hash
        assert_eq!(hash1, hash2);
        
        // Hash should be a valid hex string
        assert!(hash1.chars().all(|c| c.is_ascii_hexdigit()));
        
        // Different inputs should produce different hashes
        let different_hash = SearchSuggestionCache::hash_string("different_string");
        assert_ne!(hash1, different_hash);
    }

    #[tokio::test]
    async fn test_search_suggestion_cache_hash_empty_string() {
        let hash = SearchSuggestionCache::hash_string("");
        assert!(!hash.is_empty());
        assert!(hash.chars().all(|c| c.is_ascii_hexdigit()));
    }

    #[tokio::test]
    async fn test_get_field_filters_with_matching_field() {
        let filter = create_default_filter_criteria();
        let filters = vec![filter];
        
        let result = get_field_filters(filters, "name", "users", "test_search");
        
        assert_eq!(result.all_field_filters.len(), 1);
        assert!(result.field_filter.is_some());
    }

    #[tokio::test]
    async fn test_get_field_filters_with_non_matching_field() {
        let filter = create_default_filter_criteria();
        let filters = vec![filter];
        
        let result = get_field_filters(filters, "email", "users", "test_search");
        
        // The filter is still added to all_field_filters but not as field_filter since field doesn't match
        assert_eq!(result.all_field_filters.len(), 0); // Non-search criteria are not added when values don't match search_term
        assert!(result.field_filter.is_none());
    }

    #[tokio::test]
    async fn test_get_field_filters_with_non_matching_entity() {
        let filter = create_default_filter_criteria();
        let filters = vec![filter];
        
        let result = get_field_filters(filters, "name", "profiles", "test_search");
        
        // The filter is not added because entity doesn't match and values don't match search_term
        assert_eq!(result.all_field_filters.len(), 0);
        assert!(result.field_filter.is_none());
    }

    #[tokio::test]
    async fn test_get_field_filters_empty_filters() {
        let filters = vec![];
        
        let result = get_field_filters(filters, "name", "users", "test_search");
        
        assert_eq!(result.all_field_filters.len(), 0);
        assert!(result.field_filter.is_none());
    }

    #[tokio::test]
    async fn test_generate_concatenated_expressions() {
        let concatenate_field = create_concatenate_field();
        let concatenate_fields = vec![concatenate_field];
        
        let result = generate_concatenated_expressions(concatenate_fields, Some("YYYY-MM-DD"));
        
        assert!(result.contains_key("users"));
        let users_map = result.get("users").unwrap();
        assert!(users_map.contains_key("full_name"));
        
        let field_expr = users_map.get("full_name").unwrap();
        // The function uses PostgreSQL || operator, not CONCAT
        assert!(field_expr.expression.contains("COALESCE"));
        assert!(field_expr.expression.contains("||"));
        assert_eq!(field_expr.fields.len(), 2);
    }

    #[tokio::test]
    async fn test_generate_concatenated_expressions_empty() {
        let concatenate_fields = vec![];
        
        let result = generate_concatenated_expressions(concatenate_fields, None);
        
        assert!(result.is_empty());
    }

    #[tokio::test]
    async fn test_generate_concatenated_expressions_with_separator() {
        let mut concatenate_field = create_concatenate_field();
        concatenate_field.separator = " - ".to_string();
        let concatenate_fields = vec![concatenate_field];
        
        let result = generate_concatenated_expressions(concatenate_fields, None);
        
        let users_map = result.get("users").unwrap();
        let field_expr = users_map.get("full_name").unwrap();
        assert!(field_expr.expression.contains(" - "));
    }

    #[tokio::test]
    async fn test_format_filters_basic() {
        let filter = create_default_filter_criteria();
        let filters = vec![filter];
        let aliased_entities = vec![create_aliased_joined_entity()];
        let filtered_fields = json!({"users": ["name"]});
        
        let result = format_filters(
            filters,
            Some(&aliased_entities),
            "users",
            filtered_fields.clone(),
            String::new(),
        );
        
        assert!(!result.formatted_filters.is_empty());
        assert_eq!(result.search_term, "test_value");
        assert_eq!(result.filtered_fields, filtered_fields);
    }

    #[tokio::test]
    async fn test_format_filters_without_aliased_entities() {
        let filter = create_default_filter_criteria();
        let filters = vec![filter];
        let filtered_fields = json!({"users": ["name"]});
        
        let result = format_filters(
            filters,
            None,
            "users",
            filtered_fields.clone(),
            String::new(),
        );
        
        assert!(!result.formatted_filters.is_empty());
        assert_eq!(result.search_term, "test_value");
    }

    #[tokio::test]
    async fn test_format_filters_empty_filters() {
        let filters = vec![];
        let filtered_fields = json!({"users": ["name"]});
        
        let result = format_filters(
            filters,
            None,
            "users",
            filtered_fields.clone(),
            String::new(),
        );
        
        assert!(result.formatted_filters.is_empty());
        assert!(result.search_term.is_empty());
    }

    #[tokio::test]
    async fn test_concatenate_field_creation() {
        let concatenate_field = create_concatenate_field();
        
        assert_eq!(concatenate_field.entity, "users");
        assert_eq!(concatenate_field.field_name, "full_name");
        assert_eq!(concatenate_field.fields.len(), 2);
        assert_eq!(concatenate_field.separator, " ".to_string());
    }

    #[tokio::test]
    async fn test_concatenate_field_with_different_separator() {
        let mut concatenate_field = create_concatenate_field();
        concatenate_field.separator = "-".to_string();
        
        assert_eq!(concatenate_field.separator, "-".to_string());
    }

    #[tokio::test]
    async fn test_filter_criteria_debug_format() {
        let filter = create_default_filter_criteria();
        let debug_str = format!("{:?}", filter);
        
        assert!(debug_str.contains("Criteria"));
        assert!(debug_str.contains("name"));
        assert!(debug_str.contains("users"));
    }

    #[tokio::test]
    async fn test_aliased_joined_entity_debug_format() {
        let entity = create_aliased_joined_entity();
        let debug_str = format!("{:?}", entity);
        
        assert!(debug_str.contains("AliasedJoinedEntity"));
        assert!(debug_str.contains("profiles"));
        assert!(debug_str.contains("p"));
    }

    #[tokio::test]
    async fn test_field_expression_debug_format() {
        let field_expr = FieldExpression {
            expression: "test_expression".to_string(),
            fields: vec!["field1".to_string()],
        };
        
        let debug_str = format!("{:?}", field_expr);
        assert!(debug_str.contains("FieldExpression"));
        assert!(debug_str.contains("test_expression"));
    }

    #[tokio::test]
    async fn test_format_filter_response_debug_format() {
        let response = FormatFilterResponse {
            formatted_filters: vec![json!({"test": "value"})],
            search_term: "test".to_string(),
            filtered_fields: json!({"field": "value"}),
        };
        
        let debug_str = format!("{:?}", response);
        assert!(debug_str.contains("FormatFilterResponse"));
    }

    #[tokio::test]
    async fn test_field_filters_result_debug_format() {
        let filter = create_default_filter_criteria();
        let result = FieldFiltersResult {
            all_field_filters: vec![filter.clone()],
            field_filter: Some(filter),
        };
        
        let debug_str = format!("{:?}", result);
        assert!(debug_str.contains("FieldFiltersResult"));
    }

    // Test environment variable handling
    #[tokio::test]
    async fn test_default_search_pattern_fallback() {
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
        assert!(!result.formatted_filters.is_empty());
    }

    #[tokio::test]
    async fn test_multiple_concatenate_fields() {
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
            fields: vec!["street".to_string(), "city".to_string(), "state".to_string()],
            separator: ", ".to_string(),
            aliased_entity: None,
        };
        
        let concatenate_fields = vec![concatenate_field1, concatenate_field2];
        let result = generate_concatenated_expressions(concatenate_fields, None);
        
        assert!(result.contains_key("users"));
        let users_map = result.get("users").unwrap();
        assert!(users_map.contains_key("full_name"));
        assert!(users_map.contains_key("address"));
        
        let address_expr = users_map.get("address").unwrap();
        assert_eq!(address_expr.fields.len(), 3);
        assert!(address_expr.expression.contains(", "));
    }

    #[tokio::test]
    async fn test_multiple_entities_concatenated_expressions() {
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
        let result = generate_concatenated_expressions(concatenate_fields, None);
        
        assert!(result.contains_key("users"));
        assert!(result.contains_key("profiles"));
        
        let users_map = result.get("users").unwrap();
        let profiles_map = result.get("profiles").unwrap();
        
        assert!(users_map.contains_key("full_name"));
        assert!(profiles_map.contains_key("display_name"));
    }
}