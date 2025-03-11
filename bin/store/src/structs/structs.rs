use chrono::Utc;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use uuid::Uuid;

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

impl CreateRequestBody {
    // Process record with common fields and return a Value directly
    pub fn process_record(&self) -> Result<Value, String> {
        // Parse the JSON string into a Value for preprocessing
        let trimmed_record = self.record.trim();
        let record_value = match serde_json::from_str::<Value>(trimmed_record) {
            Ok(value) => value,
            Err(e) => {
                return Err(format!("Invalid JSON: {}. Input was: {}", e, trimmed_record));
            }
        };

        // Add common fields to the record
        let mut processed_record = self.add_common_fields(record_value);

        if let Some(timestamp) = processed_record.get_mut("timestamp") {
            if let Some(ts_str) = timestamp.as_str() {
                // Remove any trailing Z, +00:00, etc.
                let cleaned_ts = if ts_str.contains('T') && (ts_str.contains('Z') || ts_str.contains('+')) {
                    // Extract just the part before Z or +
                    let parts: Vec<&str> = ts_str.split(|c| c == 'Z' || c == '+').collect();
                    parts[0].to_string()
                } else {
                    ts_str.to_string()
                };
                *timestamp = json!(cleaned_ts);
            }
        }

        // Return the Value directly
        Ok(processed_record)
    }

    // Helper method to add common fields
    fn add_common_fields(&self, mut record: Value) -> Value {
        // Get current time for timestamps
        let now = Utc::now();
        let date_str = now.format("%Y-%m-%d").to_string();
        let time_str = now.format("%H:%M:%S").to_string();

        // Set common fields
        record["tombstone"] = json!(0);
        record["status"] = json!("active");
        record["version"] = json!(1);
        record["created_date"] = json!(date_str);
        record["created_time"] = json!(time_str);
        record["updated_date"] = json!(date_str);
        record["updated_time"] = json!(time_str);

        // Generate UUID for id if not present (as text)
        if !record.get("id").is_some() {
            record["id"] = json!(Uuid::new_v4().to_string());
        }

        record
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

