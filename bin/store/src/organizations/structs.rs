use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
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
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
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
