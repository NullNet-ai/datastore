use crate::models::crdt_message_model::{GetCrdtMessage, InsertCrdtMessage};
use crate::models::{item_model, packet_model};
use crate::schema::schema::{crdt_messages, items, packets};
use diesel::pg::Pg;
use diesel::prelude::*;
use diesel::query_builder::{AsQuery, InsertStatement, QueryFragment, QueryId};
use diesel::query_dsl::LoadQuery;
use diesel::query_dsl::methods::ExecuteDsl;
use diesel::result::Error as DieselError;
use item_model::{GetItem, InsertItem};
use packet_model::{GetPacket, InsertPacket};
use serde::Serialize;
use serde_json;

#[derive(Debug)]
pub enum Table {
    Items,
    Packets,
    CrdtMessages,
    // Add other tables here
}

impl Table {
    pub fn from_str(name: &str) -> Option<Self> {
        match name {
            "items" => Some(Table::Items),
            "packets" => Some(Table::Packets),
            "crdt_messages" => Some(Table::CrdtMessages),
            // Add other tables here
            _ => None,
        }
    }

    pub fn insert_record<'a, T, M, U>(
        &self,
        conn: &mut PgConnection,
        table: T,
        record: M,
    ) -> Result<String, DieselError>
    where
        T: diesel::Table,
        M: diesel::Insertable<T>,
        U: Serialize,
        InsertStatement<T, M::Values>: LoadQuery<'a, PgConnection, U>,
    {
        let result = diesel::insert_into(table)
            .values(record)
            .get_result::<U>(conn)?;

        // Convert the result to a JSON string
        Ok(serde_json::to_string(&result).unwrap_or_else(|_| "{}".to_string()))
    }
}
