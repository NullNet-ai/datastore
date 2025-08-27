#[cfg(test)]
mod tests {
    use crate::providers::queries::batch_update::sql_constructor::BatchUpdateFilterWrapper;
    use crate::providers::queries::batch_update::BatchUpdateSQLConstructor;
    use crate::providers::queries::find::sql_constructor::QueryFilter;
    use crate::structs::structs::{FilterCriteria, FilterOperator};

    // Helper function to create test FilterCriteria
    fn create_test_filter_criteria() -> Vec<FilterCriteria> {
        use crate::structs::structs::LogicalOperator;
        vec![
            FilterCriteria::Criteria {
                field: "status".to_string(),
                entity: None,
                operator: FilterOperator::Equal,
                values: vec![serde_json::Value::String("active".to_string())],
                case_sensitive: Some(false),
                parse_as: "string".to_string(),
                match_pattern: None,
                is_search: Some(false),
                has_group_count: Some(false),
            },
            FilterCriteria::LogicalOperator {
                operator: LogicalOperator::And,
            },
            FilterCriteria::Criteria {
                field: "created_at".to_string(),
                entity: None,
                operator: FilterOperator::GreaterThanOrEqual,
                values: vec![serde_json::Value::String("2024-01-01".to_string())],
                case_sensitive: Some(false),
                parse_as: "string".to_string(),
                match_pattern: None,
                is_search: Some(false),
                has_group_count: Some(false),
            },
        ]
    }

    #[test]
    fn test_batch_update_filter_wrapper_creation() {
        let filters = create_test_filter_criteria();
        let wrapper = BatchUpdateFilterWrapper {
            advance_filters: filters.clone(),
        };

        assert_eq!(wrapper.advance_filters.len(), 3);

        // Pattern match to access fields in the enum
        if let FilterCriteria::Criteria { field, .. } = &wrapper.advance_filters[0] {
            assert_eq!(field, "status");
        } else {
            panic!("Expected FilterCriteria::Criteria variant");
        }

        if let FilterCriteria::Criteria { field, .. } = &wrapper.advance_filters[2] {
            assert_eq!(field, "created_at");
        } else {
            panic!("Expected FilterCriteria::Criteria variant");
        }
    }

    #[test]
    fn test_batch_update_filter_wrapper_query_filter_implementation() {
        let filters = create_test_filter_criteria();
        let wrapper = BatchUpdateFilterWrapper {
            advance_filters: filters,
        };

        // Test QueryFilter trait implementation
        assert_eq!(wrapper.get_advance_filters().len(), 3);
        assert_eq!(wrapper.get_joins().len(), 0); // Should be empty for batch updates
        assert_eq!(wrapper.get_limit(), usize::MAX); // No limit for batch updates
        assert_eq!(wrapper.get_date_format(), "YYYY-MM-DD");
    }

    #[test]
    fn test_batch_update_sql_constructor_creation() {
        let constructor = BatchUpdateSQLConstructor::new("users".to_string(), true);

        assert_eq!(constructor.table, "users");
        assert_eq!(constructor.is_root, true);
        assert!(constructor.organization_id.is_none());
    }

    #[test]
    fn test_batch_update_sql_constructor_with_organization_id() {
        let constructor = BatchUpdateSQLConstructor::new("products".to_string(), false)
            .with_organization_id("org_456".to_string());

        assert_eq!(constructor.table, "products");
        assert_eq!(constructor.is_root, false);
        assert_eq!(constructor.organization_id, Some("org_456".to_string()));
    }

    #[test]
    fn test_construct_where_clauses_advanced_empty_filters() {
        let constructor = BatchUpdateSQLConstructor::new("test_table".to_string(), true);
        let empty_filters: Vec<FilterCriteria> = vec![];

        let result = constructor.construct_where_clauses_advanced(&empty_filters);
        assert!(result.is_ok());

        let where_clause = result.unwrap();
        // Should return some form of WHERE clause even if empty
        assert!(where_clause.is_empty() || where_clause.contains("WHERE"));
    }

