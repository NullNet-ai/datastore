#[cfg(test)]
mod tests {
    use crate::{
        controllers::payload_filter_scenarios::PayloadFilterScenarios,
        structs::core::{FilterCriteria, FilterOperator, GetByFilter},
    };
    use serde_json::json;
    use std::path::Path;

    /// Creates filter scenario 1: Get records with id, first_name, last_name from contacts table
    fn create_contacts_basic_fields_filter() -> GetByFilter {
        GetByFilter {
            pluck: vec![
                "id".to_string(),
                "first_name".to_string(),
                "last_name".to_string(),
            ],
            pluck_object: Default::default(),
            pluck_group_object: Default::default(),
            advance_filters: vec![],
            group_advance_filters: vec![],
            joins: vec![],
            group_by: None,
            concatenate_fields: vec![],
            multiple_sort: vec![],
            date_format: "YYYY-MM-DD HH24:MI:SS".to_string(),
            order_by: "id".to_string(),
            order_direction: "ASC".to_string(),
            is_case_sensitive_sorting: None,
            limit: 25,
            offset: 0,
            distinct_by: None,
            timezone: None,
        }
    }

    /// Creates filter scenario 2: Get records with id, status, first_name, last_name from contacts table with Active status filter
    fn create_contacts_active_status_filter() -> GetByFilter {
        GetByFilter {
            pluck: vec![
                "id".to_string(),
                "status".to_string(),
                "first_name".to_string(),
                "last_name".to_string(),
            ],
            pluck_object: Default::default(),
            pluck_group_object: Default::default(),
            advance_filters: vec![FilterCriteria::Criteria {
                field: "status".to_string(),
                entity: None,
                operator: FilterOperator::Equal,
                values: vec![json!("Active")],
                case_sensitive: None,
                parse_as: "text".to_string(),
                match_pattern: None,
                is_search: None,
                has_group_count: None,
            }],
            group_advance_filters: vec![],
            joins: vec![],
            group_by: None,
            concatenate_fields: vec![],
            multiple_sort: vec![],
            date_format: "YYYY-MM-DD HH24:MI:SS".to_string(),
            order_by: "id".to_string(),
            order_direction: "ASC".to_string(),
            is_case_sensitive_sorting: None,
            limit: 25,
            offset: 0,
            distinct_by: None,
            timezone: None,
        }
    }

    /// Test creating filter scenario 1: Basic contacts fields
    #[tokio::test]
    async fn should_create_contacts_basic_fields_scenario() {
        println!("Testing creation of contacts basic fields filter scenario...");

        let default_filter = GetByFilter {
            pluck: vec![],
            pluck_object: Default::default(),
            pluck_group_object: Default::default(),
            advance_filters: vec![],
            group_advance_filters: vec![],
            joins: vec![],
            group_by: None,
            concatenate_fields: vec![],
            multiple_sort: vec![],
            date_format: "YYYY-MM-DD HH24:MI:SS".to_string(),
            order_by: "id".to_string(),
            order_direction: "ASC".to_string(),
            is_case_sensitive_sorting: None,
            limit: 10,
            offset: 0,
            distinct_by: None,
            timezone: None,
        };

        let mut payload_filters =
            PayloadFilterScenarios::new("contacts".to_string(), default_filter);

        let scenario_filter = create_contacts_basic_fields_filter();
        let result = payload_filters.create_scenario_with_description(
            "contacts_basic_fields".to_string(),
            "Retrieve records with id, first_name, and last_name fields from the contacts table"
                .to_string(),
            scenario_filter.clone(),
        );

        println!("  ✓ Creating scenario with basic fields (id, first_name, last_name)");
        assert!(
            result.is_ok(),
            "Failed to create contacts_basic_fields scenario"
        );
        assert!(payload_filters.has_scenario("contacts_basic_fields"));

        // Verify the scenario was created correctly
        let created_scenario = payload_filters
            .get_scenario("contacts_basic_fields")
            .unwrap();
        assert_eq!(created_scenario.name, "contacts_basic_fields");
        assert_eq!(created_scenario.filter.pluck, scenario_filter.pluck);
        assert_eq!(created_scenario.filter.limit, 25);
        assert!(created_scenario.filter.advance_filters.is_empty());

        // Verify file was created
        let file_path = Path::new("scenarios/filters/contacts_basic_fields.json");
        assert!(file_path.exists(), "Scenario file should be created");

        println!("  ✓ Scenario file created at: {:?}", file_path);
        println!("  ✓ contacts_basic_fields scenario created successfully");

        // Keep the scenario file for persistent use
        println!("  ✓ Scenario file persisted at: {:?}", file_path);
        assert!(
            true,
            "Test completed - contacts basic fields scenario creation"
        );
    }

