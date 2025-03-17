use crate::models::crdt_message_model::{GetCrdtMessage, InsertCrdtMessage};
use crate::models::{item_model, packet_model};
use crate::schema::schema::{crdt_messages, items, packets};
use diesel::prelude::*;
use diesel::result::Error as DieselError;
use item_model::{GetItem, InsertItem};
use packet_model::{GetPacket, InsertPacket};
use serde_json;

#[derive(Debug)]
pub enum Table {
    Items,
    Packets,
    CrdtMessages,
    // Add other tables here
}

impl Table {
    // Method specific for items table
    pub fn insert_item(
        &self,
        conn: &mut PgConnection,
        new_item: InsertItem,
    ) -> Result<String, DieselError> {
        match self {
            Table::Items => {
                let result = diesel::insert_into(items::table)
                    .values(&new_item)
                    .get_result::<GetItem>(conn)?;

                // Convert the result to a JSON string
                Ok(serde_json::to_string(&result).unwrap_or_else(|_| "{}".to_string()))
            }
            _ => Err(DieselError::QueryBuilderError("Not an items table".into())),
        }
    }

    // Method specific for packets table
    pub fn insert_packet(
        &self,
        conn: &mut PgConnection,
        new_packet: InsertPacket,
    ) -> Result<String, DieselError> {
        match self {
            Table::Packets => {
                let result = diesel::insert_into(packets::table)
                    .values(&new_packet)
                    .get_result::<GetPacket>(conn)?;

                // Convert the result to a JSON string
                Ok(serde_json::to_string(&result).unwrap_or_else(|_| "{}".to_string()))
            }
            _ => Err(DieselError::QueryBuilderError("Not a packets table".into())),
        }
    }

    pub fn insert_crdt_message(
        &self,
        conn: &mut PgConnection,
        new_crdt_message: InsertCrdtMessage,
    ) -> Result<String, DieselError> {
        match self {
            Table::CrdtMessages => {
                let result = diesel::insert_into(crdt_messages::table)
                    .values(&new_crdt_message)
                    .get_result::<GetCrdtMessage>(conn)?;

                // Convert the result to a JSON string
                Ok(serde_json::to_string(&result).unwrap_or_else(|_| "{}".to_string()))
            }
            _ => Err(DieselError::QueryBuilderError(
                "Not a crdt_messages table".into(),
            )),
        }
    }
}
