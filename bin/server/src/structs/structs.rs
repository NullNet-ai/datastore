use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Deserialize)]
#[serde(transparent)]
pub struct CreateRequestBody {
    pub record: Value,
}



#[derive(Deserialize, Default)]
#[serde(default)]
pub struct QueryParams {
    pub start: u32,
    pub limit: u32,
    pub client_id: String,
}