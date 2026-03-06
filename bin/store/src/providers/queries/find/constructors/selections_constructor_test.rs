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

        fn get_pluck(&self) -> &[String] {
            &self.pluck
        }

        fn get_pluck_object(&self) -> &HashMap<String, Vec<String>> {
            &self.pluck_object
        }

        fn get_concatenate_fields(&self) -> &[ConcatenateField] {
            &self.concatenate_fields
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

    #[test]
    fn test_self_join_district_orgs_issue() {
        let mut pluck_object = HashMap::new();
        pluck_object.insert("district_orgs".to_string(), vec![
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
        ]);
        pluck_object.insert("district_superintendent".to_string(), vec![
            "first_name".to_string(),
            "code".to_string(),
            "last_name".to_string(),
            "username".to_string(),
        ]);

        // Create the self join that creates "district_orgs" alias
        let self_join = Join {
            r#type: "self".to_string(),
            field_relation: crate::structs::core::FieldRelation {
                to: crate::structs::core::RelationEndpoint {
                    alias: None, // No alias for "to" - it's the same entity
                    entity: "organizations".to_string(),
                    field: "id".to_string(),
                    order_direction: None,
                    order_by: None,
                    limit: None,
                    offset: None,
                    filters: vec![],
                },
                from: crate::structs::core::RelationEndpoint {
                    alias: Some("district_orgs".to_string()), // This creates the alias
                    entity: "organizations".to_string(),
                    field: "district_id".to_string(),
                    order_direction: None,
                    order_by: None,
                    limit: None,
                    offset: None,
                    filters: vec![],
                },
            },
            nested: false,
        };

        // Create the nested join that references "district_orgs" and creates "district_superintendent" alias
        let nested_join = Join {
            r#type: "left".to_string(),
            field_relation: crate::structs::core::FieldRelation {
                to: crate::structs::core::RelationEndpoint {
                    alias: Some("district_superintendent".to_string()),
                    entity: "contacts".to_string(),
                    field: "id".to_string(),
                    order_direction: None,
                    order_by: None,
                    limit: None,
                    offset: None,
                    filters: vec![],
                },
                from: crate::structs::core::RelationEndpoint {
                    alias: None,
                    entity: "district_orgs".to_string(), // This references the alias from self_join
                    field: "superintendent_id".to_string(),
                    order_direction: None,
                    order_by: None,
                    limit: None,
                    offset: None,
                    filters: vec![],
                },
            },
            nested: true,
        };

        let mock_filter = TestQueryFilter {
            pluck: vec!["id".to_string(), "name".to_string()],
            pluck_object,
            concatenate_fields: vec![],
            advance_filters: vec![],
            joins: vec![self_join, nested_join],
            limit: 100,
            offset: 0,
            order_by: "name".to_string(),
            order_direction: "asc".to_string(),
            date_format: "%Y-%m-%d".to_string(),
            time_format: "%H:%M:%S".to_string(),
            group_by: None,
            distinct_by: None,
            is_case_sensitive_sorting: None,
            multiple_sort: vec![],
            group_advance_filters: vec![],
            pluck_group_object: HashMap::new(),
        };

        // Mock functions
        let normalize_entity_name = |entity: &str| entity.to_string();
        
        let get_field = |table: &str, field: &str, _date_format: &str, _main_table: &str, _timezone: Option<&str>, _with_alias: bool| {
            format!("\"{}\".\"{}\"", table, field)
        };
        
        let get_field_with_parse_as = |table: &str, field: &str, _date_format: &str, _parse_as: Option<&str>, _main_table: &str, _timezone: Option<&str>, _with_alias: bool| {
            format!("\"{}\".\"{}\"", table, field)
        };
        
        let build_system_where_clause = |table: &str| {
            Ok(format!("\"{}\".\"tombstone\" = 0", table))
        };
        
        let build_infix_expression = |_filters: &[FilterCriteria]| {
            Ok("".to_string())
        };

        let result = SelectionsConstructor::construct_selections(
            &mock_filter,
            "organizations",
            None,
            normalize_entity_name,
            get_field,
            get_field_with_parse_as,
            build_system_where_clause,
            build_infix_expression,
        );

        println!("Generated selections for self join scenario: {}", result);
        
        // The result should contain selections for both district_orgs and district_superintendent
        assert!(result.contains("district_orgs"), "Should contain district_orgs selection, but got: {}", result);
        assert!(result.contains("district_superintendent"), "Should contain district_superintendent selection, but got: {}", result);
    }

    #[test]
    fn test_classroom2_alias_issue() {
        let mut pluck_object = HashMap::new();
        pluck_object.insert("classroom2".to_string(), vec![
            "name".to_string(),
            "code".to_string(),
            "id".to_string(),
            "school_id".to_string(),
        ]);
        pluck_object.insert("school".to_string(), vec![
            "id".to_string(),
            "name".to_string(),
        ]);

        // Create the first join (non-nested) that creates "classroom2" alias
        let join1 = Join {
            r#type: "left".to_string(),
            field_relation: crate::structs::core::FieldRelation {
                to: crate::structs::core::RelationEndpoint {
                    alias: Some("classroom2".to_string()),
                    entity: "classrooms".to_string(),
                    field: "id".to_string(),
                    order_direction: None,
                    order_by: None,
                    limit: None,
                    offset: None,
                    filters: vec![],
                },
                from: crate::structs::core::RelationEndpoint {
                    alias: None,
                    entity: "classroom_course_stories_episodes".to_string(),
                    field: "classroom_id".to_string(),
                    order_direction: None,
                    order_by: None,
                    limit: None,
                    offset: None,
                    filters: vec![],
                },
            },
            nested: false,
        };

        // Create the second join (nested) that references "classroom2" and creates "school" alias
        let join2 = Join {
            r#type: "left".to_string(),
            field_relation: crate::structs::core::FieldRelation {
                to: crate::structs::core::RelationEndpoint {
                    alias: Some("school".to_string()),
                    entity: "organizations".to_string(),
                    field: "id".to_string(),
                    order_direction: None,
                    order_by: None,
                    limit: None,
                    offset: None,
                    filters: vec![],
                },
                from: crate::structs::core::RelationEndpoint {
                    alias: None,
                    entity: "classroom2".to_string(), // This references the alias from join1
                    field: "school_id".to_string(),
                    order_direction: None,
                    order_by: None,
                    limit: None,
                    offset: None,
                    filters: vec![],
                },
            },
            nested: true,
        };

        let mock_filter = TestQueryFilter {
            pluck: vec!["id".to_string()],
            pluck_object,
            concatenate_fields: vec![],
            advance_filters: vec![],
            joins: vec![join1, join2],
            limit: 100,
            offset: 0,
            order_by: "id".to_string(),
            order_direction: "asc".to_string(),
            date_format: "%Y-%m-%d".to_string(),
            time_format: "%H:%M:%S".to_string(),
            group_by: None,
            distinct_by: None,
            is_case_sensitive_sorting: None,
            multiple_sort: vec![],
            group_advance_filters: vec![],
            pluck_group_object: HashMap::new(),
        };

        // Mock functions
        let normalize_entity_name = |entity: &str| entity.to_string();
        
        let get_field = |table: &str, field: &str, _date_format: &str, _main_table: &str, _timezone: Option<&str>, _with_alias: bool| {
            format!("\"{}\".\"{}\"", table, field)
        };
        
        let get_field_with_parse_as = |table: &str, field: &str, _date_format: &str, _parse_as: Option<&str>, _main_table: &str, _timezone: Option<&str>, _with_alias: bool| {
            format!("\"{}\".\"{}\"", table, field)
        };
        
        let build_system_where_clause = |table: &str| {
            Ok(format!("\"{}\".\"tombstone\" = 0", table))
        };
        
        let build_infix_expression = |_filters: &[FilterCriteria]| {
            Ok("".to_string())
        };

        let result = SelectionsConstructor::construct_selections(
            &mock_filter,
            "classroom_course_stories_episodes",
            None,
            normalize_entity_name,
            get_field,
            get_field_with_parse_as,
            build_system_where_clause,
            build_infix_expression,
        );

        println!("Generated selections: {}", result);
        
        // The result should contain selections for both classroom2 and school
        assert!(result.contains("classroom2"), "Should contain classroom2 selection, but got: {}", result);
        assert!(result.contains("school"), "Should contain school selection, but got: {}", result);
    }
}