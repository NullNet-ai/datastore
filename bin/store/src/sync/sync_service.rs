use crate::db;
use crate::sync::message_service::create_messages;
use diesel::Connection;
use diesel::result::Error as DieselError;
use serde_json::Value;

pub async fn insert(table: &String, row: Value) -> Result<(), DieselError> {
    let operation = "Insert".to_string();
    let mut conn = db::get_connection();
    conn.transaction(|mut tx| {
        let messages = create_messages(&mut tx, &row, table, operation)?;

        Ok::<(), DieselError>(())
    })?;

    Ok(())
}
