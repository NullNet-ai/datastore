use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(
    Queryable, Selectable, Serialize, Default, Deserialize, Clone, AsChangeset, Insertable, Debug,
)]
#[diesel(table_name = crate::schema::schema::device_rules)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[serde(default)]
pub struct DeviceRuleModel {
    pub tombstone: Option<i32>,
    pub status: Option<String>,
    pub previous_status: Option<String>,
    pub version: Option<i32>,
    pub created_date: Option<String>,
    pub created_time: Option<String>,
    pub updated_date: Option<String>,
    pub updated_time: Option<String>,
    pub organization_id: Option<String>,
    pub created_by: Option<String>,
    pub updated_by: Option<String>,
    pub deleted_by: Option<String>,
    pub requested_by: Option<String>,
    pub tags: Option<Vec<String>>,
    pub categories: Option<Vec<String>>,
    pub code: Option<String>,
    pub timestamp: Option<chrono::NaiveDateTime>,

    pub id: Option<String>,
    pub device_configuration_id: Option<String>,
    pub disabled: Option<bool>,
    pub rule_type: Option<String>,
    pub policy: Option<String>,
    pub protocol: Option<String>,
    pub source_port: Option<String>,
    pub source_addr: Option<String>,
    pub source_type: Option<String>,
    pub destination_port: Option<String>,
    pub destination_addr: Option<String>,
    pub description: Option<String>,
    pub device_rule_status: Option<String>,
    pub interface: Option<String>,
    pub order: Option<i32>,
    pub destination_inversed: Option<bool>,
    pub destination_type: Option<String>,
    pub source_inversed: Option<bool>,
}
