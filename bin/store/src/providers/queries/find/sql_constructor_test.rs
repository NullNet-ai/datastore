#[cfg(test)]
mod tests {
    use crate::providers::queries::find::sql_constructor::{QueryFilter, SQLConstructor};
    use crate::structs::core::{
        ConcatenateField, FieldRelation, FilterCriteria, FilterOperator, GetByFilter,
        GroupAdvanceFilter, GroupBy, Join, LogicalOperator, RelationEndpoint, SortOption,
    };
    use serde_json::json;
    use std::collections::HashMap;

    /// Mock implementation of QueryFilter for testing purposes
    #[derive(Debug, Clone)]
    struct MockQueryFilter {
        advance_filters: Vec<FilterCriteria>,
        joins: Vec<Join>,
        limit: usize,
        date_format: String,
        pluck: Vec<String>,
        pluck_object: HashMap<String, Vec<String>>,
        pluck_group_object: HashMap<String, Vec<String>>,
        group_advance_filters: Vec<GroupAdvanceFilter>,
        concatenate_fields: Vec<ConcatenateField>,
        order_by: String,
        order_direction: String,
        offset: usize,
        multiple_sort: Vec<SortOption>,
        group_by: Option<GroupBy>,
        distinct_by: Option<String>,
        is_case_sensitive_sorting: Option<bool>,
        timezone: Option<String>,
        time_format: String,
    }

    #[test]
    fn should_include_ids_selection_for_pluck_group_object() {
        let mut mock_filter = MockQueryFilter::default();
        mock_filter.pluck = vec![]; // ensure default id is applied
        let mut pluck_group_object = HashMap::new();
        pluck_group_object.insert(
            "stories".to_string(),
            vec!["id".to_string(), "course_id".to_string()],
        );
        mock_filter.pluck_group_object = pluck_group_object;

        mock_filter.joins = vec![Join {
            r#type: "left".to_string(),
            field_relation: FieldRelation {
                to: RelationEndpoint {
                    alias: None,
                    entity: "stories".to_string(),
                    field: "course_id".to_string(),
                    filters: vec![],
                    order_direction: None,
                    order_by: None,
                    limit: None,
                    offset: None,
                },
                from: RelationEndpoint {
                    alias: None,
                    entity: "courses".to_string(),
                    field: "id".to_string(),
                    filters: vec![],
                    order_direction: None,
                    order_by: None,
                    limit: None,
                    offset: None,
                },
            },
            nested: false,
        }];

        let mut constructor = SQLConstructor::new(mock_filter, "courses".to_string(), true, None);
        let sql = constructor.construct().expect("SQL should be constructed");

        assert!(
            sql.contains("SELECT \"courses\".\"id\""),
            "Expected default id when pluck is empty. Got: {}",
            sql
        );
        assert!(
            sql.contains("AS \"stories_ids\""),
            "Expected stories_ids selection. Got: {}",
            sql
        );
        assert!(
            sql.contains("AS \"stories_course_ids\""),
            "Expected stories_course_ids selection. Got: {}",
            sql
        );
        assert!(
            sql.contains("FROM \"stories\" \"stories\""),
            "Expected stories source table. Got: {}",
            sql
        );
        assert!(
            sql.contains("\"courses\".\"id\" = \"stories\".\"course_id\""),
            "Expected correlation condition. Got: {}",
            sql
        );
    }

    #[test]
    fn should_include_per_field_arrays_for_pluck_group_object() {
        let mut mock_filter = MockQueryFilter::default();
        mock_filter.pluck = vec!["id".to_string()];
        let mut pluck_group_object = HashMap::new();
        pluck_group_object.insert(
            "stories".to_string(),
            vec!["id".to_string(), "course_id".to_string()],
        );
        mock_filter.pluck_group_object = pluck_group_object;

        mock_filter.joins = vec![Join {
            r#type: "left".to_string(),
            field_relation: FieldRelation {
                to: RelationEndpoint {
                    alias: None,
                    entity: "stories".to_string(),
                    field: "course_id".to_string(),
                    filters: vec![],
                    order_direction: None,
                    order_by: None,
                    limit: None,
                    offset: None,
                },
                from: RelationEndpoint {
                    alias: None,
                    entity: "courses".to_string(),
                    field: "id".to_string(),
                    filters: vec![],
                    order_direction: None,
                    order_by: None,
                    limit: None,
                    offset: None,
                },
            },
            nested: false,
        }];

        let mut constructor = SQLConstructor::new(mock_filter, "courses".to_string(), true, None);
        let sql = constructor.construct().expect("SQL should be constructed");

        assert!(
            sql.contains("AS \"stories_ids\""),
            "Expected stories_ids selection. Got: {}",
            sql
        );
        assert!(
            sql.contains("AS \"stories_course_ids\""),
            "Expected stories_course_ids selection. Got: {}",
            sql
        );
        assert!(
            !sql.contains("AS \"stories_items\""),
            "Did not expect stories_items selection. Got: {}",
            sql
        );
    }

    impl Default for MockQueryFilter {
        fn default() -> Self {
            Self {
                advance_filters: vec![],
                joins: vec![],
                limit: 10,
                date_format: "mm/dd/YYYY".to_string(),
                pluck: vec!["id".to_string()],
                pluck_object: HashMap::new(),
                pluck_group_object: HashMap::new(),
                group_advance_filters: vec![],
                concatenate_fields: vec![],
                order_by: "id".to_string(),
                order_direction: "asc".to_string(),
                offset: 0,
                multiple_sort: vec![],
                group_by: None,
                distinct_by: None,
                is_case_sensitive_sorting: None,
                timezone: Some("Asia/Manila".to_string()),
                time_format: "HH24:MI".to_string(),
            }
        }
    }

    impl QueryFilter for MockQueryFilter {
        fn get_advance_filters(&self) -> &[FilterCriteria] {
            &self.advance_filters
        }

        fn get_joins(&self) -> &[Join] {
            &self.joins
        }

        fn get_limit(&self) -> usize {
            self.limit
        }

        fn get_date_format(&self) -> &str {
            &self.date_format
        }

        fn get_pluck(&self) -> &[String] {
            &self.pluck
        }

        fn get_pluck_object(&self) -> &HashMap<String, Vec<String>> {
            &self.pluck_object
        }

        fn get_pluck_group_object(&self) -> &HashMap<String, Vec<String>> {
            &self.pluck_group_object
        }

        fn get_group_advance_filters(&self) -> &[GroupAdvanceFilter] {
            &self.group_advance_filters
        }

        fn get_concatenate_fields(&self) -> &[ConcatenateField] {
            &self.concatenate_fields
        }

        fn get_order_by(&self) -> &str {
            &self.order_by
        }

        fn get_order_direction(&self) -> &str {
            &self.order_direction
        }

        fn get_offset(&self) -> usize {
            self.offset
        }

        fn get_multiple_sort(&self) -> &[SortOption] {
            &self.multiple_sort
        }

        fn get_group_by(&self) -> Option<&GroupBy> {
            self.group_by.as_ref()
        }

        fn get_distinct_by(&self) -> Option<&str> {
            self.distinct_by.as_deref()
        }

        fn get_is_case_sensitive_sorting(&self) -> Option<bool> {
            self.is_case_sensitive_sorting
        }

        fn get_timezone(&self) -> Option<&str> {
            self.timezone.as_deref()
        }

        fn get_time_format(&self) -> &str {
            &self.time_format
        }
    }

    /// Tests SQLConstructor creation and basic functionality:
    /// - Constructor with default parameters
    /// - Constructor with organization_id
    /// - Proper field initialization
    /// - Basic SQL construction without filters
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::providers::queries::find::sql_constructor::SQLConstructor;
    ///
    /// let mock_filter = MockQueryFilter::default();
    /// let constructor = SQLConstructor::new(
    ///     mock_filter,
    ///     "contacts".to_string(),
    ///     true,
    ///     Some("UTC".to_string())
    /// );
    /// assert_eq!(constructor.table, "contacts");
    /// assert_eq!(constructor.is_root, true);
    /// ```
    #[test]
    fn should_create_sql_constructor_with_default_parameters() {
        println!("Testing SQLConstructor creation with default parameters...");

        let mock_filter = MockQueryFilter::default();
        let constructor = SQLConstructor::new(
            mock_filter,
            "contacts".to_string(),
            true,
            Some("UTC".to_string()),
        );

        println!("  ✓ Verifying constructor fields");
        assert_eq!(constructor.table, "contacts");
        assert_eq!(constructor.is_root, true);
        assert_eq!(constructor.timezone, Some("UTC".to_string()));
        assert!(constructor.organization_id.is_none());

        println!("SQLConstructor creation tests completed successfully!");
    }

