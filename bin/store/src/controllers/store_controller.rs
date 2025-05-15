use crate::batch_sync::BatchSyncService;
use crate::controllers::common_controller::{convert_json_to_csv, execute_copy, process_records};
use crate::db;
use crate::db::create_connection;
use crate::schema::verify::field_exists_in_table;
use crate::structs::structs::{ApiResponse, BatchUpdateBody, QueryParams, RequestBody};
use crate::sync::sync_service::insert;
use crate::table_enum::Table;
use actix_web::error::BlockingError;
use actix_web::{http, web, HttpResponse, Responder, ResponseError};
use diesel::result::Error as DieselError;
use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;

use super::common_controller::{perform_batch_update, process_record_for_insert, process_record_for_update, sanitize_updates};

#[derive(Serialize)]
struct ApiError {
    message: String,
    status: u16,
}
impl From<BlockingError> for ApiError {
    fn from(error: BlockingError) -> Self {
        ApiError {
            status: error.status_code().as_u16(),
            message: format!("Internal server error: {:?}", error),
        }
    }
}

impl ApiError {
    fn new(status: http::StatusCode, message: impl Into<String>) -> Self {
        Self {
            status: status.as_u16(),
            message: message.into(),
        }
    }
}
impl From<DieselError> for ApiError {
    fn from(error: DieselError) -> Self {
        let status_code = match error {
            DieselError::NotFound => http::StatusCode::NOT_FOUND,
            DieselError::DatabaseError(_, _) => http::StatusCode::BAD_REQUEST,
            DieselError::DeserializationError(_) => http::StatusCode::UNPROCESSABLE_ENTITY,
            DieselError::SerializationError(_) => http::StatusCode::INTERNAL_SERVER_ERROR,
            DieselError::RollbackTransaction => http::StatusCode::INTERNAL_SERVER_ERROR,
            DieselError::AlreadyInTransaction => http::StatusCode::INTERNAL_SERVER_ERROR,
            _ => http::StatusCode::INTERNAL_SERVER_ERROR,
        };
        ApiError {
            status: status_code.as_u16(),
            message: format!("Database error: {}", error),
        }
    }
}

impl From<serde_json::Error> for ApiError {
    fn from(error: serde_json::Error) -> Self {
        Self::new(
            http::StatusCode::UNPROCESSABLE_ENTITY,
            format!("JSON processing error: {}", error),
        )
    }
}

pub async fn update_record(
    pool: web::Data<db::AsyncDbPool>,
    path_params: web::Path<(String, String)>,
    request: web::Json<RequestBody>,
    query: web::Query<QueryParams>,
) -> impl Responder {
    let (table_name, record_id) = path_params.into_inner();
    let table = match Table::from_str(table_name.as_str()) {
        Some(t) => t,
        None => {
            return HttpResponse::BadRequest().json(ApiError {
                status: http::StatusCode::BAD_REQUEST.as_u16(),
                message: format!("Unknown table: {}", table_name),
            });
        }
    };

    // Process the record using the common function
    let processed_record = match process_record_for_update(
        request.into_inner().record,
        &table_name,
        &record_id,
        &table,
    )
    .await
    {
        Ok(record) => record,
        Err(status) => {
            return HttpResponse::InternalServerError().json(ApiResponse {
                success: false,
                message: status.message().to_string(),
                count: 0,
                data: vec![],
            });
        }
    };

    let pluck_fields: Vec<String> = if query.pluck.is_empty() {
        vec!["id".to_string()]
    } else {
        query
            .pluck
            .split(',')
            .map(|s| s.trim().to_string())
            .collect()
    };

    // Use the update function from sync_service
    if let Err(e) = crate::sync::sync_service::update(&table_name, processed_record.clone(), &record_id).await {
        return HttpResponse::InternalServerError().json(ApiResponse {
            message: format!("Sync error: {}", e),
            success: false,
            count: 0,
            data: vec![],
        });
    }

    let plucked_record = table.pluck_fields(&processed_record, pluck_fields);

    let response = ApiResponse {
        success: true,
        message: format!("Record updated in '{}'", &table_name),
        count: 1,
        data: vec![plucked_record],
    };

    HttpResponse::Ok().json(response)
}

