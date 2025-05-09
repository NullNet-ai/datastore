use bytes::Bytes;
use csv::WriterBuilder;
use serde_json::Value;
use tokio_postgres::{Client, NoTls};
use futures::{SinkExt, pin_mut};
use std::env;
use crate::schema::verify::field_exists_in_table;
use crate::structs::structs::RequestBody;

#[derive(Debug)]
pub enum AppError {
    DbConnection(String),
    CopyCommand(String),
    CsvConversion(String),
    DataSend(String),
}

impl From<csv::Error> for AppError {
    fn from(e: csv::Error) -> Self {
        AppError::CsvConversion(e.to_string())
    }
}

impl From<tokio_postgres::Error> for AppError {
    fn from(e: tokio_postgres::Error) -> Self {
        AppError::DbConnection(e.to_string())
    }
}

pub async fn execute_copy(
    client: &Client,
    table_name: &str,
    columns: &[&str],
    csv_data: Vec<u8>,
) -> Result<(), AppError> {
    let stmt = format!(
        "COPY {} ({}) FROM STDIN WITH (FORMAT csv)",
        table_name,
        columns.join(",")
    );

    let sink = client.copy_in(&stmt)
        .await
        .map_err(|e| AppError::CopyCommand(e.to_string()))?;

    pin_mut!(sink);
    sink.send(Bytes::from(csv_data))
        .await
        .map_err(|e| AppError::DataSend(e.to_string()))?;

    sink.close()
        .await
        .map_err(|e| AppError::DataSend(e.to_string()))?;

    Ok(())
}


pub async fn create_connection() -> Result<Client, AppError> {
    let user = env::var("POSTGRES_USER").unwrap_or_else(|_| "admin".to_string());
    let password = env::var("POSTGRES_PASSWORD").unwrap_or_else(|_| "admin".to_string());
    let dbname = env::var("POSTGRES_DB").unwrap_or_else(|_| "nullnet".to_string());
    let host = env::var("POSTGRES_HOST").unwrap_or_else(|_| "localhost".to_string());
    let port = env::var("POSTGRES_PORT").unwrap_or_else(|_| "5433".to_string());

    let connection_string = format!(
        "host={} port={} user={} password={} dbname={}",
        host, port, user, password, dbname
    );

    let (client, connection) = tokio_postgres::connect(&connection_string, NoTls)
        .await
        .map_err(|e| AppError::DbConnection(e.to_string()))?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            log::error!("PostgreSQL connection error: {}", e);
        }
    });

    Ok(client)
}

pub fn process_records(
    records: Vec<Value>,
    table_name: &str,
) -> Result<(Vec<Value>, Vec<String>), String> {
    let hypertable_exists = field_exists_in_table(table_name, "hypertable_timestamp");
    let mut processed_records = Vec::new();

    for mut record in records {
        // Run your custom processing logic
        let mut request_body = RequestBody { record };
        request_body.process_record("create");

        // Hypertable timestamp logic
        if hypertable_exists {
            if let Some(obj) = request_body.record.as_object_mut() {
                if let Some(timestamp) = obj.get("timestamp") {
                    obj.insert("hypertable_timestamp".to_string(), timestamp.clone());
                }
            }
        }

        processed_records.push(request_body.record);
    }

    let columns = extract_columns_owned(&processed_records)?;
    Ok((processed_records, columns))
}

fn extract_columns_owned(records: &[Value]) -> Result<Vec<String>, String> {
    records.first()
        .and_then(|v| v.as_object())
        .map(|obj| obj.keys().map(|k| k.to_string()).collect())  // Use to_string() to create owned Strings
        .ok_or_else(|| "Invalid record format".to_string())
}

pub fn convert_json_to_csv(records: &[Value], columns: &[String]) -> Result<Vec<u8>, AppError> {
    let mut wtr = WriterBuilder::new().has_headers(false).from_writer(vec![]);
    
    for record in records {
        let obj = record.as_object().ok_or_else(|| {
            AppError::CsvConversion("Invalid record format".to_string())
        })?;

        let row: Vec<String> = columns.iter().map(|col| {
            serialize_value(obj.get(col).unwrap_or(&Value::Null))
        }).collect();

        wtr.write_record(&row).map_err(|e| AppError::CsvConversion(e.to_string()))?;
    }

    wtr.into_inner().map_err(|e| AppError::CsvConversion(e.to_string()))
}

fn extract_columns(records: &[Value]) -> Result<Vec<&str>, String> {
    records.first()
        .and_then(|v| v.as_object())
        .map(|obj| obj.keys().map(|k| k.as_str()).collect())
        .ok_or_else(|| "Invalid record format".to_string())
}


fn serialize_value(value: &Value) -> String {
    match value {
        Value::String(s) => s.clone(),
        Value::Null => String::new(),
        Value::Number(n) => n.to_string(),
        Value::Bool(b) => b.to_string(),
        Value::Array(arr) => format!("{{{}}}", arr.iter()
            .map(serialize_value)
            .collect::<Vec<_>>()
            .join(",")),
        Value::Object(_) => serde_json::to_string(value).unwrap_or_default(),
    }
}