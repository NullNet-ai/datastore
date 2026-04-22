use serde::{Deserialize, Serialize};
use std::fmt;
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, Clone, Default, ToSchema)]
#[serde(default)]
pub struct Register {
    // !To be deprecated
    pub id: Option<String>,
    pub name: Option<String>,
    pub contact_id: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
    pub parent_organization_id: Option<String>,
    pub code: Option<String>,
    pub categories: Option<Vec<String>>,
    pub organization_categories: Option<Vec<String>>,
    pub account_status: Option<String>,

    pub account_type: Option<AccountType>, // Replace with enum if AccountType is defined
    pub organization_name: Option<String>,
    pub organization_id: Option<String>,
    pub account_id: String,
    pub account_secret: String,
    pub is_new_user: Option<bool>,
    pub first_name: String,
    pub last_name: String,
    pub is_invited: Option<bool>,
    pub role_id: Option<String>,
    pub account_organization_status: Option<String>,
    pub account_organization_categories: Option<Vec<String>>,
    pub account_organization_id: Option<String>,
    pub contact_categories: Option<Vec<String>>,
    pub device_categories: Option<Vec<String>>,

    pub responsible_account_organization_id: Option<String>,

    /// When set (e.g. by initializers), the personal organization is created with this ID instead of a new ULID. Used only for initial super admin and system device personal orgs.
    pub initial_personal_organization_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, ToSchema)]
#[serde(rename_all = "lowercase")]
pub enum AccountType {
    Contact,
    Device,
}

impl std::str::FromStr for AccountType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "contact" => Ok(AccountType::Contact),
            "device" => Ok(AccountType::Device),
            _ => Err(format!("Unknown AccountType: {}", s)),
        }
    }
}

impl fmt::Display for AccountType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AccountType::Contact => write!(f, "contact"),
            AccountType::Device => write!(f, "device"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct LoginResponse {
    pub message: String,
    pub token: Option<String>,
    pub role_id: String,
    pub account_organization_id: Option<String>,
    pub session_id: Option<String>,
}