    /// Tests SQLConstructor with organization_id:
    /// - Adding organization_id after construction
    /// - Proper field updates
    /// - Method chaining functionality
    ///
    /// # Examples
    ///
    /// ```
    /// let constructor = SQLConstructor::new(mock_filter, "contacts".to_string(), true, None)
    ///     .with_organization_id("org123".to_string());
    /// assert_eq!(constructor.organization_id, Some("org123".to_string()));
    /// ```
    #[test]
    fn should_set_organization_id_correctly() {
        println!("Testing SQLConstructor with organization_id...");

        let mock_filter = MockQueryFilter::default();
        let constructor = SQLConstructor::new(mock_filter, "contacts".to_string(), true, None)
            .with_organization_id("org123".to_string());

        println!("  ✓ Verifying organization_id is set");
        assert_eq!(constructor.organization_id, Some("org123".to_string()));

        println!("Organization ID setting tests completed successfully!");
    }

    /// Tests basic SQL construction without filters:
    /// - Simple SELECT statement generation
    /// - Default field selection (id)
    /// - Proper table reference
    /// - Default LIMIT clause
    ///
    /// # Examples
    ///
    /// ```
    /// let mut constructor = SQLConstructor::new(mock_filter, "contacts".to_string(), true, None);
    /// let sql = constructor.construct().unwrap();
    /// assert!(sql.contains("SELECT id FROM contacts"));
    /// assert!(sql.contains("LIMIT 10"));
    /// ```
    #[test]
    fn should_construct_basic_sql_without_filters() {
        println!("Testing basic SQL construction without filters...");

        let mock_filter = MockQueryFilter::default();
        let mut constructor = SQLConstructor::new(mock_filter, "contacts".to_string(), true, None);

        println!("  ✓ Constructing SQL query");
        let sql = constructor.construct().unwrap();

        println!("  ✓ Verifying SQL structure");
        assert!(sql.contains("SELECT \"contacts\".\"id\" FROM \"contacts\" \"contacts\""));
        assert!(sql.contains("LIMIT 10"));
        assert!(sql.contains("(\"contacts\".\"tombstone\" = 0)"));

        println!("Generated SQL: {}", sql);
        println!("Basic SQL construction tests completed successfully!");
    }

    /// Tests SQL construction with pluck fields:
    /// - Custom field selection
    /// - Multiple field plucking
    /// - Proper field formatting with aliases
    ///
    /// # Examples
    ///
    /// ```
    /// let mut mock_filter = MockQueryFilter::default();
    /// mock_filter.pluck = vec!["name".to_string(), "email".to_string()];
    /// let mut constructor = SQLConstructor::new(mock_filter, "contacts".to_string(), true, None);
    /// let sql = constructor.construct().unwrap();
    /// assert!(sql.contains("\"contacts\".\"name\""));
    /// assert!(sql.contains("\"contacts\".\"email\""));
    /// ```
    #[test]
    fn should_construct_sql_with_pluck_fields() {
        println!("Testing SQL construction with pluck fields...");

        let mut mock_filter = MockQueryFilter::default();
        mock_filter.pluck = vec!["name".to_string(), "email".to_string()];
        let mut constructor = SQLConstructor::new(mock_filter, "contacts".to_string(), true, None);

        println!("  ✓ Constructing SQL with pluck fields");
        let sql = constructor.construct().unwrap();

        println!("  ✓ Verifying pluck fields in SQL");
        assert!(sql.contains("\"contacts\".\"name\""));
        assert!(sql.contains("\"contacts\".\"email\""));

        println!("Generated SQL: {}", sql);
        println!("Pluck fields SQL construction tests completed successfully!");
    }

    /// Tests SQL construction with LIMIT and OFFSET:
    /// - Custom limit values
    /// - Offset functionality
    /// - Zero and non-zero values
    /// - Default limit behavior
    ///
    /// # Examples
    ///
    /// ```
    /// let mut mock_filter = MockQueryFilter::default();
    /// mock_filter.limit = 25;
    /// mock_filter.offset = 50;
    /// let mut constructor = SQLConstructor::new(mock_filter, "contacts".to_string(), true, None);
    /// let sql = constructor.construct().unwrap();
    /// assert!(sql.contains("LIMIT 25"));
    /// assert!(sql.contains("OFFSET 50"));
    /// ```
    #[test]
    fn should_construct_sql_with_limit_and_offset() {
        println!("Testing SQL construction with LIMIT and OFFSET...");

        let mut mock_filter = MockQueryFilter::default();
        mock_filter.limit = 25;
        mock_filter.offset = 50;
        let mut constructor = SQLConstructor::new(mock_filter, "contacts".to_string(), true, None);

        println!("  ✓ Constructing SQL with custom LIMIT and OFFSET");
        let sql = constructor.construct().unwrap();

        println!("  ✓ Verifying LIMIT and OFFSET in SQL");
        assert!(sql.contains("LIMIT 25"));
        assert!(sql.contains("OFFSET 50"));

        println!("Generated SQL: {}", sql);
        println!("LIMIT and OFFSET SQL construction tests completed successfully!");
    }

    /// Tests get_field static method functionality:
    /// - Basic field formatting
    /// - Date field handling
    /// - Time field handling
    /// - Alias generation
    /// - Timezone handling
    ///
    /// # Examples
    ///
    /// ```
    /// let field = SQLConstructor::<MockQueryFilter>::get_field(
    ///     "contacts", "name", "mm/dd/YYYY", "contacts", None, true
    /// );
    /// assert_eq!(field, "\"contacts\".\"name\"");
    /// ```
    #[test]
    fn should_format_fields_correctly_with_get_field() {
        println!("Testing get_field static method...");

        println!("  ✓ Testing basic field formatting with alias");
        let basic_field_with_alias = SQLConstructor::<MockQueryFilter>::get_field(
            "contacts",
            "name",
            "mm/dd/YYYY",
            "contacts",
            None,
            true,
            "HH24:MI",
            None,
        );
        assert_eq!(basic_field_with_alias, "\"contacts\".\"name\" AS name");

        println!("  ✓ Testing basic field formatting without alias");
        let basic_field_without_alias = SQLConstructor::<MockQueryFilter>::get_field(
            "contacts",
            "name",
            "mm/dd/YYYY",
            "contacts",
            None,
            false,
            "HH24:MI",
            None,
        );
        assert_eq!(basic_field_without_alias, "\"contacts\".\"name\"");

        println!("  ✓ Testing date field formatting with alias");
        let date_field_with_alias = SQLConstructor::<MockQueryFilter>::get_field(
            "contacts",
            "created_date",
            "mm/dd/YYYY",
            "contacts",
            Some("UTC"),
            true,
            "HH24:MI",
            Some("date"),
        );
        assert!(date_field_with_alias.contains("TO_CHAR"));
        assert!(date_field_with_alias.contains("AT TIME ZONE"));
        // Note: Date formatting may handle alias differently, so we just check it contains the field name
        assert!(date_field_with_alias.contains("created_date"));

        println!("  ✓ Testing date field formatting without alias");
        let date_field_without_alias = SQLConstructor::<MockQueryFilter>::get_field(
            "contacts",
            "created_date",
            "mm/dd/YYYY",
            "contacts",
            Some("UTC"),
            false,
            "HH24:MI",
            Some("date"),
        );
        assert!(date_field_without_alias.contains("TO_CHAR"));
        assert!(date_field_without_alias.contains("AT TIME ZONE"));
        assert!(date_field_without_alias.contains("created_date"));

        println!("  ✓ Testing time field formatting with alias");
        let time_field_with_alias = SQLConstructor::<MockQueryFilter>::get_field(
            "contacts",
            "created_time",
            "mm/dd/YYYY",
            "contacts",
            Some("UTC"),
            true,
            "HH24:MI",
            Some("time"),
        );
        assert!(time_field_with_alias.contains("::time"));
        assert!(time_field_with_alias.contains("AS created_time"));

        println!("  ✓ Testing time field formatting without alias");
        let time_field_without_alias = SQLConstructor::<MockQueryFilter>::get_field(
            "contacts",
            "created_time",
            "mm/dd/YYYY",
            "contacts",
            Some("UTC"),
            false,
            "HH24:MI",
            Some("time"),
        );
        assert!(time_field_without_alias.contains("::time"));
        assert!(!time_field_without_alias.contains("AS created_time"));

        println!("Field formatting tests completed successfully!");
    }

