use crate::models::{Item, NewItem};
use crate::schema::items;
use diesel::prelude::*;
use diesel::result::Error as DieselError;


#[derive(Debug)]
pub enum Table {
    Items,
    // Add other tables here
}

impl Table {
    pub fn insert_query<'a>(&self, conn: &mut PgConnection, new_item: NewItem) -> Result<Item, DieselError> {
        match self {
            Table::Items => {
                diesel::insert_into(items::table)
                    .values(&new_item)
                    .get_result::<Item>(conn)
            }
            // Add cases for other tables here
        }
    }
}
