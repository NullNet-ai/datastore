use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct VerifyPasswordParams {
    pub account_id: String,
    pub password: String,
}
