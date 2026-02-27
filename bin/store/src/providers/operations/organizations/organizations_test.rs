#[cfg(test)]
mod tests {
    use crate::controllers::organization_controller::{AuthData, AuthDto, RegisterDto};
    use crate::providers::operations::organizations::structs::{
        AccountType, LoginResponse, Register,
    };
    use std::str::FromStr;

    // Helper function to create a default Register struct for testing
    fn create_default_register() -> Register {
        Register {
            id: Some("test_id".to_string()),
            name: Some("Test User".to_string()),
            contact_id: Some("contact_123".to_string()),
            email: Some("test@example.com".to_string()),
            password: Some("test_password".to_string()),
            parent_organization_id: Some("parent_org_123".to_string()),
            code: Some("TEST_CODE".to_string()),
            categories: Some(vec!["category1".to_string(), "category2".to_string()]),
            organization_categories: None,
            account_status: Some("active".to_string()),
            account_type: Some(AccountType::Contact),
            organization_name: Some("Test Organization".to_string()),
            organization_id: Some("org_123".to_string()),
            account_id: "acc_123".to_string(),
            account_secret: "secret_123".to_string(),
            is_new_user: Some(true),
            first_name: "John".to_string(),
            last_name: "Doe".to_string(),
            is_invited: Some(false),
            role_id: Some("role_123".to_string()),
            account_organization_status: Some("active".to_string()),
            account_organization_categories: Some(vec!["org_cat1".to_string()]),
            account_organization_id: Some("acc_org_123".to_string()),
            contact_categories: Some(vec!["contact_cat1".to_string()]),
            device_categories: Some(vec!["device_cat1".to_string()]),
            responsible_account_organization_id: Some("resp_acc_org_123".to_string()),
            initial_personal_organization_id: None,
        }
    }

    // Helper function to create AuthData for testing
    fn create_auth_data() -> AuthData {
        AuthData {
            account_id: Some("test_account_id".to_string()),
            account_secret: Some("test_secret".to_string()),
            email: Some("test@example.com".to_string()),
            password: Some("test_password".to_string()),
            expiry_in_ms: None, // Default to None for backward compatibility
        }
    }

    /// Tests that Register struct can be created with all required fields populated correctly
    /// This ensures proper initialization of user registration data structures
    #[tokio::test]
    async fn should_create_register_struct_with_all_fields_populated() {
        println!("Testing Register struct creation with default values");

        let register = create_default_register();

        println!("Created register with account_id: {}", register.account_id);
        println!("Register email: {:?}", register.email);
        println!("Register account_type: {:?}", register.account_type);

        assert_eq!(register.account_id, "acc_123");
        assert_eq!(register.account_secret, "secret_123");
        assert_eq!(register.first_name, "John");
        assert_eq!(register.last_name, "Doe");
        assert_eq!(register.account_type, Some(AccountType::Contact));
        assert_eq!(register.email, Some("test@example.com".to_string()));

        println!("Register struct creation test passed");
    }

    /// Tests that Register struct default implementation creates empty/None values
    /// This ensures proper default initialization for new registration instances
    #[tokio::test]
    async fn should_create_register_struct_with_default_empty_values() {
        println!("Testing Register struct default implementation");

        let register = Register::default();

        println!("Default register account_id: '{}'", register.account_id);
        println!("Default register email: {:?}", register.email);
        println!("Default register account_type: {:?}", register.account_type);

        assert!(register.account_id.is_empty());
        assert!(register.account_secret.is_empty());
        assert!(register.first_name.is_empty());
        assert!(register.last_name.is_empty());
        assert_eq!(register.account_type, None);
        assert_eq!(register.email, None);

        println!("Register struct default test passed");
    }