pub async fn create_record(
    pool: web::Data<db::AsyncDbPool>,
    table: web::Path<String>,
    request: web::Json<RequestBody>,
    query: web::Query<QueryParams>,
) -> impl Responder {
    let table_name = table.into_inner();
    let log_table = table_name.clone();

    // Process the record using common function
    let record_value = match process_record_for_insert(request.record.clone(), &table_name).await {
        Ok(val) => val,
        Err(e) => {
            return HttpResponse::InternalServerError().json(ApiResponse {
                success: false,
                message: format!("Failed to process record: {}", e),
                count: 0,
                data: vec![],
            });
        }
    };

    let pluck_fields: Vec<String> = query
        .pluck
        .split(',')
        .map(|s| s.trim().to_string())
        .collect();

    let table = match Table::from_str(table_name.as_str()) {
        Some(t) => t,
        None => {
            return HttpResponse::BadRequest().json(ApiResponse {
                success: false,
                message: format!("Unknown table: {log_table}"),
                count: 0,
                data: vec![],
            });
        }
    };

    if let Err(e) = insert(&log_table, record_value.clone()).await {
        return HttpResponse::InternalServerError().json(ApiResponse  {
            success: false,
            message: format!("Sync error: {}", e),
            count: 0,
            data: vec![],
        });
    }

    let plucked_record = table.pluck_fields(&record_value, pluck_fields);

    let response = ApiResponse {
        success: true,
        message: format!("Record inserted into '{}'", &log_table),
        count: 1,
        data: vec![plucked_record],
    };
    HttpResponse::Ok().json(response)
}

#[derive(Deserialize)]
pub struct BatchInsertBody {
    entity_prefix: Option<String>,
    records: Vec<Value>,
}

pub async fn batch_insert_records(
    table: web::Path<String>,
    records: web::Json<BatchInsertBody>,
) -> impl Responder {
    let table_name = table.into_inner();
    let table_clone = table_name.clone();
    let batch_data = records.into_inner();
    let json_records = batch_data.records;

    if json_records.is_empty() {
        return HttpResponse::BadRequest().json(ApiResponse {
            success: false,
            message: "No records provided".to_string(),
            count: 0,
            data: vec![],
        });
    }
    let (processed_records, columns) = match process_records(json_records, &table_name) {
        Ok((records, cols)) => (records, cols),
        Err(e) => {
            return HttpResponse::BadRequest().json(ApiResponse {
                success: false,
                message: format!("Error processing records: {}", e),
                count: 0,
                data: vec![],
            })
        }
    };

    let csv_data = match convert_json_to_csv(&processed_records, &columns) {
        Ok(data) => data,
        Err(e) => {
            return HttpResponse::BadRequest().json(ApiResponse {
                success: false,
                message: format!("Error converting records to CSV: {:?}", e),
                count: 0,
                data: vec![],
            })
        }
    };

    let client = match create_connection().await {
        Ok(client) => client,
        Err(e) => {
            return HttpResponse::InternalServerError().json(ApiResponse {
                success: false,
                message: format!("Error creating database connection: {:?}", e),
                count: 0,
                data: vec![],
            })
        }
    };

    let column_refs: Vec<&str> = columns.iter().map(|s| s.as_str()).collect();

    let records = match execute_copy(&client, &table_name, &column_refs, csv_data).await {
        Ok(_) => processed_records.clone(),
        Err(e) => {
            return HttpResponse::InternalServerError().json(ApiResponse {
                success: false,
                message: format!("Error executing COPY command: {:?}", e),
                count: 0,
                data: vec![],
            })
        }
    };

    // Convert JSON array to CSV in-memor

    for record in processed_records.iter() {
        if let Err(e) =
            BatchSyncService::send_insert_message(table_clone.clone(), record.clone()).await
        {
            return HttpResponse::InternalServerError().json(ApiResponse {
                success: false,
                message: format!("Sync error: {e}"),
                count: 0,
                data: vec![],
            });
        }
    }

    let response = ApiResponse {
        success: true,
        message: format!(
            "Inserted {} records into '{}'",
            processed_records.len(),
            table_name
        ),
        count: processed_records.len() as i32,
        data: processed_records, // Include the processed records in the response
    };

    HttpResponse::Ok().json(response)
}