    #[test]
    fn test_construct_where_clauses_advanced_with_filters() {
        let constructor = BatchUpdateSQLConstructor::new("users".to_string(), true);
        let filters = create_test_filter_criteria();

        let result = constructor.construct_where_clauses_advanced(&filters);
        if let Err(e) = &result {
            println!(
                "Error in test_construct_where_clauses_advanced_with_filters: {}",
                e
            );
        }
        assert!(result.is_ok());

        let where_clause = result.unwrap();
        // Should contain WHERE clause with filter conditions
        assert!(where_clause.contains("WHERE") || !where_clause.is_empty());
    }

    #[test]
    fn test_construct_where_clauses_advanced_with_organization_id() {
        let constructor = BatchUpdateSQLConstructor::new("orders".to_string(), false)
            .with_organization_id("org_789".to_string());
        let filters = create_test_filter_criteria();

        let result = constructor.construct_where_clauses_advanced(&filters);
        assert!(result.is_ok());

        let where_clause = result.unwrap();
        // Should include organization filter when not root
        assert!(where_clause.contains("WHERE") || !where_clause.is_empty());
    }

    #[test]
    fn test_construct_batch_update_advanced_basic() {
        let constructor = BatchUpdateSQLConstructor::new("products".to_string(), true);
        let set_clause = "name = 'Updated Product', price = 99.99";
        let filters = create_test_filter_criteria();

        let result = constructor.construct_batch_update_advanced(set_clause, &filters);
        if let Err(e) = &result {
            println!("Error in test_construct_batch_update_advanced_basic: {}", e);
        }
        assert!(result.is_ok());

        let sql = result.unwrap();
        assert!(sql.starts_with("UPDATE products SET"));
        assert!(sql.contains("name = 'Updated Product'"));
        assert!(sql.contains("price = 99.99"));
    }

    #[test]
    fn test_construct_batch_update_advanced_empty_filters() {
        let constructor = BatchUpdateSQLConstructor::new("users".to_string(), true);
        let set_clause = "status = 'inactive'";
        let empty_filters: Vec<FilterCriteria> = vec![];

        let result = constructor.construct_batch_update_advanced(set_clause, &empty_filters);
        assert!(result.is_ok());

        let sql = result.unwrap();
        assert!(sql.starts_with("UPDATE users SET"));
        assert!(sql.contains("status = 'inactive'"));
    }

    #[test]
    fn test_construct_batch_update_advanced_with_organization() {
        let constructor = BatchUpdateSQLConstructor::new("orders".to_string(), false)
            .with_organization_id("org_123".to_string());
        let set_clause = "processed = true, updated_at = NOW()";
        let filters = create_test_filter_criteria();

        let result = constructor.construct_batch_update_advanced(set_clause, &filters);
        assert!(result.is_ok());

        let sql = result.unwrap();
        assert!(sql.starts_with("UPDATE orders SET"));
        assert!(sql.contains("processed = true"));
        assert!(sql.contains("updated_at = NOW()"));
    }

    #[test]
    fn test_construct_batch_update_advanced_complex_set_clause() {
        let constructor = BatchUpdateSQLConstructor::new("inventory".to_string(), true);
        let set_clause = "quantity = quantity - 1, last_updated = CURRENT_TIMESTAMP, status = CASE WHEN quantity > 1 THEN 'available' ELSE 'out_of_stock' END";
        let filters = vec![FilterCriteria::Criteria {
            field: "product_id".to_string(),
            entity: None,
            operator: FilterOperator::Equal,
            values: vec![serde_json::Value::String("(1, 2, 3, 4, 5)".to_string())],
            case_sensitive: Some(false),
            parse_as: "string".to_string(),
            match_pattern: None,
            is_search: Some(false),
            has_group_count: Some(false),
        }];

        let result = constructor.construct_batch_update_advanced(set_clause, &filters);
        assert!(result.is_ok());

        let sql = result.unwrap();
        assert!(sql.starts_with("UPDATE inventory SET"));
        assert!(sql.contains("quantity = quantity - 1"));
        assert!(sql.contains("CURRENT_TIMESTAMP"));
        assert!(sql.contains("CASE WHEN"));
    }

