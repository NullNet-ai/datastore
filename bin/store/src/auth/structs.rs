use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginResponse {
    pub message: String,
    pub token: Option<String>,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct Account {
    pub id: String,
    pub account_id: String,
    pub organization_id: String,
    pub account_organization_id: String,
    pub account_status: String,
    pub role_id: String,
    #[serde(default)]
    pub role_name: Option<String>,

    #[serde(default)]
    pub role_level: Option<u32>,

    #[serde(default)]
    pub is_root_account:bool,

    #[serde(default)]
    pub profile: Option<serde_json::Value>,

    #[serde(default)]
    pub organization: Option<serde_json::Value>,

    #[serde(default)]
    pub contact: Option<serde_json::Value>, // Unknown shape

    #[serde(default)]
    pub device: Option<serde_json::Value>, // Unknown shape
}


#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub account: Account,
    pub sessionID: String,
    pub role_level: i32,
    pub role_name: String,
    #[serde(default)]
    pub previously_logged_in: Option<String>,
    exp: usize,
    iat: usize,
}
