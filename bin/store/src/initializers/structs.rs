#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EInitializer {
    SYSTEM_CODE_CONFIG,
    ROOT_ACCOUNT_CONFIG,
    GLOBAL_ORGANIZATION_CONFIG,
    SYSTEM_DEVICE_CONFIG,
}

impl EInitializer {
    pub fn as_str(&self) -> &'static str {
        match self {
            EInitializer::SYSTEM_CODE_CONFIG => "system_code_config",
            EInitializer::ROOT_ACCOUNT_CONFIG => "root_account_config",
            EInitializer::GLOBAL_ORGANIZATION_CONFIG => "global_organization_config",
            EInitializer::SYSTEM_DEVICE_CONFIG => "system_device_config",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "system_code_config" => Some(EInitializer::SYSTEM_CODE_CONFIG),
            "root_account_config" => Some(EInitializer::ROOT_ACCOUNT_CONFIG),
            "global_organization_config" => Some(EInitializer::GLOBAL_ORGANIZATION_CONFIG),
            "system_device_config" => Some(EInitializer::SYSTEM_DEVICE_CONFIG),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct InitializerParams {
    pub entity: String,
    pub system_code_config: Option<SystemCodeConfig>,
    pub root_account_config: Option<RootAccountConfig>,
}

#[derive(Debug, Clone, Default)]
pub struct SystemCodeConfig {
    pub default_code: Option<u32>,
    pub prefix: String,
    pub counter: Option<u32>,
    pub digits_number: Option<u32>,
}

#[derive(Debug, Clone, Default)]
pub struct RootAccountConfig {
    pub project_name: Option<String>,
}
