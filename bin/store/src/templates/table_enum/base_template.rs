use crate::{generate_get_by_id_match, generate_hypertable_timestamp_match, generate_insert_record_match, generate_upsert_record_match, generate_upsert_record_with_timestamp_match};
use crate::models::connection_model::ConnectionModel;
use crate::models::packet_model::PacketModel;
use crate::schema::schema;
use crate::schema::verify::field_exists_in_table;
use crate::structs::structs::RequestBody;
use actix_web::web;
use diesel::associations::HasTable;
use diesel::prelude::*;
use diesel::result::Error as DieselError;
use diesel_async::AsyncPgConnection;
use diesel_async::RunQueryDsl;
use serde_json::{Map, Value};

#[derive(Debug)]
pub enum Table {
    Items,
    Packets,
    CrdtMessages,
    Connections,
    // Add other tables here
}

impl Table {
    pub fn from_str(name: &str) -> Option<Self> {
        match name {
            "items" => Some(Table::Items),
            "packets" => Some(Table::Packets),
            "crdt_messages" => Some(Table::CrdtMessages),
            "connections" => Some(Table::Connections),
            // Add other tables here
            _ => None,
        }
    }

    // pub fn insert_record_generic<'a, T, M, U>(
    //     &self,
    //     conn: &mut PgConnection,
    //     table: T,
    //     record: M,
    // ) -> Result<String, DieselError>
    // where
    //     T: diesel::Table,
    //     M: diesel::Insertable<T>,
    //     U: Serialize,
    //     InsertStatement<T, M::Values>: LoadQuery<'a, PgConnection, U>,
    // {
    //     let result = diesel::insert_into(table)
    //         .values(record)
    //         .get_result::<U>(conn)?;

    //     // Convert the result to a JSON string
    //     Ok(serde_json::to_string(&result).unwrap_or_else(|_| "{}".to_string()))
    // }
    pub fn pluck_fields(&self, record_value: &Value, pluck_fields: Vec<String>) -> Value {
        if !pluck_fields.is_empty() && record_value.is_object() {
            let obj = record_value.as_object().unwrap();
            let mut filtered = Map::new();

            for field in pluck_fields {
                if let Some(val) = obj.get(&field) {
                    filtered.insert(field, val.clone());
                }
            }

            Value::Object(filtered)
        } else {
            record_value.clone() // fallback: return original value
        }
    }

    // pub async fn get_hypertable_timestamp(
    //     &self,
    //     conn: &mut AsyncPgConnection,
    //     id: &str,
    // ) -> Result<Option<String>, DieselError> {
    //     match self {
    //         generate_hypertable_timestamp_arm!(Packets),
    //         Table::Connections => {
    //             let result = connections
    //                 .filter(schema::connections::id.eq(id))
    //                 .select(schema::connections::hypertable_timestamp)
    //                 .first::<Option<String>>(conn)
    //                 .await;

    //             result
    //         }
    //         _ => {
    //             log::error!(
    //                 "Getting hypertable_timestamp for table {:?} is not implemented",
    //                 self
    //             );
    //             Err(DieselError::RollbackTransaction)
    //         }
    //     }
    // }

    pub async fn get_hypertable_timestamp(
        &self,
        conn: &mut AsyncPgConnection,
        id: &str,
    ) -> Result<Option<String>, DieselError> {
        generate_hypertable_timestamp_match!(self, conn, id, Packets, Connections)
    }

    // pub async fn insert_record(
    //     &self,
    //     conn: &mut AsyncPgConnection,
    //     record: Value,
    //     request: web::Json<RequestBody>,
    // ) -> Result<String, DieselError> {
    //     let mut request = request.into_inner();
    //     request.process_record("create");

    //     match self {
    //         Table::Packets => {
    //             let mut value: PacketModel = serde_json::from_value(record)
    //                 .map_err(|e| DieselError::DeserializationError(Box::new(e)))?;

