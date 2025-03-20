use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::schema::items)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct GetItem {
    pub tombstone: i32,
    pub status: String,
    pub previous_status: Option<String>,
    pub version: i32,
    pub created_date: Option<String>,
    pub created_time: Option<String>,
    pub updated_date: Option<String>,
    pub updated_time: Option<String>,
    pub organization_id: Option<String>,
    pub created_by: Option<String>,
    pub updated_by: Option<String>,
    pub deleted_by: Option<String>,
    pub requested_by: Option<String>,
    pub tags: Vec<String>,

    pub id: i32,
    pub name: String,
    pub description: Option<String>,
}

#[derive(Insertable, Deserialize, Serialize, Clone)]
#[diesel(table_name = crate::schema::schema::items)]
pub struct InsertItem {
    pub tombstone: i32,
    pub status: String,
    pub previous_status: Option<String>,
    pub version: i32,
    pub created_date: Option<String>,
    pub created_time: Option<String>,
    pub updated_date: Option<String>,
    pub updated_time: Option<String>,
    pub organization_id: Option<String>,
    pub created_by: Option<String>,
    pub updated_by: Option<String>,
    pub deleted_by: Option<String>,
    pub requested_by: Option<String>,
    #[serde(default)]
    pub tags: Vec<String>,

    pub name: String,
    pub description: Option<String>,
}