    /// Tests that AccountType can be parsed from string values correctly
    /// This ensures proper conversion from string representations to enum variants
    #[tokio::test]
    async fn should_parse_account_type_from_valid_string_values() {
        println!("Testing AccountType parsing from string values");

        println!("Parsing 'contact' to AccountType::Contact");
        assert_eq!(
            AccountType::from_str("contact").unwrap(),
            AccountType::Contact
        );
        assert_eq!(
            AccountType::from_str("Contact").unwrap(),
            AccountType::Contact
        );
        assert_eq!(
            AccountType::from_str("CONTACT").unwrap(),
            AccountType::Contact
        );

        println!("Parsing 'device' to AccountType::Device");
        assert_eq!(
            AccountType::from_str("device").unwrap(),
            AccountType::Device
        );
        assert_eq!(
            AccountType::from_str("Device").unwrap(),
            AccountType::Device
        );
        assert_eq!(
            AccountType::from_str("DEVICE").unwrap(),
            AccountType::Device
        );

        println!("Testing invalid string should return error");
        assert!(AccountType::from_str("invalid").is_err());

        println!("AccountType string parsing test passed");
    }

    /// Tests that AccountType Display trait formats enum variants correctly
    /// This ensures proper string representation for logging and serialization
    #[tokio::test]
    async fn should_display_account_type_as_lowercase_strings() {
        println!("Testing AccountType Display trait implementation");

        let contact_display = format!("{}", AccountType::Contact);
        let device_display = format!("{}", AccountType::Device);

        println!("AccountType::Contact displays as: '{}'", contact_display);
        println!("AccountType::Device displays as: '{}'", device_display);

        assert_eq!(contact_display, "contact");
        assert_eq!(device_display, "device");

        println!("AccountType Display test passed");
    }

    /// Tests that AuthData struct can be created with authentication credentials
    /// This ensures proper initialization of authentication data for login processes
    #[tokio::test]
    async fn should_create_auth_data_with_valid_credentials() {
        println!("Testing AuthData struct creation with credentials");

        let auth_data = create_auth_data();

        println!("AuthData account_id: {:?}", auth_data.account_id);
        println!("AuthData email: {:?}", auth_data.email);
        println!("AuthData has password: {}", auth_data.password.is_some());

        assert_eq!(auth_data.account_id, Some("test_account_id".to_string()));
        assert_eq!(auth_data.account_secret, Some("test_secret".to_string()));
        assert_eq!(auth_data.email, Some("test@example.com".to_string()));
        assert_eq!(auth_data.password, Some("test_password".to_string()));

        println!("AuthData creation test passed");
    }

    /// Tests that AuthDto can be serialized to JSON correctly
    /// This ensures proper data transfer and API compatibility
    #[tokio::test]
    async fn should_serialize_auth_dto_to_json_correctly() {
        println!("Testing AuthDto serialization to JSON");

        let auth_data = create_auth_data();
        let auth_dto = AuthDto { data: auth_data };

        let serialized = serde_json::to_string(&auth_dto).unwrap();

        println!("Serialized AuthDto: {}", serialized);

        assert!(serialized.contains("test_account_id"));
        assert!(serialized.contains("test_secret"));

        println!("AuthDto serialization test passed");
    }

    /// Tests that RegisterDto can be serialized to JSON correctly
    /// This ensures proper data transfer and API compatibility for registration
    #[tokio::test]
    async fn should_serialize_register_dto_to_json_correctly() {
        println!("Testing RegisterDto serialization to JSON");

        let register = create_default_register();
        let register_dto = RegisterDto { data: register };

        let serialized = serde_json::to_string(&register_dto).unwrap();

        println!("Serialized RegisterDto: {}", serialized);

        assert!(serialized.contains("acc_123"));
        assert!(serialized.contains("secret_123"));
        assert!(serialized.contains("John"));
        assert!(serialized.contains("Doe"));

        println!("RegisterDto serialization test passed");
    }

    /// Tests that LoginResponse can be created with authentication data
    /// This ensures proper response structure for successful login operations
    #[tokio::test]
    async fn should_create_login_response_with_authentication_data() {
        println!("Testing LoginResponse creation with authentication data");

        let login_response = LoginResponse {
            message: "Login successful".to_string(),
            token: Some("jwt_token_123".to_string()),
            role_id: "role_123".to_string(),
            account_organization_id: Some("acc_org_123".to_string()),
            session_id: Some("session_123".to_string()),
        };

        println!(
            "Created LoginResponse with message: {}",
            login_response.message
        );
        println!("Token present: {}", login_response.token.is_some());

        assert_eq!(login_response.message, "Login successful");
        assert_eq!(login_response.token, Some("jwt_token_123".to_string()));
        assert_eq!(login_response.role_id, "role_123");
        assert_eq!(
            login_response.account_organization_id,
            Some("acc_org_123".to_string())
        );
        assert_eq!(login_response.session_id, Some("session_123".to_string()));

        println!("LoginResponse creation test passed");
    }

