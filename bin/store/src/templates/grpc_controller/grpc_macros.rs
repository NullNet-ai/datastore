#[macro_export]
macro_rules! generate_create_method {
    ($table:ident) => {
        paste::paste! {
            fn [<create_ $table:lower>]<'life0, 'async_trait>(
                &'life0 self,
                request: Request<[<Create $table:camel Request>]>,
            ) -> Pin<Box<dyn std::future::Future<Output = Result<Response<[<Create $table:camel Response>]>, Status>> + Send + 'async_trait>>
            where
                'life0: 'async_trait,
                Self: 'async_trait,
            {
                Box::pin(async move {
                    let request = request.into_inner();
                    let query = match request.query {
                        Some(q) => q,
                        None => return Err(Status::invalid_argument("Query is required")),
                    };
                    let params = match request.params {
                        Some(p) => p,
                        None => return Err(Status::invalid_argument("Params are required")),
                    };
                    let table_name = params.table;
                    let record = match request.[<$table:lower>] {
                        Some(r) => r,
                        None => return Err(Status::invalid_argument("Record is required")),
                    };

                    let record_value = process_record_for_insert(record, &table_name).await?;

                    if let Err(e) = insert(&table_name, record_value.clone()).await {
                        return Err(Status::internal(format!("Failed to insert record: {}", e)));
                    }

                    let pluck_fields: Vec<String> = query
                        .pluck
                        .split(',')
                        .map(|s| s.trim().to_string())
                        .collect();

                    let data: [<$table:camel>] = serde_json::from_value(record_value)
                        .map_err(|e| Status::internal(format!("Failed to process record: {}", e)))?;

                    let response = [<Create $table:camel Response>] {
                        success: true,
                        message: format!("Record inserted into '{}'", &table_name),
                        count: 1,
                        data: Some(data),
                    };

                    Ok(Response::new(response))
                })
            }
        }
    };
}

#[macro_export]
macro_rules! generate_update_method {
    ($table:ident, $table_singular:ident) => {
        paste::paste! {
            fn [<update_ $table:lower>]<'life0, 'async_trait>(
                &'life0 self,
                request: Request<[<Update $table:camel Request>]>,
            ) -> Pin<Box<dyn std::future::Future<Output = Result<Response<[<Update $table:camel Response>]>, Status>> + Send + 'static>>
            where
                'life0: 'async_trait,
                Self: 'async_trait,
             {
                Box::pin(async move {
                    let request = request.into_inner();
                    let query = match request.query {
                        Some(q) => q,
                        None => return Err(Status::invalid_argument("Query is required")),
                    };

                    let params = match request.params {
                        Some(p) => p,
                        None => return Err(Status::invalid_argument("Params are required")),
                    };
                    let table_name = params.table;
                    let record_id = params.id;
                    let record = match request.[<$table_singular:lower>] {
                        Some(r) => r,
                        None => return Err(Status::invalid_argument("Record is required")),
                    };

                    let table = match Table::from_str(table_name.as_str()) {
                        Some(t) => t,
                        None => {
                            return Err(Status::invalid_argument(format!(
                                "Table '{}' does not exist",
                                table_name
                            )))
                        }
                    };

                    // Process record using common function
                    let processed_record = match process_record_for_update(record, &table_name, &record_id, &table).await {
                        Ok(processed) => processed,
                        Err(status) => {
                            return Err(status);
                        }
                    };

                    let pluck_fields: Vec<String> = query
                        .pluck
                        .split(',')
                        .map(|s| s.trim().to_string())
                        .collect();

                    if let Err(e) = update(&table_name, processed_record.clone(), &record_id).await {
                        return Err(Status::internal(format!("Failed to update record: {}", e)));
                    }

                    let data: [<$table:camel>] = serde_json::from_value(processed_record)
                        .map_err(|e| Status::internal(format!("Failed to process record: {}", e)))?;

                    let response = [<Update $table:camel Response>] {
                        data: Some(data),
                        message: format!("Record updated in '{}'", &table_name),
                        count: 1,
                        success: true,
                    };

                    Ok(Response::new(response))
                })
            }
        }
    };
}

