#[cfg(test)]
mod tests {
    use crate::{
        config::core::EnvConfig, providers::queries::find::SQLConstructor,
        structs::core::GetByFilter,
    };

    fn get_raw_query(
        payload: &serde_json::Value,
        table: &str,
        is_root: bool,
        timezone: Option<String>,
        organization_id: Option<String>,
    ) -> Result<String, String> {
        crate::test_init::init_test_state();
        let filter: GetByFilter = serde_json::from_value(payload.clone())
            .map_err(|e| format!("Failed to parse payload as GetByFilter: {}", e))?;
        SQLConstructor::new(filter, table.to_string(), is_root, timezone)
            .with_organization_id(organization_id.unwrap_or_default())
            .construct()
    }

    #[test]
    fn multiple_sort_should_aggregate_for_joined_field_desc() {
        let env_config = EnvConfig::default();
        let expected_joins = serde_json::json!([
            {
                "type": "left",
                "field_relation": {
                    "to": {
                        "entity": "contact_emails",
                        "field": "contact_id",
                        "order_direction": null,
                        "order_by": null,
                        "limit": null,
                        "offset": null
                    },
                    "from": {
                        "entity": "contacts",
                        "field": "id",
                        "order_direction": null,
                        "order_by": null,
                        "limit": null,
                        "offset": null
                    }
                },
                "nested": false
            }
        ]);
        let payload = serde_json::json!({
            "pluck": ["id"],
            "joins": expected_joins,
            "multiple_sort": [
                {"by_field": "contact_emails.email", "by_direction": "desc", "is_case_sensitive_sorting": false}
            ],
            "limit": 10
        });

        let table = String::from("contacts");
        let is_root = false;
        let timezone = None;

        let query_result = get_raw_query(
            &payload,
            &table,
            is_root,
            timezone,
            Some(env_config.default_organization_id.to_string()),
        );
        assert!(
            query_result.is_ok(),
            "Failed to generate query: {:?}",
            query_result.err()
        );
        let query = query_result.unwrap();

        let expected_clause = "ORDER BY MAX(LOWER(\"contact_emails\".\"email\")) DESC NULLS LAST";
        assert!(
            query.contains(expected_clause),
            "Expected '{}', got: {}",
            expected_clause,
            query
        );
    }

    #[test]
    fn multiple_sort_should_aggregate_for_joined_field_asc() {
        let env_config = EnvConfig::default();
        let expected_joins = serde_json::json!([
            {
                "type": "left",
                "field_relation": {
                    "to": {
                        "entity": "contact_emails",
                        "field": "contact_id",
                        "order_direction": null,
                        "order_by": null,
                        "limit": null,
                        "offset": null
                    },
                    "from": {
                        "entity": "contacts",
                        "field": "id",
                        "order_direction": null,
                        "order_by": null,
                        "limit": null,
                        "offset": null
                    }
                },
                "nested": false
            }
        ]);
        let payload = serde_json::json!({
            "pluck": ["id"],
            "joins": expected_joins,
            "multiple_sort": [
                {"by_field": "contact_emails.email", "by_direction": "asc", "is_case_sensitive_sorting": false}
            ],
            "limit": 10
        });

        let table = String::from("contacts");
        let is_root = false;
        let timezone = None;

        let query_result = get_raw_query(
            &payload,
            &table,
            is_root,
            timezone,
            Some(env_config.default_organization_id.to_string()),
        );
        assert!(
            query_result.is_ok(),
            "Failed to generate query: {:?}",
            query_result.err()
        );
        let query = query_result.unwrap();

        let expected_clause = "ORDER BY MIN(LOWER(\"contact_emails\".\"email\")) ASC NULLS FIRST";
        assert!(
            query.contains(expected_clause),
            "Expected '{}', got: {}",
            expected_clause,
            query
        );
    }

