use serde::{Deserialize, Serialize};
#[derive(Serialize)]
pub struct Response<T> {
    pub success: bool,
    pub message: String,
    pub count: usize,
    pub data: Vec<T>,
}

#[derive(Deserialize)]
pub struct CreateQuery {
    pub pluck: Option<String>,
}

pub struct CreateParams {
    pub table: String,
}