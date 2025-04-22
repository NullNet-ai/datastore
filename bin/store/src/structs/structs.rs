use crate::sync::hlc::mutable_timestamp::MutableTimestamp;
use chrono::Utc;
use diesel::AsExpression;
use diesel::sql_types::{Text, Uuid};
use merkle::MerkleTree;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use uuid::Uuid as uuid_crate;

#[derive(Serialize, Deserialize)]
pub struct ApiResponse {
    pub success: bool,
    pub message: String,
    pub count: i32,
    pub data: Vec<Value>,
}

#[derive(Deserialize)]
#[serde(transparent)]
pub struct CreateRequestBody {
    pub record: Value,
}

impl CreateRequestBody {
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
            }
            "delete" => {
                self.record["status"] = json!("Deleted");
                self.record["tombstone"] = json!(1);
            }
            _ => {
                // Handle other operations if needed
            }
        }
        // Generate UUID for id if not present (as text)
        if !self.record.get("id").is_some() {
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
    Timestamp(chrono::DateTime<chrono::FixedOffset>),
}

impl ColumnValue {
    pub fn to_string_value(&self) -> String {
        match self {
            ColumnValue::String(s) => s.clone(),
            ColumnValue::Array(arr) => {
                // Format as PostgreSQL array literal
                format!("{{{}}}", arr.join(","))
            }
            ColumnValue::Timestamp(dt) => dt.to_rfc3339(),
        }
    }

    // For use with Diesel's insert/update operations
    pub fn to_json_value(&self) -> serde_json::Value {
        match self {
            ColumnValue::String(s) => serde_json::Value::String(s.clone()),
            ColumnValue::Array(arr) => {
                // Convert to a JSON array
                serde_json::Value::Array(
                    arr.iter()
                        .map(|s| serde_json::Value::String(s.clone()))
                        .collect(),
                )
            }
            ColumnValue::Timestamp(dt) => serde_json::Value::String(dt.to_rfc3339()),
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