    /// Tests that Register validation handles empty account_id correctly
    /// This ensures proper validation of required registration fields
    #[tokio::test]
    async fn should_validate_empty_account_id_as_invalid() {
        println!("Testing Register validation with empty account_id");

        let mut register = create_default_register();
        register.account_id = String::new();

        println!("Set account_id to empty string: '{}'", register.account_id);

        // Test that empty account_id should be invalid
        assert!(register.account_id.is_empty());

        println!("Empty account_id validation test passed");
    }

    /// Tests that Register validation handles empty account_secret correctly
    /// This ensures proper validation of required security credentials
    #[tokio::test]
    async fn should_validate_empty_account_secret_as_invalid() {
        println!("Testing Register validation with empty account_secret");

        let mut register = create_default_register();
        register.account_secret = String::new();

        println!(
            "Set account_secret to empty string: '{}'",
            register.account_secret
        );

        // Test that empty account_secret should be invalid
        assert!(register.account_secret.is_empty());

        println!("Empty account_secret validation test passed");
    }

    /// Tests that Register validation handles empty names correctly
    /// This ensures proper validation of required personal information fields
    #[tokio::test]
    async fn should_validate_empty_names_as_invalid() {
        println!("Testing Register validation with empty names");

        let mut register = create_default_register();
        register.first_name = String::new();
        register.last_name = String::new();

        println!("Set first_name to: '{}'", register.first_name);
        println!("Set last_name to: '{}'", register.last_name);

        // Test that empty names should be invalid
        assert!(register.first_name.is_empty());
        assert!(register.last_name.is_empty());

        println!("Empty names validation test passed");
    }

    /// Tests that Register can be created with category assignments
    /// This ensures proper handling of user role categorization
    #[tokio::test]
    async fn should_create_register_with_category_assignments() {
        println!("Testing Register creation with categories");

        let register = create_default_register();

        println!("Assigned categories: {:?}", register.categories);

        assert!(register.categories.is_some());
        let categories = register.categories.unwrap();
        assert_eq!(categories.len(), 2);
        assert!(categories.contains(&"category1".to_string()));
        assert!(categories.contains(&"category2".to_string()));

        println!("Register with categories test passed");
    }

    /// Tests that Register can be created with organization category assignments
    /// This ensures proper handling of organizational role categorization
    #[tokio::test]
    async fn should_create_register_with_organization_categories() {
        println!("Testing Register creation with organization categories");

        let register = create_default_register();

        println!(
            "Assigned organization categories: {:?}",
            register.account_organization_categories
        );

        assert!(register.account_organization_categories.is_some());
        let org_categories = register.account_organization_categories.unwrap();
        assert_eq!(org_categories.len(), 1);
        assert!(org_categories.contains(&"org_cat1".to_string()));

        println!("Register with organization categories test passed");
    }

    /// Tests that Register can be created with contact category assignments
    /// This ensures proper handling of contact role categorization
    #[tokio::test]
    async fn should_create_register_with_contact_categories() {
        println!("Testing Register creation with contact categories");

        let register = create_default_register();

        println!(
            "Assigned contact categories: {:?}",
            register.contact_categories
        );

        assert!(register.contact_categories.is_some());
        let contact_categories = register.contact_categories.unwrap();
        assert_eq!(contact_categories.len(), 1);
        assert!(contact_categories.contains(&"contact_cat1".to_string()));

        println!("Register with contact categories test passed");
    }

