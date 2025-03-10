use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ApiResponse {
    pub success: bool,
    pub message: String,
    pub count: i32,
    pub data: Vec<serde_json::Value>,
}


#[derive(Deserialize)]
pub struct CreateRequestBody {
    pub record: String,
}
