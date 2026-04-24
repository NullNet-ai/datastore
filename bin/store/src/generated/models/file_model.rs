use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use chrono::{NaiveDateTime};

#[derive(
    Queryable, Selectable, Serialize, Default, Deserialize, Clone, AsChangeset, Insertable, Debug,
)]
#[diesel(table_name = crate::generated::schema::files)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[serde(default)]
pub struct FileModel {
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
    pub timestamp: Option<NaiveDateTime>,
    pub tags: Option<Vec<String>>,
    pub categories: Option<Vec<String>>,
    pub code: Option<String>,
    pub id: Option<String>,
    pub sensitivity_level: Option<i32>,
    pub sync_status: Option<String>,
    pub is_batch: Option<bool>,
    pub image_url: Option<String>,
    pub fieldname: Option<String>,
    pub originalname: Option<String>,
    pub encoding: Option<String>,
    pub mimetype: Option<String>,
    pub destination: Option<String>,
    pub filename: Option<String>,
    pub path: Option<String>,
    pub size: Option<i32>,
    pub uploaded_by: Option<String>,
    pub downloaded_by: Option<String>,
    pub etag: Option<String>,
    pub versionId: Option<String>,
    pub download_path: Option<String>,
    pub presignedURL: Option<String>,
    pub presignedURLExpires: Option<i32>,
}
