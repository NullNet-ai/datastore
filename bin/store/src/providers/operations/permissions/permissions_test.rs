#[cfg(test)]
mod tests {
    use super::super::{permission_utils::*, permissions_queries::*, structs::*};
    use crate::providers::operations::auth::structs::{Session, User};
    use actix_web::http::header::{HeaderMap, HeaderName, HeaderValue};
    use actix_web::http::{Method, Uri};
    use serde_json::json;
    use std::collections::HashMap;
    use std::str::FromStr;

    // Helper function to create a test session
    fn create_test_session() -> Session {
        use crate::providers::operations::auth::structs::{Cookie, Origin};

        Session {
            session_id: "test_session_123".to_string(),
            user: User {
                role_id: "admin".to_string(),
                is_root_user: false,
                account_id: "user_123".to_string(),
            },
            origin: Some(Origin {
                user_agent: Some("test-agent/1.0".to_string()),
                host: "localhost:8080".to_string(),
                url: "/api/users".to_string(),
            }),
            token: "test_token_123".to_string(),
            cookie: Cookie {
                path: "/".to_string(),
                expires: "2024-01-01T00:00:00Z".to_string(),
                originalMaxAge: 86400,
                httpOnly: true,
            },
            field_permissions: None,
            role_permissions: None,
            record_permissions: None,
            valid_pass_keys: None,
            ip_address: Some("127.0.0.1".to_string()),
            location: Some("Test Location".to_string()),
            browser_name: Some("Test Browser".to_string()),
            operating_system: Some("Test OS".to_string()),
            device_name: Some("Test Device".to_string()),
            account_organization_id: Some("org_123".to_string()),
        }
    }

