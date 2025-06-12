use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(
    Queryable, Selectable, Serialize, Default, Deserialize, Clone, AsChangeset, Insertable, Debug,
)]
#[diesel(table_name = crate::schema::schema::data_permissions)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[serde(default)]
pub struct DataPermissionModel {
    pub id: Option<String>,
    pub entity_field_id: Option<String>,
    pub inherited_permission_id: Option<String>,
    pub account_organization_id: Option<String>,
    pub version: Option<i32>,
    pub created_by: Option<String>,
    pub updated_by: Option<String>,
    pub deleted_by: Option<String>,
    pub timestamp: Option<String>,
    pub tombstone: Option<i32>,
    pub permission_id: Option<String>,

}