#[macro_export]
macro_rules! generate_batch_insert_method {
    ($table:ident) => {
        paste::paste! {
            fn [<batch_insert_ $table:lower>]<'life0, 'async_trait>(
                &'life0 self,
                request: Request<[<BatchInsert $table:camel Request>]>,
            ) -> Pin<Box<dyn std::future::Future<Output = Result<Response<[<BatchInsert $table:camel Response>]>, Status>> + Send + 'async_trait>>
            where
                'life0: 'async_trait,
                Self: 'async_trait,
            {
                Box::pin(async move {
                    let request = request.into_inner();
                    let params = match request.params {
                        Some(p) => p,
                        None => return Err(Status::invalid_argument("Params are required")),
                    };
                    let table_name = params.table;
                    let records = match request.body {
                        Some(batch_body) => batch_body.$table,
                        None => return Err(Status::invalid_argument(format!("No {} provided", stringify!($table)))),
                    };

                    if records.is_empty() {
                        return Err(Status::invalid_argument("No records provided"));
                    }

                    let json_records: Vec<serde_json::Value> = records
                        .into_iter()
                        .map(|record| serde_json::to_value(&record))
                        .collect::<Result<Vec<_>, _>>()
                        .map_err(|e| Status::internal(format!("Failed to process records: {}", e)))?;

                    // Process records using common controller method
                    let (processed_records, columns) = match process_records(json_records, &table_name) {
                        Ok((records, cols)) => (records, cols),
                        Err(e) => return Err(Status::internal(format!("Error processing records: {}", e))),
                    };

                    // Convert JSON to CSV
                    let csv_data = match convert_json_to_csv(&processed_records, &columns) {
                        Ok(data) => data,
                        Err(e) => {
                            return Err(Status::internal(format!(
                                "Error converting records to CSV: {:?}",
                                e
                            )))
                        }
                    };

                    // Create database connection
                    let client = match create_connection().await {
                        Ok(client) => client,
                        Err(e) => {
                            return Err(Status::internal(format!(
                                "Error creating database connection: {:?}",
                                e
                            )))
                        }
                    };

                    // Convert Vec<String> to Vec<&str> for execute_copy
                    let columns_refs: Vec<&str> = columns.iter().map(|s| s.as_str()).collect();

                    // Execute COPY command
                    if let Err(e) = execute_copy(&client, &table_name, &columns_refs, csv_data).await {
                        return Err(Status::internal(format!(
                            "Error executing COPY command: {:?}",
                            e
                        )));
                    }

                    // Send sync messages for each record
                    for record in processed_records.iter() {
                        if let Err(e) = crate::batch_sync::BatchSyncService::send_insert_message(
                            table_name.clone(),
                            record.clone(),
                        )
                        .await
                        {
                            return Err(Status::internal(format!("Sync error: {}", e)));
                        }
                    }

                    // Convert processed records back to protobuf messages
                    let response_records: Vec<[<$table:camel>]> = processed_records
                        .into_iter()
                        .map(|record| serde_json::from_value(record))
                        .collect::<Result<Vec<_>, _>>()
                        .map_err(|e| Status::internal(format!("Failed to convert records: {}", e)))?;

                    // Create response
                    let response = [<BatchInsert $table:camel Response>] {
                        success: true,
                        message: format!(
                            "Inserted {} records into '{}'",
                            response_records.len(),
                            table_name
                        ),
                        count: response_records.len() as i32,
                        data: response_records,
                    };

                    Ok(Response::new(response))
                })
            }
        }
    };
}

#[macro_export]
macro_rules! generate_batch_update_method {
    ($table:ident) => {
        paste::paste! {
            fn [<batch_update_ $table:lower>]<'life0, 'async_trait>(
                &'life0 self,
                request: Request<[<BatchUpdate $table:camel Request>]>,
            ) -> Pin<Box<dyn std::future::Future<Output = Result<Response<[<BatchUpdate $table:camel Response>]>, Status>> + Send + 'async_trait>>
            where
                'life0: 'async_trait,
                Self: 'async_trait,
            {
                Box::pin(async move {
                    let request = request.into_inner();
                    let params = request
                        .params
                        .ok_or_else(|| Status::invalid_argument("Params are required"))?;
                    let body = request
                        .body
                        .ok_or_else(|| Status::invalid_argument("Body is required"))?;
                    let updates = body
                        .updates
                        .ok_or_else(|| Status::invalid_argument("Updates are required"))?;

                    let filters: Vec<Value> = body
                        .advance_filters
                        .into_iter()
                        .map(|mut filter| {
                            let mut value = serde_json::to_value(filter).unwrap_or_default();
                            if let Value::Object(ref mut map) = value {
                                if let Some(Value::String(s)) = map.get_mut("values") {
                                    if let Ok(parsed) = serde_json::from_str::<Value>(s) {
                                        if parsed.is_array() {
                                            *map.get_mut("values").unwrap() = parsed;
                                        }
                                    }
                                }
                            }
                            value
                        })
                        .collect();

                    let updates_map = match serde_json::to_value(&updates) {
                        Ok(Value::Object(map)) => map,
                        Ok(_) => return Err(Status::invalid_argument("Updates must be a JSON object")),
                        Err(e) => {
                            return Err(Status::internal(format!(
                                "Failed to convert data to JSON: {}",
                                e
                            )))
                        }
                    };

                    let mut updates_value = sanitize_updates(updates_map)
                        .ok_or_else(|| Status::invalid_argument("No valid fields to update"))?;

                    // Remove all null values from updates
                    if let Value::Object(ref mut map) = updates_value {
                        // Create a list of keys to remove (can't remove while iterating)
                        let null_keys: Vec<String> = map
                            .iter()
                            .filter(|(_, v)| v.is_null())
                            .map(|(k, _)| k.clone())
                            .collect();

                        // Remove all null values
                        for key in null_keys {
                            map.remove(&key);
                        }
                    }

                    let (count, _) = perform_batch_update(&params.table, updates_value, filters)
                        .await
                        .map_err(Status::internal)?;

                    let response = [<BatchUpdate $table:camel Response>] {
                        success: true,
                        message: format!("Updated {} records in '{}'", count, params.table),
                        count: count as i32,
                        data: vec![],
                    };

                    Ok(Response::new(response))
                })
            }
        }
    };
}