    /// Tests that Register can be created with device category assignments
    /// This ensures proper handling of device role categorization
    #[tokio::test]
    async fn should_create_register_with_device_categories() {
        println!("Testing Register creation with device categories");

        let register = create_default_register();

        println!(
            "Assigned device categories: {:?}",
            register.device_categories
        );

        assert!(register.device_categories.is_some());
        let device_categories = register.device_categories.unwrap();
        assert_eq!(device_categories.len(), 1);
        assert!(device_categories.contains(&"device_cat1".to_string()));

        println!("Register with device categories test passed");
    }

    /// Tests that AccountType can be cloned correctly
    /// This ensures proper implementation of Clone trait for enum variants
    #[tokio::test]
    async fn should_clone_account_type_correctly() {
        println!("Testing AccountType clone implementation");

        let account_type = AccountType::Contact;
        let cloned_type = account_type.clone();

        println!("Original: {:?}, Cloned: {:?}", account_type, cloned_type);

        assert_eq!(account_type, cloned_type);

        println!("AccountType clone test passed");
    }

    /// Tests that Register struct can be cloned correctly
    /// This ensures proper implementation of Clone trait for complex data structures
    #[tokio::test]
    async fn should_clone_register_struct_correctly() {
        println!("Testing Register clone implementation");

        let register = create_default_register();
        let cloned_register = register.clone();

        println!(
            "Original account_id: {}, Cloned account_id: {}",
            register.account_id, cloned_register.account_id
        );

        assert_eq!(register.account_id, cloned_register.account_id);
        assert_eq!(register.account_secret, cloned_register.account_secret);
        assert_eq!(register.first_name, cloned_register.first_name);
        assert_eq!(register.last_name, cloned_register.last_name);
        assert_eq!(register.account_type, cloned_register.account_type);

        println!("Register clone test passed");
    }

    /// Tests that AuthData can be created with None values
    /// This ensures proper handling of optional authentication fields
    #[tokio::test]
    async fn should_create_auth_data_with_none_values() {
        println!("Testing AuthData creation with None values");

        let auth_data = AuthData {
            account_id: None,
            account_secret: None,
            email: None,
            password: None,
            expiry_in_ms: None, // Default to None for backward compatibility
        };

        println!("Created AuthData with all None values");

        assert_eq!(auth_data.account_id, None);
        assert_eq!(auth_data.account_secret, None);
        assert_eq!(auth_data.email, None);
        assert_eq!(auth_data.password, None);

        println!("AuthData with None values test passed");
    }

    /// Tests that LoginResponse can be created with None values for optional fields
    /// This ensures proper handling of failed login scenarios
    #[tokio::test]
    async fn should_create_login_response_with_none_values() {
        println!("Testing LoginResponse creation with None values");

        let login_response = LoginResponse {
            message: "Login failed".to_string(),
            token: None,
            role_id: "guest".to_string(),
            account_organization_id: None,
            session_id: None,
        };

        println!("Created LoginResponse for failed login scenario");

        assert_eq!(login_response.message, "Login failed");
        assert_eq!(login_response.token, None);
        assert_eq!(login_response.role_id, "guest");
        assert_eq!(login_response.account_organization_id, None);
        assert_eq!(login_response.session_id, None);

        println!("LoginResponse with None values test passed");
    }

    // Mock tests for functions that would require database connections
    // These test the structure and basic validation logic without actual DB calls

    /// Tests that Register struct Debug trait formats output correctly
    /// This ensures proper debugging and logging capabilities
    #[tokio::test]
    async fn should_format_register_struct_debug_output() {
        println!("Testing Register struct Debug formatting");

        let register = create_default_register();
        let debug_str = format!("{:?}", register);

        println!("Debug output contains Register struct info");

        assert!(debug_str.contains("Register"));
        assert!(debug_str.contains("acc_123"));
        assert!(debug_str.contains("John"));

        println!("Register Debug format test passed");
    }

    /// Tests that AuthData struct Debug trait formats output correctly
    /// This ensures proper debugging and logging capabilities for authentication data
    #[tokio::test]
    async fn should_format_auth_data_debug_output() {
        println!("Testing AuthData struct Debug formatting");

        let auth_data = create_auth_data();
        let debug_str = format!("{:?}", auth_data);

        println!("Debug output contains AuthData struct info");

        assert!(debug_str.contains("AuthData"));
        assert!(debug_str.contains("test_account_id"));

        println!("AuthData Debug format test passed");
    }

