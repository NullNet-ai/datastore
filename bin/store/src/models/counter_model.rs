use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Serialize, Deserialize, Clone, AsChangeset, Insertable, Debug)]
#[diesel(table_name = crate::schema::schema::counters)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct CounterModel {
    pub entity: String,
    pub default_code: i32,
    pub prefix: String,
    pub counter: i32,
}
