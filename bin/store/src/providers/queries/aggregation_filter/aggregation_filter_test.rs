#[cfg(test)]
mod tests {
    use crate::providers::queries::aggregation_filter::sql_constructor::{
        AggregationFilterWrapper, AggregationQueryFilter, AggregationSQLConstructor,
    };
    use crate::providers::queries::find::sql_constructor::QueryFilter;
    use crate::structs::structs::{Aggregation, AggregationFilter, AggregationType};

    // Helper function to create a test AggregationFilter
    fn create_test_aggregation_filter() -> AggregationFilter {
        AggregationFilter {
            entity: "transactions".to_string(),
            aggregations: vec![Aggregation {
                aggregation: AggregationType::Sum,
                aggregate_on: "amount".to_string(),
                bucket_name: "total_amount".to_string(),
            }],
            advance_filters: vec![],
            joins: vec![],
            limit: 100,
            bucket_size: Some("1 hour".to_string()),
            timezone: Some("UTC".to_string()),
            order: None,
        }
    }

    // Helper function to create a test AggregationFilterWrapper
    fn create_test_aggregation_filter_wrapper() -> AggregationFilterWrapper {
        let request = crate::generated::store::AggregationFilterRequest {
            params: Some(crate::generated::store::AggregationFilterParams {
                r#type: "aggregation".to_string(),
            }),
            body: Some(
                crate::generated::store::aggregation_filter_request::AggregationFilterBody {
                    entity: "transactions".to_string(),
                    aggregations: vec![crate::generated::store::Aggregation {
                        aggregation: 0, // Sum
                        aggregate_on: "amount".to_string(),
                        bucket_name: "total_amount".to_string(),
                    }],
                    advance_filters: vec![],
                    joins: vec![],
                    limit: Some(100),
                    bucket_size: Some("1 hour".to_string()),
                    timezone: Some("UTC".to_string()),
                    order: None,
                },
            ),
        };
        AggregationFilterWrapper::new(request)
    }

    #[test]
    fn test_aggregation_query_filter_defaults() {
        let filter = create_test_aggregation_filter();

        // Test default implementations
        assert!(QueryFilter::get_advance_filters(&filter).is_empty());
        assert!(QueryFilter::get_joins(&filter).is_empty());
        assert_eq!(QueryFilter::get_limit(&filter), 100);
        assert_eq!(AggregationQueryFilter::get_aggregations(&filter).len(), 1);
        assert_eq!(
            AggregationQueryFilter::get_bucket_size(&filter),
            Some("1 hour")
        );
        assert_eq!(AggregationQueryFilter::get_timezone(&filter), Some("UTC"));
        assert!(AggregationQueryFilter::get_aggregation_order(&filter).is_none());
        assert_eq!(
            AggregationQueryFilter::get_entity(&filter),
            Some("transactions")
        );
    }

    #[test]
    fn test_query_filter_implementation() {
        let filter = create_test_aggregation_filter();

        // Test QueryFilter trait implementation
        assert!(QueryFilter::get_advance_filters(&filter).is_empty());
        assert!(QueryFilter::get_joins(&filter).is_empty());
        assert_eq!(QueryFilter::get_limit(&filter), 100);
    }

    #[test]
    fn test_aggregation_sql_constructor_new() {
        let filter = create_test_aggregation_filter();
        // TODO: Need to check again
        #[allow(unused)]
        let constructor = AggregationSQLConstructor::new(
            filter,
            "test_table".to_string(),
            false,
            Some("UTC".to_string()),
        );

        // Test that constructor is created successfully
        // We can't directly test internal state, but we can test methods
        assert!(true); // Constructor creation succeeded
    }

    #[test]
    fn test_aggregation_sql_constructor_with_organization_id() {
        let filter = create_test_aggregation_filter();
        let constructor = AggregationSQLConstructor::new(
            filter,
            "test_table".to_string(),
            false,
            Some("UTC".to_string()),
        );

        let _result = constructor.with_organization_id("org123".to_string());
        // Test passes if no panic occurs
    }