    /// Test creating filter scenario 2: Active contacts with status filter
    #[tokio::test]
    async fn should_create_contacts_active_status_scenario() {
        println!("Testing creation of contacts active status filter scenario...");

        let default_filter = GetByFilter {
            pluck: vec![],
            pluck_object: Default::default(),
            pluck_group_object: Default::default(),
            advance_filters: vec![],
            group_advance_filters: vec![],
            joins: vec![],
            group_by: None,
            concatenate_fields: vec![],
            multiple_sort: vec![],
            date_format: "YYYY-MM-DD HH24:MI:SS".to_string(),
            order_by: "id".to_string(),
            order_direction: "ASC".to_string(),
            is_case_sensitive_sorting: None,
            limit: 10,
            offset: 0,
            distinct_by: None,
            timezone: None,
        };

        let mut payload_filters =
            PayloadFilterScenarios::new("contacts".to_string(), default_filter);

        let scenario_filter = create_contacts_active_status_filter();
        let result = payload_filters.create_scenario_with_description(
            "contacts_active_status".to_string(),
            "Retrieve records with id, status, first_name, and last_name fields from the contacts table, filtered for Active status".to_string(),
            scenario_filter.clone(),
        );

        println!("  ✓ Creating scenario with status filter (Active contacts only)");
        assert!(
            result.is_ok(),
            "Failed to create contacts_active_status scenario"
        );
        assert!(payload_filters.has_scenario("contacts_active_status"));

        // Verify the scenario was created correctly
        let created_scenario = payload_filters
            .get_scenario("contacts_active_status")
            .unwrap();
        assert_eq!(created_scenario.name, "contacts_active_status");
        assert_eq!(created_scenario.filter.pluck, scenario_filter.pluck);
        assert_eq!(created_scenario.filter.limit, 25);

        // Verify advance_filters contains the Active status filter
        assert!(!created_scenario.filter.advance_filters.is_empty());
        let filters = &created_scenario.filter.advance_filters;
        assert_eq!(filters.len(), 1);

        match &filters[0] {
            FilterCriteria::Criteria {
                field,
                operator,
                values,
                ..
            } => {
                assert_eq!(field, "status");
                assert!(matches!(operator, FilterOperator::Equal));
                assert_eq!(values.len(), 1);
                assert_eq!(values[0], json!("Active"));
            }
            _ => panic!("Expected Criteria filter, got LogicalOperator"),
        }

        // Verify file was created
        let file_path = Path::new("scenarios/filters/contacts_active_status.json");
        assert!(file_path.exists(), "Scenario file should be created");

        println!("  ✓ Scenario file created at: {:?}", file_path);
        println!("  ✓ contacts_active_status scenario created successfully");

        // Keep the scenario file for persistent use
        println!("  ✓ Scenario file persisted at: {:?}", file_path);
        assert!(
            true,
            "Test completed - contacts active status scenario creation"
        );
    }

    /// Test loading and using both created scenarios
    #[tokio::test]
    async fn should_load_and_use_created_scenarios() {
        println!("Testing loading and using both created filter scenarios...");

        let default_filter = GetByFilter {
            pluck: vec![],
            pluck_object: Default::default(),
            pluck_group_object: Default::default(),
            advance_filters: vec![],
            group_advance_filters: vec![],
            joins: vec![],
            group_by: None,
            concatenate_fields: vec![],
            multiple_sort: vec![],
            date_format: "YYYY-MM-DD HH24:MI:SS".to_string(),
            order_by: "id".to_string(),
            order_direction: "ASC".to_string(),
            is_case_sensitive_sorting: None,
            limit: 10,
            offset: 0,
            distinct_by: None,
            timezone: None,
        };

        let mut payload_filters =
            PayloadFilterScenarios::new("contacts".to_string(), default_filter);

        // Create both scenarios
        let basic_filter = create_contacts_basic_fields_filter();
        let active_filter = create_contacts_active_status_filter();

        payload_filters
            .create_scenario_with_description(
                "contacts_basic_fields".to_string(),
                "Basic contacts fields scenario".to_string(),
                basic_filter,
            )
            .unwrap();

        payload_filters
            .create_scenario_with_description(
                "contacts_active_status".to_string(),
                "Active contacts scenario".to_string(),
                active_filter,
            )
            .unwrap();

        println!("  ✓ Both scenarios created successfully");
        assert_eq!(payload_filters.scenario_count(), 2);

        // Test setting current scenario to basic fields
        payload_filters
            .set_current_scenario("contacts_basic_fields")
            .unwrap();
        let current_filter = payload_filters.get_current_filter();
        assert_eq!(current_filter.pluck.len(), 3);
        assert!(current_filter.advance_filters.is_empty());
        println!("  ✓ Basic fields scenario set as current and verified");

        // Test setting current scenario to active status
        payload_filters
            .set_current_scenario("contacts_active_status")
            .unwrap();
        let current_filter = payload_filters.get_current_filter();
        assert_eq!(current_filter.pluck.len(), 4);
        assert!(!current_filter.advance_filters.is_empty());
        assert_eq!(current_filter.advance_filters.len(), 1);
        println!("  ✓ Active status scenario set as current and verified");

        // Keep the scenario files for persistent use
        println!("  ✓ Both scenario files persisted for future use");
        println!("  ✓ Both scenarios loaded and used successfully");
        assert!(true, "Test completed - scenario loading and usage");
    }
}
