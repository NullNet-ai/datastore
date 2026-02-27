use crate::providers::queries::find::sql_constructor::QueryFilter;
use crate::structs::core::{ConcatenateField, FilterCriteria, GroupBy, Join, SortOption};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::providers::queries::find::constructors::selections_constructor::SelectionsConstructor;

    /// Test implementation of QueryFilter for testing
    #[derive(Debug, Clone, Serialize, Deserialize)]
    struct TestQueryFilter {
        pluck: Vec<String>,
        pluck_object: HashMap<String, Vec<String>>,
        concatenate_fields: Vec<ConcatenateField>,
        advance_filters: Vec<FilterCriteria>,
        joins: Vec<Join>,
        limit: usize,
        offset: usize,
        order_by: String,
        order_direction: String,
        date_format: String,
        time_format: String,
        group_by: Option<GroupBy>,
        distinct_by: Option<String>,
        is_case_sensitive_sorting: Option<bool>,
        multiple_sort: Vec<SortOption>,
        group_advance_filters: Vec<crate::structs::core::GroupAdvanceFilter>,
        pluck_group_object: HashMap<String, Vec<String>>,
    }

    impl QueryFilter for TestQueryFilter {
        fn get_advance_filters(&self) -> &[FilterCriteria] {
            &self.advance_filters
        }

        fn get_joins(&self) -> &[Join] {
            &self.joins
        }

        fn get_limit(&self) -> usize {
            self.limit
        }

        fn get_offset(&self) -> usize {
            self.offset
        }

        fn get_order_by(&self) -> &str {
            &self.order_by
        }

        fn get_order_direction(&self) -> &str {
            &self.order_direction
        }

        fn get_date_format(&self) -> &str {
            &self.date_format
        }

        fn get_time_format(&self) -> &str {
            &self.time_format
        }

        fn get_pluck(&self) -> &[String] {
            &self.pluck
        }

        fn get_pluck_object(&self) -> &HashMap<String, Vec<String>> {
            &self.pluck_object
        }

        fn get_concatenate_fields(&self) -> &[ConcatenateField] {
            &self.concatenate_fields
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

        fn get_multiple_sort(&self) -> &[SortOption] {
            &self.multiple_sort
        }

        fn get_group_advance_filters(&self) -> &[crate::structs::core::GroupAdvanceFilter] {
            &self.group_advance_filters
        }

        fn get_pluck_group_object(&self) -> &HashMap<String, Vec<String>> {
            &self.pluck_group_object
        }
    }

    /// Helper function to create test concatenate field
    fn create_concatenate_field(
        fields: Vec<&str>,
        field_name: &str,
        separator: &str,
        entity: &str,
        aliased_entity: Option<&str>,
    ) -> ConcatenateField {
        ConcatenateField {
            fields: fields.iter().map(|s| s.to_string()).collect(),
            field_name: field_name.to_string(),
            separator: separator.to_string(),
            entity: entity.to_string(),
            aliased_entity: aliased_entity.map(|s| s.to_string()),
        }
    }

    /// Helper function to create test query filter
    fn create_test_filter(
        pluck: Vec<&str>,
        pluck_object: HashMap<String, Vec<&str>>,
        concatenate_fields: Vec<ConcatenateField>,
    ) -> TestQueryFilter {
        let mut pluck_object_converted = HashMap::new();
        for (key, values) in pluck_object {
            pluck_object_converted.insert(
                key.to_string(),
                values.iter().map(|s| s.to_string()).collect(),
            );
        }

        TestQueryFilter {
            pluck: pluck.iter().map(|s| s.to_string()).collect(),
            pluck_object: pluck_object_converted,
            concatenate_fields,
            advance_filters: vec![],
            joins: vec![],
            limit: 100,
            offset: 0,
            order_by: "id".to_string(),
            order_direction: "asc".to_string(),
            date_format: "mm/dd/YYYY".to_string(),
            time_format: "HH24:MI".to_string(),
            group_by: None,
            distinct_by: None,
            is_case_sensitive_sorting: None,
            multiple_sort: vec![],
            group_advance_filters: vec![],
            pluck_group_object: HashMap::new(),
        }
    }

    /// Mock field getter functions for testing
    fn mock_get_field(
        table: &str,
        field: &str,
        _date_format: &str,
        _alias_table: &str,
        _timezone: Option<&str>,
        _with_alias: bool,
    ) -> String {
        if _with_alias {
            format!("{}.{}", table, field)
        } else {
            format!("{}.{}", table, field)
        }
    }

    fn mock_get_field_with_parse_as(
        table: &str,
        field: &str,
        _parse_as: &str,
        _date_format: Option<&str>,
        _alias_table: &str,
        _timezone: Option<&str>,
        _with_alias: bool,
    ) -> String {
        format!("{}.{}", table, field)
    }

    fn mock_normalize_entity_name(entity: &str) -> String {
        entity.to_string()
    }

    fn mock_build_system_where_clause(_table: &str) -> Result<String, String> {
        Ok("".to_string())
    }

    fn mock_build_infix_expression(_filters: &[FilterCriteria]) -> Result<String, String> {
        Ok("".to_string())
    }

    #[test]
    fn test_construct_selections_with_concatenated_fields_override() {
        // Create concatenate fields with aliased entities
        let concatenate_fields = vec![
            create_concatenate_field(
                vec!["first_name", "last_name"],
                "full_name",
                " ",
                "contacts",
                Some("created_by"),
            ),
            create_concatenate_field(
                vec!["first_name", "last_name"],
                "full_name",
                " ",
                "contacts",
                Some("updated_by"),
            ),
        ];

        // Create pluck object that includes the aliased entities
        let mut pluck_object = HashMap::new();
        pluck_object.insert(
            "created_by".to_string(),
            vec!["id", "first_name", "last_name", "full_name"],
        );
        pluck_object.insert(
            "updated_by".to_string(),
            vec!["id", "first_name", "last_name", "full_name"],
        );
        pluck_object.insert(
            "organizations".to_string(),
            vec!["id", "name", "created_by"],
        );

        let filter = create_test_filter(
            vec!["id"], // non-empty pluck to trigger pluck_object path
            pluck_object,
            concatenate_fields,
        );

        let result = SelectionsConstructor::construct_selections(
            &filter,
            "created_by",
            None,
            mock_normalize_entity_name,
            mock_get_field,
            mock_get_field_with_parse_as,
            mock_build_system_where_clause,
            mock_build_infix_expression,
        );

        println!("Result: {}", result);

        // The result should contain the concatenated full_name field
        assert!(
            result.contains("full_name"),
            "Should contain full_name concatenated field"
        );

        // The result should contain the concatenated field expression with COALESCE and proper field references
        assert!(
            result.contains("COALESCE(created_by.first_name, '')"),
            "Should contain COALESCE expression for first_name in concatenated field"
        );
        assert!(
            result.contains("COALESCE(created_by.last_name, '')"),
            "Should contain COALESCE expression for last_name in concatenated field"
        );
    }

    #[test]
    fn test_construct_selections_with_concatenated_fields_override_different_entity() {
        // Create concatenate fields with different entities
        let concatenate_fields = vec![
            create_concatenate_field(
                vec!["first_name", "last_name"],
                "full_name",
                " ",
                "contacts",
                Some("created_by"),
            ),
            create_concatenate_field(
                vec!["name", "description"],
                "org_info",
                " - ",
                "organizations",
                None,
            ),
        ];

        // Create pluck object that includes both entities
        let mut pluck_object = HashMap::new();
        pluck_object.insert(
            "created_by".to_string(),
            vec!["id", "first_name", "last_name", "full_name"],
        );
        pluck_object.insert(
            "organizations".to_string(),
            vec!["id", "name", "description"],
        );

        let filter = create_test_filter(
            vec!["id"], // non-empty pluck to trigger pluck_object path
            pluck_object,
            concatenate_fields,
        );

        let result = SelectionsConstructor::construct_selections(
            &filter,
            "created_by",
            None,
            mock_normalize_entity_name,
            mock_get_field,
            mock_get_field_with_parse_as,
            mock_build_system_where_clause,
            mock_build_infix_expression,
        );

        println!("Result: {}", result);

        // The result should contain the concatenated fields
        assert!(
            result.contains("full_name"),
            "Should contain full_name concatenated field"
        );

        // The result should contain the concatenated field expressions with COALESCE and proper field references
        assert!(
            result.contains("COALESCE(created_by.first_name, '')"),
            "Should contain COALESCE expression for first_name in concatenated field"
        );
        assert!(
            result.contains("COALESCE(created_by.last_name, '')"),
            "Should contain COALESCE expression for last_name in concatenated field"
        );
        // Note: The current implementation only processes concatenated fields for the main table entity
        // The organizations concatenated field (org_info) is not included in the result
    }

    #[test]
    fn test_construct_selections_with_concatenated_fields_override_mixed_scenario() {
        // Create a mix of concatenated and regular fields
        let concatenate_fields = vec![create_concatenate_field(
            vec!["first_name", "last_name"],
            "full_name",
            " ",
            "contacts",
            Some("created_by"),
        )];

        // Create pluck object with both concatenated and regular fields
        let mut pluck_object = HashMap::new();
        pluck_object.insert(
            "created_by".to_string(),
            vec!["id", "first_name", "last_name", "email", "full_name"],
        );
        pluck_object.insert("organizations".to_string(), vec!["id", "name"]);

        let filter = create_test_filter(
            vec!["id"], // non-empty pluck to trigger pluck_object path
            pluck_object,
            concatenate_fields,
        );

        let result = SelectionsConstructor::construct_selections(
            &filter,
            "created_by",
            None,
            mock_normalize_entity_name,
            mock_get_field,
            mock_get_field_with_parse_as,
            mock_build_system_where_clause,
            mock_build_infix_expression,
        );

        println!("Result: {}", result);

        // The result should contain the concatenated field
        assert!(
            result.contains("full_name"),
            "Should contain full_name concatenated field"
        );

        // The result should contain the concatenated field expressions with COALESCE and proper field references
        assert!(
            result.contains("COALESCE(created_by.first_name, '')"),
            "Should contain COALESCE expression for first_name in concatenated field"
        );
        assert!(
            result.contains("COALESCE(created_by.last_name, '')"),
            "Should contain COALESCE expression for last_name in concatenated field"
        );

        // Note: The current implementation only returns concatenated fields when using pluck_object
        // Regular fields from pluck_object are not included in the result
    }

    #[test]
    fn test_construct_selections_with_concatenated_fields_override_no_aliased_entity() {
        // Create concatenate fields without aliased entities (main table)
        let concatenate_fields = vec![create_concatenate_field(
            vec!["first_name", "last_name"],
            "full_name",
            " ",
            "contacts",
            None,
        )];

        // Create pluck object for main table
        let mut pluck_object = HashMap::new();
        pluck_object.insert(
            "contacts".to_string(),
            vec!["id", "first_name", "last_name", "email", "full_name"],
        );

        let filter = create_test_filter(
            vec!["id"], // non-empty pluck to trigger pluck_object path
            pluck_object,
            concatenate_fields,
        );

        let result = SelectionsConstructor::construct_selections(
            &filter,
            "contacts",
            None,
            mock_normalize_entity_name,
            mock_get_field,
            mock_get_field_with_parse_as,
            mock_build_system_where_clause,
            mock_build_infix_expression,
        );

        println!("Result: {}", result);

        // The result should contain the concatenated field
        assert!(
            result.contains("full_name"),
            "Should contain full_name concatenated field"
        );

        // The result should contain the concatenated field expressions with COALESCE and proper field references
        assert!(
            result.contains("COALESCE(contacts.first_name, '')"),
            "Should contain COALESCE expression for first_name in concatenated field"
        );
        assert!(
            result.contains("COALESCE(contacts.last_name, '')"),
            "Should contain COALESCE expression for last_name in concatenated field"
        );

        // Note: The current implementation only returns concatenated fields when using pluck_object
        // Regular fields from pluck_object are not included in the result
    }

    #[test]
    fn test_construct_selections_with_concatenated_fields_auto_injection() {
        // Test that concatenated fields are automatically injected even when not in pluck_object
        // This simulates the classrooms_filter.json scenario where full_name is defined in
        // concatenate_fields but not explicitly listed in pluck_object for created_by/updated_by
        
        let concatenate_fields = vec![
            create_concatenate_field(
                vec!["first_name", "last_name"],
                "full_name",
                " ",
                "contacts",
                Some("created_by"),
            ),
            create_concatenate_field(
                vec!["first_name", "last_name"],
                "full_name",
                " ",
                "contacts",
                Some("updated_by"),
            ),
        ];

        // Create pluck object that does NOT include full_name for created_by and updated_by
        // This simulates the classrooms_filter.json configuration
        let mut pluck_object = HashMap::new();
        pluck_object.insert(
            "created_by".to_string(),
            vec!["id", "first_name", "last_name"], // Note: full_name is NOT included
        );
        pluck_object.insert(
            "updated_by".to_string(),
            vec!["id", "first_name", "last_name"], // Note: full_name is NOT included
        );

        let filter = create_test_filter(
            vec!["id"], // non-empty pluck to trigger pluck_object path
            pluck_object,
            concatenate_fields,
        );

        // Test created_by - should get full_name even though it's not in pluck_object
        let created_by_result = SelectionsConstructor::construct_selections(
            &filter,
            "created_by",
            None,
            mock_normalize_entity_name,
            mock_get_field,
            mock_get_field_with_parse_as,
            mock_build_system_where_clause,
            mock_build_infix_expression,
        );

        println!("Created by result: {}", created_by_result);

        // The result should contain the concatenated full_name field even though it's not in pluck_object
        assert!(
            created_by_result.contains("full_name"),
            "Should contain full_name concatenated field even when not in pluck_object"
        );

        // The result should contain the concatenated field expressions with COALESCE and proper field references
        assert!(
            created_by_result.contains("COALESCE(created_by.first_name, '')"),
            "Should contain COALESCE expression for first_name in concatenated field"
        );
        assert!(
            created_by_result.contains("COALESCE(created_by.last_name, '')"),
            "Should contain COALESCE expression for last_name in concatenated field"
        );

        // Test updated_by - should get full_name even though it's not in pluck_object
        let updated_by_result = SelectionsConstructor::construct_selections(
            &filter,
            "updated_by",
            None,
            mock_normalize_entity_name,
            mock_get_field,
            mock_get_field_with_parse_as,
            mock_build_system_where_clause,
            mock_build_infix_expression,
        );

        println!("Updated by result: {}", updated_by_result);

        // The result should contain the concatenated full_name field even though it's not in pluck_object
        assert!(
            updated_by_result.contains("full_name"),
            "Should contain full_name concatenated field even when not in pluck_object"
        );

        // Test contacts - should NOT get full_name because it's not an aliased_entity
        let contacts_result = SelectionsConstructor::construct_selections(
            &filter,
            "contacts",
            None,
            mock_normalize_entity_name,
            mock_get_field,
            mock_get_field_with_parse_as,
            mock_build_system_where_clause,
            mock_build_infix_expression,
        );

        println!("Contacts result: {}", contacts_result);

        // The result should NOT contain full_name because contacts is not an aliased_entity
        assert!(
            !contacts_result.contains("full_name"),
            "Should NOT contain full_name for contacts entity (not an aliased_entity)"
        );
    }
}
