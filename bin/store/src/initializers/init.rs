use crate::controllers::store_controller::ApiError;
use crate::initializers::code_prefix_init::get_code_prefix_initializer;
use crate::initializers::root_account_init::get_root_account_initializer;
use crate::initializers::structs::{EInitializer, InitializerParams};

pub async fn initialize(initializer_type: EInitializer, params: Option<InitializerParams>) -> Result<(), ApiError> {
    match initializer_type {
        EInitializer::SYSTEM_CODE_CONFIG => {
            // Initialize code prefix
            Ok(())
        },
        EInitializer::ROOT_ACCOUNT_CONFIG => {
            // Initialize root account
            get_root_account_initializer().initialize(params).await
        },
    }
}

pub async fn initialize_all(params: Option<InitializerParams>) -> Result<(), ApiError> {
    // Initialize code prefix first
    initialize(EInitializer::SYSTEM_CODE_CONFIG, params.clone()).await?;
    
    // Then initialize root account
    initialize(EInitializer::ROOT_ACCOUNT_CONFIG, params).await?;
    
    Ok(())
}