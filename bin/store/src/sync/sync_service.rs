use crate::sync::message_service::generate_messages_from_value;
use crate::table_enum::Table;
use diesel::result::Error as DieselError;
use diesel::PgConnection;
use serde_json::Value;

pub async fn insert(conn: &mut PgConnection, table: &String, row: Value) -> Result<(), DieselError> {
    let operation = "Insert".to_string();
    let messages = generate_messages_from_value(&row, table, operation);

    for message in messages {
        Table::CrdtMessages.insert_crdt_message(conn, message)?;
    }

    Ok(())
}