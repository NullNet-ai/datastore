#[derive(Debug, Serialize, Deserialize)]
pub struct LoginResponse {
    pub message: String,
    pub token: Option<String>,
}