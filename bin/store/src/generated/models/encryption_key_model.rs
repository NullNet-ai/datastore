use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(
    Queryable, Selectable, Serialize, Default, Deserialize, Clone, AsChangeset, Insertable, Debug,
)]
#[diesel(table_name = crate::generated::schema::encryption_keys)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[serde(default)]
pub struct EncryptionKeyModel {
    pub id: Option<String>,
    pub organization_id: Option<String>,
    pub entity: Option<String>,
    pub created_by: Option<String>,
    pub timestamp: Option<String>,
    #[serde(default = "default_tombstone")]
    pub tombstone: Option<i32>,
}

// Default function implementation
fn default_tombstone() -> Option<i32> {
    Some(0)
}
