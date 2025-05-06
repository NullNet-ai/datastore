use crate::sync::hlc::mutable_timestamp::MutableTimestamp;
use chrono::Utc;
use diesel::sql_types::{Text, Uuid};
use diesel::AsExpression;
use merkle::MerkleTree;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use uuid::Uuid as uuid_crate;

#[derive(Serialize, Deserialize)]
pub struct ApiResponse {
    pub success: bool,
    pub message: String,
    pub count: i32,
    pub data: Vec<Value>,
}

#[derive(Deserialize, Serialize, Clone)]
#[serde(transparent)]
pub struct RequestBody {
    pub record: Value,
}

impl RequestBody {
    // Process record with common fields and return a Value directly
    pub fn process_record(&mut self, operation: &str) {
        // // Add common fields to the record
        self.add_common_fields(operation);

        if let Some(timestamp) = self.record.get_mut("timestamp") {
            if let Some(ts_str) = timestamp.as_str() {
                // Remove any trailing Z, +00:00, etc.
                let cleaned_ts =
                    if ts_str.contains('T') && (ts_str.contains('Z') || ts_str.contains('+')) {
                        // Extract just the part before Z or +
                        let parts: Vec<&str> = ts_str.split(|c| c == 'Z' || c == '+').collect();
                        parts[0].to_string()
                    } else {
                        ts_str.to_string()
                    };
                *timestamp = json!(cleaned_ts);
            }
        }
    }

    // Helper method to add common fields
    fn add_common_fields(&mut self, operation: &str) {
        // Get current time for timestamps
        let now = Utc::now();
        let date_str = now.format("%Y-%m-%d").to_string();
        let time_str = now.format("%H:%M:%S").to_string();

        // Set common fields

        match operation {
            "create" => {
                self.record["status"] = json!("Active");
                self.record["created_date"] = json!(date_str);
                self.record["created_time"] = json!(time_str);
                self.record["updated_date"] = json!(date_str);
                self.record["updated_time"] = json!(time_str);
                self.record["version"] = json!(1);
                self.record["tombstone"] = json!(0);
            }
            "update" => {
                self.record["updated_date"] = json!(date_str);
                self.record["updated_time"] = json!(time_str);
                self.record["version"] = json!(0); // ! don't change this, it gets discarded in the end, it's just a placeholder

            }
            "delete" => {
                self.record["status"] = json!("Deleted");
                self.record["tombstone"] = json!(1);
            }
            _ => {
                // Handle other operations if needed
            }
        }

        if !self.record.get("id").is_some()
            || self.record["id"].is_null()
            || self.record["id"]
                .as_str()
                .map_or(true, |s| s.trim().is_empty())
        {
            self.record["id"] = json!(uuid_crate::new_v4().to_string());
        }
    }
}

#[derive(Deserialize)]
pub struct QueryParams {
    #[serde(default = "default_pluck")]
    pub pluck: String,
}

fn default_pluck() -> String {
    "id".to_string()
}

#[derive(Clone)]
pub struct Clock {
    pub timestamp: MutableTimestamp,
    pub merkle: MerkleTree,
}

#[derive(Debug, AsExpression)]
#[diesel(sql_type = diesel::sql_types::Array<diesel::sql_types::Text>)]
pub enum ColumnValue {
    String(String),
    Array(Vec<String>),
    Integer(i32),
    Timestamp(chrono::DateTime<chrono::FixedOffset>),
}

impl ColumnValue {
    pub fn to_string_value(&self) -> String {
        match self {
            ColumnValue::String(s) => s.clone(),
            ColumnValue::Array(arr) => {
                format!("{{{}}}", arr.join(","))
            }
            ColumnValue::Timestamp(dt) => dt.to_rfc3339(),
            ColumnValue::Integer(i) => i.to_string(),
        }
    }

    pub fn to_json_value(&self) -> serde_json::Value {
        match self {
            ColumnValue::String(s) => json!(s),
            ColumnValue::Array(arr) => json!(arr),
            ColumnValue::Timestamp(dt) => json!(dt.naive_utc()),
            ColumnValue::Integer(i) => json!(i),
        }
    }
}

#[derive(Debug, AsExpression)]
#[diesel(sql_type = Text)]
pub enum Id {
    Text(String),
    Uuid(uuid::Uuid),
}

impl Id {
    pub fn as_expression(
        &self,
    ) -> Box<dyn diesel::expression::Expression<SqlType = diesel::sql_types::Text>> {
        match self {
            Id::Text(text) => Box::new(diesel::dsl::sql::<diesel::sql_types::Text>(&format!(
                "'{}'",
                text
            ))),
            Id::Uuid(uuid) => Box::new(diesel::dsl::sql::<diesel::sql_types::Text>(&format!(
                "'{}'",
                uuid.to_string()
            ))),
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Id::Text(text) => text.clone(),
            Id::Uuid(uuid) => uuid.to_string(),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Endpoint {
    pub url: String,
    pub username: Option<String>,
    pub password: Option<String>,
    pub id: Option<String>,
    pub group_id: Option<String>,
    pub status: Option<String>,
}
