use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DataPermissions {
    pub requested_fields: Vec<String>,
    pub query: String,
    pub account_organization_id: String,
    pub schema: Vec<SchemaItem>,
    pub valid_pass_keys_query: String,
    pub record_valid_pass_keys_query: String,
    pub role_permissions_query: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SchemaItem {
    pub entity: String,
    pub alias: String,
    pub field: String,
    pub property_name: String,
    pub path: String,
}
