#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(warnings)]
pub enum EInitializer {
    SYSTEM_CODE_CONFIG,
    ROOT_ACCOUNT_CONFIG,
    GLOBAL_ORGANIZATION_CONFIG,
    SYSTEM_DEVICE_CONFIG,
    BACKGROUND_SERVICES_CONFIG,
    INITIAL_ENTITY_DATA_CONFIG,
    GENERATE_SCHEMA_CONFIG,
}
#[allow(warnings)]
impl EInitializer {
    pub fn as_str(&self) -> &'static str {
        match self {
            EInitializer::SYSTEM_CODE_CONFIG => "system_code_config",
            EInitializer::ROOT_ACCOUNT_CONFIG => "root_account_config",
            EInitializer::GLOBAL_ORGANIZATION_CONFIG => "global_organization_config",
            EInitializer::SYSTEM_DEVICE_CONFIG => "system_device_config",
            EInitializer::BACKGROUND_SERVICES_CONFIG => "background_services_config",
            EInitializer::INITIAL_ENTITY_DATA_CONFIG => "initial_entity_data_config",
            EInitializer::GENERATE_SCHEMA_CONFIG => "generate_schema_config",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "system_code_config" => Some(EInitializer::SYSTEM_CODE_CONFIG),
            "root_account_config" => Some(EInitializer::ROOT_ACCOUNT_CONFIG),
            "global_organization_config" => Some(EInitializer::GLOBAL_ORGANIZATION_CONFIG),
            "system_device_config" => Some(EInitializer::SYSTEM_DEVICE_CONFIG),
            "background_services_config" => Some(EInitializer::BACKGROUND_SERVICES_CONFIG),
            "initial_entity_data_config" => Some(EInitializer::INITIAL_ENTITY_DATA_CONFIG),
            "generate_schema_config" => Some(EInitializer::GENERATE_SCHEMA_CONFIG),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Default)]
#[allow(warnings)]
pub struct InitializerParams {
    pub entity: String,
    pub system_code_config: Option<SystemCodeConfig>,
    pub root_account_config: Option<RootAccountConfig>,
}

#[derive(Debug, Clone, Default)]
#[allow(warnings)]
pub struct SystemCodeConfig {
    pub default_code: Option<u32>,
    pub prefix: String,
    pub counter: Option<u32>,
    pub digits_number: Option<u32>,
}

#[derive(Debug, Clone, Default)]
#[allow(warnings)]
pub struct RootAccountConfig {
    pub project_name: Option<String>,
}