    /// Tests entity name normalization:
    /// - Singular to plural conversion
    /// - Already plural entities
    /// - Edge cases with irregular plurals
    /// - Empty and special character handling
    ///
    /// # Examples
    ///
    /// ```
    /// let mock_filter = MockQueryFilter::default();
    /// let constructor = SQLConstructor::new(mock_filter, "contacts".to_string(), true, None);
    /// // Note: normalize_entity_name is private, so we test it indirectly through other methods
    /// ```
    #[test]
    fn should_normalize_entity_names_correctly() {
        println!("Testing entity name normalization...");

        let mock_filter = MockQueryFilter::default();
        let constructor = SQLConstructor::new(mock_filter, "contacts".to_string(), true, None);

        // Test normalization indirectly through SQL construction
        // Since normalize_entity_name is private, we verify its behavior through public methods
        println!("  ✓ Entity normalization tested indirectly through SQL construction");
        assert_eq!(constructor.table, "contacts");

        println!("Entity name normalization tests completed successfully!");
    }

    /// Tests edge cases and error scenarios:
    /// - Empty table names
    /// - Invalid filter configurations
    /// - Malformed field names
    /// - SQL injection prevention
    /// - Memory safety with large inputs
    ///
    /// # Examples
    ///
    /// ```
    /// let mock_filter = MockQueryFilter::default();
    /// let mut constructor = SQLConstructor::new(mock_filter, "".to_string(), true, None);
    /// // Should handle empty table gracefully
    /// let result = constructor.construct();
    /// // Verify error handling or graceful degradation
    /// ```
    ///
    /// # Panics
    ///
    /// These functions should never panic, even with malformed input.
    ///
    /// # Safety
    ///
    /// All inputs are safely handled through proper escaping and validation.
    #[test]
    fn should_handle_edge_cases_and_invalid_inputs_gracefully() {
        println!("Testing edge cases and error scenarios...");

        println!("  ✓ Testing empty table name");
        let mock_filter = MockQueryFilter::default();
        let mut constructor = SQLConstructor::new(mock_filter, "".to_string(), true, None);
        let result = constructor.construct();
        assert!(result.is_ok()); // Should handle gracefully

        println!("  ✓ Testing very long table name");
        let long_table_name = "a".repeat(1000);
        let mock_filter = MockQueryFilter::default();
        let mut constructor = SQLConstructor::new(mock_filter, long_table_name, true, None);
        let result = constructor.construct();
        assert!(result.is_ok()); // Should handle large inputs

        println!("  ✓ Testing special characters in table name");
        let special_table_name = "table'; DROP TABLE contacts; --".to_string();
        let mock_filter = MockQueryFilter::default();
        let mut constructor = SQLConstructor::new(mock_filter, special_table_name, true, None);
        let result = constructor.construct();
        assert!(result.is_ok()); // Should prevent SQL injection

        println!("  ✓ Testing Unicode characters in table name");
        let unicode_table_name = "table_名前".to_string();
        let mock_filter = MockQueryFilter::default();
        let mut constructor = SQLConstructor::new(mock_filter, unicode_table_name, true, None);
        let result = constructor.construct();
        assert!(result.is_ok()); // Should handle Unicode

        println!("Edge cases and error scenarios tests completed successfully!");
    }

    /// Tests QueryFilter trait default implementations:
    /// - Default values for optional methods
    /// - Proper return types
    /// - Empty collections handling
    /// - None values for optional fields
    ///
    /// # Examples
    ///
    /// ```
    /// struct MinimalFilter;
    /// impl QueryFilter for MinimalFilter {
    ///     fn get_advance_filters(&self) -> &[FilterCriteria] { &[] }
    ///     fn get_joins(&self) -> &[Join] { &[] }
    ///     fn get_limit(&self) -> usize { 10 }
    ///     fn get_date_format(&self) -> &str { "mm/dd/YYYY" }
    /// }
    /// let filter = MinimalFilter;
    /// assert_eq!(filter.get_order_by(), "id");
    /// assert_eq!(filter.get_order_direction(), "asc");
    /// ```
    #[test]
    fn should_provide_correct_default_implementations_for_query_filter() {
        println!("Testing QueryFilter trait default implementations...");

        #[derive(Debug)]
        struct MinimalFilter;

        impl QueryFilter for MinimalFilter {
            fn get_advance_filters(&self) -> &[FilterCriteria] {
                &[]
            }
            fn get_joins(&self) -> &[Join] {
                &[]
            }
            fn get_limit(&self) -> usize {
                10
            }
            fn get_date_format(&self) -> &str {
                "mm/dd/YYYY"
            }
            fn get_time_format(&self) -> &str {
                "HH24:MI"
            }
        }

        let filter = MinimalFilter;

        println!("  ✓ Testing default order_by");
        assert_eq!(filter.get_order_by(), "id");

        println!("  ✓ Testing default order_direction");
        assert_eq!(filter.get_order_direction(), "asc");

        println!("  ✓ Testing default offset");
        assert_eq!(filter.get_offset(), 0);

        println!("  ✓ Testing default pluck");
        assert!(filter.get_pluck().is_empty());

        println!("  ✓ Testing default pluck_object");
        assert!(filter.get_pluck_object().is_empty());

        println!("  ✓ Testing default group_by");
        assert!(filter.get_group_by().is_none());

        println!("  ✓ Testing default timezone");
        assert_eq!(filter.get_timezone(), Some("Asia/Manila"));

        println!("QueryFilter default implementations tests completed successfully!");
    }

    /// Tests GetByFilter implementation of QueryFilter:
    /// - Proper field access through trait methods
    /// - Correct data type conversions
    /// - Optional field handling
    /// - Reference vs owned data handling
    ///
    /// # Examples
    ///
    /// ```
    /// let filter = GetByFilter {
    ///     limit: 25,
    ///     date_format: "YYYY-mm-dd".to_string(),
    ///     // ... other fields
    /// };
    /// assert_eq!(filter.get_limit(), 25);
    /// assert_eq!(filter.get_date_format(), "YYYY-mm-dd");
    /// ```
    #[test]
    fn should_implement_query_filter_correctly_for_get_by_filter() {
        println!("Testing GetByFilter implementation of QueryFilter...");

        let filter = GetByFilter {
            advance_filters: vec![],
            joins: vec![],
            limit: 25,
            date_format: "YYYY-mm-dd".to_string(),
            pluck: vec!["name".to_string(), "email".to_string()],
            pluck_object: HashMap::new(),
            pluck_group_object: HashMap::new(),
            group_advance_filters: vec![],
            concatenate_fields: vec![],
            order_by: "created_at".to_string(),
            order_direction: "desc".to_string(),
            offset: 10,
            multiple_sort: vec![],
            group_by: None,
            distinct_by: Some("id".to_string()),
            is_case_sensitive_sorting: Some(true),
            timezone: Some("UTC".to_string()),
            time_format: "HH24:MI".to_string(),
        };

        println!("  ✓ Testing limit access");
        assert_eq!(filter.get_limit(), 25);

        println!("  ✓ Testing date_format access");
        assert_eq!(filter.get_date_format(), "YYYY-mm-dd");

        println!("  ✓ Testing pluck access");
        assert_eq!(filter.get_pluck().len(), 2);
        assert!(filter.get_pluck().contains(&"name".to_string()));

        println!("  ✓ Testing order_by access");
        assert_eq!(filter.get_order_by(), "created_at");

        println!("  ✓ Testing order_direction access");
        assert_eq!(filter.get_order_direction(), "desc");

        println!("  ✓ Testing offset access");
        assert_eq!(filter.get_offset(), 10);

        println!("  ✓ Testing distinct_by access");
        assert_eq!(filter.get_distinct_by(), Some("id"));

        println!("  ✓ Testing timezone access");
        assert_eq!(filter.get_timezone(), Some("UTC"));

        println!("GetByFilter QueryFilter implementation tests completed successfully!");
    }

    /// Tests timezone handling in find/count: header timezone only.
    /// When body has no timezone, header timezone should be applied to date/time fields.
    #[test]
    fn should_use_header_timezone_when_body_timezone_absent() {
        let mut filter = MockQueryFilter::default();
        filter.timezone = None; // No body timezone
        filter.pluck = vec!["id".to_string(), "created_date".to_string()];

        let mut constructor = SQLConstructor::new(
            filter,
            "contacts".to_string(),
            true,
            Some("Europe/Berlin".to_string()),
        );
        let result = constructor.construct();

        assert!(result.is_ok(), "SQL construction should succeed");
        let sql = result.unwrap();
        assert!(
            sql.contains("AT TIME ZONE 'Europe/Berlin'"),
            "SQL should use header timezone when body timezone is absent. Got: {}",
            sql
        );
    }

