use crate::controllers::store_controller::ApiError;
use crate::providers::operations::organizations::organization_service;
use crate::providers::operations::organizations::structs::{AccountType, Register};

pub struct SystemDeviceInitializer;

impl SystemDeviceInitializer {
    pub fn new() -> Self {
        SystemDeviceInitializer
    }

    pub async fn initialize(
        &self,
        _params: Option<crate::initializers::system_initialization::structs::InitializerParams>,
    ) -> Result<(), ApiError> {
        // Get environment variables
        let default_organization_id = std::env::var("DEFAULT_ORGANIZATION_ID")
            .unwrap_or_else(|_| "01JBHKXHYSKPP247HZZWHA3JCT".to_string());
        let default_organization_name = std::env::var("DEFAULT_ORGANIZATION_NAME")
            .unwrap_or_else(|_| "global-organization".to_string());
        let default_device_id =
            std::env::var("DEFAULT_DEVICE_ID").unwrap_or_else(|_| "system_device".to_string());
        let default_device_secret = std::env::var("DEFAULT_DEVICE_SECRET")
            .unwrap_or_else(|_| "ch@ng3m3Pl3@s3!!".to_string());

        // Static ID for system device's initial personal organization (never changes across setups).
        const SYSTEM_DEVICE_PERSONAL_ORGANIZATION_ID: &str = "01JSN4XA2C3A7RHN3MNZZJGBR5";

        let default_device_setup = Register {
            account_type: Some(AccountType::Device),
            organization_id: Some(default_organization_id.clone()),
            organization_name: Some(default_organization_name.clone()),
            account_id: default_device_id,
            account_secret: default_device_secret,
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
            initial_personal_organization_id: Some(SYSTEM_DEVICE_PERSONAL_ORGANIZATION_ID.to_string()),
        };

        // Call the existing initialize function with our device setup
        organization_service::initialize(Some(default_device_setup)).await
    }
}

pub fn get_system_device_initializer() -> SystemDeviceInitializer {
    SystemDeviceInitializer::new()
}
