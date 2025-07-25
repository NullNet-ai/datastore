use serde_json::{json, Value};
use chrono::Utc;
use std::env;

pub fn get_initial_connections() -> Vec<Value> {
    let now = Utc::now();
    let current_date = now.format("%Y-%m-%d").to_string();
    let current_time = now.format("%H:%M:%S").to_string();
    let timestamp = now.format("%Y-%m-%dT%H:%M:%S%.6f+00:00").to_string();
    let hypertable_timestamp = timestamp.clone();
    let default_organization_id = env::var("DEFAULT_ORGANIZATION_ID")
        .unwrap_or_else(|_| "01JBHKXHYSKPP247HZZWHA3JCT".to_string());
    
    vec![
        json!({
            "id": "01K0ZEXPJ2MCC6KJ56XHS3JBH2",
            "status": "active",
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
            "id": "01K0ZEY1CAW941F5A0A5QA7G31",
            "status": "active",
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
        json!({
            "id": "01K0ZEY83R0PHFX4Z6812DV84W",
            "status": "closed",
            "version": 1,
            "created_date": current_date,
            "created_time": current_time,
            "timestamp": timestamp.clone(),
            "hypertable_timestamp": hypertable_timestamp.clone(),
            "organization_id": "01JBHKXHYSKPP247HZZWHA3JCT",
            "source_ip": "172.16.0.10",
            "destination_ip": "172.16.0.20",
            "source_port": 22,
            "destination_port": 22,
            "protocol": "TCP",
            "sensitivity_level": 3
        })
    ]
}