use crate::controllers::store_controller::ApiError;
use crate::initializers::initial_entity_data::init::get_initial_entity_data_initializer;
use crate::initializers::system_initialization::background_services_init::get_background_services_initializer;
use crate::initializers::system_initialization::code_prefix_init::get_code_prefix_initializer;
use crate::initializers::system_initialization::generate_schema_init::get_generate_schema_initializer;
use crate::initializers::system_initialization::global_organization_init::get_global_organization_initializer;
use crate::initializers::system_initialization::root_account_init::get_root_account_initializer;
use crate::initializers::system_initialization::structs::{EInitializer, InitializerParams};
use crate::initializers::system_initialization::system_device_init::get_system_device_initializer;
use crate::providers::storage::cache::cache;

pub async fn initialize(
    initializer_type: EInitializer,
    params: Option<InitializerParams>,
) -> Result<(), ApiError> {
    cache.get("test");

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
            get_global_organization_initializer()
                .initialize(params)
                .await
        }
        EInitializer::SYSTEM_DEVICE_CONFIG => {
            // Initialize system device
            get_system_device_initializer().initialize(params).await
        }
        EInitializer::BACKGROUND_SERVICES_CONFIG => {
            // Initialize background services
            get_background_services_initializer()
                .initialize(params)
                .await
        }
        EInitializer::INITIAL_ENTITY_DATA_CONFIG => {
            // Initialize initial entity data
            get_initial_entity_data_initializer()
                .initialize(params)
                .await
        }
        EInitializer::GENERATE_SCHEMA_CONFIG => {
            // Initialize generate schema
            get_generate_schema_initializer()
                .initialize(params)
                .await
        }
    }
}
#[allow(warnings)]
pub async fn initialize_all(params: Option<InitializerParams>) -> Result<(), ApiError> {
    // Initialize code prefix first
    initialize(EInitializer::SYSTEM_CODE_CONFIG, params.clone()).await?;

    // Initialize generate schema if needed
    let initialize_schema = std::env::var("INITIALIZE_SCHEMA")
        .unwrap_or_else(|_| "false".to_string())
        .to_lowercase()
        == "true";

    if initialize_schema {
        initialize(EInitializer::GENERATE_SCHEMA_CONFIG, params.clone()).await?;
    }

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
    initialize(EInitializer::ROOT_ACCOUNT_CONFIG, params.clone()).await?;

    // Initialize initial entity data if needed
    let initialize_entity_data = std::env::var("INITIALIZE_ENTITY_DATA")
        .unwrap_or_else(|_| "false".to_string())
        .to_lowercase()
        == "true";

    if initialize_entity_data {
        initialize(EInitializer::INITIAL_ENTITY_DATA_CONFIG, params.clone()).await?;
    }

    // Finally initialize background services
    initialize(EInitializer::BACKGROUND_SERVICES_CONFIG, params).await?;

    Ok(())
}