    #[test]
    fn test_construct_aggregation_success() {
        let filter = create_test_aggregation_filter();
        let mut constructor = AggregationSQLConstructor::new(
            filter,
            "transactions".to_string(),
            false,
            Some("UTC".to_string()),
        )
        .with_organization_id("test-org-123".to_string());

        let result = constructor.construct_aggregation();
        if let Err(e) = &result {
            println!("Error in test_construct_aggregation_success: {}", e);
        }
        assert!(result.is_ok());

        let sql = result.unwrap();
        assert!(sql.contains("SELECT"));
        assert!(sql.contains("time_bucket"));
        assert!(sql.contains("SUM"));
        assert!(sql.contains("transactions"));
    }

    #[test]
    fn test_construct_aggregation_missing_aggregations() {
        let mut filter = create_test_aggregation_filter();
        filter.aggregations = vec![]; // Remove aggregations

        let mut constructor = AggregationSQLConstructor::new(
            filter,
            "transactions".to_string(),
            false,
            Some("UTC".to_string()),
        );

        let result = constructor.construct_aggregation();
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .contains("Missing required parameter: aggregations cannot be empty"));
    }

    #[test]
    fn test_construct_aggregation_missing_bucket_size() {
        let mut filter = create_test_aggregation_filter();
        filter.bucket_size = None; // Remove bucket size

        let mut constructor = AggregationSQLConstructor::new(
            filter,
            "transactions".to_string(),
            false,
            Some("UTC".to_string()),
        );

        let result = constructor.construct_aggregation();
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .contains("Missing required parameter: bucket_size"));
    }

    #[test]
    fn test_construct_aggregation_missing_entity() {
        let mut filter = create_test_aggregation_filter();
        filter.entity = "".to_string(); // Empty entity

        let mut constructor = AggregationSQLConstructor::new(
            filter,
            "transactions".to_string(),
            false,
            Some("UTC".to_string()),
        );

        let result = constructor.construct_aggregation();
        assert!(result.is_err());
        let error_msg = result.unwrap_err();
        println!(
            "Error in test_construct_aggregation_missing_entity: {}",
            error_msg
        );
        assert!(error_msg.contains("Missing required parameter: entity"));
    }

    #[test]
    fn test_aggregation_filter_wrapper_new() {
        let wrapper = create_test_aggregation_filter_wrapper();

        // Test that wrapper is created successfully
        assert_eq!(wrapper.converted_aggregations.len(), 1);
        assert_eq!(wrapper.converted_filters.len(), 0);
        assert_eq!(wrapper.converted_joins.len(), 0);
    }

    #[test]
    fn test_aggregation_filter_wrapper_query_filter() {
        let wrapper = create_test_aggregation_filter_wrapper();

        // Test QueryFilter implementation
        assert!(QueryFilter::get_advance_filters(&wrapper).is_empty());
        assert!(QueryFilter::get_joins(&wrapper).is_empty());
        assert_eq!(QueryFilter::get_limit(&wrapper), 100);
    }

    #[test]
    fn test_aggregation_filter_wrapper_aggregation_query_filter() {
        let wrapper = create_test_aggregation_filter_wrapper();

        // Test AggregationQueryFilter implementation
        assert_eq!(AggregationQueryFilter::get_aggregations(&wrapper).len(), 1);
        assert_eq!(
            AggregationQueryFilter::get_bucket_size(&wrapper),
            Some("1 hour")
        );
        assert_eq!(AggregationQueryFilter::get_timezone(&wrapper), Some("UTC"));
        assert_eq!(
            AggregationQueryFilter::get_entity(&wrapper),
            Some("transactions")
        );
    }

    #[test]
    fn test_aggregation_filter_debug() {
        let filter = create_test_aggregation_filter();
        let debug_str = format!("{:?}", filter);
        assert!(debug_str.contains("AggregationFilter"));
    }

    #[test]
    fn test_aggregation_filter_clone() {
        let filter = create_test_aggregation_filter();
        let cloned = filter.clone();
        assert_eq!(filter.entity, cloned.entity);
        assert_eq!(filter.limit, cloned.limit);
    }
}