#[macro_export]
macro_rules! generate_get_method {
    ($table:ident) => {
        paste::paste! {
            fn [<get_ $table:lower>]<'life0, 'async_trait>(
                &'life0 self,
                request: Request<[<Get $table:camel Request>]>,
            ) -> Pin<Box<dyn std::future::Future<Output = Result<Response<[<Get $table:camel Response>]>, Status>> + Send + 'async_trait>>
            where
                'life0: 'async_trait,
                Self: 'async_trait,
            {
                Box::pin(async move {
                    // Implementation for Get method
                    todo!()
                })
            }
        }
    };
}

#[macro_export]
macro_rules! generate_delete_method {
    ($table:ident) => {
        paste::paste! {
            fn [<delete_ $table:lower>]<'life0, 'async_trait>(
                &'life0 self,
                request: Request<[<Delete $table:camel Request>]>,
            ) -> Pin<Box<dyn std::future::Future<Output = Result<Response<[<Delete $table:camel Response>]>, Status>> + Send + 'async_trait>>
            where
                'life0: 'async_trait,
                Self: 'async_trait,
            {
                Box::pin(async move {
                    // Implementation for Delete method
                    todo!()
                })
            }
        }
    };
}

#[macro_export]
macro_rules! generate_batch_delete_method {
    ($table:ident) => {
        paste::paste! {
            fn [<batch_delete_ $table:lower>]<'life0, 'async_trait>(
                &'life0 self,
                request: Request<[<BatchDelete $table:camel Request>]>,
            ) -> Pin<Box<dyn std::future::Future<Output = Result<Response<[<BatchDelete $table:camel Response>]>, Status>> + Send + 'async_trait>>
            where
                'life0: 'async_trait,
                Self: 'async_trait,
            {
                Box::pin(async move {
                    let request = request.into_inner();
                    let params = request
                        .params
                        .ok_or_else(|| Status::invalid_argument("Params are required"))?;
                    let body = request
                        .body
                        .ok_or_else(|| Status::invalid_argument("Body is required"))?;

                    let mut delete_updates = RequestBody {
                        record: serde_json::json!({}),
                    };

                    // Process the record through the common processing logic
                    delete_updates.process_record("delete");
                    if let Some(record) = delete_updates.record.as_object_mut() {
                        record.remove("version");
                    }

                    let updates = delete_updates.record;

                    let filters: Vec<Value> = body
                        .advance_filters
                        .into_iter()
                        .map(|mut filter| {
                            let mut value = serde_json::to_value(filter).unwrap_or_default();
                            if let Value::Object(ref mut map) = value {
                                if let Some(Value::String(s)) = map.get_mut("values") {
                                    if let Ok(parsed) = serde_json::from_str::<Value>(s) {
                                        if parsed.is_array() {
                                            *map.get_mut("values").unwrap() = parsed;
                                        }
                                    }
                                }
                            }
                            value
                        })
                        .collect();

                    let updates_map = match serde_json::to_value(&updates) {
                        Ok(Value::Object(map)) => map,
                        Ok(_) => return Err(Status::invalid_argument("Updates must be a JSON object")),
                        Err(e) => {
                            return Err(Status::internal(format!(
                                "Failed to convert data to JSON: {}",
                                e
                            )))
                        }
                    };

                    let updates_value = sanitize_updates(updates_map)
                        .ok_or_else(|| Status::invalid_argument("No valid fields to update"))?;

                    let (count, _) = perform_batch_update(&params.table, updates_value, filters)
                        .await
                        .map_err(Status::internal)?;

                    let response = [<BatchDelete $table:camel Response>] {
                        success: true,
                        message: format!("Updated {} records in '{}'", count, params.table),
                        count: count as i32,
                        data: None,
                    };

                    Ok(Response::new(response))
                })
            }
        }
    };
}
