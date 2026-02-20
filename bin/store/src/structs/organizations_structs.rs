use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VerifyPasswordParams {
    pub account_id: String,
    pub password: String,
}
