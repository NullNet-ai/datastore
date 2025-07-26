use diesel::sql_types::*;
use diesel::QueryableByName;
use serde::Serialize;

#[derive(QueryableByName, Debug, Serialize)]
pub struct DynamicResult {
    #[diesel(sql_type = Nullable<Json>)]
    pub row_to_json: Option<serde_json::Value>, // Store as JSON directly
}

#[derive(QueryableByName, Debug, Serialize)]
pub struct QueryResult {
    #[diesel(sql_type = Nullable<Text>)]
    pub id: Option<String>, // Store as JSON string
    #[diesel(sql_type = Nullable<Text>)]
    pub status: Option<String>, // Store as JSON string
}