pub async fn batch_update_records(
    pool: web::Data<db::AsyncDbPool>,
    table: web::Path<String>,
    request: web::Json<BatchUpdateBody>,
) -> impl Responder {
    let table_name = table.into_inner();
    let batch_data = request.into_inner();
    let filters = batch_data.advance_filters;
    let mut updates = batch_data.updates;
    if updates
        .record
        .as_object()
        .map_or(true, |obj| obj.is_empty())
    {
        return HttpResponse::BadRequest().json(ApiResponse {
            success: false,
            message: "No update fields provided".to_string(),
            count: 0,
            data: vec![],
        });
    }
    updates.process_record("update");
    if let Some(record) = updates.record.as_object_mut() {
        record.remove("version");
    }
    let updates_value = match serde_json::to_value(updates) {
        Ok(Value::Object(map)) => {
            sanitize_updates(map).unwrap_or(Value::Object(Default::default()))
        }
        Ok(_) => Value::Object(Default::default()),
        Err(e) => {
            log::error!("Failed to serialize updates to JSON: {}", e);
            return HttpResponse::InternalServerError().json(ApiResponse {
                success: false,
                message: format!("Failed to process update data: {}", e),
                count: 0,
                data: vec![],
            });
        }
    };

    if updates_value.as_object().map_or(true, |o| o.is_empty()) {
        return HttpResponse::BadRequest().json(ApiResponse {
            success: false,
            message: "No valid fields to update".to_string(),
            count: 0,
            data: vec![],
        });
    }

    match perform_batch_update(&table_name, updates_value, filters).await {
        Ok((count, _)) => HttpResponse::Ok().json(ApiResponse {
            success: true,
            message: format!("Updated {} records in '{}'", count, table_name),
            count: count as i32,
            data: vec![],
        }),
        Err(e) => HttpResponse::InternalServerError().json(ApiResponse {
            success: false,
            message: e,
            count: 0,
            data: vec![],
        }),
    }

    //use the below code if you want to return the updated fields to the user, can be inefficient if the updated fields are large

    //print rows here
    // let mut json_rows: Vec<serde_json::Value> = Vec::new();
    // for row in &rows {
    //     let mut json_obj = serde_json::Map::new();

    //     // Extract id
    //     if let Ok(id) = row.try_get::<_, String>("id") {
    //         json_obj.insert("id".to_string(), serde_json::Value::String(id));
    //     }

    //     // Extract version
    //     if let Ok(version) = row.try_get::<_, i32>("version") {
    //         json_obj.insert("version".to_string(), serde_json::Value::Number(serde_json::Number::from(version)));
    //     }

    //     // Extract updated_date
    //     if let Ok(updated_date) = row.try_get::<_, String>("updated_date") {
    //         json_obj.insert("updated_date".to_string(), serde_json::Value::String(updated_date));
    //     }

    //     // Extract updated_time
    //     if let Ok(updated_time) = row.try_get::<_, String>("updated_time") {
    //         json_obj.insert("updated_time".to_string(), serde_json::Value::String(updated_time));
    //     }

    //     // Extract updated_by
    //     if let Ok(updated_by) = row.try_get::<_, String>("updated_by") {
    //         json_obj.insert("updated_by".to_string(), serde_json::Value::String(updated_by));
    //     }

    //     // Extract hypertable_timestamp if it exists
    //     if field_exists_in_table(&table_name, "hypertable_timestamp") {
    //         if let Ok(timestamp) = row.try_get::<_, String>("hypertable_timestamp") {
    //             json_obj.insert("hypertable_timestamp".to_string(), serde_json::Value::String(timestamp));
    //         }
    //     }

    //     // Extract any updated fields
    //     if let Some(update_obj) = updates.as_object() {
    //         for key in update_obj.keys() {
    //             if key != "record" {
    //                 // Try to get the value as different types
    //                 if let Ok(val) = row.try_get::<_, String>(key.as_str()) {
    //                     json_obj.insert(key.clone(), serde_json::Value::String(val));
    //                 } else if let Ok(val) = row.try_get::<_, i32>(key.as_str()) {
    //                     json_obj.insert(key.clone(), serde_json::Value::Number(serde_json::Number::from(val)));
    //                 } else if let Ok(val) = row.try_get::<_, i64>(key.as_str()) {
    //                     json_obj.insert(key.clone(), serde_json::Value::Number(serde_json::Number::from(val)));
    //                 } else if let Ok(val) = row.try_get::<_, f64>(key.as_str()) {
    //                     if let Some(num) = serde_json::Number::from_f64(val) {
    //                         json_obj.insert(key.clone(), serde_json::Value::Number(num));
    //                     }
    //                 } else if let Ok(val) = row.try_get::<_, bool>(key.as_str()) {
    //                     json_obj.insert(key.clone(), serde_json::Value::Bool(val));
    //                 }
    //             }
    //         }
    //     }

    //     json_rows.push(serde_json::Value::Object(json_obj));
    // }
}

pub async fn batch_delete_records(
    pool: web::Data<db::AsyncDbPool>,
    table: web::Path<String>,
    request: web::Json<BatchUpdateBody>,
) -> impl Responder {
    let table_name = table.into_inner();
    let batch_data = request.into_inner();
    let filters = batch_data.advance_filters;

    // Create delete updates (setting tombstone and status)
    let mut delete_updates = RequestBody {
        record: serde_json::json!({}),
    };

    // Process the record through the common processing logic
    delete_updates.process_record("delete");
    if let Some(record) = delete_updates.record.as_object_mut() {
        record.remove("version");
    }

    let updates_value = delete_updates.record;

    match perform_batch_update(&table_name, updates_value, filters).await {
        Ok((count, _)) => HttpResponse::Ok().json(ApiResponse {
            success: true,
            message: format!("Deleted {} records in '{}'", count, table_name),
            count: count as i32,
            data: vec![],
        }),
        Err(e) => HttpResponse::InternalServerError().json(ApiResponse {
            success: false,
            message: e,
            count: 0,
            data: vec![],
        }),
    }
}