    #[test]
    fn multiple_sort_should_not_aggregate_for_main_field() {
        let env_config = EnvConfig::default();
        let payload = serde_json::json!({
            "pluck": ["id", "first_name"],
            "multiple_sort": [
                {"by_field": "first_name", "by_direction": "desc", "is_case_sensitive_sorting": false}
            ],
            "limit": 10
        });

        let table = String::from("contacts");
        let is_root = false;
        let timezone = None;

        let query_result = get_raw_query(
            &payload,
            &table,
            is_root,
            timezone,
            Some(env_config.default_organization_id.to_string()),
        );
        assert!(
            query_result.is_ok(),
            "Failed to generate query: {:?}",
            query_result.err()
        );
        let query = query_result.unwrap();

        let expected_clause = "ORDER BY LOWER(\"contacts\".\"first_name\") DESC NULLS LAST";
        assert!(
            query.contains(expected_clause),
            "Expected '{}', got: {}",
            expected_clause,
            query
        );
        assert!(
            !query.contains("MAX(") && !query.contains("MIN("),
            "Expected no aggregate for main table field. Got: {}",
            query
        );
    }

    #[test]
    fn single_order_by_should_aggregate_for_joined_field_desc() {
        let env_config = EnvConfig::default();
        let expected_joins = serde_json::json!([
            {
                "type": "left",
                "field_relation": {
                    "to": {
                        "entity": "contact_emails",
                        "field": "contact_id",
                        "order_direction": null,
                        "order_by": null,
                        "limit": null,
                        "offset": null
                    },
                    "from": {
                        "entity": "contacts",
                        "field": "id",
                        "order_direction": null,
                        "order_by": null,
                        "limit": null,
                        "offset": null
                    }
                },
                "nested": false
            }
        ]);
        let payload = serde_json::json!({
            "pluck": ["id"],
            "joins": expected_joins,
            "order_by": "contact_emails.email",
            "order_direction": "desc",
            "limit": 10
        });

        let table = String::from("contacts");
        let is_root = false;
        let timezone = None;

        let query_result = get_raw_query(
            &payload,
            &table,
            is_root,
            timezone,
            Some(env_config.default_organization_id.to_string()),
        );
        assert!(
            query_result.is_ok(),
            "Failed to generate query: {:?}",
            query_result.err()
        );
        let query = query_result.unwrap();

        let expected_clause = "ORDER BY MAX(LOWER(\"contact_emails\".\"email\")) DESC NULLS LAST";
        assert!(
            query.contains(expected_clause),
            "Expected '{}', got: {}",
            expected_clause,
            query
        );
    }

    #[test]
    fn single_order_by_should_not_aggregate_for_main_field() {
        let env_config = EnvConfig::default();
        let payload = serde_json::json!({
            "pluck": ["id", "first_name"],
            "order_by": "first_name",
            "order_direction": "asc",
            "limit": 10
        });

        let table = String::from("contacts");
        let is_root = false;
        let timezone = None;

        let query_result = get_raw_query(
            &payload,
            &table,
            is_root,
            timezone,
            Some(env_config.default_organization_id.to_string()),
        );
        assert!(
            query_result.is_ok(),
            "Failed to generate query: {:?}",
            query_result.err()
        );
        let query = query_result.unwrap();

        let expected_clause = "ORDER BY LOWER(\"contacts\".\"first_name\") ASC NULLS FIRST";
        assert!(
            query.contains(expected_clause),
            "Expected '{}', got: {}",
            expected_clause,
            query
        );
        assert!(
            !query.contains("MAX(") && !query.contains("MIN("),
            "Expected no aggregate for main table field. Got: {}",
            query
        );
    }

    #[test]
    fn single_order_by_should_use_lower_when_case_insensitive() {
        let env_config = EnvConfig::default();
        let payload = serde_json::json!({
            "pluck": ["id", "first_name"],
            "order_by": "first_name",
            "order_direction": "asc",
            "is_case_sensitive_sorting": false,
            "limit": 10
        });

        let table = String::from("contacts");
        let is_root = false;
        let timezone = None;

        let query_result = get_raw_query(
            &payload,
            &table,
            is_root,
            timezone,
            Some(env_config.default_organization_id.to_string()),
        );
        assert!(
            query_result.is_ok(),
            "Failed to generate query: {:?}",
            query_result.err()
        );
        let query = query_result.unwrap();

        assert!(
            query.contains("ORDER BY LOWER("),
            "Expected ORDER BY LOWER(...) for case-insensitive sorting. Got: {}",
            query
        );
        assert!(
            query.contains("ASC NULLS FIRST"),
            "Expected ASC NULLS FIRST for asc direction. Got: {}",
            query
        );
    }
}
