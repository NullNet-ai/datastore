use crate::models::packet_model::Packet;
use crate::schema::schema;
use crate::schema::schema::packets::dsl::packets;
use diesel::associations::HasTable;
use diesel::result::Error as DieselError;
use diesel_async::AsyncPgConnection;
use actix_web::{ web};
use crate::structs::structs::{ CreateRequestBody};
use serde_json::{Value, Map};
use diesel_async::RunQueryDsl;

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

    pub async fn insert_record(
        &self,
        conn: &mut AsyncPgConnection,
        record: Value,
        request:web::Json<CreateRequestBody>,
    ) -> Result<String, DieselError> {
    let mut request = request.into_inner();
        request.process_record("create");

        match self {
            Table::Packets => {
                let mut value: Packet = serde_json::from_value(record)
                    .map_err(|e| DieselError::DeserializationError(Box::new(e)))?;
                value.hypertable_timestamp = value.timestamp.to_string();
                let result = diesel::insert_into(packets::table())
                    .values(value)
                    .get_result::<Packet>(conn).await?;
                Ok(serde_json::to_string(&result).unwrap_or_else(|_| "{}".to_string()))
            }
            _ => panic!(),
        }
    }

    pub async fn upsert_record_with_id(
        &self,
        conn: &mut AsyncPgConnection,
        record: Value,
    ) -> Result<(), DieselError> {
        match self {
            Table::Packets => {
                let mut value: Packet = serde_json::from_value(record)
                    .map_err(|e| DieselError::DeserializationError(Box::new(e)))?;

                diesel::insert_into(packets::table())
                    .values(value.clone())
                    .on_conflict((schema::packets::id, schema::packets::timestamp))
                    .do_update()
                    .set(value)
                    .execute(conn).await
                    .map(|_| ())
            }
            _ => panic!(),
        }
    }

    pub async fn upsert_record_with_id_timestamp(
        &self,
        conn: &mut AsyncPgConnection,
        record: Value,
    ) -> Result<(), DieselError> {
        match self {
            Table::Packets => {
                println!("Upserting record: {:?}", record); // Add this line for debugg
                let value: Packet = serde_json::from_value(record)
                .map_err(|e| {
                    println!("Deserialization error: {:?}", e);
                    println!("Failed to deserialize record:");
                    DieselError::DeserializationError(Box::new(e))
                })?;

                diesel::insert_into(packets::table())
                    .values(value.clone())
                    .on_conflict((schema::packets::id, schema::packets::timestamp))
                    .do_update()
                    .set(value)
                    .execute(conn).await
                    .map(|_| ())
            }
            _ => panic!(),
        }
    }
}
