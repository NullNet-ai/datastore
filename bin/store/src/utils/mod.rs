// use diesel_async::AsyncPgConnection;
// use crate::db::AsyncDbPool;
// use futures::future::join_all;
// use serde_json::Value;
// use std::error::Error;
// use std::io::Write;
// use chrono::{DateTime, Utc};
// use tokio_postgres::binary_copy::BinaryCopyInWriter;
// use tokio_postgres::types::Type;

// pub async fn copy_data_batches(
//     table_name: &str,
//     batches: &[Vec<Value>],
//     table_columns: &[String],
//     pool: AsyncDbPool,
// ) -> Result<usize, Box<dyn Error>> {
//     // Pre-acquire all needed connections before processing
//     let mut clients = Vec::new();
//     for _ in 0..batches.len() {
//         clients.push(pool.get().await?);
//     }

//     let results = join_all(
//         batches
//             .iter()
//             .enumerate()
//             .map(|(index, batch)| {
//                 let client = &clients[index];
//                 let table_name = table_name.to_string();
//                 let batch = batch.clone();
//                 let table_columns = table_columns.clone();
                
//                 async move {
//                     process_batch_with_client(client, &table_name, &batch, &table_columns).await
//                 }
//             })
//     ).await;

//     // Count total processed records and check for errors
//     let mut total_count = 0;
//     for result in results {
//         match result {
//             Ok(count) => total_count += count,
//             Err(e) => return Err(e),
//         }
//     }

//     Ok(total_count)
// }


// pub async fn process_batch_with_client(
//     client: &mut AsyncPgConnection,
//     table_name: &str,
//     json_data: &[Value],
//     table_columns: &[String],
// ) -> Result<usize, Box<dyn Error>> {
//     let transaction = client.transaction().await?;
    
//     let columns_str = table_columns
//         .iter()
//         .map(|col| format!("\"{}\"", col))
//         .collect::<Vec<_>>()
//         .join(",");
    
//     let copy_query = format!("COPY {} ({}) FROM STDIN CSV HEADER", table_name, columns_str);
    
//     let copy_sink = transaction.copy_in(&copy_query).await?;
//     let mut writer = BinaryCopyInWriter::new(copy_sink, &[]);
    
//     // Generate CSV header
//     let header = table_columns.join(",") + "\n";
//     writer.write_all(header.as_bytes())?;
    
//     // Write each record as CSV
//     for record in json_data {
//         let line = generate_csv_line(record, table_columns)?;
//         writer.write_all(line.as_bytes())?;
//         writer.write_all(b"\n")?;
//     }
    
//     writer.finish().await?;
//     transaction.commit().await?;
    
//     Ok(json_data.len())
// }

// fn generate_csv_line(record: &Value, columns: &[String]) -> Result<String, Box<dyn Error>> {
//     let record_obj = record.as_object().ok_or("Record is not an object")?;
    
//     let values: Vec<String> = columns
//         .iter()
//         .map(|key| {
//             let value = record_obj.get(key);
            
//             match value {
//                 Some(Value::Null) => "null".to_string(),
//                 None => "".to_string(),
//                 Some(Value::String(s)) => {
//                     let key_lower = key.to_lowercase();
//                     if key_lower.contains("timestamp") {
//                         // Try to parse as DateTime and format as ISO string
//                         if let Ok(dt) = s.parse::<DateTime<Utc>>() {
//                             return format!("\"{}\"", dt.to_rfc3339());
//                         }
//                     }
//                     // Escape quotes in CSV format (double them)
//                     format!("\"{}\"", s.replace("\"", "\"\""))
//                 },
//                 Some(Value::Number(n)) => n.to_string(),
//                 Some(Value::Bool(b)) => b.to_string(),
//                 Some(Value::Array(_)) | Some(Value::Object(_)) => {
//                     // Serialize complex objects to JSON strings
//                     let json_str = serde_json::to_string(value.unwrap()).unwrap_or_default();
//                     format!("\"{}\"", json_str.replace("\"", "\"\""))
//                 }
//             }
//         })
//         .collect();
    
//     Ok(values.join(","))
// }