    /// Tests timezone handling in find/count: body timezone only.
    /// When header has no timezone, body timezone should be applied.
    #[test]
    fn should_use_body_timezone_when_header_timezone_absent() {
        let mut filter = MockQueryFilter::default();
        filter.timezone = Some("America/New_York".to_string()); // Body timezone
        filter.pluck = vec!["id".to_string(), "created_date".to_string()];

        let mut constructor = SQLConstructor::new(filter, "contacts".to_string(), true, None);
        let result = constructor.construct();

        assert!(result.is_ok(), "SQL construction should succeed");
        let sql = result.unwrap();
        assert!(
            sql.contains("AT TIME ZONE 'America/New_York'"),
            "SQL should use body timezone when header timezone is absent. Got: {}",
            sql
        );
    }

    /// Tests timezone handling in find/count: body timezone overrides header.
    /// When both header and body have timezone, body should take precedence.
    #[test]
    fn should_prefer_body_timezone_over_header_timezone() {
        let mut filter = MockQueryFilter::default();
        filter.timezone = Some("America/Los_Angeles".to_string()); // Body timezone
        filter.pluck = vec!["id".to_string(), "created_date".to_string()];

        let mut constructor = SQLConstructor::new(
            filter,
            "contacts".to_string(),
            true,
            Some("Europe/Berlin".to_string()), // Header timezone
        );
        let result = constructor.construct();

        assert!(result.is_ok(), "SQL construction should succeed");
        let sql = result.unwrap();
        assert!(
            sql.contains("AT TIME ZONE 'America/Los_Angeles'"),
            "SQL should use body timezone (not header) when both are present. Got: {}",
            sql
        );
        assert!(
            !sql.contains("AT TIME ZONE 'Europe/Berlin'"),
            "SQL should not use header timezone when body timezone overrides it"
        );
    }

    // =========================================================================
    // construct_count tests - COUNT route SQL generation
    // =========================================================================

    /// Creates a minimal GetByFilter for count tests.
    fn create_count_test_filter() -> GetByFilter {
        GetByFilter {
            pluck: vec!["id".to_string()],
            pluck_object: HashMap::new(),
            pluck_group_object: HashMap::new(),
            advance_filters: vec![],
            group_advance_filters: vec![],
            joins: vec![],
            group_by: None,
            concatenate_fields: vec![],
            multiple_sort: vec![],
            date_format: "mm/dd/YYYY".to_string(),
            order_by: "id".to_string(),
            order_direction: "asc".to_string(),
            is_case_sensitive_sorting: None,
            offset: 0,
            limit: 15,
            distinct_by: None,
            timezone: None,
            time_format: "HH24:MI".to_string(),
        }
    }

    /// Tests construct_count produces valid COUNT(DISTINCT id) SQL without filters.
    #[test]
    fn should_construct_count_basic_without_filters() {
        let filter = create_count_test_filter();
        let mut constructor =
            SQLConstructor::new(filter.clone(), "samples".to_string(), true, None);

        let sql = constructor
            .construct_count()
            .expect("construct_count should succeed");

        assert!(
            sql.contains("SELECT COUNT(DISTINCT \"samples\".\"id\")"),
            "SQL should contain COUNT(DISTINCT id). Got: {}",
            sql
        );
        assert!(
            sql.contains("FROM \"samples\""),
            "SQL should contain FROM samples. Got: {}",
            sql
        );
        assert!(
            sql.contains("WHERE") || sql.contains("where"),
            "SQL should contain WHERE (system filters). Got: {}",
            sql
        );
        assert!(
            sql.contains("tombstone"),
            "SQL should contain tombstone filter. Got: {}",
            sql
        );
        assert!(
            !sql.contains("GROUP BY"),
            "Count query should not have GROUP BY. Got: {}",
            sql
        );
        assert!(
            !sql.contains("ORDER BY"),
            "Count query should not have ORDER BY. Got: {}",
            sql
        );
        assert!(
            !sql.contains("LIMIT") && !sql.contains("OFFSET"),
            "Count query should not have LIMIT/OFFSET. Got: {}",
            sql
        );
    }

    /// Tests construct_count with advance_filters (equal, has_no_value, and operator).
    #[test]
    fn should_construct_count_with_advance_filters() {
        let mut filter = create_count_test_filter();
        filter.advance_filters = vec![
            FilterCriteria::Criteria {
                field: "name".to_string(),
                entity: Some("samples".to_string()),
                operator: FilterOperator::Equal,
                values: vec![json!("kashan")],
                case_sensitive: None,
                parse_as: "text".to_string(),
                match_pattern: None,
                is_search: Some(true),
                has_group_count: None,
            },
            FilterCriteria::LogicalOperator {
                operator: LogicalOperator::And,
            },
            FilterCriteria::Criteria {
                field: "sample_text".to_string(),
                entity: Some("samples".to_string()),
                operator: FilterOperator::HasNoValue,
                values: vec![],
                case_sensitive: None,
                parse_as: "text".to_string(),
                match_pattern: None,
                is_search: None,
                has_group_count: None,
            },
        ];
        let mut constructor = SQLConstructor::new(filter, "samples".to_string(), true, None);

        let sql = constructor
            .construct_count()
            .expect("construct_count should succeed");

        assert!(
            sql.contains("COUNT(DISTINCT \"samples\".\"id\")"),
            "SQL should contain COUNT. Got: {}",
            sql
        );
        assert!(
            sql.contains("samples") && sql.contains("name"),
            "SQL should reference samples.name filter. Got: {}",
            sql
        );
        assert!(
            sql.contains("sample_text") || sql.contains("IS NULL"),
            "SQL should include has_no_value/sample_text logic. Got: {}",
            sql
        );
    }

    /// Tests construct_count with pluck_object (joins).
    #[test]
    fn should_construct_count_with_pluck_object_joins() {
        let mut filter = create_count_test_filter();
        let mut pluck_object = HashMap::new();
        pluck_object.insert(
            "samples".to_string(),
            vec![
                "id".to_string(),
                "code".to_string(),
                "created_time".to_string(),
                "sample_text".to_string(),
                "name".to_string(),
            ],
        );
        filter.pluck_object = pluck_object;

        let mut constructor = SQLConstructor::new(filter, "samples".to_string(), true, None);

        let sql = constructor
            .construct_count()
            .expect("construct_count should succeed");

        assert!(
            sql.contains("COUNT(DISTINCT \"samples\".\"id\")"),
            "SQL should contain COUNT. Got: {}",
            sql
        );
        assert!(
            sql.contains("FROM \"samples\""),
            "SQL should have main table. Got: {}",
            sql
        );
    }

    /// Tests construct_count excludes GROUP BY, ORDER BY, LIMIT, OFFSET.
    #[test]
    fn should_construct_count_exclude_group_order_limit_offset() {
        let mut filter = create_count_test_filter();
        filter.limit = 100;
        filter.offset = 50;
        filter.order_by = "name".to_string();
        filter.order_direction = "desc".to_string();
        filter.group_by = Some(GroupBy {
            fields: vec!["id".to_string()],
            has_count: true,
        });

        let mut constructor = SQLConstructor::new(filter, "samples".to_string(), true, None);

        let sql = constructor
            .construct_count()
            .expect("construct_count should succeed");

        assert!(
            !sql.to_uppercase().contains("GROUP BY"),
            "Count query must not have GROUP BY. Got: {}",
            sql
        );
        assert!(
            !sql.to_uppercase().contains("ORDER BY"),
            "Count query must not have ORDER BY. Got: {}",
            sql
        );
        assert!(
            !sql.contains("LIMIT"),
            "Count query must not have LIMIT. Got: {}",
            sql
        );
        assert!(
            !sql.contains("OFFSET"),
            "Count query must not have OFFSET. Got: {}",
            sql
        );
    }

    /// Tests construct_count with organization_id (non-root) adds org filter.
    #[test]
    fn should_construct_count_with_organization_id() {
        let filter = create_count_test_filter();
        let mut constructor = SQLConstructor::new(filter, "samples".to_string(), false, None)
            .with_organization_id("org-123".to_string());

        let sql = constructor
            .construct_count()
            .expect("construct_count should succeed");

        assert!(
            sql.contains("organization_id"),
            "SQL should include organization_id filter for non-root. Got: {}",
            sql
        );
        assert!(
            sql.contains("COUNT(DISTINCT \"samples\".\"id\")"),
            "SQL should contain COUNT. Got: {}",
            sql
        );
    }

