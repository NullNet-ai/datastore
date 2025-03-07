use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::sql_query;
use serde_json::Value;
// Import RunQueryDsl trait

pub fn create(conn: &mut PgConnection, table: &str, record: &Value) -> Result<(), diesel::result::Error> {
    let (columns, values): (Vec<String>, Vec<String>) = record.as_object()
        .expect("Expected JSON object")
        .iter()
        .map(|(key, value)| (key.clone(), format!("'{}'", value))) // Wrap values in single quotes
        .unzip();

    let column_names = columns.join(", ");
    let value_list = values.join(", ");

    let query = format!(
        "INSERT INTO {} ({}) VALUES ({})",
        table, column_names, value_list
    );

    sql_query(query).execute(conn)?;

    Ok(())
}