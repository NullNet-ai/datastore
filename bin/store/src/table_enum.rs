use crate::schema::schema;
use diesel::associations::HasTable;
use diesel::prelude::*;
use diesel::query_builder::InsertStatement;
use diesel::query_dsl::LoadQuery;
use diesel::result::Error as DieselError;
use serde::Serialize;
use serde_json;
use serde_json::Value;
use crate::models::packet_model::Packet;
use crate::schema::schema::packets::dsl::packets;

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

    pub fn upsert_record(
        &self,
        conn: &mut PgConnection,
        record: Value,
    ) -> Result<(), DieselError> {
        match self {
            Table::Packets => {
                let value: Packet = serde_json::from_value(record).map_err(|e| {
                    DieselError::DeserializationError(Box::new(e))
                })?;
                diesel::insert_into(packets::table())
                    .values(value.clone())
                    .on_conflict(schema::packets::id)
                    .do_update()
                    .set(value)
                    .execute(conn)
                    .map(|_| ())
            }
            _ => panic!(),
        }
    }
}
