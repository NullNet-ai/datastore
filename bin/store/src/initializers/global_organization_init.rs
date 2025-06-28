use crate::controllers::store_controller::ApiError;
use crate::organizations::{
    organization_service,
    structs::{AccountType, Register},
};

pub struct GlobalOrganizationInitializer;

impl GlobalOrganizationInitializer {
    pub fn new() -> Self {
        GlobalOrganizationInitializer
    }

    pub async fn initialize(
        &self,
        _params: Option<crate::initializers::structs::InitializerParams>,
    ) -> Result<(), ApiError> {
        // Get environment variables
        let default_organization_id = std::env::var("DEFAULT_ORGANIZATION_ID")
            .unwrap_or_else(|_| "01JBHKXHYSKPP247HZZWHA3JCT".to_string());
        let default_organization_name = std::env::var("DEFAULT_ORGANIZATION_NAME")
            .unwrap_or_else(|_| "global-organization".to_string());
        let default_organization_admin_password =
            std::env::var("DEFAULT_ORGANIZATION_ADMIN_PASSWORD")
                .unwrap_or_else(|_| "ch@ng3m3Pl3@s3!!".to_string());

        // Create default account setup
        let default_account_setup = Register {
            account_type: Some(AccountType::Contact),
            organization_id: Some(default_organization_id.clone()),
            organization_name: Some(default_organization_name.clone()),
            account_id: "superadmin@dnamicro.com".to_string(),
            account_secret: default_organization_admin_password,
            first_name: "Super".to_string(),
            last_name: "Admin".to_string(),
            is_new_user: Some(true),
            account_status: Some("Active".to_string()),
            contact_categories: Some(vec!["Contact".to_string(), "User".to_string()]),
            role_id: Some("super_admin".to_string()),
            // Initialize other fields with None/default values
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
            account_organization_categories: None,
            account_organization_id: None,
            device_categories: None,
            responsible_account_organization_id: None,
        };

        // Call the existing initialize function with our custom setup
        organization_service::initialize(Some(default_account_setup)).await
    }
}

pub fn get_global_organization_initializer() -> GlobalOrganizationInitializer {
    GlobalOrganizationInitializer::new()
}