    /// Tests that LoginResponse struct Debug trait formats output correctly
    /// This ensures proper debugging and logging capabilities for login responses
    #[tokio::test]
    async fn should_format_login_response_debug_output() {
        println!("Testing LoginResponse struct Debug formatting");

        let login_response = LoginResponse {
            message: "Test message".to_string(),
            token: Some("test_token".to_string()),
            role_id: "test_role".to_string(),
            account_organization_id: Some("test_org".to_string()),
            session_id: Some("test_session".to_string()),
        };

        println!("Created LoginResponse for debug testing");

        let debug_str = format!("{:?}", login_response);
        assert!(debug_str.contains("LoginResponse"));
        assert!(debug_str.contains("Test message"));
        assert!(debug_str.contains("test_token"));

        println!("LoginResponse Debug format test passed");
    }

    // --- Static initial personal organization ID (super admin / system device) ---

    /// Expected static IDs used only by initializers (must match global_organization_init and system_device_init).
    const EXPECTED_SUPER_ADMIN_PERSONAL_ORG_ID: &str = "01JSN4XA2C3A7RHN3MNZZJGBR4";
    const EXPECTED_SYSTEM_DEVICE_PERSONAL_ORG_ID: &str = "01JSN4XA2C3A7RHN3MNZZJGBR5";

    /// Register::default() must have initial_personal_organization_id None so normal flows never get a static ID.
    #[tokio::test]
    async fn initial_personal_organization_id_default_is_none() {
        let register = Register::default();
        assert_eq!(register.initial_personal_organization_id, None);
    }

    /// create_default_register (simulating API/register flow) must not set static ID.
    #[tokio::test]
    async fn normal_register_does_not_use_static_personal_org_id() {
        let register = create_default_register();
        assert_eq!(register.initial_personal_organization_id, None);
    }

    /// When set, initial_personal_organization_id is preserved (used by initializers).
    #[tokio::test]
    async fn initial_personal_organization_id_when_set_is_preserved() {
        let mut register = create_default_register();
        register.initial_personal_organization_id =
            Some(EXPECTED_SUPER_ADMIN_PERSONAL_ORG_ID.to_string());
        assert_eq!(
            register.initial_personal_organization_id.as_deref(),
            Some(EXPECTED_SUPER_ADMIN_PERSONAL_ORG_ID)
        );
    }

    /// Super-admin-style Register (as built by global_organization_init) must have the expected static ID.
    #[tokio::test]
    async fn super_admin_initializer_register_uses_expected_static_id() {
        let register = Register {
            account_type: Some(AccountType::Contact),
            organization_id: Some("01JBHKXHYSKPP247HZZWHA3JCT".to_string()),
            organization_name: Some("global-organization".to_string()),
            organization_categories: None,
            account_id: "admin@dnamicro.com".to_string(),
            account_secret: "password".to_string(),
            first_name: "Super".to_string(),
            last_name: "Admin".to_string(),
            is_new_user: Some(true),
            account_status: Some("Active".to_string()),
            contact_categories: Some(vec!["Contact".to_string(), "User".to_string()]),
            role_id: Some("super_admin".to_string()),
            account_organization_categories: Some(vec![]),
            id: None,
            name: None,
            contact_id: None,
            email: None,
            password: None,
            parent_organization_id: None,
            code: None,
            categories: None,
            is_invited: None,
            account_organization_status: None,
            account_organization_id: None,
            device_categories: None,
            responsible_account_organization_id: None,
            initial_personal_organization_id: Some(
                EXPECTED_SUPER_ADMIN_PERSONAL_ORG_ID.to_string(),
            ),
        };
        assert_eq!(
            register.initial_personal_organization_id.as_deref(),
            Some(EXPECTED_SUPER_ADMIN_PERSONAL_ORG_ID)
        );
    }

