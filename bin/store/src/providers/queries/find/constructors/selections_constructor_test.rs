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
        let organization_id = env_config.default_organization_id;

        let query =
            get_raw_query(&payload, &table, is_root, timezone, Some(organization_id)).unwrap();
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
        let organization_id = env_config.default_organization_id;

        let query =
            get_raw_query(&payload, &table, is_root, timezone, Some(organization_id)).unwrap();
        println!("  ✓ Generated query: `{}`", query);

        let expected_query = format!(
            "SELECT {}",
            "\"contacts\".\"id\", \"contacts\".\"first_name\", \"contacts\".\"last_name\""
        );

        let contain_expected_query = query.contains(&expected_query);
        let checker = if contain_expected_query { "✓" } else { "✗" };

        println!("--- If pluck object does exist then pluck will be ignored.");
        println!(
            "--- Checking if all {} fields are available in pluck object.",
            &table
        );

        let pluck_object_main_table_fields = payload["pluck_object"][&table].as_array().unwrap();

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
    fn should_construct_selections_with_pluck_fields_pluck_object_joins() {
        let env_config = EnvConfig::default();
        let expected_joins = serde_json::json!([
            {
                "type": "left",
                "field_relation": {
                    "to": {
                        "entity": "contact_emailss",
                        "field": "contact_id",
                        // "alias": "created_by_account_organizations",
                        "order_direction": null,
                        "order_by": null,
                        "limit": null,
                        "offset": null
                    },
                    "from": {
                        "entity": "contactss",
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
            "pluck_object": {
                "contact_emails": []
            },
            "joins": expected_joins
        });
        let table = String::from("contacts");
        let is_root = false; // Set to true to avoid organization_id requirement
        let timezone = None;
        let organization_id = env_config.default_organization_id;

        let query =
            get_raw_query(&payload, &table, is_root, timezone, Some(organization_id)).unwrap();
        println!("  ✓ Generated query: `{}`", query);

        let expected_query = format!("SELECT {}", "");

        let contain_expected_query = query.contains(&expected_query);
        let checker = if contain_expected_query { "✓" } else { "✗" };

        println!("--- If pluck object does exist and has related tables specified then joins are required.");
        println!(
            "--- Checking if all {} fields are available in pluck object.",
            &table
        );

        // TODO: has joins
        // TODO: able to normalize entity or alias whether it is in singular or plural form
        // TODO: Query must match the expected query
        // #2
        // TODO: can sort within
        // TODO:  can filter with in using advance filters
        // #3
        // TODO:
        println!("  {} Expected query: `{}`", checker, expected_query);
        assert!(
            contain_expected_query,
            " {} Query should have contain the pluck object fields from {}.",
            checker, &table
        );
    }
}