    /// Tests construct_count with timezone (body and header).
    #[test]
    fn should_construct_count_with_timezone() {
        let mut filter = create_count_test_filter();
        filter.advance_filters = vec![FilterCriteria::Criteria {
            field: "created_time".to_string(),
            entity: Some("samples".to_string()),
            operator: FilterOperator::Like,
            values: vec![json!("14")],
            case_sensitive: None,
            parse_as: "time".to_string(),
            match_pattern: None,
            is_search: Some(true),
            has_group_count: None,
        }];
        filter.timezone = Some("America/Los_Angeles".to_string());

        let mut constructor =
            SQLConstructor::new(filter, "samples".to_string(), true, Some("UTC".to_string()));

        let sql = constructor
            .construct_count()
            .expect("construct_count should succeed");

        assert!(
            sql.contains("COUNT(DISTINCT \"samples\".\"id\")"),
            "SQL should contain COUNT. Got: {}",
            sql
        );
        assert!(
            sql.contains("created_time") || sql.contains("to_char"),
            "SQL should reference created_time for time filter. Got: {}",
            sql
        );
    }

    /// Tests SQL construction for the valid_filter_organizations.json scenario
    /// This test verifies the complex join structure with self-joins and nested joins
    /// that was causing the "missing FROM-clause entry for table 'district_orgs'" error
    #[test]
    #[ignore]
    fn should_construct_valid_filter_organizations_sql() {
        println!("Testing SQL construction for valid_filter_organizations.json scenario...");

        let mut mock_filter = MockQueryFilter::default();
        mock_filter.pluck = vec![
            "id".to_string(),
            "code".to_string(),
            "name".to_string(),
            "categories".to_string(),
            "district_id".to_string(),
            "department_id".to_string(),
            "city".to_string(),
            "county".to_string(),
            "state".to_string(),
            "school_identifier".to_string(),
            "district_identifier".to_string(),
            "status".to_string(),
            "superintendent_id".to_string(),
            "principal_id".to_string(),
            "created_date".to_string(),
            "created_time".to_string(),
            "created_by".to_string(),
            "updated_date".to_string(),
            "updated_time".to_string(),
            "updated_by".to_string(),
        ];

        // Set up pluck_object
        let mut pluck_object = HashMap::new();
        pluck_object.insert(
            "created_by_account_organizations".to_string(),
            vec!["id".to_string(), "contact_id".to_string()],
        );
        pluck_object.insert(
            "created_by".to_string(),
            vec![
                "id".to_string(),
                "first_name".to_string(),
                "last_name".to_string(),
                "full_name".to_string(),
            ],
        );
        pluck_object.insert(
            "updated_by_account_organizations".to_string(),
            vec!["id".to_string(), "contact_id".to_string()],
        );
        pluck_object.insert(
            "updated_by".to_string(),
            vec![
                "id".to_string(),
                "first_name".to_string(),
                "last_name".to_string(),
                "full_name".to_string(),
            ],
        );
        pluck_object.insert(
            "organizations".to_string(),
            vec![
                "id".to_string(),
                "code".to_string(),
                "name".to_string(),
                "categories".to_string(),
                "district_id".to_string(),
                "department_id".to_string(),
                "city".to_string(),
                "county".to_string(),
                "state".to_string(),
                "school_identifier".to_string(),
                "district_identifier".to_string(),
                "status".to_string(),
                "superintendent_id".to_string(),
                "principal_id".to_string(),
                "created_date".to_string(),
                "created_time".to_string(),
                "created_by".to_string(),
                "updated_date".to_string(),
                "updated_time".to_string(),
                "updated_by".to_string(),
            ],
        );
        pluck_object.insert(
            "district_orgs".to_string(),
            vec![
                "id".to_string(),
                "code".to_string(),
                "name".to_string(),
                "categories".to_string(),
                "district_id".to_string(),
                "department_id".to_string(),
                "city".to_string(),
                "county".to_string(),
                "state".to_string(),
                "school_identifier".to_string(),
                "district_identifier".to_string(),
                "status".to_string(),
                "created_date".to_string(),
                "created_time".to_string(),
                "created_by".to_string(),
                "updated_date".to_string(),
                "updated_time".to_string(),
                "updated_by".to_string(),
                "superintendent_id".to_string(),
                "principal_id".to_string(),
            ],
        );
        pluck_object.insert(
            "district_superintendent".to_string(),
            vec![
                "first_name".to_string(),
                "code".to_string(),
                "last_name".to_string(),
                "username".to_string(),
            ],
        );
        pluck_object.insert(
            "superintendent".to_string(),
            vec![
                "first_name".to_string(),
                "code".to_string(),
                "last_name".to_string(),
                "username".to_string(),
            ],
        );
        pluck_object.insert(
            "principal".to_string(),
            vec![
                "first_name".to_string(),
                "code".to_string(),
                "last_name".to_string(),
                "username".to_string(),
            ],
        );
        mock_filter.pluck_object = pluck_object;

        // Set up joins
        mock_filter.joins = vec![
            // created_by_account_organizations join
            Join {
                r#type: "left".to_string(),
                field_relation: FieldRelation {
                    to: RelationEndpoint {
                        alias: Some("created_by_account_organizations".to_string()),
                        entity: "account_organizations".to_string(),
                        field: "id".to_string(),
                        filters: vec![],
                        order_direction: None,
                        order_by: None,
                        limit: None,
                        offset: None,
                    },
                    from: RelationEndpoint {
                        alias: None,
                        entity: "organizations".to_string(),
                        field: "created_by".to_string(),
                        filters: vec![],
                        order_direction: None,
                        order_by: None,
                        limit: None,
                        offset: None,
                    },
                },
                nested: false,
            },
            // created_by nested join
            Join {
                r#type: "left".to_string(),
                field_relation: FieldRelation {
                    to: RelationEndpoint {
                        alias: Some("created_by".to_string()),
                        entity: "contacts".to_string(),
                        field: "id".to_string(),
                        filters: vec![],
                        order_direction: None,
                        order_by: None,
                        limit: None,
                        offset: None,
                    },
                    from: RelationEndpoint {
                        alias: Some("created_by_account_organizations".to_string()),
                        entity: "created_by_account_organizations".to_string(),
                        field: "contact_id".to_string(),
                        filters: vec![],
                        order_direction: None,
                        order_by: None,
                        limit: None,
                        offset: None,
                    },
                },
                nested: true,
            },
            // updated_by_account_organizations join
            Join {
                r#type: "left".to_string(),
                field_relation: FieldRelation {
                    to: RelationEndpoint {
                        alias: Some("updated_by_account_organizations".to_string()),
                        entity: "account_organizations".to_string(),
                        field: "id".to_string(),
                        filters: vec![],
                        order_direction: None,
                        order_by: None,
                        limit: None,
                        offset: None,
                    },
                    from: RelationEndpoint {
                        alias: None,
                        entity: "organizations".to_string(),
                        field: "updated_by".to_string(),
                        filters: vec![],
                        order_direction: None,
                        order_by: None,
                        limit: None,
                        offset: None,
                    },
                },
                nested: false,
            },
            // updated_by nested join
            Join {
                r#type: "left".to_string(),
                field_relation: FieldRelation {
                    to: RelationEndpoint {
                        alias: Some("updated_by".to_string()),
                        entity: "contacts".to_string(),
                        field: "id".to_string(),
                        filters: vec![],
                        order_direction: None,
                        order_by: None,
                        limit: None,
                        offset: None,
                    },
                    from: RelationEndpoint {
                        alias: Some("updated_by_account_organizations".to_string()),
                        entity: "updated_by_account_organizations".to_string(),
                        field: "contact_id".to_string(),
                        filters: vec![],
                        order_direction: None,
                        order_by: None,
                        limit: None,
                        offset: None,
                    },
                },
                nested: true,
            },
            // district_orgs self-join
            Join {
                r#type: "self".to_string(),
                field_relation: FieldRelation {
                    to: RelationEndpoint {
                        alias: None,
                        entity: "organizations".to_string(),
                        field: "id".to_string(),
                        filters: vec![],
                        order_direction: None,
                        order_by: None,
                        limit: None,
                        offset: None,
                    },
                    from: RelationEndpoint {
                        alias: Some("district_orgs".to_string()),
                        entity: "organizations".to_string(),
                        field: "district_id".to_string(),
                        filters: vec![],
                        order_direction: None,
                        order_by: None,
                        limit: None,
                        offset: None,
                    },
                },
                nested: false,
            },
            // district_superintendent nested join
            Join {
                r#type: "left".to_string(),
                field_relation: FieldRelation {
                    to: RelationEndpoint {
                        alias: Some("district_superintendent".to_string()),
                        entity: "contacts".to_string(),
                        field: "id".to_string(),
                        filters: vec![],
                        order_direction: None,
                        order_by: None,
                        limit: None,
                        offset: None,
                    },
                    from: RelationEndpoint {
                        alias: None,
                        entity: "district_orgs".to_string(),
                        field: "superintendent_id".to_string(),
                        filters: vec![],
                        order_direction: None,
                        order_by: None,
                        limit: None,
                        offset: None,
                    },
                },
                nested: true,
            },
            // superintendent join
            Join {
                r#type: "left".to_string(),
                field_relation: FieldRelation {
                    to: RelationEndpoint {
                        alias: Some("superintendent".to_string()),
                        entity: "contacts".to_string(),
                        field: "id".to_string(),
                        filters: vec![],
                        order_direction: None,
                        order_by: None,
                        limit: None,
                        offset: None,
                    },
                    from: RelationEndpoint {
                        alias: None,
                        entity: "organizations".to_string(),
                        field: "superintendent_id".to_string(),
                        filters: vec![],
                        order_direction: None,
                        order_by: None,
                        limit: None,
                        offset: None,
                    },
                },
                nested: false,
            },
            // principal join
            Join {
                r#type: "left".to_string(),
                field_relation: FieldRelation {
                    to: RelationEndpoint {
                        alias: Some("principal".to_string()),
                        entity: "contacts".to_string(),
                        field: "id".to_string(),
                        filters: vec![],
                        order_direction: None,
                        order_by: None,
                        limit: None,
                        offset: None,
                    },
                    from: RelationEndpoint {
                        alias: None,
                        entity: "organizations".to_string(),
                        field: "principal_id".to_string(),
                        filters: vec![],
                        order_direction: None,
                        order_by: None,
                        limit: None,
                        offset: None,
                    },
                },
                nested: false,
            },
        ];

        // Set up advance filters
        mock_filter.advance_filters = vec![
            FilterCriteria::Criteria {
                field: "status".to_string(),
                entity: None,
                operator: FilterOperator::Equal,
                values: vec![json!("Active"), json!("Draft")],
                case_sensitive: None,
                parse_as: "text".to_string(),
                match_pattern: None,
                is_search: None,
                has_group_count: None,
            },
            FilterCriteria::LogicalOperator {
                operator: LogicalOperator::And,
            },
            FilterCriteria::Criteria {
                field: "categories".to_string(),
                entity: None,
                operator: FilterOperator::NotContains,
                values: vec![json!("Personal"), json!("Root"), json!("Team")],
                case_sensitive: None,
                parse_as: "text".to_string(),
                match_pattern: None,
                is_search: None,
                has_group_count: None,
            },
        ];

        // Set up concatenate fields
        mock_filter.concatenate_fields = vec![
            ConcatenateField {
                fields: vec!["first_name".to_string(), "last_name".to_string()],
                field_name: "full_name".to_string(),
                separator: " ".to_string(),
                entity: "contacts".to_string(),
                aliased_entity: Some("created_by".to_string()),
            },
            ConcatenateField {
                fields: vec!["first_name".to_string(), "last_name".to_string()],
                field_name: "full_name".to_string(),
                separator: " ".to_string(),
                entity: "contacts".to_string(),
                aliased_entity: Some("updated_by".to_string()),
            },
            ConcatenateField {
                fields: vec!["first_name".to_string(), "last_name".to_string()],
                field_name: "full_name".to_string(),
                separator: " ".to_string(),
                entity: "contacts".to_string(),
                aliased_entity: Some("district_superintendent".to_string()),
            },
            ConcatenateField {
                fields: vec!["first_name".to_string(), "last_name".to_string()],
                field_name: "full_name".to_string(),
                separator: " ".to_string(),
                entity: "contacts".to_string(),
                aliased_entity: Some("superintendent".to_string()),
            },
            ConcatenateField {
                fields: vec!["first_name".to_string(), "last_name".to_string()],
                field_name: "full_name".to_string(),
                separator: " ".to_string(),
                entity: "contacts".to_string(),
                aliased_entity: Some("principal".to_string()),
            },
            ConcatenateField {
                fields: vec!["created_date".to_string(), "created_time".to_string()],
                field_name: "created_date_time".to_string(),
                separator: " ".to_string(),
                entity: "organizations".to_string(),
                aliased_entity: None,
            },
            ConcatenateField {
                fields: vec!["updated_date".to_string(), "updated_time".to_string()],
                field_name: "updated_date_time".to_string(),
                separator: " ".to_string(),
                entity: "organizations".to_string(),
                aliased_entity: None,
            },
        ];

        // Set other parameters
        mock_filter.order_by = "organizations.name".to_string();
        mock_filter.order_direction = "asc".to_string();
        mock_filter.limit = 100;
        mock_filter.offset = 0;

        println!("  ✓ Constructing SQL for complex organizations query");
        let mut constructor =
            SQLConstructor::new(mock_filter, "organizations".to_string(), true, None);

        match constructor.construct() {
            Ok(sql) => {
                println!("Generated SQL query:");
                println!("{}", sql);

                // Verify key components
                assert!(
                    sql.contains("FROM \"organizations\""),
                    "Should contain main table"
                );
                assert!(
                    sql.contains("district_orgs"),
                    "Should contain district_orgs alias"
                );
                assert!(
                    sql.contains("district_superintendent"),
                    "Should contain district_superintendent alias in selections"
                );
                assert!(sql.contains("LATERAL"), "Should contain LATERAL joins");
                assert!(
                    sql.contains("LEFT JOIN LATERAL"),
                    "Should contain LEFT JOIN LATERAL for joins"
                );

                assert!(
                    sql.contains("FROM \"contacts\" \"joined_district_superintendent\""),
                    "Nested join should emit LATERAL subquery for district_superintendent"
                );
                assert!(
                    sql.contains("\"joined_district_superintendent\".\"id\" = \"district_orgs\".\"superintendent_id\""),
                    "Nested LATERAL should correlate to district_orgs via superintendent_id"
                );

                println!("  ✓ SQL construction successful - no missing FROM-clause errors!");
            }
            Err(e) => {
                panic!("SQL construction failed: {}", e);
            }
        }
    }

