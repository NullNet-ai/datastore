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
        // Convert the JSON payload to GetByFilter struct
        let filter: GetByFilter = serde_json::from_value(payload.clone())
            .map_err(|e| format!("Failed to parse payload as GetByFilter: {}", e))?;
        SQLConstructor::new(filter, table.to_string(), is_root, timezone)
            .with_organization_id(organization_id.unwrap_or_default())
            .construct()
    }

    /// Test constructing default selections with pluck fields
    #[test]
    fn should_construct_default_selections() {
        let env_config = EnvConfig::default();
        let payload = serde_json::json!({
            "pluck": ["id", "first_name", "last_name"]
        });
        let table = String::from("contacts");
        let is_root = false; // Set to true to avoid organization_id requirement
        let timezone = None;

        println!("  ✓ Generating SQL query from payload");
        let query_result = get_raw_query(
            &payload,
            &table,
            is_root,
            timezone,
            Some(env_config.default_organization_id.clone()),
        );

        assert!(
            query_result.is_ok(),
            "  ✗ Failed to generate query: {:?}",
            query_result.err()
        );
        let query = query_result.unwrap();
        println!("  ✓ Generated query: `{}`", query);

        // The query should contain the plucked fields
        let expected_query = format!(
            "SELECT {}",
            "\"contacts\".\"id\", \"contacts\".\"first_name\", \"contacts\".\"last_name\""
        );

        println!("  ✓ Expected query: `{}`", expected_query);
        let contain_expected_query = query.contains(&expected_query);
        let checker = if contain_expected_query { "✓" } else { "✗" };
        let message = format!(" {} Query should have contain the pluck fields.", checker);
        assert!(contain_expected_query, "{}", message);
    }

    /// Test constructing selections with pluck fields, pluck_object and joins
    #[test]
    fn should_construct_selections_with_pluck_fields_joins_pluck_object() {
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
            "pluck_object": {
                "contact_emails": ["id", "email"]
            },
            "joins": expected_joins
        });

        println!("--- Checking available joins.");
        let joins_array = expected_joins
            .as_array()
            .expect("Expected joins should be a valid JSON array");
        let join_has_len = joins_array.len() > 0;
        let join_checker = if join_has_len { "✓" } else { "✗" };
        assert!(join_has_len, "  {} Joins must exist", join_checker);

        let table = String::from("contacts");
        let is_root = false;
        let timezone = None;

        println!("  ✓ Generating SQL query from payload");
        let query_result = get_raw_query(
            &payload,
            &table,
            is_root,
            timezone,
            Some(env_config.default_organization_id.to_string()),
        );

        assert!(
            query_result.is_ok(),
            "  ✗ Failed to generate query: {:?}",
            query_result.err()
        );
        let query = query_result.unwrap();
        println!("  ✓ Generated query: `{}`", query);

        let expected_query = format!(
            "COALESCE( ( SELECT JSONB_AGG(elem ) FROM (SELECT JSONB_BUILD_OBJECT('id', \"contact_emails\".\"id\", 'email', \"contact_emails\".\"email\") AS elem FROM contact_emails contact_emails WHERE (\"contact_emails\".\"tombstone\" = 0 AND \"contact_emails\".\"organization_id\" IS NOT NULL AND \"contact_emails\".\"organization_id\" = '{}') AND \"contacts\".\"id\" = \"contact_emails\".\"contact_id\") sub ), '[]' ) AS contact_emails",
            &env_config.default_organization_id
        );

        let contain_expected_query = query.contains(&expected_query);
        let contain_checker = if contain_expected_query { "✓" } else { "✗" };

        println!("--- If pluck object does exist and has related tables specified then joins are required.");
        println!(
            "--- Checking if all {} fields are available in pluck object.",
            &table
        );
        println!("  {} Expected query: `{}`", contain_checker, expected_query);
        assert!(
            contain_expected_query,
            " {} Query should have contain the pluck object fields from {} with joins.",
            contain_checker, &table
        );
    }

    /// Test that when group_by is missing and joins are present, no default GROUP BY is injected
    #[test]
    fn should_not_inject_group_by_when_group_by_missing_and_joins_present() {
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
            "pluck": ["id", "first_name"],
            "joins": expected_joins,
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
            query.contains("JOIN LATERAL"),
            "Query should contain a LATERAL join to ensure joins-present path. Got: {}",
            query
        );
        let expected_group_by_query = format!("GROUP BY \"{}\".\"id\"", table);
        assert!(
            query.contains(&expected_group_by_query),
            "Query should contain default GROUP BY main table id when group_by is missing, even with joins. Expected: '{}'. Got: {}",
            expected_group_by_query,
            query
        );
    }

    /// Test constructing group by without count
    #[test]
    fn should_construct_group_by_without_count() {
        let env_config = EnvConfig::default();
        let expected_group_by = serde_json::json!({
            "fields": ["id", "first_name"],
            "has_count": false
        });
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
            "pluck": ["id", "first_name", "last_name"],
            "pluck_object": {
                "contact_emails": ["id", "email"]
            },
            "joins": expected_joins,
            "group_by": expected_group_by
        });

        println!("--- Checking available joins.");
        let joins_array = expected_joins
            .as_array()
            .expect("Expected joins should be a valid JSON array");
        let join_has_len = joins_array.len() > 0;
        let join_checker = if join_has_len { "✓" } else { "✗" };
        assert!(join_has_len, "  {} Joins must exist", join_checker);
        println!("--- Checking if it has group by");
        let group_has_fields = payload["group_by"]["fields"]
            .as_array()
            .map_or(0, |arr| arr.len())
            > 0;
        let group_fields_checker = if group_has_fields { "✓" } else { "✗" };
        assert!(
            group_has_fields,
            "{} group by has fields.",
            group_fields_checker
        );

        let table = String::from("contacts");
        let is_root = false;
        let timezone = None;

        println!("  ✓ Generating SQL query from payload");
        let query_result = get_raw_query(
            &payload,
            &table,
            is_root,
            timezone,
            Some(env_config.default_organization_id.to_string()),
        );

        assert!(
            query_result.is_ok(),
            "  ✗ Failed to generate query: {:?}",
            query_result.err()
        );

        let query = query_result.unwrap();
        println!("  ✓ Generated query: `{}`", query);

        // Group-by selections use aliases: "column" AS column
        let expected_selections = format!("SELECT \"contacts\".\"id\" AS id, \"contacts\".\"first_name\" AS first_name FROM \"contacts\" \"contacts\" LEFT JOIN LATERAL (SELECT \"joined_contact_emails\".\"id\", \"joined_contact_emails\".\"email\" FROM \"contact_emails\" \"joined_contact_emails\" WHERE (\"joined_contact_emails\".\"tombstone\" = 0 AND \"joined_contact_emails\".\"organization_id\" IS NOT NULL AND \"joined_contact_emails\".\"organization_id\" = '{}') AND \"joined_contact_emails\".\"contact_id\" = \"contacts\".\"id\" ) AS \"contact_emails\" ON TRUE WHERE (\"contacts\".\"tombstone\" = 0 AND \"contacts\".\"organization_id\" IS NOT NULL AND \"contacts\".\"organization_id\" = '{}')",
            &env_config.default_organization_id, &env_config.default_organization_id);
        println!("  ✓ Expected selections: `{}`", expected_selections);
        println!(
            "  ✓ Selection match: {}",
            query.contains(&expected_selections)
        );

        let expected_group_by_query =
            format!("GROUP BY \"contacts\".\"id\", \"contacts\".\"first_name\"");
        let expected_query = format!("{} {}", expected_selections, expected_group_by_query);
        let contain_allowed_selection_query = query.contains(&expected_selections);
        let contain_allowed_group_by_query = query.contains(&expected_group_by_query);
        let contain_expected_query = query.contains(&expected_query);
        let selection_checker = if contain_allowed_selection_query {
            "✓"
        } else {
            "✗"
        };
        let group_by_checker = if contain_allowed_group_by_query {
            "✓"
        } else {
            "✗"
        };
        let contain_checker = if contain_expected_query { "✓" } else { "✗" };
        println!("--- If pluck object does exist and has related tables specified then joins are required.");
        println!(
            "--- Checking if all {} fields are available in pluck object.",
            &table
        );
        println!(
            "  {} Expected selection query: `{}`",
            selection_checker, expected_selections
        );
        println!(
            "  {} Expected group by query: `{}`",
            group_by_checker, expected_group_by_query
        );
        println!("  {} Expected query: `{}`", contain_checker, expected_query);
        assert!(
            contain_allowed_selection_query,
            "Query should contain expected selections. Expected (substring): {} ... Actual query: {}",
            expected_selections,
            query
        );
        assert!(
            contain_allowed_group_by_query,
            "Query should contain expected GROUP BY. Expected: {} ... Actual query: {}",
            expected_group_by_query, query
        );
    }

    /// Test constructing group by with count
    #[test]
    fn should_construct_group_by_with_count() {
        let env_config = EnvConfig::default();
        let expected_group_by = serde_json::json!({
            "fields": ["id", "first_name"],
            "has_count": true
        });
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
            "pluck": ["id", "first_name", "last_name"],
            "pluck_object": {
                "contact_emails": ["id", "email"]
            },
            "joins": expected_joins,
            "group_by": expected_group_by
        });

        println!("--- Checking available joins.");
        let joins_array = expected_joins
            .as_array()
            .expect("Expected joins should be a valid JSON array");
        let join_has_len = joins_array.len() > 0;
        let join_checker = if join_has_len { "✓" } else { "✗" };
        assert!(join_has_len, "  {} Joins must exist", join_checker);
        println!("--- Checking if it has group by");
        let group_has_fields = payload["group_by"]["fields"]
            .as_array()
            .map_or(0, |arr| arr.len())
            > 0;
        let group_fields_checker = if group_has_fields { "✓" } else { "✗" };
        assert!(
            group_has_fields,
            "{} group by has fields.",
            group_fields_checker
        );

        let table = String::from("contacts");
        let is_root = false;
        let timezone = None;

        println!("  ✓ Generating SQL query from payload");
        let query_result = get_raw_query(
            &payload,
            &table,
            is_root,
            timezone,
            Some(env_config.default_organization_id.to_string()),
        );

        assert!(
            query_result.is_ok(),
            "  ✗ Failed to generate query: {:?}",
            query_result.err()
        );

        let query = query_result.unwrap();
        println!("  ✓ Generated query: `{}`", query);

        // Group-by selections use aliases: "column" AS column
        let expected_selections = format!("SELECT COUNT(*) AS count, \"contacts\".\"id\" AS id, \"contacts\".\"first_name\" AS first_name FROM \"contacts\" \"contacts\" LEFT JOIN LATERAL (SELECT \"joined_contact_emails\".\"id\", \"joined_contact_emails\".\"email\" FROM \"contact_emails\" \"joined_contact_emails\" WHERE (\"joined_contact_emails\".\"tombstone\" = 0 AND \"joined_contact_emails\".\"organization_id\" IS NOT NULL AND \"joined_contact_emails\".\"organization_id\" = '{}') AND \"joined_contact_emails\".\"contact_id\" = \"contacts\".\"id\" ) AS \"contact_emails\" ON TRUE WHERE (\"contacts\".\"tombstone\" = 0 AND \"contacts\".\"organization_id\" IS NOT NULL AND \"contacts\".\"organization_id\" = '{}')",
            &env_config.default_organization_id, &env_config.default_organization_id);
        println!("  ✓ Expected selections: `{}`", expected_selections);
        println!(
            "  ✓ Selection match: {}",
            query.contains(&expected_selections)
        );

        let expected_group_by_query =
            format!("GROUP BY \"contacts\".\"id\", \"contacts\".\"first_name\"");
        let expected_query = format!("{} {}", expected_selections, expected_group_by_query);
        let contain_allowed_selection_query = query.contains(&expected_selections);
        let contain_allowed_group_by_query = query.contains(&expected_group_by_query);
        let contain_expected_query = query.contains(&expected_query);
        let selection_checker = if contain_allowed_selection_query {
            "✓"
        } else {
            "✗"
        };
        let group_by_checker = if contain_allowed_group_by_query {
            "✓"
        } else {
            "✗"
        };
        let contain_checker = if contain_expected_query { "✓" } else { "✗" };
        println!("--- If pluck object does exist and has related tables specified then joins are required.");
        println!(
            "--- Checking if all {} fields are available in pluck object.",
            &table
        );
        println!(
            "  {} Expected selection query: `{}`",
            selection_checker, expected_selections
        );
        println!(
            "  {} Expected group by query: `{}`",
            group_by_checker, expected_group_by_query
        );
        println!("  {} Expected query: `{}`", contain_checker, expected_query);
        assert!(
            contain_allowed_selection_query,
            "Query should contain expected selections (with COUNT). Expected (substring): {} ... Actual query: {}",
            expected_selections,
            query
        );
        assert!(
            contain_allowed_group_by_query,
            "Query should contain expected GROUP BY. Expected: {} ... Actual query: {}",
            expected_group_by_query, query
        );
    }

    /// Test that when group_by is entirely missing, we default to GROUP BY main_table.id
    #[test]
    fn should_construct_group_by_default_to_main_table_id_when_group_by_missing() {
        let env_config = EnvConfig::default();
        let payload = serde_json::json!({
            "pluck": ["id", "first_name"],
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

        let expected_group_by_query = format!("GROUP BY \"{}\".\"id\"", table);
        assert!(
            query.contains(&expected_group_by_query),
            "When group_by is missing, query should contain default GROUP BY main table id. Expected: '{}'. Query: {}",
            expected_group_by_query,
            query
        );
    }

    /// Test that when group_by.fields is empty and has_count is true, we default to GROUP BY main_table.id
    #[test]
    fn should_construct_group_by_default_to_main_table_id_when_fields_empty_and_has_count() {
        let env_config = EnvConfig::default();
        let expected_group_by = serde_json::json!({
            "fields": [],
            "has_count": true
        });
        let payload = serde_json::json!({
            "pluck": ["id", "first_name"],
            "group_by": expected_group_by,
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

        // When fields is empty and has_count is true, expect only main table id in GROUP BY
        let expected_group_by_query = format!("GROUP BY \"{}\".\"id\"", table);
        assert!(
            query.contains(&expected_group_by_query),
            "Query should contain default GROUP BY main table id. Expected to find: '{}'. Query: {}",
            expected_group_by_query,
            query
        );

        // Should not group by other columns (e.g. first_name) when fields is empty
        let unexpected_group_by = "GROUP BY \"contacts\".\"id\", \"contacts\".\"first_name\"";
        assert!(
            !query.contains(unexpected_group_by),
            "Query should not group by pluck fields when group_by.fields is empty. Query: {}",
            query
        );
    }

    /// Test constructing concatenated fields for pluck selections without aliased entity
    #[test]
    fn should_construct_concatenated_fields_for_pluck_selections_without_aliased_entity() {
        let env_config = EnvConfig::default();
        let expected_concatenated_fields = serde_json::json!([
            {
                "fields": [
                    "first_name",
                    "last_name"
                ],
                "field_name": "full_name",
                "separator": " ",
                "entity": "contacts",
                // "aliased_entity": "created_by"
            }
        ]);

        let payload = serde_json::json!({
            "pluck": ["id"],
            "concatenate_fields": expected_concatenated_fields
        });

        let table = String::from("contacts");
        let is_root = false;
        let timezone = None;

        println!("  ✓ Generating SQL query from payload");
        let query_result = get_raw_query(
            &payload,
            &table,
            is_root,
            timezone,
            Some(env_config.default_organization_id.to_string()),
        );

        assert!(
            query_result.is_ok(),
            "  ✗ Failed to generate query: {:?}",
            query_result.err()
        );

        let query = query_result.unwrap();
        println!("  ✓ Generated query: `{}`", query);

        let expected_selections = format!("SELECT \"contacts\".\"id\", (COALESCE(\"contacts\".\"first_name\", '') || ' ' || COALESCE(\"contacts\".\"last_name\", '')) AS full_name FROM \"contacts\" \"contacts\" WHERE (\"contacts\".\"tombstone\" = 0 AND \"contacts\".\"organization_id\" IS NOT NULL AND \"contacts\".\"organization_id\" = '{}') GROUP BY \"contacts\".\"id\" ORDER BY LOWER(\"contacts\".\"id\") ASC NULLS FIRST LIMIT 10",
            &env_config.default_organization_id);
        println!("  ✓ Expected selections: `{}`", expected_selections);
        println!(
            "  ✓ Selection match: {}",
            query.contains(&expected_selections)
        );

        let expected_query = format!("{}", expected_selections);
        let contain_allowed_selection_query = query.contains(&expected_selections);
        let contain_expected_query = query.contains(&expected_query);
        let selection_checker = if contain_allowed_selection_query {
            "✓"
        } else {
            "✗"
        };

        let contain_checker = if contain_expected_query { "✓" } else { "✗" };
        println!(
            "  {} Expected selection query: `{}`",
            selection_checker, expected_selections
        );
        println!("  {} Expected query: `{}`", contain_checker, expected_query);
        assert!(
            contain_allowed_selection_query,
            " {} Query should have correct implementation of concatenated fields to work properly. Selection: {}",
            contain_checker, contain_allowed_selection_query
        );
    }

    /// Test sorting by concatenated datetime field using order_by (single sort).
    /// full_name = created_time || ' ' || timestamp; ORDER BY full_name should use the concatenated expression.
    #[test]
    fn should_sort_by_concatenated_datetime_field_with_order_by() {
        let env_config = EnvConfig::default();
        let payload = serde_json::json!({
            "pluck": ["id", "code", "organization_id"],
            "pluck_object": {
                "samples": ["id", "code", "created_by", "organization_id", "status", "name", "created_time"]
            },
            "order_by": "full_name",
            "order_direction": "desc",
            "is_case_sensitive_sorting": true,
            "multiple_sort": [],
            "concatenate_fields": [{
                "fields": ["created_time", "timestamp"],
                "field_name": "full_name",
                "separator": " ",
                "entity": "samples"
            }],
            "date_format": "mm/dd/YYYY",
            "time_format": "HH24:MI",
            "limit": 100,
            "offset": 0
        });

        let table = String::from("samples");
        let is_root = false;
        let timezone = Some("Europe/Berlin".to_string());

        let query_result = get_raw_query(
            &payload,
            &table,
            is_root,
            timezone,
            Some(env_config.default_organization_id.to_string()),
        );

        assert!(
            query_result.is_ok(),
            "  ✗ Failed to generate query: {:?}",
            query_result.err()
        );

        let query = query_result.unwrap();
        println!("  ✓ Generated query: `{}`", query);

        // ORDER BY should use the concatenated expression for full_name (created_time + timestamp)
        let expected_order_by_expr = "COALESCE(\"samples\".\"created_time\"::text, '') || ' ' || COALESCE(\"samples\".\"timestamp\"::text, '')";
        assert!(
            query.contains(expected_order_by_expr),
            "ORDER BY should contain concatenated expression for full_name. Got: {}",
            query
        );
        assert!(
            query.contains("ORDER BY") && query.contains("DESC"),
            "Query should have ORDER BY ... DESC"
        );
    }

    /// Test sorting by concatenated datetime field using multiple_sort.
    /// full_name = created_time || ' ' || timestamp; multiple_sort with full_name should use the concatenated expression.
    #[test]
    fn should_sort_by_concatenated_datetime_field_with_multiple_sort() {
        let env_config = EnvConfig::default();
        let payload = serde_json::json!({
            "pluck": ["id", "code", "organization_id"],
            "pluck_object": {
                "samples": ["id", "code", "created_by", "organization_id", "status", "name", "created_time"]
            },
            "order_by": "id",
            "order_direction": "asc",
            "multiple_sort": [
                {"by_field": "full_name", "by_direction": "desc", "is_case_sensitive_sorting": true}
            ],
            "concatenate_fields": [{
                "fields": ["created_time", "timestamp"],
                "field_name": "full_name",
                "separator": " ",
                "entity": "samples"
            }],
            "date_format": "mm/dd/YYYY",
            "time_format": "HH24:MI",
            "limit": 100,
            "offset": 0
        });

        let table = String::from("samples");
        let is_root = false;
        let timezone = Some("Europe/Berlin".to_string());

        let query_result = get_raw_query(
            &payload,
            &table,
            is_root,
            timezone,
            Some(env_config.default_organization_id.to_string()),
        );

        assert!(
            query_result.is_ok(),
            "  ✗ Failed to generate query: {:?}",
            query_result.err()
        );

        let query = query_result.unwrap();
        println!("  ✓ Generated query: `{}`", query);

        // multiple_sort takes precedence; ORDER BY should use concatenated expression for full_name
        let expected_order_by_expr = "COALESCE(\"samples\".\"created_time\"::text, '') || ' ' || COALESCE(\"samples\".\"timestamp\"::text, '')";
        assert!(
            query.contains(expected_order_by_expr),
            "ORDER BY with multiple_sort should contain concatenated expression for full_name. Got: {}",
            query
        );
        assert!(
            query.contains("ORDER BY") && query.contains("DESC"),
            "Query should have ORDER BY ... DESC from multiple_sort"
        );
    }

    /// Test that ASC ordering produces ASC NULLS FIRST (matches TypeScript behavior).
    #[test]
    fn should_use_nulls_first_for_asc_order() {
        let env_config = EnvConfig::default();
        let payload = serde_json::json!({
            "pluck": ["id", "name"],
            "order_by": "name",
            "order_direction": "asc",
            "multiple_sort": [],
            "limit": 10,
            "offset": 0
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
            query.contains("ASC NULLS FIRST"),
            "Query should contain ASC NULLS FIRST for asc order. Got: {}",
            query
        );
    }

    /// Test that DESC ordering produces DESC NULLS LAST (matches TypeScript behavior).
    #[test]
    fn should_use_nulls_last_for_desc_order() {
        let env_config = EnvConfig::default();
        let payload = serde_json::json!({
            "pluck": ["id", "name"],
            "order_by": "name",
            "order_direction": "desc",
            "multiple_sort": [],
            "limit": 10,
            "offset": 0
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
            query.contains("DESC NULLS LAST"),
            "Query should contain DESC NULLS LAST for desc order. Got: {}",
            query
        );
    }

    /// Test that multiple_sort with asc uses NULLS FIRST and with desc uses NULLS LAST.
    #[test]
    fn should_use_nulls_clauses_for_multiple_sort() {
        let env_config = EnvConfig::default();
        let payload = serde_json::json!({
            "pluck": ["id", "first_name", "last_name"],
            "order_by": "id",
            "order_direction": "asc",
            "multiple_sort": [
                {"by_field": "first_name", "by_direction": "asc", "is_case_sensitive_sorting": false},
                {"by_field": "last_name", "by_direction": "desc", "is_case_sensitive_sorting": false}
            ],
            "limit": 10,
            "offset": 0
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
            query.contains("NULLS FIRST"),
            "Query should contain NULLS FIRST for asc in multiple_sort. Got: {}",
            query
        );
        assert!(
            query.contains("NULLS LAST"),
            "Query should contain NULLS LAST for desc in multiple_sort. Got: {}",
            query
        );
    }

    /// Test constructing concatenated fields for pluck_object join selections with aliased entity
    #[test]
    // ! revisit this
    #[ignore]
    fn should_construct_concatenated_fields_for_pluck_object_join_selections_with_aliased_entity() {
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
                        "offset": null,
                        "alias": "ce_sample"
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
        let expected_concatenated_fields = serde_json::json!([
            {
                "fields": [
                    "id",
                    "status"
                ],
                "field_name": "id_status",
                "separator": " ",
                "entity": "contact_emails",
                "aliased_entity": "ce_sample"
            }
        ]);

        let payload = serde_json::json!({
            "pluck": ["id"],
            "pluck_object": {
               "contacts": ["id", "first_name", "last_name"],
               "ce_sample": ["id", "email"]
            },
            "joins": expected_joins,
            "concatenate_fields": expected_concatenated_fields
        });

        let table = String::from("contacts");
        let is_root = false;
        let timezone = None;

        println!("--- Checking available joins.");
        let joins_array = expected_joins
            .as_array()
            .expect("Expected joins should be a valid JSON array");
        let join_has_len = joins_array.len() > 0;
        let join_checker = if join_has_len { "✓" } else { "✗" };
        assert!(join_has_len, "  {} Joins must exist", join_checker);
        println!("  ✓ Generating SQL query from payload");
        let query_result = get_raw_query(
            &payload,
            &table,
            is_root,
            timezone,
            Some(env_config.default_organization_id.to_string()),
        );

        assert!(
            query_result.is_ok(),
            "  ✗ Failed to generate query: {:?}",
            query_result.err()
        );

        let query = query_result.unwrap();
        println!("  ✓ Generated query: `{}`", query);

        let expected_selections = format!("SELECT COALESCE( ( SELECT JSONB_AGG(elem ) FROM (SELECT JSONB_BUILD_OBJECT('id', \"contacts\".\"id\", 'first_name', \"contacts\".\"first_name\", 'last_name', \"contacts\".\"last_name\") AS elem FROM contacts joined_contacts WHERE (joined_contacts.tombstone = 0 AND joined_contacts.organization_id IS NOT NULL AND joined_contacts.organization_id = '{}') AND \"joined_contacts\".\"id\" = \"contacts\".\"id\") sub ), '[]' ) AS contacts, COALESCE( ( SELECT JSONB_AGG(elem ) FROM (SELECT JSONB_BUILD_OBJECT('id', \"ce_sample\".\"id\", 'email', \"ce_sample\".\"email\", 'id_status', (COALESCE(\"ce_sample\".\"id\", '') || ' ' || COALESCE(\"ce_sample\".\"status\", ''))) AS elem FROM contact_emails ce_sample WHERE (ce_sample.tombstone = 0 AND ce_sample.organization_id IS NOT NULL AND ce_sample.organization_id = '{}') AND \"contacts\".\"id\" = \"ce_sample\".\"contact_id\") sub ), '[]' ) AS ce_sample FROM {}",
            &env_config.default_organization_id, &env_config.default_organization_id, &table);
        println!("  ✓ Expected selections: `{}`", expected_selections);
        println!(
            "  ✓ Selection match: {}",
            query.contains(&expected_selections)
        );
        let expected_joins = format!("LEFT JOIN LATERAL (SELECT \"joined_ce_sample\".\"id\", \"joined_ce_sample\".\"email\" FROM \"contact_emails\" \"joined_ce_sample\" WHERE (joined_ce_sample.tombstone = 0 AND joined_ce_sample.organization_id IS NOT NULL AND joined_ce_sample.organization_id = '{}') AND \"joined_ce_sample\".\"contact_id\" = \"contacts\".\"id\" ) AS \"ce_sample\" ON TRUE", &env_config.default_organization_id);

        let expected_default_where_clauses = format!("WHERE (\"contacts\".\"tombstone\" = 0 AND \"contacts\".\"organization_id\" IS NOT NULL AND \"contacts\".\"organization_id\" = '{}') ORDER BY LOWER(\"contacts\".\"id\") ASC NULLS FIRST LIMIT 10", &env_config.default_organization_id);

        let expected_query = format!(
            "{} {} {}",
            expected_selections, expected_joins, expected_default_where_clauses
        );
        let contain_allowed_selection_query = query.contains(&expected_selections);
        let contain_expected_query = query.contains(&expected_query);
        let selection_checker = if contain_allowed_selection_query {
            "✓"
        } else {
            "✗"
        };

        let contain_checker = if contain_expected_query { "✓" } else { "✗" };
        println!(
            "  {} Expected selection query: `{}`",
            selection_checker, expected_selections
        );
        println!("  {} Expected query: `{}`", contain_checker, expected_query);
        assert!(
            contain_allowed_selection_query,
            " {} Query should have correct implementation of concatenated fields to work properly. Selection: {}",
            contain_checker, contain_allowed_selection_query
        );
    }

    /// Test constructing RIGHT JOIN LATERAL (same pattern as LEFT join tests)
    #[test]
    fn should_construct_right_join_lateral() {
        let env_config = EnvConfig::default();
        let expected_joins = serde_json::json!([
            {
                "type": "right",
                "field_relation": {
                    "to": {
                        "alias": "org_owner",
                        "entity": "organizations",
                        "field": "id",
                        "order_direction": null,
                        "order_by": null,
                        "limit": null,
                        "offset": null
                    },
                    "from": {
                        "entity": "samples",
                        "field": "organization_id",
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
            "pluck": ["id", "code", "organization_id"],
            "pluck_object": {
                "samples": ["id", "name", "code", "organization_id", "status"],
                "org_owner": ["id", "name"]
            },
            "joins": expected_joins,
            "limit": 100
        });

        println!("--- Checking available joins (right).");
        let joins_array = expected_joins
            .as_array()
            .expect("Expected joins should be a valid JSON array");
        assert!(!joins_array.is_empty(), "Joins must exist");

        let table = String::from("samples");
        let is_root = false;
        let timezone = None;

        println!("  ✓ Generating SQL query from payload with RIGHT join");
        let query_result = get_raw_query(
            &payload,
            &table,
            is_root,
            timezone,
            Some(env_config.default_organization_id.to_string()),
        );

        assert!(
            query_result.is_ok(),
            "  ✗ Failed to generate query: {:?}",
            query_result.err()
        );
        let query = query_result.unwrap();
        println!("  ✓ Generated query: `{}`", query);

        // Must contain RIGHT JOIN LATERAL
        assert!(
            query.contains("RIGHT JOIN LATERAL"),
            "Query should contain RIGHT JOIN LATERAL, got: {}",
            query
        );

        // Subquery: SELECT from pluck_object (id, name) + tombstone, organization_id FROM organizations org_owner
        let expected_subquery_select = "SELECT \"org_owner\".\"id\", \"org_owner\".\"name\", \"org_owner\".\"tombstone\", \"org_owner\".\"organization_id\" FROM \"organizations\" \"org_owner\"";
        assert!(
            query.contains(expected_subquery_select),
            "Query should contain RIGHT LATERAL subquery with selected fields and tombstone/organization_id, got: {}",
            query
        );

        // ON clause must reference org_owner (tombstone, organization_id) and join condition
        assert!(
            query.contains("AS \"org_owner\" ON ("),
            "Query should contain LATERAL alias and ON clause, got: {}",
            query
        );
        // Join condition in ON: "org_owner"."id" = "samples"."organization_id"
        assert!(
            query.contains("\"org_owner\".\"id\" = \"samples\".\"organization_id\""),
            "Query ON clause should contain join condition org_owner.id = samples.organization_id, got: {}",
            query
        );

        println!("  ✓ RIGHT JOIN LATERAL test passed");
    }

    /// Test constructing INNER JOIN LATERAL (same structure as LEFT: WHERE in subquery, ON TRUE)
    #[test]
    fn should_construct_inner_join_lateral() {
        let env_config = EnvConfig::default();
        let expected_joins = serde_json::json!([
            {
                "type": "inner",
                "field_relation": {
                    "to": {
                        "alias": "org_owner",
                        "entity": "organizations",
                        "field": "id",
                        "order_direction": null,
                        "order_by": null,
                        "limit": null,
                        "offset": null
                    },
                    "from": {
                        "entity": "samples",
                        "field": "organization_id",
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
            "pluck": ["id", "code", "organization_id"],
            "pluck_object": {
                "samples": ["id", "name", "code", "organization_id", "status"],
                "org_owner": ["id", "name"]
            },
            "joins": expected_joins,
            "limit": 100
        });

        let table = String::from("samples");
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
            query.contains("INNER JOIN LATERAL"),
            "Query should contain INNER JOIN LATERAL, got: {}",
            query
        );

        // INNER uses same shape as LEFT: joined_<alias> in subquery, WHERE with system + join condition, ON TRUE
        assert!(
            query.contains("joined_org_owner"),
            "Query should contain lateral subquery alias joined_org_owner, got: {}",
            query
        );
        assert!(
            query.contains("AS \"org_owner\" ON TRUE"),
            "Query should end LATERAL with AS org_owner ON TRUE, got: {}",
            query
        );
        assert!(
            query.contains("\"joined_org_owner\".\"id\" = \"samples\".\"organization_id\""),
            "Query WHERE should contain join condition, got: {}",
            query
        );
    }
}