    //             value.hypertable_timestamp = Some(value.timestamp.to_string());
    //             diesel::insert_into(packets::table())
    //                 .values(value.clone())
    //                 .execute(conn) // Use execute instead of get_result
    //                 .await?;
    //             Ok(serde_json::to_string(&value).unwrap_or_else(|_| "{}".to_string()))
    //         }
    //         _ => panic!(),
    //     }
    // }

    pub async fn insert_record(
        &self,
        conn: &mut AsyncPgConnection,
        record: Value,
        request: web::Json<RequestBody>,
    ) -> Result<String, DieselError> {
        generate_insert_record_match!(
            self,
            conn,
            record,
            request,
            Packets,
            PacketModel,
            Connections,
            ConnectionModel // Add other tables and their models here as needed
        )
    }

    // pub async fn get_by_id(
    //     &self,
    //     conn: &mut AsyncPgConnection,
    //     id: &str,
    // ) -> Result<Option<Value>, DieselError> {
    //     match self {
    //         Table::Packets => {
    //             let result = packets
    //                 .filter(schema::packets::id.eq(id))
    //                 .select(schema::packets::all_columns)
    //                 .first::<PacketModel>(conn)
    //                 .await
    //                 .optional()?;

    //             Ok(result.map(|packet| serde_json::to_value(packet).unwrap_or_default()))
    //         }
    //         Table::Connections => {
    //             let result = connections
    //                 .filter(schema::connections::id.eq(id))
    //                 .select(schema::connections::all_columns)
    //                 .first::<ConnectionModel>(conn)
    //                 .await
    //                 .optional()?;

    //             Ok(result.map(|connection| serde_json::to_value(connection).unwrap_or_default()))
    //         }
    //         _ => {
    //             log::error!(
    //                 "Getting record by id for table {:?} is not implemented",
    //                 self
    //             );
    //             Err(DieselError::RollbackTransaction)
    //         }
    //     }
    // }

    // ... existing code ...

    pub async fn get_by_id(
        &self,
        conn: &mut AsyncPgConnection,
        id: &str,
    ) -> Result<Option<Value>, DieselError> {
        generate_get_by_id_match!(
            self,
            conn,
            id,
            Packets,
            PacketModel,
            Connections,
            ConnectionModel // Add other tables and their models here as needed
        )
    }

    // pub async fn upsert_record_with_id(
    //     &self,
    //     conn: &mut AsyncPgConnection,
    //     record: Value,
    // ) -> Result<(), DieselError> {
    //     let has_version = record.get("version").is_some();
    //     match self {
    //         Table::Packets => {
    //             let value: PacketModel = serde_json::from_value(record).map_err(|e| {
    //                 log::error!("Deserialization error: {:?}", e);
    //                 DieselError::DeserializationError(Box::new(e))
    //             })?;
    //             if (has_version) {
    //                 diesel::insert_into(packets::table())
    //                     .values(value.clone())
    //                     .on_conflict((schema::packets::id))
    //                     .do_update()
    //                     .set(schema::packets::version.eq(schema::packets::version + 1))
    //                     .execute(conn)
    //                     .await
    //                     .map(|_| ())
    //             } else {
    //                 diesel::insert_into(packets::table())
    //                     .values(value.clone())
    //                     .on_conflict((schema::packets::id))
    //                     .do_update()
    //                     .set(value)
    //                     .execute(conn)
    //                     .await
    //                     .map(|_| ())
    //             }
    //         }
    //         Table::Connections => {
    //             let value: ConnectionModel = serde_json::from_value(record).map_err(|e| {
    //                 log::error!("Deserialization error: {:?}", e);
    //                 DieselError::DeserializationError(Box::new(e))
    //             })?;