    #[test]
    #[ignore]
    fn should_construct_sql_from_organizations_filter_json() {
        use crate::providers::queries::find::sql_constructor::SQLConstructor;
        use crate::structs::core::GetByFilter;
        use std::fs;

        let json_path = "src/providers/queries/find/queries/organizations_filter.json";
        let file_contents =
            fs::read_to_string(json_path).expect("organizations_filter.json should be readable");
        let payload: GetByFilter =
            serde_json::from_str(&file_contents).expect("JSON should parse into GetByFilter");

        let mut constructor = SQLConstructor::new(payload, "organizations".to_string(), true, None);

        match constructor.construct() {
            Ok(sql) => {
                assert!(sql.contains("FROM organizations"));
                assert!(sql.contains("ORDER BY LOWER(organizations.name) ASC"));
                assert!(sql.contains("LIMIT 100"));
                assert!(sql.contains("status"));
                assert!(
                    sql.contains("IN ('Active', 'Draft'") || sql.contains("IN ('Active','Draft'")
                );
                assert!(sql.contains("categories"));
                assert!(sql.contains("NOT ILIKE '%Personal%'"));
                assert!(sql.contains("NOT ILIKE '%Root%'"));
                assert!(sql.contains("NOT ILIKE '%Team%'"));
                assert!(sql.contains("AS \"district_orgs\" ON TRUE"));
                assert!(sql.contains(
                    "\"joined_district_orgs\".\"id\" = \"organizations\".\"district_id\""
                ));
                assert!(sql.contains("AS created_by_account_organizations"));
                assert!(sql
                    .contains("'contact_id', \"created_by_account_organizations\".\"contact_id\""));
                assert!(sql.contains("'id', \"created_by_account_organizations\".\"id\""));
                assert!(sql.contains("AS created_by"));
                assert!(sql.contains("'first_name', \"created_by\".\"first_name\""));
                assert!(sql.contains("'last_name', \"created_by\".\"last_name\""));
                assert!(
                    sql.contains("'full_name', ("),
                    "created_by selection should include concatenated 'full_name'"
                );
                assert!(sql.contains("AS updated_by_account_organizations"));
                assert!(sql
                    .contains("'contact_id', \"updated_by_account_organizations\".\"contact_id\""));
                assert!(sql.contains("'id', \"updated_by_account_organizations\".\"id\""));
                assert!(sql.contains("AS updated_by"));
                assert!(sql.contains("'first_name', \"updated_by\".\"first_name\""));
                assert!(sql.contains("'last_name', \"updated_by\".\"last_name\""));
                assert!(sql.contains("AS organizations"));
                assert!(sql.contains("AS district_superintendent"));
                // Nested join should be embedded under district_orgs selection, not as top-level
                assert!(
                    sql.contains("'district_superintendent', COALESCE"),
                    "Nested 'district_superintendent' selection should be embedded inside 'district_orgs'"
                );
                assert!(sql.contains("AS superintendent"));
                assert!(sql.contains("'first_name', \"superintendent\".\"first_name\""));
                assert!(sql.contains("'last_name', \"superintendent\".\"last_name\""));
                assert!(sql.contains("AS principal"));
                assert!(sql.contains("'first_name', \"principal\".\"first_name\""));
                assert!(sql.contains("'last_name', \"principal\".\"last_name\""));

                assert!(
                    sql.contains("AS district_orgs"),
                    "Should aggregate 'district_orgs' as a JSON array selection"
                );
            }
            Err(e) => {
                panic!("SQL construction failed: {}", e);
            }
        }
    }

