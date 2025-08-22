use serde::{Deserialize, Serialize};

use crate::providers::operations::permissions::permissions_queries::PermissionQueryParams;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DataPermissions {
    pub requested_fields: Vec<String>,
    pub data_permissions_query_params: PermissionQueryParams,
    pub account_organization_id: String,
    pub schema: Vec<SchemaItem>,
    pub valid_pass_keys_query_params: PermissionQueryParams,
    pub group_by_field_record_permissions_params: PermissionQueryParams,
    pub role_permissions_query_params: PermissionQueryParams,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SchemaItem {
    pub entity: String,
    pub alias: String,
    pub field: String,
    pub property_name: String,
    pub path: String,
}
