use chrono::Utc;
use serde_json::{json, Value};
use std::env;

pub fn get_initial_packets() -> Vec<Value> {
    let now = Utc::now();
    let current_date = now.format("%Y-%m-%d").to_string();
    let current_time = now.format("%H:%M:%S").to_string();
    let timestamp = now.format("%Y-%m-%dT%H:%M:%S%.6f+00:00").to_string();
    let hypertable_timestamp = timestamp.clone();
    let default_organization_id = env::var("DEFAULT_ORGANIZATION_ID")
        .unwrap_or_else(|_| "01JBHKXHYSKPP247HZZWHA3JCT".to_string());

    vec![
        json!({
            "id": "01K0ZEYF7PBVCK2PSZ1KWHH89Y",
            "status": "processed",
            "version": 1,
            "created_date": current_date,
            "created_time": current_time,
            "timestamp": timestamp,
            "hypertable_timestamp": hypertable_timestamp,
            "organization_id": default_organization_id.clone(),
            "source_ip": "192.168.1.100",
            "destination_ip": "192.168.1.200",
            "source_port": 8080,
            "destination_port": 443,
            "protocol": "TCP",
            "sensitivity_level": 1
        }),
        json!({
            "id": "01K0ZEYT5456W97P62H19K93XD",
            "status": "processed",
            "version": 1,
            "created_date": current_date,
            "created_time": current_time,
            "timestamp": timestamp.clone(),
            "hypertable_timestamp": hypertable_timestamp.clone(),
            "organization_id": default_organization_id.clone(),
            "source_ip": "192.168.1.200",
            "destination_ip": "192.168.1.100",
            "source_port": 443,
            "destination_port": 8080,
            "protocol": "TCP",
            "sensitivity_level": 1
        }),
        json!({
            "id": "01K0ZEYZQG6TG9PFEQ2DDG8B53",
            "status": "processed",
            "version": 1,
            "created_date": current_date,
            "created_time": current_time,
            "timestamp": timestamp.clone(),
            "hypertable_timestamp": hypertable_timestamp.clone(),
            "organization_id": default_organization_id.clone(),
            "source_ip": "10.0.0.50",
            "destination_ip": "8.8.8.8",
            "source_port": 53,
            "destination_port": 53,
            "protocol": "UDP",
            "sensitivity_level": 2
        }),
    ]
}
