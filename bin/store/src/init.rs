use crate::{
    controllers::store_controller::ApiError, initializers::{init::initialize, structs::{EInitializer, InitializerParams}}, organizations::{
        organization_service,
        structs::{AccountType, Register},
    }

};

/// Initialize both organization and device in a single function
pub async fn initialize_services() -> Result<(), ApiError> {
    // Initialize organization
    if let Err(e) = initialize_organization().await {
        log::error!("Failed to initialize organization: {}", e);
        return Err(e);
    } else {
        log::info!("Organization initialized successfully");
    }

     // Initialize root account
     if let Err(e) = initialize(EInitializer::ROOT_ACCOUNT_CONFIG, Some(InitializerParams {
        entity: "account_organizations".to_string(),
        system_code_config: None,
        root_account_config: None,
    })).await {
        log::error!("Failed to initialize root account: {}", e);
        return Err(e);
    } else {
        log::info!("Root account initialized successfully");
    }

    Ok(())
}

/// Initialize the organization with custom settings
async fn initialize_organization() -> Result<(), ApiError> {
    // Get environment variables
    let default_organization_id = std::env::var("DEFAULT_ORGANIZATION_ID")
        .unwrap_or_else(|_| "01JBHKXHYSKPP247HZZWHA3JCT".to_string());
    let default_organization_name = std::env::var("DEFAULT_ORGANIZATION_NAME")
        .unwrap_or_else(|_| "global-organization".to_string());
    let default_organization_admin_password = std::env::var("DEFAULT_ORGANIZATION_ADMIN_PASSWORD")
        .unwrap_or_else(|_| "ch@ng3m3Pl3@s3!!".to_string());
    let default_device_id =
        std::env::var("DEFAULT_DEVICE_ID").unwrap_or_else(|_| "system_device".to_string());
    let default_device_secret =
        std::env::var("DEFAULT_DEVICE_SECRET").unwrap_or_else(|_| "ch@ng3m3Pl3@s3!!".to_string());
    let initialize_device = std::env::var("INITIALIZE_DEVICE")
        .unwrap_or_else(|_| "false".to_string())
        .to_lowercase()
        == "true";

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
    };

    // Call the existing initialize function with our custom setup
    if initialize_device {
        organization_service::initialize(Some(default_device_setup)).await?;
    }
    organization_service::initialize(Some(default_account_setup)).await
}
