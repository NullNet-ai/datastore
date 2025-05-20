use crate::db;
use crate::models::connection_model::ConnectionModel;
use crate::models::counter_model::CounterModel;
use crate::models::device_ssh_key_model::DeviceSshKeyModel;
use crate::models::device_group_setting_model::DeviceGroupSettingModel;
use crate::models::packet_model::PacketModel;
use crate::schema::schema;
use crate::schema::verify::field_exists_in_table;
use crate::structs::structs::{Auth, RequestBody};
use crate::{
    generate_get_by_id_match, generate_hypertable_timestamp_match, generate_insert_record_match,
    generate_upsert_record_match, generate_upsert_record_with_timestamp_match,
};
use actix_web::web;
use diesel::associations::HasTable;
use diesel::prelude::*;
use diesel::result::Error as DieselError;
use diesel_async::AsyncPgConnection;
use diesel_async::RunQueryDsl;
use serde_json::{Map, Value};

#[derive(Debug)]
pub enum Table {
    Packets,
    Connections,
    DeviceSshKeys,
    DeviceGroupSettings,
    // Add other tables here
}

impl Table {
    pub fn from_str(name: &str) -> Option<Self> {
        match name {
            "packets" => Some(Table::Packets),
            "connections" => Some(Table::Connections),
            "device_ssh_keys" => Some(Table::DeviceSshKeys),
            "device_group_settings" => Some(Table::DeviceGroupSettings),
            // Add other tables here
            _ => None,
        }
    }

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
        generate_hypertable_timestamp_match!(self, conn, id, Packets, Connections)
    }

    pub async fn insert_record(
        &self,
        conn: &mut AsyncPgConnection,
        record: Value,
        request: web::Json<RequestBody>,
        auth: &Auth,
    ) -> Result<String, DieselError> {
        generate_insert_record_match!(
            self,
            auth,
            conn,
            record,
            request,
            Packets,
            PacketModel,
            Connections,
            ConnectionModel,
            DeviceSshKeys,
            DeviceSshKeyModel,
            DeviceGroupSettingModel
        )
    }

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
            ConnectionModel,
            DeviceSshKeys,
            DeviceSshKeyModel,
            DeviceGroupSettingModel // Add other tables and their models here as needed
        )
    }

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
            ConnectionModel,
            DeviceSshKeys,
            DeviceSshKeyModel,
            DeviceGroupSettingModel // Add other tables and their models here as needed
        )
    }

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
            ConnectionModel,
            DeviceSshKeys,
            DeviceSshKeyModel,
            DeviceGroupSettingModel // Add other tables and their models here as needed
        )
    }
}
pub async fn generate_code(
    table: &str,
    prefix_param: &str,
    default_code_param: i32,
) -> Result<String, DieselError> {
    let mut conn = db::get_async_connection().await;

    let new_counter = CounterModel {
        entity: table.to_string(),
        counter: 1,
        prefix: prefix_param.to_string(),
        default_code: default_code_param,
    };

    // Attempt the insert with conflict handling
    let result = diesel::insert_into(schema::counters::dsl::counters::table())
        .values(&new_counter)
        .on_conflict(schema::counters::entity)
        .do_update()
        .set(schema::counters::counter.eq(schema::counters::counter + 1))
        .returning((
            schema::counters::prefix,
            schema::counters::default_code,
            schema::counters::counter,
        ))
        .get_result::<(String, i32, i32)>(&mut conn)
        .await
        .map_err(|e| {
            log::error!("Error generating code: {}", e);
            e
        })?;

    // Format the code
    let (prefix_val, default_code_val, counter_val) = result;
    let code = format!("{}{}", prefix_val, default_code_val + counter_val);

    Ok(code)
}
