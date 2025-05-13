use crate::models::packet_model::Packet;
use crate::models::connections_model::ConnectionModel;
use crate::schema::schema;
use crate::schema::schema::packets::dsl::packets;
use crate::schema::schema::connections::dsl::connections;
use crate::structs::structs::RequestBody;
use actix_web::web;
use diesel::prelude::*;
use diesel::associations::HasTable;
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

    pub async fn get_hypertable_timestamp(
        &self,
        conn: &mut AsyncPgConnection,
        id: &str,
    ) -> Result<Option<String>, DieselError> {
        match self {
            Table::Packets => {
                
                
                let result = packets
                    .filter(schema::packets::id.eq(id))
                    .select(schema::packets::hypertable_timestamp)
                    .first::<Option<String>>(conn)
                    .await;
                
                result
            },
            Table::Connections => {
                
                let result = connections
                    .filter(schema::connections::id.eq(id))
                    .select(schema::connections::hypertable_timestamp)
                    .first::<Option<String>>(conn)
                    .await;
                
                result
            },
            _ => {
                log::error!("Getting hypertable_timestamp for table {:?} is not implemented", self);
                Err(DieselError::RollbackTransaction)
            }
        }
    }

    pub async fn insert_record(
        &self,
        conn: &mut AsyncPgConnection,
        record: Value,
        request: web::Json<RequestBody>,
    ) -> Result<String, DieselError> {
        let mut request = request.into_inner();
        request.process_record("create");

        match self {
            Table::Packets => {
                let mut value: Packet = serde_json::from_value(record)
                    .map_err(|e| DieselError::DeserializationError(Box::new(e)))?;
                
                value.hypertable_timestamp = Some(value.timestamp.to_string());
                diesel::insert_into(packets::table())
                .values(value.clone())
                .execute(conn) // Use execute instead of get_result
                .await?;
            Ok(serde_json::to_string(&value).unwrap_or_else(|_| "{}".to_string()))
            }
            _ => panic!(),
        }
    }

    pub async fn upsert_record_with_id(
        &self,
        conn: &mut AsyncPgConnection,
        record: Value,
    ) -> Result<(), DieselError> {
        let has_version = record.get("version").is_some();
        match self {
            Table::Packets => {
                let value: Packet = serde_json::from_value(record).map_err(|e| {
                    log::error!("Deserialization error: {:?}", e);
                    DieselError::DeserializationError(Box::new(e))
                })?;
                if(has_version){
                    diesel::insert_into(packets::table())
                    .values(value.clone())
                    .on_conflict((schema::packets::id))
                    .do_update()
                    .set(
                        schema::packets::version.eq(schema::packets::version + 1),
                    )
                    .execute(conn)
                    .await
                    .map(|_| ())

                }
                else{
                    diesel::insert_into(packets::table())
                    .values(value.clone())
                    .on_conflict((schema::packets::id))
                    .do_update()
                    .set(value)
                    .execute(conn)
                    .await
                    .map(|_| ())

                }

                
            },
            Table::Connections => {

                let value: ConnectionModel = serde_json::from_value(record).map_err(|e| {
                    log::error!("Deserialization error: {:?}", e);
                    DieselError::DeserializationError(Box::new(e))
                })?;

                if has_version {
                    diesel::insert_into(connections::table())
                    .values(value.clone())
                    .on_conflict((schema::connections::id))
                    .do_update()
                    .set(
                        schema::connections::version.eq(schema::connections::version + 1)
                    )
                    .execute(conn)
                    .await
                    .map(|_| ())
                }else{

                diesel::insert_into(connections::table())
                    .values(value.clone())
                    .on_conflict((schema::connections::id))
                    .do_update()
                    .set(value)
                    .execute(conn)
                    .await
                    .map(|_| ())}
            }
            _ => {
                log::error!("Upserting record with id for table {:?} is not implemented", self);
                Err(DieselError::RollbackTransaction)
            }
        }
    }

    pub async fn upsert_record_with_id_timestamp(
        &self,
        conn: &mut AsyncPgConnection,
        record: Value,
    ) -> Result<(), DieselError> {
        let has_version = record.get("version").is_some();
        match self {
            Table::Packets => {
                let value: Packet = serde_json::from_value(record).map_err(|e| {
                    println!("Deserialization error: {:?}", e);
                    println!("Failed to deserialize record:");
                    DieselError::DeserializationError(Box::new(e))
                })?;

                if(has_version){
                    diesel::insert_into(packets::table())
                    .values(value.clone())
                    .on_conflict((schema::packets::id, schema::packets::timestamp))
                    .do_update()
                    .set(schema::packets::version.eq(schema::packets::version + 1))
                    .execute(conn)
                    .await
                    .map(|_| ())
                }else{
                    diesel::insert_into(packets::table())
                    .values(value.clone())
                    .on_conflict((schema::packets::id, schema::packets::timestamp))
                    .do_update()
                    .set(value)
                    .execute(conn)
                    .await
                    .map(|_| ())
                }

                
            },
            Table::Connections => {
                let value: ConnectionModel = serde_json::from_value(record).map_err(|e| {
                    println!("Deserialization error: {:?}", e);
                    println!("Failed to deserialize record:");
                    DieselError::DeserializationError(Box::new(e))
                })?;

                if(has_version){
                    diesel::insert_into(connections::table())
                    .values(value.clone())
                    .on_conflict((schema::connections::id, schema::connections::timestamp))
                    .do_update()
                    .set(schema::connections::version.eq(schema::connections::version + 1))
                    .execute(conn)
                    .await
                    .map(|_| ())
                }
                else{
                    diesel::insert_into(connections::table())
                    .values(value.clone())
                    .on_conflict((schema::connections::id, schema::connections::timestamp))
                    .do_update()
                    .set(value)
                    .execute(conn)
                    .await
                    .map(|_| ())
                }

                
            }
            _ => {
                log::error!("Upserting record with id for table {:?} is not implemented", self);
                Err(DieselError::RollbackTransaction)
            },
        }
    }
}
