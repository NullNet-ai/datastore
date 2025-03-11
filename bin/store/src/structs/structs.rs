use chrono::Utc;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use uuid::Uuid;

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
    pub fn process_record(&mut self) {
        // // Add common fields to the record
        self.add_common_fields();

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
    fn add_common_fields(&mut self) {
        // Get current time for timestamps
        let now = Utc::now();
        let date_str = now.format("%Y-%m-%d").to_string();
        let time_str = now.format("%H:%M:%S").to_string();

        // Set common fields
        self.record["tombstone"] = json!(0);
        self.record["status"] = json!("active");
        self.record["version"] = json!(1);
        self.record["created_date"] = json!(date_str);
        self.record["created_time"] = json!(time_str);
        self.record["updated_date"] = json!(date_str);
        self.record["updated_time"] = json!(time_str);

        // Generate UUID for id if not present (as text)
        if !self.record.get("id").is_some() {
            self.record["id"] = json!(Uuid::new_v4().to_string());
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
