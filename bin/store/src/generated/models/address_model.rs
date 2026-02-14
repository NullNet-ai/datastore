use crate::database::schema::common_defaults::default_sensitivity_level;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(
    Queryable, Selectable, Serialize, Default, Deserialize, Clone, AsChangeset, Insertable, Debug,
)]
#[diesel(table_name = crate::generated::schema::addresses)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[serde(default)]
pub struct AddressModel {
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
    #[serde(default = "default_sensitivity_level")]
    pub sensitivity_level: Option<i32>,
    pub sync_status: Option<String>,
    pub is_batch: Option<bool>,

    pub id: Option<String>,
    pub address: Option<String>,
    pub address_line_one: Option<String>,
    pub address_line_two: Option<String>,
    pub latitude: Option<f32>,
    pub longitude: Option<f32>,
    pub place_id: Option<String>,
    pub street_number: Option<String>,
    pub street: Option<String>,
    pub region: Option<String>,
    pub region_code: Option<String>,
    pub country_code: Option<String>,
    pub postal_code: Option<String>,
    pub country: Option<String>,
    pub state: Option<String>,
    pub city: Option<String>,
}