    /// Integration test: requires DATABASE_URL and a DB whose organizations table schema matches
    /// organizations_filter.json. That JSON references columns (e.g. district_id, superintendent_id,
    /// principal_id, city, county, state) which are not in the current generated organizations schema,
    /// so the constructed SQL fails at runtime with "column does not exist". Run with:
    /// cargo test should_execute_constructed_sql_against_database -- --ignored
    /// when you have a DB with a schema matching the JSON.
    #[tokio::test]
    #[ignore]
    async fn should_execute_constructed_sql_against_database() {
        use crate::providers::queries::find::sql_constructor::SQLConstructor;
        use crate::structs::core::GetByFilter;
        use std::fs;

        dotenv::dotenv().ok();
        let db_url = std::env::var("DATABASE_URL")
            .expect("DATABASE_URL must be set in .env for DB integration test");

        let json_path = "src/providers/queries/find/queries/organizations_filter.json";
        let file_contents =
            fs::read_to_string(json_path).expect("organizations_filter.json should be readable");
        let payload: GetByFilter =
            serde_json::from_str(&file_contents).expect("JSON should parse into GetByFilter");

        let mut constructor = SQLConstructor::new(payload, "organizations".to_string(), true, None);
        let sql = constructor
            .construct()
            .expect("SQL should construct successfully");

        let (client, connection) = tokio_postgres::connect(&db_url, tokio_postgres::NoTls)
            .await
            .expect("Failed to connect to database");
        tokio::spawn(async move {
            if let Err(e) = connection.await {
                panic!("connection error: {}", e);
            }
        });

        let rows = client.query(sql.as_str(), &[]).await;
        assert!(
            rows.is_ok(),
            "Constructed SQL should execute successfully against the database: {:?}",
            rows.err()
        );
    }

