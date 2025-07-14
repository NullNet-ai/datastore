use diesel::prelude::*;
use diesel::sql_types::*;
use diesel::QueryableByName;
use serde::{Deserialize, Serialize};

#[derive(QueryableByName, Debug, Serialize, Deserialize)]
pub struct QueryResult {
    #[diesel(sql_type = Nullable<Text>)]
    pub id: Option<String>,
}

#[derive(QueryableByName, Debug, Serialize)]
pub struct DynamicResult {
    #[diesel(sql_type = Nullable<Text>)]
    pub row_to_json: Option<String>, // Store as JSON string
}