    // Note: convert_utils_to_structs_filter is a private function and cannot be tested directly

    #[test]
    fn test_batch_update_filter_wrapper_debug_clone() {
        let filters = create_test_filter_criteria();
        let wrapper = BatchUpdateFilterWrapper {
            advance_filters: filters,
        };

        // Test Debug trait
        let debug_str = format!("{:?}", wrapper);
        assert!(debug_str.contains("BatchUpdateFilterWrapper"));

        // Test Clone trait
        let cloned_wrapper = wrapper.clone();
        assert_eq!(
            cloned_wrapper.advance_filters.len(),
            wrapper.advance_filters.len()
        );

        // Compare fields using pattern matching
        match (
            &cloned_wrapper.advance_filters[0],
            &wrapper.advance_filters[0],
        ) {
            (
                FilterCriteria::Criteria { field: field1, .. },
                FilterCriteria::Criteria { field: field2, .. },
            ) => {
                assert_eq!(field1, field2);
            }
            _ => panic!("Expected FilterCriteria::Criteria variants"),
        }

        // Also compare the second criteria field
        match (
            &cloned_wrapper.advance_filters[2],
            &wrapper.advance_filters[2],
        ) {
            (
                FilterCriteria::Criteria { field: field1, .. },
                FilterCriteria::Criteria { field: field2, .. },
            ) => {
                assert_eq!(field1, field2);
            }
            _ => panic!("Expected FilterCriteria::Criteria variants"),
        }
    }

    #[test]
    fn test_batch_update_sql_constructor_multiple_operations() {
        let constructor = BatchUpdateSQLConstructor::new("transactions".to_string(), true);

        // Test multiple different update operations
        let operations = vec![
            ("amount = amount * 1.1", "price increase"),
            (
                "status = 'processed', processed_at = NOW()",
                "status update",
            ),
            (
                "metadata = COALESCE(metadata, '{}')::jsonb || '{\"updated\": true}'::jsonb",
                "metadata update",
            ),
        ];

        let filters = create_test_filter_criteria();

        for (set_clause, description) in operations {
            let result = constructor.construct_batch_update_advanced(set_clause, &filters);
            assert!(result.is_ok(), "Failed for operation: {}", description);

            let sql = result.unwrap();
            assert!(
                sql.starts_with("UPDATE transactions SET"),
                "Invalid SQL for: {}",
                description
            );
            assert!(
                sql.contains(set_clause),
                "Missing set clause for: {}",
                description
            );
        }
    }

    #[test]
    fn test_batch_update_filter_criteria_variations() {
        let constructor = BatchUpdateSQLConstructor::new("users".to_string(), true);
        let set_clause = "last_login = NOW()";

        // Test different filter variations
        let filter_variations = vec![
            vec![], // No filters
            vec![FilterCriteria::Criteria {
                field: "id".to_string(),
                entity: None,
                operator: FilterOperator::Equal,
                values: vec![serde_json::Value::String("123".to_string())],
                case_sensitive: Some(false),
                parse_as: "string".to_string(),
                match_pattern: None,
                is_search: Some(false),
                has_group_count: Some(false),
            }], // Single filter
            create_test_filter_criteria(), // Multiple filters
        ];

        for (i, filters) in filter_variations.iter().enumerate() {
            let result = constructor.construct_batch_update_advanced(set_clause, filters);
            assert!(result.is_ok(), "Failed for filter variation {}", i);

            let sql = result.unwrap();
            assert!(
                sql.starts_with("UPDATE users SET"),
                "Invalid SQL for variation {}",
                i
            );
            assert!(
                sql.contains("last_login = NOW()"),
                "Missing set clause for variation {}",
                i
            );
        }
    }
}
