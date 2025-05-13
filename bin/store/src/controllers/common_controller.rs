use bytes::Bytes;
use csv::WriterBuilder;
use serde_json::Value;
use tokio_postgres::{Client};
use futures::{SinkExt, pin_mut};
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


//BATCH UPDATE FUNCTIONS


pub fn convert_params_to_sql_types(params: &[serde_json::Value]) -> Vec<Box<dyn tokio_postgres::types::ToSql + Sync>> {
    let mut converted_values: Vec<Box<dyn tokio_postgres::types::ToSql + Sync>> = Vec::new();
    
    for p in params {
        let boxed_value: Box<dyn tokio_postgres::types::ToSql + Sync> = match p {
            serde_json::Value::Null => Box::new(None::<String>),
            serde_json::Value::Bool(b) => Box::new(*b),
            serde_json::Value::Number(n) => {
                if n.is_i64() {
                    let i_val = n.as_i64().unwrap();
                    if i_val >= i32::MIN as i64 && i_val <= i32::MAX as i64 {
                        Box::new(i_val as i32)
                    } else {
                        Box::new(i_val)
                    }
                } else if n.is_u64() {
                    let u_val = n.as_u64().unwrap();
                    if u_val <= i32::MAX as u64 {
                        Box::new(u_val as i32)
                    } else if u_val <= i64::MAX as u64 {
                        Box::new(u_val as i64)
                    } else {
                        Box::new(u_val.to_string())
                    }
                } else {
                    Box::new(n.as_f64().unwrap())
                }
            },
            serde_json::Value::String(s) => Box::new(s.clone()),
            serde_json::Value::Array(arr) => {
                // Convert array elements to Vec<String>
                let string_array: Vec<String> = arr.iter()
                    .map(|v| match v {
                        serde_json::Value::String(s) => s.clone(),
                        _ => v.to_string(),
                    })
                    .collect();
                Box::new(string_array)
            },
            _ => Box::new(format!("{}", p)),
        };
        converted_values.push(boxed_value);
    }
    
    converted_values
}

pub fn process_result_rows(
    rows: &[tokio_postgres::Row], 
    update_fields: &[(&String, &serde_json::Value)],
    is_hypertable: bool
) -> Vec<serde_json::Value> {
    let mut result_rows: Vec<serde_json::Value> = Vec::new();
    
    for row in rows {
        let mut obj = serde_json::Map::new();
        
        if let Ok(id) = row.try_get::<_, String>("id") {
            obj.insert("id".to_string(), serde_json::Value::String(id));
        }
    
        if let Ok(version) = row.try_get::<_, i32>("version") {
            obj.insert("version".to_string(), serde_json::Value::Number(version.into()));
        }

        if is_hypertable {
            if let Ok(timestamp) = row.try_get::<_, String>("hypertable_timestamp") {
                obj.insert("hypertable_timestamp".to_string(), serde_json::Value::String(timestamp));
            }
        }
    
        for (key, value) in update_fields.iter() {
            obj.insert(key.to_string(), (*value).clone());
        }
    
        result_rows.push(serde_json::Value::Object(obj));
    }
    
    result_rows
}