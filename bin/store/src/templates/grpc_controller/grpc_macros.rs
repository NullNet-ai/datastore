// Session management macro for automatic session persistence
#[macro_export]
macro_rules! with_session_management {
    ($request:ident, $body:block) => {
        {
            // Load and populate session using centralized function (similar to HTTP middleware)
            let session = crate::middlewares::session_middleware::load_and_populate_session_for_grpc(&$request).await;
            
            // Store session in request extensions for use in business logic before consuming the request
            if let Some(ref session) = session {
                $request.extensions_mut().insert(session.clone());
            }
            
            // Execute the business logic
            let result = $body;
            
            // Save session after processing if it was modified
            if let Some(session) = session {
                if let Err(e) = crate::middlewares::session_middleware::save_session_after_request(&session).await {
                    log::warn!("Failed to save session: {}", e);
                }
            }
            
            result
        }
    };
}

#[macro_export]
macro_rules! generate_create_method {
    ($table:ident) => {
        paste::paste! {
            fn [<create_ $table:lower>]<'life0, 'async_trait>(
                &'life0 self,
                mut request: Request<[<Create $table:camel Request>]>,
            ) -> Pin<Box<dyn std::future::Future<Output = Result<Response<[<Create $table:camel Response>]>, Status>> + Send + 'async_trait>>
            where
                'life0: 'async_trait,
                Self: 'async_trait,
            {
                Box::pin(async move {
                    with_session_management!(request, {

                    // Extract params first to get the type for validation
                    let params = match request.get_ref().params {
                        Some(ref p) => p.clone(),
                        None => return Err(Status::invalid_argument("Params are required")),
                    };
                    
                    // Use common validation function
                    let (auth_data, _claims) = crate::middlewares::auth_middleware::validate_grpc_request_with_root_access(&request, &params.r#type)?;
                    
                    let request_inner = request.into_inner();
                    let query = match request_inner.query {
                        Some(q) => q,
                        None => return Err(Status::invalid_argument("Query is required")),
                    };
                    
                    let table_name = params.table;
                    let record = match request_inner.[<$table:lower>] {
                        Some(r) => r,
                        None => return Err(Status::invalid_argument("Record is required")),
                    };

                    let record_value = serde_json::to_value(&record)
                        .map_err(|e| Status::internal(format!("Failed to convert record to JSON: {}", e)))?;


                    let pluck_fields: Vec<String> = query
                        .pluck
                        .split(',')
                        .map(|s| s.trim().to_string())
                        .collect();

                        match process_and_insert_record(&table_name, record_value, Some(pluck_fields), &auth_data, params.r#type == "root").await {
                            Ok(api_response) => {
                                // Convert the data back to the specific type
                                let data: [<$table:camel>] = serde_json::from_value(api_response.data[0].clone())
                                    .map_err(|e| Status::internal(format!("Failed to convert response: {}", e)))?;

                                let response = [<Create $table:camel Response>] {
                                    success: api_response.success,
                                    message: api_response.message,
                                    count: api_response.count,
                                    data: Some(data),
                                };

                                Ok(Response::new(response))
                            },
                            Err(e) => Err(Status::internal(e.message)),
                        }
                    })
                })
            }
        }
    };
}

#[macro_export]
macro_rules! generate_aggregation_filter_method {
    () => {
        paste::paste! {
            fn aggregation_filter<'life0, 'async_trait>(
                &'life0 self,
                mut request: Request<AggregationFilterRequest>,
            ) -> Pin<Box<dyn std::future::Future<Output = Result<Response<AggregationFilterResponse>, Status>> + Send + 'async_trait>>
            where
                'life0: 'async_trait,
                Self: 'async_trait,
            {
                Box::pin(async move {
                    with_session_management!(request, {
                        let auth_data = match request.extensions().get::<Auth>() {
                        Some(data) => data.clone(),
                        None => {
                            return Err(tonic::Status::internal(
                                "Authentication information not available",
                            ));
                        }
                    };

                    let request = request.into_inner();

                    // Extract params and validate
                    let params = match request.params.as_ref() {
                        Some(p) => p,
                        None => return Err(Status::invalid_argument("Params are required")),
                    };

                    // Get table from body.entity instead of params.table
                    let table = request.body.as_ref()
                        .map(|b| b.entity.clone())
                        .ok_or_else(|| Status::invalid_argument("Body with entity is required"))?;

                    // Extract type from params to determine if it's a root request
                    let request_type = params.r#type.as_str();
                    let is_root_request = request_type == "root";

                    // Create SQLConstructor with organization_id if available
                    let wrapper = crate::providers::find::sql_constructor::AggregationFilterWrapper::new(request);
                    let mut sql_constructor = SQLConstructor::new(wrapper, table.clone(), is_root_request);
                    sql_constructor = sql_constructor.with_organization_id(auth_data.organization_id.clone());

                    let query = match sql_constructor.construct_aggregation() {
                        Ok(sql) => sql,
                        Err(e) => {
                            return Err(Status::invalid_argument(format!(
                                "Invalid aggregation configuration: {}", e
                            )));
                        }
                    };

                    // Get a connection from the pool
                    let mut conn = db::get_async_connection().await;

                    // Wrap your original query with row_to_json
                    let final_query = format!("SELECT row_to_json(t) FROM ({}) t", query);

                    let results = match diesel_async::RunQueryDsl::load::<DynamicResult>(diesel::dsl::sql_query(&final_query), &mut conn)
                        .await
                    {
                        Ok(results) => results,
                        Err(e) => {
                            return Err(Status::internal(format!("Query execution error: {}", e)));
                        }
                    };

                    // Extract JSON values and serialize as a single JSON string
                    let data_values: Vec<serde_json::Value> = results
                        .into_iter()
                        .filter_map(|result| result.row_to_json)
                        .collect();

                    // Serialize the entire array as a single JSON string
                    let data = serde_json::to_string(&data_values)
                        .unwrap_or_else(|_| "[]".to_string());

                    let response = AggregationFilterResponse {
                        success: true,
                        message: format!("Aggregation operation completed for table: {}", &table),
                        count: data_values.len() as i32,
                        data,
                    };

                        Ok(Response::new(response))
                    })
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
                mut request: Request<[<Update $table:camel Request>]>,
            ) -> Pin<Box<dyn std::future::Future<Output = Result<Response<[<Update $table:camel Response>]>, Status>> + Send + 'static>>
            where
                'life0: 'async_trait,
                Self: 'async_trait,
             {
                Box::pin(async move {
                    with_session_management!(request, {
                    // Extract params first to get the type for validation
                    let params = match request.get_ref().params {
                        Some(ref p) => p.clone(),
                        None => return Err(Status::invalid_argument("Params are required")),
                    };
                    
                    // Use common validation function
                    let (auth_data, _claims) = crate::middlewares::auth_middleware::validate_grpc_request_with_root_access(&request, &params.r#type)?;
                    
                    let request_inner = request.into_inner();
                    let query = match request_inner.query {
                        Some(q) => q,
                        None => return Err(Status::invalid_argument("Query is required")),
                    };
                    
                    let table_name = params.table;
                    let record_id = params.id;
                    let record = match request_inner.[<$table_singular:lower>] {
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
                    let processed_record = match process_record_for_update(record, &table_name, &record_id, &table,"update", &auth_data, params.r#type == "root").await {
                        Ok(processed) => processed,
                        Err(status) => {
                            return Err(status);
                        }
                    };

                    let _pluck_fields: Vec<String> = query
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
                mut request: Request<[<BatchInsert $table:camel Request>]>,
            ) -> Pin<Box<dyn std::future::Future<Output = Result<Response<[<BatchInsert $table:camel Response>]>, Status>> + Send + 'async_trait>>
            where
                'life0: 'async_trait,
                Self: 'async_trait,
            {
                Box::pin(async move {
                    with_session_management!(request, {
                        let auth_data = match request.extensions().get::<Auth>() {
                        Some(data) => data.clone(), // Clone the auth data
                        None => {
                            return Err(tonic::Status::internal(
                                "Authentication information not available",
                            ));
                        }
                    };

                    let request = request.into_inner();
                    let params = match request.params {
                        Some(p) => p,
                        None => return Err(Status::invalid_argument("Params are required")),
                    };
                    let table_name = params.table;

                    // Extract type from params to determine if it's a root request
                    let request_type = params.r#type.as_str();
                    let is_root_request = request_type == "root";

                    let temp_table= format!("temp_{}", table_name);
                    match table_exists(&temp_table) {
                        Ok(_table) => {
                            // Table exists, proceed with your logic using the table
                        },
                        Err(_error) => {
                            return Err(Status::internal(format!(
                                "Table '{}' does not exist",
                                temp_table
                            )));
                        }
                    }
                   let records;
                     match request.body {
                        Some(batch_body) => {
                            records=batch_body.$table}
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
                    let (processed_records, columns) = match process_records(json_records, &table_name, &auth_data.clone(), is_root_request) {
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

                        if let Some(id) = record.get("id").and_then(|v| v.as_str()) {
                            if let Err(e) =
crate::batch_sync::BatchSyncService::send_code_assignment_message(table_name.clone(), id.to_string(), "".to_string(), auth_data.clone(), true).await
                            {
                                log::error!("Code assignment error with id {id}: {e}");
                            }
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
                mut request: Request<[<BatchUpdate $table:camel Request>]>,
            ) -> Pin<Box<dyn std::future::Future<Output = Result<Response<[<BatchUpdate $table:camel Response>]>, Status>> + Send + 'async_trait>>
            where
                'life0: 'async_trait,
                Self: 'async_trait,
            {
                Box::pin(async move {
                    with_session_management!(request, {
                        // Extract params first to get the type for validation
                        let params = match request.get_ref().params {
                            Some(ref p) => p.clone(),
                            None => return Err(Status::invalid_argument("Params are required")),
                        };
                        
                        // Use common validation function
                        let (auth_data, _claims) = crate::middlewares::auth_middleware::validate_grpc_request_with_root_access(&request, &params.r#type)?;
                        
                        let request_inner = request.into_inner();
                    
                    let body = request_inner
                        .body
                        .ok_or_else(|| Status::invalid_argument("Body is required"))?;
                    let updates = body
                        .updates
                        .ok_or_else(|| Status::invalid_argument("Updates are required"))?;

                    let filters: Vec<Value> = body
                        .advance_filters
                        .into_iter()
                        .map(|filter| {
    let mut value = serde_json::to_value(filter).unwrap_or_default();
    if let Value::Object(ref mut map) = value {
        if let Some(Value::String(s)) = map.get_mut("values") {
            if let Ok(parsed) = serde_json::from_str::<Value>(s) {
                if parsed.is_array() {
                    // Safe approach: use the mutable reference we already have
                    if let Some(values_field) = map.get_mut("values") {
                        *values_field = parsed;
                    }
                }
            }
        }
    }
    value
})
                        .collect();

                    // Process the updates through common processing logic
                    let mut request_body = RequestBody {
                        record: serde_json::to_value(&updates)
                            .map_err(|e| Status::internal(format!("Failed to convert updates to JSON: {}", e)))?,
                    };
                    request_body.process_record("update", &auth_data, params.r#type == "root", &params.table);
                    if let Some(record) = request_body.record.as_object_mut() {
                        record.remove("version");
                    }

                    let updates_map = match serde_json::to_value(&request_body.record) {
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
                    // Get authentication data from request extensions
                    let auth_data = match request.extensions().get::<Auth>() {
                        Some(data) => data.clone(), // Clone the auth data
                        None => {
                            return Err(tonic::Status::internal(
                                "Authentication information not available",
                            ));
                        }
                    };

                    // Extract request parameters
                    let request = request.into_inner();
                    let params = request
                        .params
                        .ok_or_else(|| Status::invalid_argument("Params are required"))?;
                    let id = params.id;

                    // Extract query and parse pluck_fields
                    let query = request
                        .query
                        .ok_or_else(|| Status::invalid_argument("Query is required"))?;
                    
                    let pluck_fields = if !query.pluck.is_empty() {
                        Some(query.pluck.split(',').map(|s| s.trim().to_string()).collect())
                    } else {
                        Some(vec!["id".to_string()])
                    };

                    // Check if ID is provided
                    if id.is_empty() {
                        return Err(Status::invalid_argument("ID is required"));
                    }

                    // Get the table name based on the macro parameter
                    let table_name = stringify!($table).to_string();

                    // Process and get record by ID
                    match process_and_get_record_by_id(&table_name, &id, pluck_fields, auth_data.is_root_account, Some(&auth_data.organization_id)).await {
                        Ok(response) => {
                            // Check if we have data
                            if response.data.is_empty() {
                                return Err(Status::not_found(
                                    format!("Record with ID '{}' not found in '{}'", id, table_name)
                                ));
                            }


                            // Convert the first item to the specific type
                            let typed_data: [<$table:camel>] = serde_json::from_value(response.data[0].clone())
                                .map_err(|e| Status::internal(format!("Failed to convert response data: {}", e)))?;

                            // Create the gRPC response
                            let grpc_response = [<Get $table:camel Response>] {
                                success: response.success,
                                message: response.message,
                                data: Some(typed_data),
                            };

                            Ok(Response::new(grpc_response))
                        },
                        Err(error) => {
                            // Map API error to gRPC status
                            Err(Status::internal(error.message))
                        }
                    }
                })
            }
        }
    };
}

#[macro_export]
macro_rules! generate_upsert_method {
    ($table:ident) => {
        paste::paste! {
            fn [<upsert_ $table:lower>]<'life0, 'async_trait>(
                &'life0 self,
                mut request: Request<[<Upsert $table:camel Request>]>,
            ) -> Pin<Box<dyn std::future::Future<Output = Result<Response<[<Upsert $table:camel Response>]>, Status>> + Send + 'async_trait>>
            where
                'life0: 'async_trait,
                Self: 'async_trait,
            {
                Box::pin(async move {
                    with_session_management!(request, {
                        // Extract params first to get the type for validation
                        let params = match request.get_ref().params {
                            Some(ref p) => p.clone(),
                            None => return Err(Status::invalid_argument("Params are required")),
                        };
                        
                        // Use common validation function
                        let (auth_data, _claims) = crate::middlewares::auth_middleware::validate_grpc_request_with_root_access(&request, &params.r#type)?;
                        
                        let request_inner = request.into_inner();

                    let query = request_inner
                        .query
                        .ok_or_else(|| Status::invalid_argument("Query is required"))?;
                    let body = request_inner
                        .body
                        .ok_or_else(|| Status::invalid_argument("Body is required"))?;
                    // Extract pluck fields if provided
                    let pluck_fields = if !query.pluck.is_empty() {
                        Some(query.pluck.split(',').map(|s| s.trim().to_string()).collect())
                    } else {
                        None
                    };

                    let data_value = serde_json::to_value(&body.data)
                        .map_err(|e| Status::internal(format!("Failed to convert data to JSON: {}", e)))?;


                    // Call the reusable function
                    match perform_upsert(
                        &params.table,
                        body.conflict_columns,
                        data_value,
                        pluck_fields,
                        &auth_data,
                        params.r#type == "root",
                    ).await {
                        Ok(response) => {
                            // Convert ApiResponse to gRPC response
                            let typed_data: Vec<[<$table:camel>]> = response.data
                            .into_iter()
                            .map(|value| serde_json::from_value(value))
                            .collect::<Result<Vec<_>, _>>()
                            .map_err(|e| Status::internal(format!("Failed to convert response data: {}", e)))?;
                            let grpc_response = [<Upsert $table:camel Response>] {
                                success: response.success,
                                message: response.message,
                                count: response.count,
                                data: typed_data,
                            };
                            Ok(Response::new(grpc_response))
                        },
                        Err(error) => {
                            Err(Status::internal(error.message))
                        }
                    }
                    })
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
                mut request: Request<[<Delete $table:camel Request>]>,
            ) -> Pin<Box<dyn std::future::Future<Output = Result<Response<[<Delete $table:camel Response>]>, Status>> + Send + 'async_trait>>
            where
                'life0: 'async_trait,
                Self: 'async_trait,
            {
                Box::pin(async move {
                    with_session_management!(request, {
                        // Extract params first to get the type for validation
                        let params = match request.get_ref().params {
                            Some(ref p) => p.clone(),
                            None => return Err(Status::invalid_argument("Params are required")),
                        };
                        
                        // Use common validation function
                        let (auth_data, _claims) = crate::middlewares::auth_middleware::validate_grpc_request_with_root_access(&request, &params.r#type)?;
                        
                        let request_inner = request.into_inner();

                    let _query = request_inner
                        .query
                        .ok_or_else(|| Status::invalid_argument("Query is required"))?;

                    let table_name = params.table;
                    let record_id = params.id;

                    // Create empty delete updates
                    let delete_updates = serde_json::json!({});

                    // Process record using common function
                    match process_and_update_record(&table_name, delete_updates, &record_id, None, "delete", &auth_data, params.r#type == "root").await {
                        Ok(response) => {
                            // Convert response to Value to modify message
                            // let mut response_value: serde_json::Value = serde_json::from_str(&serde_json::to_string(&response).unwrap())
                            //     .map_err(|e| Status::internal(format!("Failed to parse response: {}", e)))?;

                            let mut response_value: serde_json::Value = serde_json::to_value(&response)
                                .map_err(|e| Status::internal(format!("Failed to convert response to JSON: {}", e)))?;

                            if let Some(obj) = response_value.as_object_mut() {
                                obj["message"] = serde_json::Value::String(
                                    format!("Record with ID '{}' deleted successfully from '{}'", record_id, table_name)
                                );
                            }

                            // Convert the modified response back to DeleteResponse
                            let response: [<Delete $table:camel Response>] = serde_json::from_value(response_value)
                                .map_err(|e| Status::internal(format!("Failed to create response: {}", e)))?;

                            Ok(Response::new(response))
                        },
                        Err(error) => {
                            let response = [<Delete $table:camel Response>] {
                                success: false,
                                message: error.to_string(),
                                count: 0,
                                data: None,
                            };
                            Ok(Response::new(response))
                        }
                    }
                    })
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
                mut request: Request<[<BatchDelete $table:camel Request>]>,
            ) -> Pin<Box<dyn std::future::Future<Output = Result<Response<[<BatchDelete $table:camel Response>]>, Status>> + Send + 'async_trait>>
            where
                'life0: 'async_trait,
                Self: 'async_trait,
            {
                Box::pin(async move {
                    with_session_management!(request, {
                        let auth_data = match request.extensions().get::<Auth>() {
                        Some(data) => data.clone(), // Clone the auth data
                        None => {
                            return Err(tonic::Status::internal(
                                "Authentication information not available",
                            ));
                        }
                    };
                    let request = request.into_inner();
                    let params = request
                        .params
                        .ok_or_else(|| Status::invalid_argument("Params are required"))?;

                    // Extract type from params to determine if it's a root request
                    let request_type = params.r#type.as_str();
                    let is_root_request = request_type == "root";

                    let body = request
                        .body
                        .ok_or_else(|| Status::invalid_argument("Body is required"))?;

                    let mut delete_updates = RequestBody {
                        record: serde_json::json!({}),
                    };

                    // Process the record through the common processing logic
                    delete_updates.process_record("delete", &auth_data, is_root_request, &params.table);
                    if let Some(record) = delete_updates.record.as_object_mut() {
                        record.remove("version");
                    }

                    let updates = delete_updates.record;

                    let filters: Vec<Value> = body
                    .advance_filters
                    .into_iter()
                    .map(|filter| {
                        let mut value = serde_json::to_value(filter).unwrap_or_default();
                        if let Value::Object(ref mut map) = value {
                        if let Some(Value::String(s)) = map.get_mut("values") {
                        if let Ok(parsed) = serde_json::from_str::<Value>(s) {
                            if parsed.is_array() {
                                if let Some(values_field) = map.get_mut("values") {
                                    *values_field = parsed;
                                }
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
                })
            }
        }
    };
}
