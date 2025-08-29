use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::models::crdt_messages::CrdtMessage;

#[derive(Deserialize)]
#[serde(transparent)]
#[allow(warnings)]
pub struct CreateRequestBody {
    pub record: Value,
}

#[derive(Deserialize, Default)]
#[serde(default)]
pub struct QueryParams {
    pub start: usize,
    pub limit: usize,
    pub client_id: String,
}

#[derive(Deserialize, Clone, Debug, Serialize)]
pub struct SyncRequestBody {
    pub group_id: String,
    pub client_id: String,
    pub messages: Vec<CrdtMessage>,
    pub merkle: Option<String>,
}
