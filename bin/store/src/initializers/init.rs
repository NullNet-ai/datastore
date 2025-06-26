use crate::controllers::store_controller::ApiError;
use crate::initializers::code_prefix_init::get_code_prefix_initializer;
use crate::initializers::global_organization_init::get_global_organization_initializer;
use crate::initializers::root_account_init::get_root_account_initializer;
use crate::initializers::structs::{EInitializer, InitializerParams};
use crate::initializers::system_device_init::get_system_device_initializer;

pub async fn initialize(
    initializer_type: EInitializer,
    params: Option<InitializerParams>,
) -> Result<(), ApiError> {
    match initializer_type {
        EInitializer::SYSTEM_CODE_CONFIG => {
            // Initialize code prefix
            get_code_prefix_initializer().initialize().await
        }
        EInitializer::ROOT_ACCOUNT_CONFIG => {
            // Initialize root account
            get_root_account_initializer().initialize(params).await
        }
        EInitializer::GLOBAL_ORGANIZATION_CONFIG => {
            // Initialize global organization
            get_global_organization_initializer().initialize(params).await
        }
        EInitializer::SYSTEM_DEVICE_CONFIG => {
            // Initialize system device
            get_system_device_initializer().initialize(params).await
        }
    }
}

pub async fn initialize_all(params: Option<InitializerParams>) -> Result<(), ApiError> {
    // Initialize code prefix first
    initialize(EInitializer::SYSTEM_CODE_CONFIG, params.clone()).await?;

    // Initialize global organization
    initialize(EInitializer::GLOBAL_ORGANIZATION_CONFIG, params.clone()).await?;

    // Initialize system device if needed
    let initialize_device = std::env::var("INITIALIZE_DEVICE")
        .unwrap_or_else(|_| "false".to_string())
        .to_lowercase()
        == "true";
    
    if initialize_device {
        initialize(EInitializer::SYSTEM_DEVICE_CONFIG, params.clone()).await?;
    }

    // Then initialize root account
    initialize(EInitializer::ROOT_ACCOUNT_CONFIG, params).await?;

    Ok(())
}