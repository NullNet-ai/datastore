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
            Some(env_config.default_organization_id),
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

    /// Test constructing selections with pluck fields and pluck_object
    #[test]
    fn should_construct_selections_with_pluck_fields_pluck_object() {
        let env_config = EnvConfig::default();
        let expected_fields = ["id", "first_name", "last_name"];
        let payload = serde_json::json!({
            "pluck": ["id", "first_name"],
            "pluck_object": {
                "contacts": expected_fields
            }
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
            Some(env_config.default_organization_id),
        );

        assert!(
            query_result.is_ok(),
            "  ✗ Failed to generate query: {:?}",
            query_result.err()
        );
        let query = query_result.unwrap();
        println!("  ✓ Generated query: `{}`", query);
        let expected_default_queries = format!("WHERE (contacts.tombstone = 0 AND contacts.organization_id IS NOT NULL AND contacts.organization_id = '01JBHKXHYSKPP247HZZWHA3JCT') ORDER BY LOWER(contacts.id) ASC LIMIT 10");
        let expected_query = format!(
            "SELECT \"contacts\".\"id\", \"contacts\".\"first_name\" FROM {} {}",
            &table, expected_default_queries
        );

        let contain_expected_query = query.contains(&expected_query);
        let checker = if contain_expected_query { "✓" } else { "✗" };

        println!("--- If pluck object does exist then pluck will be ignored.");
        println!(
            "--- Checking if all {} fields are available in pluck object.",
            &table
        );

        let pluck_object_main_table_fields = payload["pluck_object"][&table]
            .as_array()
            .expect("Pluck object main table fields should be a valid JSON array");

        for field in expected_fields.iter() {
            assert!(
                pluck_object_main_table_fields
                    .contains(&serde_json::Value::String(field.to_string())),
                "Pluck object from {} fields must contain {}",
                &table,
                field
            );
        }

        assert!(
            pluck_object_main_table_fields.len() == expected_fields.len(),
            "Pluck object from {} fields must contain {} fields",
            &table,
            expected_fields.len()
        );

        println!("  ✓ All {} fields are available in pluck object.", &table);
        println!("  {} Expected query: `{}`", checker, expected_query);

        assert!(
            contain_expected_query,
            " {} Query should have contain the pluck object fields from {}.",
            checker, &table
        );
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
            "SELECT COALESCE( ( SELECT JSONB_AGG(elem ) FROM (SELECT JSONB_BUILD_OBJECT('id', \"contact_emails\".\"id\", 'email', \"contact_emails\".\"email\") AS elem FROM contact_emails contact_emails WHERE (contact_emails.tombstone = 0 AND contact_emails.organization_id IS NOT NULL AND contact_emails.organization_id = '{}') AND \"contacts\".\"id\" = \"contact_emails\".\"contact_id\") sub ), '[]' ) AS contact_emails",
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

    #[test]
    fn should_construct_selections_with_pluck_fields_joins_pluck_object_with_main() {
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
                "contacts": ["first_name"],
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
            "SELECT \"contacts\".\"first_name\", COALESCE( ( SELECT JSONB_AGG(elem ) FROM (SELECT JSONB_BUILD_OBJECT('id', \"contact_emails\".\"id\", 'email', \"contact_emails\".\"email\") AS elem FROM contact_emails contact_emails WHERE (contact_emails.tombstone = 0 AND contact_emails.organization_id IS NOT NULL AND contact_emails.organization_id = '{}') AND \"contacts\".\"id\" = \"contact_emails\".\"contact_id\") sub ), '[]' ) AS contact_emails",
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

        let expected_selections = format!("SELECT \"contacts\".\"id\", \"contacts\".\"first_name\" FROM contacts LEFT JOIN LATERAL (SELECT \"joined_contact_emails\".\"id\", \"joined_contact_emails\".\"email\" FROM \"contact_emails\" \"joined_contact_emails\" WHERE (joined_contact_emails.tombstone = 0 AND joined_contact_emails.organization_id IS NOT NULL AND joined_contact_emails.organization_id = '{}') AND \"joined_contact_emails\".\"contact_id\" = \"contacts\".\"id\" ) AS \"contact_emails\" ON TRUE WHERE (contacts.tombstone = 0 AND contacts.organization_id IS NOT NULL AND contacts.organization_id = '{}')",
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
            contain_allowed_selection_query && contain_allowed_group_by_query,
            " {} Query should have correct implementation of selections, order by and group by to work properly. Selection: {}, Group By: {}",
            contain_checker, contain_allowed_selection_query, contain_allowed_group_by_query
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

        let expected_selections = format!("SELECT COUNT(*) AS count, \"contacts\".\"id\", \"contacts\".\"first_name\" FROM contacts LEFT JOIN LATERAL (SELECT \"joined_contact_emails\".\"id\", \"joined_contact_emails\".\"email\" FROM \"contact_emails\" \"joined_contact_emails\" WHERE (joined_contact_emails.tombstone = 0 AND joined_contact_emails.organization_id IS NOT NULL AND joined_contact_emails.organization_id = '{}') AND \"joined_contact_emails\".\"contact_id\" = \"contacts\".\"id\" ) AS \"contact_emails\" ON TRUE WHERE (contacts.tombstone = 0 AND contacts.organization_id IS NOT NULL AND contacts.organization_id = '{}')",
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
            contain_allowed_selection_query && contain_allowed_group_by_query,
            " {} Query should have correct implementation of selections, order by and group by to work properly. Selection: {}, Group By: {}",
            contain_checker, contain_allowed_selection_query, contain_allowed_group_by_query
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
            "pluck": ["id", "first_name", "last_name"],
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

        let expected_selections = format!("SELECT \"contacts\".\"id\", \"contacts\".\"first_name\", \"contacts\".\"last_name\", (COALESCE(\"contacts\".\"first_name\", '') || ' ' || COALESCE(\"contacts\".\"last_name\", '')) AS full_name FROM contacts WHERE (contacts.tombstone = 0 AND contacts.organization_id IS NOT NULL AND contacts.organization_id = '{}') ORDER BY LOWER(contacts.id) ASC LIMIT 10",
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

    /// Test constructing concatenated fields for pluck_object join selections with aliased entity
    #[test]
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
            "pluck_object": {
               "contacts": ["id", "first_name", "last_name"],
               "contact_emails": ["id", "email"]
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

        let expected_selections = format!("SELECT \"contacts\".\"id\", \"contacts\".\"first_name\", \"contacts\".\"last_name\", (COALESCE(\"contacts\".\"first_name\", '') || ' ' || COALESCE(\"contacts\".\"last_name\", '')) AS full_name, COALESCE( ( SELECT JSONB_AGG(elem ) FROM (SELECT JSONB_BUILD_OBJECT('id', \"contact_emails\".\"id\", 'email', \"contact_emails\".\"email\") AS elem FROM contact_emails contact_emails WHERE (contact_emails.tombstone = 0 AND contact_emails.organization_id IS NOT NULL AND contact_emails.organization_id = '{}') AND \"contacts\".\"id\" = \"contact_emails\".\"contact_id\") sub ), '[]' ) AS contact_emails FROM {}",
            &env_config.default_organization_id, &table);
        println!("  ✓ Expected selections: `{}`", expected_selections);
        println!(
            "  ✓ Selection match: {}",
            query.contains(&expected_selections)
        );
        let expected_joins = format!("LEFT JOIN LATERAL (SELECT \"joined_contact_emails\".\"id\", \"joined_contact_emails\".\"email\" FROM \"contact_emails\" \"joined_contact_emails\" WHERE (joined_contact_emails.tombstone = 0 AND joined_contact_emails.organization_id IS NOT NULL AND joined_contact_emails.organization_id = '{}') AND \"joined_contact_emails\".\"contact_id\" = \"contacts\".\"id\" ) AS \"contact_emails\" ON TRUE", &env_config.default_organization_id);

        let expected_default_where_clauses = format!("WHERE (contacts.tombstone = 0 AND contacts.organization_id IS NOT NULL AND contacts.organization_id = '{}') ORDER BY LOWER(contacts.id) ASC LIMIT 10", &env_config.default_organization_id);

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


    /// Test constructing complex query
    #[test]
    fn should_construct_complex_query() {
        let env_config = EnvConfig::default();
      
        let payload = serde_json::json!({
  "pluck": [
    "id",
    "categories",
    "organization_id",
    "first_name",
    "middle_name",
    "last_name"
  ],
  "pluck_object": {
    "updated_by": [
      "id",
      "first_name",
      "last_name"
    ],
    "contact_emails": [
      "email",
      "is_primary"
    ],
    "created_by_account_organizations": [
      "id",
      "contact_id"
    ],
    "contact_phone_numbers": [
      "phone_number_raw"
    ],
    "created_by": [
      "id",
      "first_name",
      "last_name"
    ],
    "contacts": [
      "id",
      "code",
      "categories",
      "organization_id",
      "first_name",
      "middle_name",
      "last_name",
      "status",
      "created_date",
      "updated_date",
      "created_time",
      "updated_time",
      "created_by",
      "updated_by",
      "previous_status"
    ],
    "updated_by_account_organizations": [
      "id",
      "contact_id"
    ]
  },
  "pluck_group_object": {
    "contact_emails": [
      "email",
      "is_primary"
    ],
    "contact_phone_numbers": [
      "phone_number_raw"
    ]
  },
  "advance_filters": [
    {
      "type": "criteria",
      "field": "created_date_time",
      "entity": "contacts",
      "operator": "like",
      "values": [
        "08/20/2025 14"
      ],
      "case_sensitive": false,
      "parse_as": "",
      "match_pattern": null,
      "is_search": null,
      "has_group_count": null
    },
    {
      "type": "operator",
      "operator": "and"
    },
    {
      "type": "criteria",
      "field": "status",
      "entity": "contacts",
      "operator": "equal",
      "values": [
        "Active",
        "Draft"
      ],
      "case_sensitive": false,
      "parse_as": "",
      "match_pattern": null,
      "is_search": null,
      "has_group_count": null
    }
  ],
  "group_advance_filters": [],
  "joins": [
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
    },
    {
      "type": "left",
      "field_relation": {
        "to": {
          "entity": "contact_phone_numbers",
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
    },
    {
      "type": "left",
      "field_relation": {
        "to": {
          "entity": "account_organizations",
          "field": "id",
          "alias": "created_by_account_organizations",
          "order_direction": null,
          "order_by": null,
          "limit": null,
          "offset": null
        },
        "from": {
          "entity": "contacts",
          "field": "created_by",
          "order_direction": null,
          "order_by": null,
          "limit": null,
          "offset": null
        }
      },
      "nested": false
    },
    {
      "type": "left",
      "field_relation": {
        "to": {
          "entity": "contacts",
          "field": "id",
          "alias": "created_by",
          "order_direction": null,
          "order_by": null,
          "limit": null,
          "offset": null
        },
        "from": {
          "entity": "created_by_account_organizations",
          "field": "contact_id",
          "order_direction": null,
          "order_by": null,
          "limit": null,
          "offset": null
        }
      },
      "nested": true
    },
    {
      "type": "left",
      "field_relation": {
        "to": {
          "entity": "account_organizations",
          "field": "id",
          "alias": "updated_by_account_organizations",
          "order_direction": null,
          "order_by": null,
          "limit": null,
          "offset": null
        },
        "from": {
          "entity": "contacts",
          "field": "updated_by",
          "order_direction": null,
          "order_by": null,
          "limit": null,
          "offset": null
        }
      },
      "nested": false
    },
    {
      "type": "left",
      "field_relation": {
        "to": {
          "entity": "contacts",
          "field": "id",
          "alias": "updated_by",
          "order_direction": null,
          "order_by": null,
          "limit": null,
          "offset": null
        },
        "from": {
          "entity": "updated_by_account_organizations",
          "field": "contact_id",
          "order_direction": null,
          "order_by": null,
          "limit": null,
          "offset": null
        }
      },
      "nested": true
    }
  ],
  "group_by": {
    "fields": [],
    "has_count": true
  },
  "concatenate_fields": [
    {
      "fields": [
        "first_name",
        "last_name"
      ],
      "field_name": "full_name",
      "separator": " ",
      "entity": "contacts",
      "aliased_entity": "created_by"
    },
    {
      "fields": [
        "first_name",
        "last_name"
      ],
      "field_name": "full_name",
      "separator": " ",
      "entity": "contacts",
      "aliased_entity": "updated_by"
    },
    {
      "fields": [
        "created_date",
        "created_time"
      ],
      "field_name": "created_date_time",
      "separator": " ",
      "entity": "contacts",
      "aliased_entity": null
    },
    {
      "fields": [
        "updated_date",
        "updated_time"
      ],
      "field_name": "updated_date_time",
      "separator": " ",
      "entity": "contacts",
      "aliased_entity": null
    }
  ],
  "multiple_sort": [
    {
      "by_field": "status",
      "by_direction": "asc",
      "is_case_sensitive_sorting": false
    }
  ],
  "date_format": "mm/dd/YYYY",
  "order_by": "id",
  "order_direction": "asc",
  "is_case_sensitive_sorting": false,
  "offset": 0,
  "limit": 100,
  "distinct_by": "",
  "timezone": null
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
        let expected_query = format!(
            "SELECT {}",
            "",
        );
        let contain_expected_query = query.contains(&expected_query);
      

        let contain_checker = if contain_expected_query { "✓" } else { "✗" };
        println!(
            "  {} Expected query: `{}`",
            contain_checker, expected_query
        );
        assert!(
            contain_expected_query,
            " {} Query should have correct implementation of concatenated fields to work properly. Selection: {}",
            contain_checker, contain_expected_query
        );
    }

}
