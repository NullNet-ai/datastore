use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::items)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Item {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = crate::schema::items)]
pub struct NewItem {
    pub name: String,
    pub description: Option<String>,
}