    // Helper function to create test headers
    fn create_test_headers() -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.insert(
            HeaderName::from_static("user-agent"),
            HeaderValue::from_static("test-agent/1.0"),
        );
        headers
    }

    // Helper function to create test permissions context
    fn create_test_permissions_context() -> PermissionsContext {
        let data_permissions = DataPermissions {
            requested_fields: vec!["id".to_string(), "name".to_string()],
            data_permissions_query_params: PermissionQueryParams::DataPermissions {
                tables: vec!["users".to_string()],
                main_fields: vec!["id".to_string(), "name".to_string()],
                sensitivity_level: 1,
                account_organization_id: "org_123".to_string(),
            },
            account_organization_id: "org_123".to_string(),
            schema: vec![SchemaItem {
                entity: "users".to_string(),
                alias: "u".to_string(),
                field: "id".to_string(),
                property_name: "id".to_string(),
                path: "u.id".to_string(),
            }],
            valid_pass_keys_query_params: PermissionQueryParams::ValidPassKeys {
                organization_id: "org_123".to_string(),
                table: "users".to_string(),
                pgp_sym_key: "test_key".to_string(),
            },
            group_by_field_record_permissions_params:
                PermissionQueryParams::GroupByFieldRecordPermissions {
                    table: "users".to_string(),
                    role_id: "admin".to_string(),
                },
            role_permissions_query_params: PermissionQueryParams::RolePermissions {
                role_id: "admin".to_string(),
            },
        };

        PermissionsContext {
            permissions_query: data_permissions,
            host: "localhost:8080".to_string(),
            headers: create_test_headers(),
            table: "users".to_string(),
            account_organization_id: "org_123".to_string(),
            body: json!({"name": "test"}),
            metadata: HashMap::new(),
            account_id: "user_123".to_string(),
            query: HashMap::new(),
            method: Method::GET,
            uri: Uri::from_str("/api/users").unwrap(),
            session: create_test_session(),
            session_data: None,
        }
    }

    /// Tests that DataPermissions default implementation creates empty structures
    /// This ensures proper initialization of permission data with safe defaults
    #[test]
    fn should_create_data_permissions_with_empty_default_values() {
        println!("Testing DataPermissions default implementation");

        let permissions = DataPermissions::default();

        println!(
            "Default requested_fields length: {}",
            permissions.requested_fields.len()
        );
        println!(
            "Default account_organization_id: '{}'",
            permissions.account_organization_id
        );
        println!("Default schema length: {}", permissions.schema.len());

        assert!(permissions.requested_fields.is_empty());
        assert_eq!(permissions.account_organization_id, "");
        assert!(permissions.schema.is_empty());

        println!("DataPermissions default test passed");
    }

    /// Tests that SchemaItem can be created with database schema field information
    /// This ensures proper mapping between database fields and API properties
    #[test]
    fn should_create_schema_item_with_database_field_mapping() {
        println!("Testing SchemaItem creation with field mapping");

        let schema_item = SchemaItem {
            entity: "users".to_string(),
            alias: "u".to_string(),
            field: "email".to_string(),
            property_name: "email".to_string(),
            path: "u.email".to_string(),
        };

        println!("Schema item entity: {}", schema_item.entity);
        println!("Schema item alias: {}", schema_item.alias);
        println!("Schema item field: {}", schema_item.field);
        println!("Schema item path: {}", schema_item.path);

        assert_eq!(schema_item.entity, "users");
        assert_eq!(schema_item.alias, "u");
        assert_eq!(schema_item.field, "email");
        assert_eq!(schema_item.property_name, "email");
        assert_eq!(schema_item.path, "u.email");

        println!("SchemaItem creation test passed");
    }

    /// Tests that PermissionQueryParams can be created with default values
    /// This ensures proper initialization of permission query parameters
    #[test]
    fn should_create_permission_query_params_with_default_values() {
        println!("Testing PermissionQueryParams default implementation");

        let params = PermissionQueryParams::default();

        println!("Created default PermissionQueryParams");

        match params {
            PermissionQueryParams::DataPermissions {
                tables,
                main_fields,
                sensitivity_level,
                account_organization_id,
            } => {
                assert!(tables.is_empty());
                assert!(main_fields.is_empty());
                assert_eq!(sensitivity_level, 0);
                assert_eq!(account_organization_id, "");
            }
            _ => panic!("Expected DataPermissions variant"),
        }

        println!("PermissionQueryParams default values test passed");
    }

    /// Tests that permissions query is generated correctly for data access control
    /// This ensures proper SQL generation for database permission validation
    #[test]
    fn should_generate_valid_permissions_query_for_data_access() {
        println!("Testing permissions query generation for data access");

        let tables = vec!["users".to_string(), "roles".to_string()];
        let main_fields = vec!["id".to_string(), "name".to_string()];
        let sensitivity_level = 2;
        let account_organization_id = "org_123".to_string();

        println!("Generating query for tables: {:?}", tables);
        println!("Main fields: {:?}", main_fields);
        println!("Sensitivity level: {}", sensitivity_level);
        println!("Organization ID: {}", account_organization_id);

        let query = get_permissions_query(
            &tables,
            &main_fields,
            sensitivity_level,
            &account_organization_id,
        );

        println!("Generated query length: {}", query.len());
        println!("Query contains SELECT: {}", query.contains("SELECT"));
        println!(
            "Query contains data_permissions: {}",
            query.contains("data_permissions")
        );

        assert!(query.contains("SELECT"));
        assert!(query.contains("data_permissions"));
        assert!(query.contains("p.id as permission_id"));
        assert!(query.contains("entities.name as entity"));
        assert!(query.contains("fields.name as field"));

        println!("Permissions query generation test passed");
    }

    /// Tests that valid pass keys query is generated correctly
    /// This ensures proper SQL generation for pass key validation
    #[test]
    fn should_generate_valid_pass_keys_query_correctly() {
        println!("Testing valid pass keys query generation");

        let organization_id = "org_123";
        let table = "users";
        let pgp_sym_key = "test_key";

        println!(
            "Generating query for org: {}, table: {}",
            organization_id, table
        );

        let query = get_valid_pass_keys_query(organization_id, table, pgp_sym_key);

        println!("Generated query contains required elements");

        assert!(query.contains("SELECT"));
        assert!(query.contains("id"));
        assert!(query.contains(organization_id));
        assert!(query.contains(table));
        assert!(query.contains(pgp_sym_key));

        println!("Valid pass keys query generation test passed");
    }

    /// Tests that group by field record permissions query is generated correctly
    /// This ensures proper SQL generation for field-level record permissions
    #[test]
    fn should_generate_group_by_field_record_permissions_query_correctly() {
        println!("Testing group by field record permissions query generation");

        let table = "users";
        let role_id = "admin";

        println!("Generating query for table: {}, role: {}", table, role_id);

        let query = get_group_by_field_record_permissions(table, role_id);

        println!("Generated query contains required elements");

        assert!(query.contains("SELECT"));
        assert!(query.contains("COUNT"));
        assert!(query.contains("GROUP BY"));
        assert!(query.contains(table));
        assert!(query.contains(role_id));

        println!("Group by field record permissions query generation test passed");
    }

    /// Tests that role permissions query is generated correctly
    /// This ensures proper SQL generation for role-based permissions
    #[test]
    fn should_generate_role_permissions_query_correctly() {
        println!("Testing role permissions query generation");

        let role_id = "admin";

        println!("Generating query for role: {}", role_id);

        let query = get_role_permissions_query(role_id);

        println!("Generated query contains required elements");

        assert!(query.contains("SELECT"));
        assert!(query.contains("role_permissions"));
        assert!(query.contains(role_id));

        println!("Role permissions query generation test passed");
    }

    /// Tests that PermissionQueryResult can be created with all fields
    /// This validates the struct definition and field assignments
    #[test]
    fn should_create_permission_query_result_with_all_fields() {
        println!("Testing PermissionQueryResult creation with all fields");

        // Test that we can create a PermissionQueryResult with all fields
        // This tests the struct definition and serialization capabilities
        let result = PermissionQueryResult {
            permission_id: Some("perm_123".to_string()),
            entity_field_id: Some("field_123".to_string()),
            account_organization_id: Some("org_123".to_string()),
            id: Some("id_123".to_string()),
            role_permission_id: Some("role_perm_123".to_string()),
            record_id: Some("record_123".to_string()),
            record_entity: Some("users".to_string()),
            role: Some("admin".to_string()),
            sensitivity_level: Some(2),
            entity: Some("users".to_string()),
            field: Some("email".to_string()),
            is_encryptable: Some(true),
            is_system_field: Some(false),
            is_searchable: Some(true),
            is_allowed_to_return: Some(true),
            sensitive: Some(true),
            read: Some(true),
            write: Some(false),
            encrypt: Some(true),
            decrypt: Some(true),
            required: Some(false),
            archive: Some(false),
            delete: Some(false),
        };

        println!("Validating PermissionQueryResult field values");

        assert_eq!(result.permission_id, Some("perm_123".to_string()));
        assert_eq!(result.role, Some("admin".to_string()));
        assert_eq!(result.sensitivity_level, Some(2));
        assert_eq!(result.read, Some(true));
        assert_eq!(result.write, Some(false));

        println!("PermissionQueryResult creation test passed");
    }

    /// Tests that ValidPassKeyResult can be created correctly
    /// This validates the struct definition and field assignment
    #[test]
    fn should_create_valid_pass_key_result_correctly() {
        println!("Testing ValidPassKeyResult creation");

        let result = ValidPassKeyResult {
            id: "key_123".to_string(),
        };

        println!("Validating ValidPassKeyResult field value");

        assert_eq!(result.id, "key_123");

        println!("ValidPassKeyResult creation test passed");
    }

    /// Tests that GroupByFieldRecordPermissionsResult can be created correctly
    /// This validates the struct definition and field assignments
    #[test]
    fn should_create_group_by_field_record_permissions_result_correctly() {
        println!("Testing GroupByFieldRecordPermissionsResult creation");

        let result = GroupByFieldRecordPermissionsResult {
            role: Some("admin".to_string()),
            entity: "users".to_string(),
            total_fields: 10,
            total_fields_with_write: 5,
            sensitive: true,
            read: true,
            write: false,
            encrypt: true,
            decrypt: true,
            required: false,
            archive: false,
            delete: false,
        };

        println!("Validating GroupByFieldRecordPermissionsResult field values");

        assert_eq!(result.role, Some("admin".to_string()));
        assert_eq!(result.entity, "users");
        assert_eq!(result.total_fields, 10);
        assert_eq!(result.total_fields_with_write, 5);
        assert!(result.sensitive);
        assert!(result.read);
        assert!(!result.write);

        println!("GroupByFieldRecordPermissionsResult creation test passed");
    }

    /// Tests that RolePermissionResult can be created correctly
    /// This validates the struct definition and field assignments
    #[test]
    fn should_create_role_permission_result_correctly() {
        println!("Testing RolePermissionResult creation");

        let result = RolePermissionResult {
            pid: Some("perm_123".to_string()),
            role: Some("admin".to_string()),
            sensitivity_level: Some(3),
            sensitive: Some(true),
            read: Some(true),
            write: Some(true),
            encrypt: Some(true),
            decrypt: Some(true),
            required: Some(false),
            archive: Some(true),
            delete: Some(false),
        };

        println!("Validating RolePermissionResult field values");

        assert_eq!(result.pid, Some("perm_123".to_string()));
        assert_eq!(result.role, Some("admin".to_string()));
        assert_eq!(result.sensitivity_level, Some(3));
        assert_eq!(result.read, Some(true));
        assert_eq!(result.write, Some(true));

        println!("RolePermissionResult creation test passed");
    }

    /// Tests that PermissionsContext can be created correctly
    /// This validates the context structure and field assignments
    #[test]
    fn should_create_permissions_context_correctly() {
        println!("Testing PermissionsContext creation");

        let context = create_test_permissions_context();

        println!("Validating PermissionsContext field values");

        assert_eq!(context.table, "users");
        assert_eq!(context.account_organization_id, "org_123");
        assert_eq!(context.account_id, "user_123");
        assert_eq!(context.session.user.role_id, "admin");
        assert_eq!(context.host, "localhost:8080");
        assert_eq!(context.method, Method::GET);

        println!("PermissionsContext creation test passed");
    }

    /// Tests that all PermissionQueryParams variants can be created and matched correctly
    /// This validates the enum variants and their field assignments
    #[test]
    fn should_create_and_match_permission_query_params_variants_correctly() {
        println!("Testing PermissionQueryParams variants creation and matching");

        // Test DataPermissions variant
        let data_params = PermissionQueryParams::DataPermissions {
            tables: vec!["users".to_string()],
            main_fields: vec!["id".to_string()],
            sensitivity_level: 1,
            account_organization_id: "org_123".to_string(),
        };

        match data_params {
            PermissionQueryParams::DataPermissions { tables, .. } => {
                assert_eq!(tables, vec!["users".to_string()]);
            }
            _ => panic!("Expected DataPermissions variant"),
        }

        // Test ValidPassKeys variant
        let pass_key_params = PermissionQueryParams::ValidPassKeys {
            organization_id: "org_123".to_string(),
            table: "users".to_string(),
            pgp_sym_key: "key_123".to_string(),
        };

        match pass_key_params {
            PermissionQueryParams::ValidPassKeys {
                organization_id, ..
            } => {
                assert_eq!(organization_id, "org_123");
            }
            _ => panic!("Expected ValidPassKeys variant"),
        }

        // Test GroupByFieldRecordPermissions variant
        let group_params = PermissionQueryParams::GroupByFieldRecordPermissions {
            table: "users".to_string(),
            role_id: "admin".to_string(),
        };

        match group_params {
            PermissionQueryParams::GroupByFieldRecordPermissions { table, role_id } => {
                assert_eq!(table, "users");
                assert_eq!(role_id, "admin");
            }
            _ => panic!("Expected GroupByFieldRecordPermissions variant"),
        }

        // Test RolePermissions variant
        let role_params = PermissionQueryParams::RolePermissions {
            role_id: "admin".to_string(),
        };

        match role_params {
            PermissionQueryParams::RolePermissions { role_id } => {
                assert_eq!(role_id, "admin");
            }
            _ => panic!("Expected RolePermissions variant"),
        }

        println!("PermissionQueryParams variants test passed");
    }

    /// Tests that all PermissionQueryType enum variants can be created and matched
    /// This validates the enum definition and variant matching
    #[test]
    fn should_create_and_match_permission_query_type_enum_variants() {
        println!("Testing PermissionQueryType enum variants");

        // Test that all enum variants can be created
        let permissions_type = PermissionQueryType::Permissions;
        let valid_keys_type = PermissionQueryType::ValidPassKeys;
        let group_type = PermissionQueryType::GroupByFieldRecordPermissions;
        let role_type = PermissionQueryType::RolePermissions;

        println!("Testing enum variant matching");

        // These should compile without issues, demonstrating the enum works correctly
        match permissions_type {
            PermissionQueryType::Permissions => assert!(true),
            _ => assert!(false),
        }

        match valid_keys_type {
            PermissionQueryType::ValidPassKeys => assert!(true),
            _ => assert!(false),
        }

        match group_type {
            PermissionQueryType::GroupByFieldRecordPermissions => assert!(true),
            _ => assert!(false),
        }

        match role_type {
            PermissionQueryType::RolePermissions => assert!(true),
            _ => assert!(false),
        }

        println!("PermissionQueryType enum test passed");
    }

    /// Tests that DataPermissions can be populated with schema items
    /// This validates schema item addition and field mapping
    #[test]
    fn should_populate_data_permissions_with_schema_items() {
        println!("Testing DataPermissions with schema items");

        let mut permissions = DataPermissions::default();

        println!("Adding schema items to DataPermissions");

        permissions.schema.push(SchemaItem {
            entity: "users".to_string(),
            alias: "u".to_string(),
            field: "id".to_string(),
            property_name: "user_id".to_string(),
            path: "u.id".to_string(),
        });

        permissions.schema.push(SchemaItem {
            entity: "users".to_string(),
            alias: "u".to_string(),
            field: "email".to_string(),
            property_name: "user_email".to_string(),
            path: "u.email".to_string(),
        });

        println!("Validating schema items in DataPermissions");

        assert_eq!(permissions.schema.len(), 2);
        assert_eq!(permissions.schema[0].entity, "users");
        assert_eq!(permissions.schema[0].field, "id");
        assert_eq!(permissions.schema[1].field, "email");

        println!("DataPermissions with schema test passed");
    }

    /// Tests that DataPermissions can be serialized to JSON correctly
    /// This validates the serialization capabilities of the struct
    #[test]
    fn should_serialize_data_permissions_to_json_correctly() {
        println!("Testing DataPermissions JSON serialization");

        let permissions = DataPermissions {
            requested_fields: vec!["id".to_string(), "name".to_string()],
            data_permissions_query_params: PermissionQueryParams::DataPermissions {
                tables: vec!["users".to_string()],
                main_fields: vec!["id".to_string()],
                sensitivity_level: 1,
                account_organization_id: "org_123".to_string(),
            },
            account_organization_id: "org_123".to_string(),
            schema: vec![],
            valid_pass_keys_query_params: PermissionQueryParams::default(),
            group_by_field_record_permissions_params: PermissionQueryParams::default(),
            role_permissions_query_params: PermissionQueryParams::default(),
        };

        println!("Serializing DataPermissions to JSON");

        // Test that the struct can be serialized to JSON
        let json_result = serde_json::to_string(&permissions);
        assert!(json_result.is_ok());

        let json_str = json_result.unwrap();
        assert!(json_str.contains("requested_fields"));
        assert!(json_str.contains("account_organization_id"));

        println!("DataPermissions serialization test passed");
    }

    /// Tests that SchemaItem can be serialized to JSON correctly
    /// This validates the serialization capabilities of the struct
    #[test]
    fn should_serialize_schema_item_to_json_correctly() {
        println!("Testing SchemaItem JSON serialization");

        let schema_item = SchemaItem {
            entity: "users".to_string(),
            alias: "u".to_string(),
            field: "email".to_string(),
            property_name: "user_email".to_string(),
            path: "u.email".to_string(),
        };

        println!("Serializing SchemaItem to JSON");

        // Test that the struct can be serialized to JSON
        let json_result = serde_json::to_string(&schema_item);
        assert!(json_result.is_ok());

        let json_str = json_result.unwrap();
        assert!(json_str.contains("entity"));
        assert!(json_str.contains("users"));
        assert!(json_str.contains("email"));

        println!("SchemaItem serialization test passed");
    }

    /// Tests that PermissionQueryResult handles optional fields correctly
    /// This validates that the struct works properly with None values
    #[test]
    fn should_handle_permission_query_result_optional_fields_correctly() {
        println!("Testing PermissionQueryResult with optional fields");

        // Test that PermissionQueryResult works with None values
        let result = PermissionQueryResult {
            permission_id: None,
            entity_field_id: None,
            account_organization_id: Some("org_123".to_string()),
            id: None,
            role_permission_id: None,
            record_id: None,
            record_entity: None,
            role: Some("user".to_string()),
            sensitivity_level: None,
            entity: Some("users".to_string()),
            field: Some("name".to_string()),
            is_encryptable: None,
            is_system_field: None,
            is_searchable: None,
            is_allowed_to_return: None,
            sensitive: None,
            read: Some(true),
            write: None,
            encrypt: None,
            decrypt: None,
            required: None,
            archive: None,
            delete: None,
        };

        println!("Validating optional field values");

        assert_eq!(result.permission_id, None);
        assert_eq!(result.account_organization_id, Some("org_123".to_string()));
        assert_eq!(result.role, Some("user".to_string()));
        assert_eq!(result.read, Some(true));
        assert_eq!(result.write, None);

        println!("PermissionQueryResult optional fields test passed");
    }
}