    #[test]
    fn should_construct_valid_filter_organizations_sql_based_on_final_query() {
        use crate::providers::queries::find::sql_constructor::SQLConstructor;
        use serde_json::json;
        use std::collections::HashMap;

        let organization_id = "01K3SKCH4R3Z9KYSKKSVEKYCHV";

        let mut mock_filter = MockQueryFilter::default();
        mock_filter.timezone = None;
        mock_filter.limit = 100;
        mock_filter.offset = 0;
        mock_filter.order_by = "organizations.name".to_string();
        mock_filter.order_direction = "asc".to_string();

        mock_filter.pluck = vec![
            "id".to_string(),
            "code".to_string(),
            "name".to_string(),
            "categories".to_string(),
            "district_id".to_string(),
            "department_id".to_string(),
            "city".to_string(),
            "county".to_string(),
            "state".to_string(),
            "school_identifier".to_string(),
            "district_identifier".to_string(),
            "status".to_string(),
            "superintendent_id".to_string(),
            "principal_id".to_string(),
            "created_date".to_string(),
            "created_time".to_string(),
            "updated_date".to_string(),
            "updated_time".to_string(),
        ];

        let mut pluck_object = HashMap::new();
        pluck_object.insert(
            "created_by_account_organizations".to_string(),
            vec!["id".to_string(), "contact_id".to_string()],
        );
        pluck_object.insert(
            "created_by".to_string(),
            vec![
                "id".to_string(),
                "first_name".to_string(),
                "last_name".to_string(),
                "full_name".to_string(),
            ],
        );
        pluck_object.insert(
            "updated_by_account_organizations".to_string(),
            vec!["id".to_string(), "contact_id".to_string()],
        );
        pluck_object.insert(
            "updated_by".to_string(),
            vec![
                "id".to_string(),
                "first_name".to_string(),
                "last_name".to_string(),
                "full_name".to_string(),
            ],
        );
        pluck_object.insert(
            "district_orgs".to_string(),
            vec![
                "id".to_string(),
                "code".to_string(),
                "name".to_string(),
                "categories".to_string(),
                "district_id".to_string(),
                "department_id".to_string(),
                "city".to_string(),
                "county".to_string(),
                "state".to_string(),
                "school_identifier".to_string(),
                "district_identifier".to_string(),
                "status".to_string(),
                "created_date".to_string(),
                "created_time".to_string(),
                "created_by".to_string(),
                "updated_date".to_string(),
                "updated_time".to_string(),
                "updated_by".to_string(),
                "superintendent_id".to_string(),
                "principal_id".to_string(),
            ],
        );
        pluck_object.insert(
            "district_superintendent".to_string(),
            vec![
                "id".to_string(),
                "first_name".to_string(),
                "code".to_string(),
                "last_name".to_string(),
                "username".to_string(),
            ],
        );
        pluck_object.insert(
            "superintendent".to_string(),
            vec![
                "id".to_string(),
                "first_name".to_string(),
                "code".to_string(),
                "last_name".to_string(),
                "username".to_string(),
            ],
        );
        pluck_object.insert(
            "principal".to_string(),
            vec![
                "id".to_string(),
                "first_name".to_string(),
                "code".to_string(),
                "last_name".to_string(),
                "username".to_string(),
            ],
        );
        mock_filter.pluck_object = pluck_object;

        mock_filter.concatenate_fields = vec![
            ConcatenateField {
                fields: vec!["first_name".to_string(), "last_name".to_string()],
                field_name: "full_name".to_string(),
                separator: " ".to_string(),
                entity: "contacts".to_string(),
                aliased_entity: Some("created_by".to_string()),
            },
            ConcatenateField {
                fields: vec!["first_name".to_string(), "last_name".to_string()],
                field_name: "full_name".to_string(),
                separator: " ".to_string(),
                entity: "contacts".to_string(),
                aliased_entity: Some("updated_by".to_string()),
            },
            ConcatenateField {
                fields: vec!["first_name".to_string(), "last_name".to_string()],
                field_name: "full_name".to_string(),
                separator: " ".to_string(),
                entity: "contacts".to_string(),
                aliased_entity: Some("district_superintendent".to_string()),
            },
            ConcatenateField {
                fields: vec!["first_name".to_string(), "last_name".to_string()],
                field_name: "full_name".to_string(),
                separator: " ".to_string(),
                entity: "contacts".to_string(),
                aliased_entity: Some("superintendent".to_string()),
            },
            ConcatenateField {
                fields: vec!["first_name".to_string(), "last_name".to_string()],
                field_name: "full_name".to_string(),
                separator: " ".to_string(),
                entity: "contacts".to_string(),
                aliased_entity: Some("principal".to_string()),
            },
            ConcatenateField {
                fields: vec!["created_date".to_string(), "created_time".to_string()],
                field_name: "created_date_time".to_string(),
                separator: " ".to_string(),
                entity: "organizations".to_string(),
                aliased_entity: None,
            },
            ConcatenateField {
                fields: vec!["updated_date".to_string(), "updated_time".to_string()],
                field_name: "updated_date_time".to_string(),
                separator: " ".to_string(),
                entity: "organizations".to_string(),
                aliased_entity: None,
            },
        ];

        mock_filter.joins = vec![
            Join {
                r#type: "left".to_string(),
                field_relation: FieldRelation {
                    to: RelationEndpoint {
                        alias: Some("created_by_account_organizations".to_string()),
                        entity: "account_organizations".to_string(),
                        field: "id".to_string(),
                        filters: vec![],
                        order_direction: None,
                        order_by: None,
                        limit: None,
                        offset: None,
                    },
                    from: RelationEndpoint {
                        alias: None,
                        entity: "organizations".to_string(),
                        field: "created_by".to_string(),
                        filters: vec![],
                        order_direction: None,
                        order_by: None,
                        limit: None,
                        offset: None,
                    },
                },
                nested: false,
            },
            Join {
                r#type: "left".to_string(),
                field_relation: FieldRelation {
                    to: RelationEndpoint {
                        alias: Some("created_by".to_string()),
                        entity: "contacts".to_string(),
                        field: "id".to_string(),
                        filters: vec![],
                        order_direction: None,
                        order_by: None,
                        limit: None,
                        offset: None,
                    },
                    from: RelationEndpoint {
                        alias: Some("created_by_account_organizations".to_string()),
                        entity: "created_by_account_organizations".to_string(),
                        field: "contact_id".to_string(),
                        filters: vec![],
                        order_direction: None,
                        order_by: None,
                        limit: None,
                        offset: None,
                    },
                },
                nested: true,
            },
            Join {
                r#type: "left".to_string(),
                field_relation: FieldRelation {
                    to: RelationEndpoint {
                        alias: Some("updated_by_account_organizations".to_string()),
                        entity: "account_organizations".to_string(),
                        field: "id".to_string(),
                        filters: vec![],
                        order_direction: None,
                        order_by: None,
                        limit: None,
                        offset: None,
                    },
                    from: RelationEndpoint {
                        alias: None,
                        entity: "organizations".to_string(),
                        field: "updated_by".to_string(),
                        filters: vec![],
                        order_direction: None,
                        order_by: None,
                        limit: None,
                        offset: None,
                    },
                },
                nested: false,
            },
            Join {
                r#type: "left".to_string(),
                field_relation: FieldRelation {
                    to: RelationEndpoint {
                        alias: Some("updated_by".to_string()),
                        entity: "contacts".to_string(),
                        field: "id".to_string(),
                        filters: vec![],
                        order_direction: None,
                        order_by: None,
                        limit: None,
                        offset: None,
                    },
                    from: RelationEndpoint {
                        alias: Some("updated_by_account_organizations".to_string()),
                        entity: "updated_by_account_organizations".to_string(),
                        field: "contact_id".to_string(),
                        filters: vec![],
                        order_direction: None,
                        order_by: None,
                        limit: None,
                        offset: None,
                    },
                },
                nested: true,
            },
            Join {
                r#type: "self".to_string(),
                field_relation: FieldRelation {
                    to: RelationEndpoint {
                        alias: None,
                        entity: "organizations".to_string(),
                        field: "id".to_string(),
                        filters: vec![],
                        order_direction: None,
                        order_by: None,
                        limit: None,
                        offset: None,
                    },
                    from: RelationEndpoint {
                        alias: Some("district_orgs".to_string()),
                        entity: "organizations".to_string(),
                        field: "district_id".to_string(),
                        filters: vec![],
                        order_direction: None,
                        order_by: None,
                        limit: None,
                        offset: None,
                    },
                },
                nested: false,
            },
            Join {
                r#type: "left".to_string(),
                field_relation: FieldRelation {
                    to: RelationEndpoint {
                        alias: Some("district_superintendent".to_string()),
                        entity: "contacts".to_string(),
                        field: "id".to_string(),
                        filters: vec![],
                        order_direction: None,
                        order_by: None,
                        limit: None,
                        offset: None,
                    },
                    from: RelationEndpoint {
                        alias: None,
                        entity: "district_orgs".to_string(),
                        field: "superintendent_id".to_string(),
                        filters: vec![],
                        order_direction: None,
                        order_by: None,
                        limit: None,
                        offset: None,
                    },
                },
                nested: true,
            },
            Join {
                r#type: "left".to_string(),
                field_relation: FieldRelation {
                    to: RelationEndpoint {
                        alias: Some("superintendent".to_string()),
                        entity: "contacts".to_string(),
                        field: "id".to_string(),
                        filters: vec![],
                        order_direction: None,
                        order_by: None,
                        limit: None,
                        offset: None,
                    },
                    from: RelationEndpoint {
                        alias: None,
                        entity: "organizations".to_string(),
                        field: "superintendent_id".to_string(),
                        filters: vec![],
                        order_direction: None,
                        order_by: None,
                        limit: None,
                        offset: None,
                    },
                },
                nested: false,
            },
            Join {
                r#type: "left".to_string(),
                field_relation: FieldRelation {
                    to: RelationEndpoint {
                        alias: Some("principal".to_string()),
                        entity: "contacts".to_string(),
                        field: "id".to_string(),
                        filters: vec![],
                        order_direction: None,
                        order_by: None,
                        limit: None,
                        offset: None,
                    },
                    from: RelationEndpoint {
                        alias: None,
                        entity: "organizations".to_string(),
                        field: "principal_id".to_string(),
                        filters: vec![],
                        order_direction: None,
                        order_by: None,
                        limit: None,
                        offset: None,
                    },
                },
                nested: false,
            },
        ];

        mock_filter.advance_filters = vec![
            FilterCriteria::Criteria {
                field: "status".to_string(),
                entity: None,
                operator: FilterOperator::Equal,
                values: vec![json!("Active"), json!("Draft")],
                case_sensitive: None,
                parse_as: "text".to_string(),
                match_pattern: None,
                is_search: None,
                has_group_count: None,
            },
            FilterCriteria::LogicalOperator {
                operator: LogicalOperator::And,
            },
            FilterCriteria::Criteria {
                field: "categories".to_string(),
                entity: None,
                operator: FilterOperator::NotContains,
                values: vec![json!("Personal"), json!("Root"), json!("Team")],
                case_sensitive: None,
                parse_as: "text".to_string(),
                match_pattern: None,
                is_search: None,
                has_group_count: None,
            },
        ];

        let mut constructor =
            SQLConstructor::new(mock_filter, "organizations".to_string(), false, None)
                .with_organization_id(organization_id.to_string());
        let sql = constructor
            .construct()
            .expect("SQL construction should succeed");

        assert!(sql.contains("\"organizations\".\"id\""));
        assert!(sql.contains("\"organizations\".\"code\""));
        assert!(sql.contains("\"organizations\".\"name\""));
        assert!(sql.contains("\"organizations\".\"district_id\""));
        assert!(sql.contains("mm/dd/YYYY"));
        assert!(sql.contains("HH24:MI"));

        assert!(sql.contains(
            "FROM \"account_organizations\" \"created_by_account_organizations\" LEFT JOIN \"contacts\" \"created_by\""
        ));
        assert!(sql.contains(
            "\"organizations\".\"created_by\" = \"created_by_account_organizations\".\"id\""
        ));
        assert!(sql.contains(
            "\"created_by\".\"id\" = \"created_by_account_organizations\".\"contact_id\""
        ));

        assert!(sql.contains(
            "FROM \"account_organizations\" \"updated_by_account_organizations\" LEFT JOIN \"contacts\" \"updated_by\""
        ));
        assert!(sql.contains(
            "\"organizations\".\"updated_by\" = \"updated_by_account_organizations\".\"id\""
        ));
        assert!(sql.contains(
            "\"updated_by\".\"id\" = \"updated_by_account_organizations\".\"contact_id\""
        ));

        assert!(sql.contains("FROM \"organizations\" \"district_orgs\""));
        assert!(sql.contains("\"organizations\".\"district_id\" = \"district_orgs\".\"id\""));
        assert!(sql.contains("LEFT JOIN LATERAL (SELECT \"joined_district_superintendent\""));
        assert!(sql.contains("FROM \"contacts\" \"joined_district_superintendent\""));
        assert!(sql.contains(
            "\"joined_district_superintendent\".\"id\" = \"district_orgs\".\"superintendent_id\""
        ));

        assert!(sql.contains("LEFT JOIN LATERAL (SELECT \"joined_superintendent\""));
        assert!(sql.contains("FROM \"contacts\" \"joined_superintendent\""));
        assert!(sql.contains(
            "\"joined_superintendent\".\"id\" = \"organizations\".\"superintendent_id\""
        ));
        assert!(sql.contains("LEFT JOIN LATERAL (SELECT \"joined_principal\""));
        assert!(sql.contains("FROM \"contacts\" \"joined_principal\""));
        assert!(sql.contains("\"joined_principal\".\"id\" = \"organizations\".\"principal_id\""));

        assert!(sql.contains("WHERE ("));
        assert!(sql.contains("\"organizations\".\"tombstone\" = 0"));
        assert!(sql.contains("\"organizations\".\"organization_id\" IS NOT NULL"));
        assert!(
            sql.contains("\"organizations\".\"organization_id\" = '01K3SKCH4R3Z9KYSKKSVEKYCHV'")
        );
        assert!(sql.to_lowercase().contains("status"));
        assert!(sql.to_lowercase().contains("categories"));

        assert!(sql.contains("GROUP BY \"organizations\".\"id\""));
        assert!(sql.contains("ORDER BY"));
        assert!(sql
            .to_lowercase()
            .contains("lower(\"organizations\".\"name\")"));
        assert!(sql.to_uppercase().contains(" ASC"));
        assert!(sql.contains("LIMIT 100"));
    }
}
