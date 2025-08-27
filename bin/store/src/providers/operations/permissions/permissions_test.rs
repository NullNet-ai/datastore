#[cfg(test)]
mod tests {
    use super::super::{permission_utils::*, permissions_queries::*, structs::*};
    use crate::providers::operations::auth::structs::{Session, User};
    use crate::utils::request_type_handler::RequestType;
    use actix_web::http::header::{HeaderMap, HeaderName, HeaderValue};
    use actix_web::http::{Method, Uri};
    use serde_json::{json, Value};
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

    #[test]
    fn test_data_permissions_default() {
        let permissions = DataPermissions::default();

        assert!(permissions.requested_fields.is_empty());
        assert_eq!(permissions.account_organization_id, "");
        assert!(permissions.schema.is_empty());
    }

    #[test]
    fn test_schema_item_creation() {
        let schema_item = SchemaItem {
            entity: "users".to_string(),
            alias: "u".to_string(),
            field: "email".to_string(),
            property_name: "email".to_string(),
            path: "u.email".to_string(),
        };

        assert_eq!(schema_item.entity, "users");
        assert_eq!(schema_item.alias, "u");
        assert_eq!(schema_item.field, "email");
        assert_eq!(schema_item.property_name, "email");
        assert_eq!(schema_item.path, "u.email");
    }

    #[test]
    fn test_permission_query_params_default() {
        let params = PermissionQueryParams::default();

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
    }

    #[test]
    fn test_get_permissions_query_generation() {
        let tables = vec!["users".to_string(), "roles".to_string()];
        let main_fields = vec!["id".to_string(), "name".to_string()];
        let sensitivity_level = 2;
        let account_organization_id = "org_123".to_string();

        let query = get_permissions_query(
            &tables,
            &main_fields,
            sensitivity_level,
            &account_organization_id,
        );

        assert!(query.contains("SELECT"));
        assert!(query.contains("data_permissions"));
        assert!(query.contains("p.id as permission_id"));
        assert!(query.contains("entities.name as entity"));
        assert!(query.contains("fields.name as field"));
    }

    #[test]
    fn test_get_valid_pass_keys_query_generation() {
        let organization_id = "org_123";
        let table = "users";
        let pgp_sym_key = "test_key";

        let query = get_valid_pass_keys_query(organization_id, table, pgp_sym_key);

        assert!(query.contains("SELECT"));
        assert!(query.contains("id"));
        assert!(query.contains(organization_id));
        assert!(query.contains(table));
        assert!(query.contains(pgp_sym_key));
    }

    #[test]
    fn test_get_group_by_field_record_permissions_query() {
        let table = "users";
        let role_id = "admin";

        let query = get_group_by_field_record_permissions(table, role_id);

        assert!(query.contains("SELECT"));
        assert!(query.contains("COUNT"));
        assert!(query.contains("GROUP BY"));
        assert!(query.contains(table));
        assert!(query.contains(role_id));
    }

    #[test]
    fn test_get_role_permissions_query_generation() {
        let role_id = "admin";

        let query = get_role_permissions_query(role_id);

        assert!(query.contains("SELECT"));
        assert!(query.contains("role_permissions"));
        assert!(query.contains(role_id));
    }

    #[test]
    fn test_permission_query_result_creation() {
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

        assert_eq!(result.permission_id, Some("perm_123".to_string()));
        assert_eq!(result.role, Some("admin".to_string()));
        assert_eq!(result.sensitivity_level, Some(2));
        assert_eq!(result.read, Some(true));
        assert_eq!(result.write, Some(false));
    }

    #[test]
    fn test_valid_pass_key_result_creation() {
        let result = ValidPassKeyResult {
            id: "key_123".to_string(),
        };

        assert_eq!(result.id, "key_123");
    }

    #[test]
    fn test_group_by_field_record_permissions_result_creation() {
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

        assert_eq!(result.role, Some("admin".to_string()));
        assert_eq!(result.entity, "users");
        assert_eq!(result.total_fields, 10);
        assert_eq!(result.total_fields_with_write, 5);
        assert!(result.sensitive);
        assert!(result.read);
        assert!(!result.write);
    }

    #[test]
    fn test_role_permission_result_creation() {
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

        assert_eq!(result.pid, Some("perm_123".to_string()));
        assert_eq!(result.role, Some("admin".to_string()));
        assert_eq!(result.sensitivity_level, Some(3));
        assert_eq!(result.read, Some(true));
        assert_eq!(result.write, Some(true));
    }

    #[test]
    fn test_permissions_context_creation() {
        let context = create_test_permissions_context();

        assert_eq!(context.table, "users");
        assert_eq!(context.account_organization_id, "org_123");
        assert_eq!(context.account_id, "user_123");
        assert_eq!(context.session.user.role_id, "admin");
        assert_eq!(context.host, "localhost:8080");
        assert_eq!(context.method, Method::GET);
    }

    #[test]
    fn test_permission_query_params_variants() {
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
    }

    #[test]
    fn test_permission_query_type_enum() {
        // Test that all enum variants can be created
        let permissions_type = PermissionQueryType::Permissions;
        let valid_keys_type = PermissionQueryType::ValidPassKeys;
        let group_type = PermissionQueryType::GroupByFieldRecordPermissions;
        let role_type = PermissionQueryType::RolePermissions;

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
    }

    #[test]
    fn test_data_permissions_with_schema() {
        let mut permissions = DataPermissions::default();

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

        assert_eq!(permissions.schema.len(), 2);
        assert_eq!(permissions.schema[0].entity, "users");
        assert_eq!(permissions.schema[0].field, "id");
        assert_eq!(permissions.schema[1].field, "email");
    }

    #[test]
    fn test_data_permissions_serialization() {
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

        // Test that the struct can be serialized to JSON
        let json_result = serde_json::to_string(&permissions);
        assert!(json_result.is_ok());

        let json_str = json_result.unwrap();
        assert!(json_str.contains("requested_fields"));
        assert!(json_str.contains("account_organization_id"));
    }

    #[test]
    fn test_schema_item_serialization() {
        let schema_item = SchemaItem {
            entity: "users".to_string(),
            alias: "u".to_string(),
            field: "email".to_string(),
            property_name: "user_email".to_string(),
            path: "u.email".to_string(),
        };

        // Test that the struct can be serialized to JSON
        let json_result = serde_json::to_string(&schema_item);
        assert!(json_result.is_ok());

        let json_str = json_result.unwrap();
        assert!(json_str.contains("entity"));
        assert!(json_str.contains("users"));
        assert!(json_str.contains("email"));
    }

    #[test]
    fn test_permission_query_result_optional_fields() {
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

        assert_eq!(result.permission_id, None);
        assert_eq!(result.account_organization_id, Some("org_123".to_string()));
        assert_eq!(result.role, Some("user".to_string()));
        assert_eq!(result.read, Some(true));
        assert_eq!(result.write, None);
    }
}
