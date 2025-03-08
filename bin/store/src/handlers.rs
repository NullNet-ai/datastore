use crate::models::{NewTableName, TableName};
use crate::schema::table_name;
use diesel::prelude::*;

pub fn create_record(conn: &mut PgConnection, name: &str, description: &str) -> TableName {
    let new_record = NewTableName { name, description };
    println!("{} {}", name, description);

    diesel::insert_into(table_name::table)
        .values(&new_record)
        .get_result(conn)
        .expect("Error saving new record")
}
