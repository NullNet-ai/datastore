use serde::{Deserialize, Serialize};
use serde_json::Value;

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
    pub is_root_account: bool,

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
    pub role_name: String,
    pub sensitivity_level: u32,
    #[serde(default)]
    pub previously_logged_in: Option<String>,
    exp: usize,
    iat: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct User {
    pub role_id: String,
    #[serde(default)]
    pub is_root_user: bool,
    pub account_id: String, // Add other user fields as needed
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Origin {
    pub user_agent: Option<String>,
    pub host: String,
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Cookie {
    pub path: String,
    pub expires: String, // Using String for ISO date format
    pub originalMaxAge: i64,
    pub httpOnly: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Session {
    pub user: User,
    pub session_id: String,
    pub origin: Option<Origin>,
    pub token: String,
    pub cookie: Cookie,
}