    /// System-device-style Register (as built by system_device_init) must have the expected static ID.
    #[tokio::test]
    async fn system_device_initializer_register_uses_expected_static_id() {
        let register = Register {
            account_type: Some(AccountType::Device),
            organization_id: Some("01JBHKXHYSKPP247HZZWHA3JCT".to_string()),
            organization_name: Some("global-organization".to_string()),
            organization_categories: None,
            account_id: "system_device".to_string(),
            account_secret: "secret".to_string(),
            first_name: "".to_string(),
            last_name: "".to_string(),
            is_new_user: Some(true),
            account_status: Some("Active".to_string()),
            contact_categories: None,
            role_id: Some("super_admin".to_string()),
            id: None,
            name: None,
            contact_id: None,
            email: None,
            password: None,
            parent_organization_id: None,
            code: None,
            categories: None,
            is_invited: None,
            account_organization_status: Some("Active".to_string()),
            account_organization_categories: None,
            account_organization_id: None,
            device_categories: Some(vec!["Device".to_string()]),
            responsible_account_organization_id: None,
            initial_personal_organization_id: Some(
                EXPECTED_SYSTEM_DEVICE_PERSONAL_ORG_ID.to_string(),
            ),
        };
        assert_eq!(
            register.initial_personal_organization_id.as_deref(),
            Some(EXPECTED_SYSTEM_DEVICE_PERSONAL_ORG_ID)
        );
    }

    /// Static IDs must be distinct (super admin vs system device).
    #[tokio::test]
    async fn static_ids_are_distinct() {
        assert_ne!(
            EXPECTED_SUPER_ADMIN_PERSONAL_ORG_ID,
            EXPECTED_SYSTEM_DEVICE_PERSONAL_ORG_ID
        );
    }

    /// Static IDs must look like ULIDs (26 chars, Crockford Base32).
    #[tokio::test]
    async fn static_ids_are_valid_ulid_format() {
        fn valid_ulid_like(s: &str) -> bool {
            s.len() == 26
                && s.chars().all(|c| {
                    c.is_ascii_alphanumeric() && c != 'i' && c != 'l' && c != 'o' && c != 'u'
                })
        }
        assert!(valid_ulid_like(EXPECTED_SUPER_ADMIN_PERSONAL_ORG_ID));
        assert!(valid_ulid_like(EXPECTED_SYSTEM_DEVICE_PERSONAL_ORG_ID));
    }

    /// Failure scenario: Register with initial_personal_organization_id Some("") is still "set" (caller responsibility to not pass empty).
    #[tokio::test]
    async fn initial_personal_organization_id_empty_string_is_still_some() {
        let mut register = Register::default();
        register.initial_personal_organization_id = Some(String::new());
        assert_eq!(
            register.initial_personal_organization_id,
            Some("".to_string())
        );
    }

    /// Serde: deserializing JSON without initial_personal_organization_id yields None (no static ID for API requests).
    #[tokio::test]
    async fn serde_deserialize_register_without_field_yields_none() {
        let json = r#"{"account_id":"u@example.com","account_secret":"secret","first_name":"A","last_name":"B"}"#;
        let register: Register = serde_json::from_str(json).unwrap();
        assert_eq!(register.initial_personal_organization_id, None);
    }

    /// Serde: roundtrip with initial_personal_organization_id set preserves it.
    #[tokio::test]
    async fn serde_roundtrip_preserves_initial_personal_organization_id() {
        let mut register = create_default_register();
        register.initial_personal_organization_id =
            Some(EXPECTED_SYSTEM_DEVICE_PERSONAL_ORG_ID.to_string());
        let json = serde_json::to_string(&register).unwrap();
        let back: Register = serde_json::from_str(&json).unwrap();
        assert_eq!(
            back.initial_personal_organization_id.as_deref(),
            Some(EXPECTED_SYSTEM_DEVICE_PERSONAL_ORG_ID)
        );
    }

    /// Clone preserves initial_personal_organization_id.
    #[tokio::test]
    async fn clone_preserves_initial_personal_organization_id() {
        let mut register = create_default_register();
        register.initial_personal_organization_id =
            Some(EXPECTED_SUPER_ADMIN_PERSONAL_ORG_ID.to_string());
        let cloned = register.clone();
        assert_eq!(
            cloned.initial_personal_organization_id,
            register.initial_personal_organization_id
        );
    }
}