    //             if has_version {
    //                 diesel::insert_into(connections::table())
    //                     .values(value.clone())
    //                     .on_conflict((schema::connections::id))
    //                     .do_update()
    //                     .set(schema::connections::version.eq(schema::connections::version + 1))
    //                     .execute(conn)
    //                     .await
    //                     .map(|_| ())
    //             } else {
    //                 diesel::insert_into(connections::table())
    //                     .values(value.clone())
    //                     .on_conflict((schema::connections::id))
    //                     .do_update()
    //                     .set(value)
    //                     .execute(conn)
    //                     .await
    //                     .map(|_| ())
    //             }
    //         }
    //         _ => {
    //             log::error!(
    //                 "Upserting record with id for table {:?} is not implemented",
    //                 self
    //             );
    //             Err(DieselError::RollbackTransaction)
    //         }
    //     }
    // }

    pub async fn upsert_record_with_id(
        &self,
        conn: &mut AsyncPgConnection,
        record: Value,
    ) -> Result<(), DieselError> {
        generate_upsert_record_match!(
            self,
            conn,
            record,
            Packets,
            PacketModel,
            Connections,
            ConnectionModel // Add other tables and their models here as needed
        )
    }

    //     pub async fn upsert_record_with_id_timestamp(
    //         &self,
    //         conn: &mut AsyncPgConnection,
    //         record: Value,
    //     ) -> Result<(), DieselError> {
    //         let has_version = record.get("version").is_some();
    //         match self {
    //             Table::Packets => {
    //                 let value: PacketModel = serde_json::from_value(record).map_err(|e| {
    //                     println!("Deserialization error: {:?}", e);
    //                     println!("Failed to deserialize record:");
    //                     DieselError::DeserializationError(Box::new(e))
    //                 })?;

    //                 if (has_version) {
    //                     diesel::insert_into(packets::table())
    //                         .values(value.clone())
    //                         .on_conflict((schema::packets::id, schema::packets::timestamp))
    //                         .do_update()
    //                         .set(schema::packets::version.eq(schema::packets::version + 1))
    //                         .execute(conn)
    //                         .await
    //                         .map(|_| ())
    //                 } else {
    //                     diesel::insert_into(packets::table())
    //                         .values(value.clone())
    //                         .on_conflict((schema::packets::id, schema::packets::timestamp))
    //                         .do_update()
    //                         .set(value)
    //                         .execute(conn)
    //                         .await
    //                         .map(|_| ())
    //                 }
    //             }
    //             Table::Connections => {
    //                 let value: ConnectionModel = serde_json::from_value(record).map_err(|e| {
    //                     println!("Deserialization error: {:?}", e);
    //                     println!("Failed to deserialize record:");
    //                     DieselError::DeserializationError(Box::new(e))
    //                 })?;

    //                 if (has_version) {
    //                     diesel::insert_into(connections::table())
    //                         .values(value.clone())
    //                         .on_conflict((schema::connections::id, schema::connections::timestamp))
    //                         .do_update()
    //                         .set(schema::connections::version.eq(schema::connections::version + 1))
    //                         .execute(conn)
    //                         .await
    //                         .map(|_| ())
    //                 } else {
    //                     diesel::insert_into(connections::table())
    //                         .values(value.clone())
    //                         .on_conflict((schema::connections::id, schema::connections::timestamp))
    //                         .do_update()
    //                         .set(value)
    //                         .execute(conn)
    //                         .await
    //                         .map(|_| ())
    //                 }
    //             }
    //             _ => {
    //                 log::error!(
    //                     "Upserting record with id for table {:?} is not implemented",
    //                     self
    //                 );
    //                 Err(DieselError::RollbackTransaction)
    //             }
    //         }
    //     }
    // }

    // ... existing code ...

    pub async fn upsert_record_with_id_timestamp(
        &self,
        conn: &mut AsyncPgConnection,
        record: Value,
    ) -> Result<(), DieselError> {
        generate_upsert_record_with_timestamp_match!(
            self,
            conn,
            record,
            Packets,
            PacketModel,
            Connections,
            ConnectionModel // Add other tables and their models here as needed
        )
    }
}
