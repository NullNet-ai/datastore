use crate::auth::auth_middleware::GrpcAuthInterceptor;
use crate::controllers::common_controller::{convert_json_to_csv, execute_copy, process_records};
use crate::{db, generate_batch_delete_method, generate_batch_insert_method, generate_batch_update_method, generate_create_method, generate_update_method};
use crate::db::create_connection;
use crate::generated::store::store_service_server::{StoreService, StoreServiceServer};
use crate::generated::store::{
    BatchDeleteConnectionsRequest, BatchDeleteConnectionsResponse, BatchDeleteCrdtMerklesRequest, BatchDeleteCrdtMerklesResponse, BatchDeleteCrdtMessagesRequest, BatchDeleteCrdtMessagesResponse, BatchDeleteItemsRequest, BatchDeleteItemsResponse, BatchDeletePacketsRequest, BatchDeletePacketsResponse, BatchDeleteQueueItemsRequest, BatchDeleteQueueItemsResponse, BatchDeleteQueuesRequest, BatchDeleteQueuesResponse, BatchDeleteSyncEndpointsRequest, BatchDeleteSyncEndpointsResponse, BatchDeleteTransactionsRequest, BatchDeleteTransactionsResponse, BatchInsertConnectionsRequest, BatchInsertConnectionsResponse, BatchInsertCrdtMerklesRequest, BatchInsertCrdtMerklesResponse, BatchInsertCrdtMessagesRequest, BatchInsertCrdtMessagesResponse, BatchInsertItemsRequest, BatchInsertItemsResponse, BatchInsertPacketsRequest, BatchInsertPacketsResponse, BatchInsertQueueItemsRequest, BatchInsertQueueItemsResponse, BatchInsertQueuesRequest, BatchInsertQueuesResponse, BatchInsertSyncEndpointsRequest, BatchInsertSyncEndpointsResponse, BatchInsertTransactionsRequest, BatchInsertTransactionsResponse, BatchUpdateConnectionsRequest, BatchUpdateConnectionsResponse, BatchUpdateCrdtMerklesRequest, BatchUpdateCrdtMerklesResponse, BatchUpdateCrdtMessagesRequest, BatchUpdateCrdtMessagesResponse, BatchUpdateItemsRequest, BatchUpdateItemsResponse, BatchUpdatePacketsRequest, BatchUpdatePacketsResponse, BatchUpdateQueueItemsRequest, BatchUpdateQueueItemsResponse, BatchUpdateQueuesRequest, BatchUpdateQueuesResponse, BatchUpdateSyncEndpointsRequest, BatchUpdateSyncEndpointsResponse, BatchUpdateTransactionsRequest, BatchUpdateTransactionsResponse, Connections, CreateConnectionsRequest, CreateConnectionsResponse, CreateCrdtMerklesRequest, CreateCrdtMerklesResponse, CreateCrdtMessagesRequest, CreateCrdtMessagesResponse, CreateItemsRequest, CreateItemsResponse, CreatePacketsRequest, CreatePacketsResponse, CreateQueueItemsRequest, CreateQueueItemsResponse, CreateQueuesRequest, CreateQueuesResponse, CreateSyncEndpointsRequest, CreateSyncEndpointsResponse, CreateTransactionsRequest, CreateTransactionsResponse, DeleteConnectionsRequest, DeleteConnectionsResponse, DeleteCrdtMerklesRequest, DeleteCrdtMerklesResponse, DeleteCrdtMessagesRequest, DeleteCrdtMessagesResponse, DeleteItemsRequest, DeleteItemsResponse, DeletePacketsRequest, DeletePacketsResponse, DeleteQueueItemsRequest, DeleteQueueItemsResponse, DeleteQueuesRequest, DeleteQueuesResponse, DeleteSyncEndpointsRequest, DeleteSyncEndpointsResponse, DeleteTransactionsRequest, DeleteTransactionsResponse, GetConnectionsRequest, GetConnectionsResponse, GetCrdtMerklesRequest, GetCrdtMerklesResponse, GetCrdtMessagesRequest, GetCrdtMessagesResponse, GetItemsRequest, GetItemsResponse, GetPacketsRequest, GetPacketsResponse, GetQueueItemsRequest, GetQueueItemsResponse, GetQueuesRequest, GetQueuesResponse, GetSyncEndpointsRequest, GetSyncEndpointsResponse, GetTransactionsRequest, GetTransactionsResponse, Packets, UpdateConnectionsRequest, UpdateConnectionsResponse, UpdateCrdtMerklesRequest, UpdateCrdtMerklesResponse, UpdateCrdtMessagesRequest, UpdateCrdtMessagesResponse, UpdateItemsRequest, UpdateItemsResponse, UpdatePacketsRequest, UpdatePacketsResponse, UpdateQueueItemsRequest, UpdateQueueItemsResponse, UpdateQueuesRequest, UpdateQueuesResponse, UpdateSyncEndpointsRequest, UpdateSyncEndpointsResponse, UpdateTransactionsRequest, UpdateTransactionsResponse
};
use crate::structs::structs::RequestBody;
use crate::sync::sync_service::insert;
use crate::table_enum::Table;
use actix_web::{web, HttpResponse, Responder};
use serde_json::Value;
use std::net::SocketAddr;
use tonic::{transport::Server, Request, Response, Status};
use std::pin::Pin;

use super::common_controller::{perform_batch_update, process_record_for_insert, process_record_for_update, sanitize_updates};

// Define your gRPC service struct
pub struct GrpcController {}

impl GrpcController {
    pub fn new() -> Self {
        GrpcController {}
    }

    // Initialize the gRPC server
    pub async fn init(addr: &str) -> Result<(), Box<dyn std::error::Error>> {
        let addr: SocketAddr = addr.parse()?; // Specify the type here
        let grpc_controller = GrpcController::new();

        println!("gRPC Server listening on {}", addr);

        Server::builder()
            .add_service(StoreServiceServer::with_interceptor(
                grpc_controller,
                GrpcAuthInterceptor,
            ))
            .serve(addr)
            .await?;

        Ok(())
    }
}
#[tonic::async_trait]
impl StoreService for GrpcController {
    async fn create_items(
        &self,
        request: Request<CreateItemsRequest>,
    ) -> Result<Response<CreateItemsResponse>, Status> {
        // Implementation for CreateItems method
        todo!()
    }

    async fn get_items(
        &self,
        request: Request<GetItemsRequest>,
    ) -> Result<Response<GetItemsResponse>, Status> {
        // Implementation for GetItems method
        todo!()
    }

    async fn update_items(
        &self,
        request: Request<UpdateItemsRequest>,
    ) -> Result<Response<UpdateItemsResponse>, Status> {
        // Implementation for UpdateItems method
        todo!()
    }

    async fn delete_items(
        &self,
        request: Request<DeleteItemsRequest>,
    ) -> Result<Response<DeleteItemsResponse>, Status> {
        // Implementation for DeleteItems method
        todo!()
    }

    async fn batch_insert_items(
        &self,
        request: Request<BatchInsertItemsRequest>,
    ) -> Result<Response<BatchInsertItemsResponse>, Status> {
        // Implementation for DeleteItems method
        todo!()
    }

    async fn batch_update_items(
        &self,
        request: Request<BatchUpdateItemsRequest>,
    ) -> Result<Response<BatchUpdateItemsResponse>, Status> {
        // Implementation for DeleteItems method
        todo!()
    }

    async fn batch_delete_items(
        &self,
        request: Request<BatchDeleteItemsRequest>,
    ) -> Result<Response<BatchDeleteItemsResponse>, Status> {
        // Implementation for DeleteItems method
        todo!()
    }

    async fn create_connections(
        &self,
        request: Request<CreateConnectionsRequest>,
    ) -> Result<Response<CreateConnectionsResponse>, Status> {
        // Implementation for GetItems method
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
        let mut record = match request.connections {
            Some(r) => r,
            None => return Err(Status::invalid_argument("Record is required")),
        };

        let record_value = process_record_for_insert(record, &table_name).await?;

        if let Err(e) = insert(&table_name.clone(), record_value.clone()).await {
            return Err(Status::internal(format!("Failed to insert record: {}", e)));
        }

        let pluck_fields: Vec<String> = query
            .pluck
            .split(',')
            .map(|s| s.trim().to_string())
            .collect();

        let packets: Connections = serde_json::from_value(record_value)
            .map_err(|e| Status::internal(format!("Failed to process record: {}", e)))?;

        //change plucked_record to CreatePacketsResponse
        let response = CreateConnectionsResponse {
            data: Some(packets),
            message: format!("Record inserted into '{}'", &table_name.clone()),
            count: 1,
            success: true,
        };
        Ok(Response::new(response))
    }

    async fn get_connections(
        &self,
        request: Request<GetConnectionsRequest>,
    ) -> Result<Response<GetConnectionsResponse>, Status> {
        // Implementation for GetItems method
        todo!()
    }

    async fn update_connections(
        &self,
        request: Request<UpdateConnectionsRequest>,
    ) -> Result<Response<UpdateConnectionsResponse>, Status> {
        // Implementation for UpdateItems method
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
    let record = match request.connection {
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
            // Since process_record_for_update already returns a Status,
            // we can directly return it
            return Err(status);
        }
    };

    let pluck_fields: Vec<String> = query
        .pluck
        .split(',')
        .map(|s| s.trim().to_string())
        .collect();

    if let Err(e) = crate::sync::sync_service::update(&table_name, processed_record.clone(), &record_id).await {
        return Err(Status::internal(format!("Failed to update record: {}", e)));
    }

    let plucked_record = table.pluck_fields(&processed_record, pluck_fields);
    let connections: Connections = serde_json::from_value(plucked_record)
        .map_err(|e| Status::internal(format!("Failed to process record: {}", e)))?;

    let response = UpdateConnectionsResponse {
        data: Some(connections),
        message: format!("Record updated in '{}'", &table_name),
        count: 1,
        success: true,
    };

    Ok(Response::new(response))
    }

    async fn delete_connections(
        &self,
        request: Request<DeleteConnectionsRequest>,
    ) -> Result<Response<DeleteConnectionsResponse>, Status> {
        // Implementation for DeleteItems method
        todo!()
    }

    async fn batch_insert_connections(
        &self,
        request: Request<BatchInsertConnectionsRequest>,
    ) -> Result<Response<BatchInsertConnectionsResponse>, Status> {
        // Implementation for DeleteItems method
        let request = request.into_inner();
        let params = match request.params {
            Some(p) => p,
            None => return Err(Status::invalid_argument("Params are required")),
        };
        let table_name = params.table;
        let connections = match request.body {
            Some(batch_body) => batch_body.connections,
            None => return Err(Status::invalid_argument("No packets provided")),
        };

        if connections.is_empty() {
            return Err(Status::invalid_argument("No records provided"));
        }

        let json_records: Vec<serde_json::Value> = connections
            .into_iter()
            .map(|connection| serde_json::to_value(&connection))
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
        let response_connections: Vec<Connections> = processed_records
            .into_iter()
            .map(|record| serde_json::from_value(record))
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| Status::internal(format!("Failed to convert records: {}", e)))?;

        // Convert Vec<String> to Vec<&str> for execute_copy
        // Create response
        let response = BatchInsertConnectionsResponse {
            success: true,
            message: format!(
                "Inserted {} records into '{}'",
                response_connections.len(),
                table_name
            ),
            count: response_connections.len() as i32,
            data: response_connections.clone(),
        };

        Ok(Response::new(response))
    }

    generate_batch_update_method!(connections);

    async fn batch_delete_connections(
        &self,
        request: Request<BatchDeleteConnectionsRequest>,
    ) -> Result<Response<BatchDeleteConnectionsResponse>, Status> {
        // Implementation for DeleteItems method
        todo!()
    }

    generate_create_method!(packets);

    async fn get_packets(
        &self,
        request: Request<GetPacketsRequest>,
    ) -> Result<Response<GetPacketsResponse>, Status> {
        // Implementation for GetPackets method
        todo!()
    }

    generate_update_method!(packets, packet);

    async fn delete_packets(
        &self,
        request: Request<DeletePacketsRequest>,
    ) -> Result<Response<DeletePacketsResponse>, Status> {
        // Implementation for DeletePackets method
        todo!()
    }

    generate_batch_insert_method!(packets);

    async fn batch_update_packets(
        &self,
        request: Request<BatchUpdatePacketsRequest>,
    ) -> Result<Response<BatchUpdatePacketsResponse>, Status> {
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

        //remove all the updates with values Null
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

        let response = BatchUpdatePacketsResponse {
            success: true,
            message: format!("Updated {} records in '{}'", count, params.table),
            count: count as i32,
            data: vec![],
        };

        Ok(Response::new(response))
    }

    async fn batch_delete_packets(
        &self,
        request: Request<BatchDeletePacketsRequest>,
    ) -> Result<Response<BatchDeletePacketsResponse>, Status> {
        // Implementation for DeleteItems method
        
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

        let response = BatchDeletePacketsResponse {
            success: true,
            message: format!("Updated {} records in '{}'", count, params.table),
            count: count as i32,
            data: None,
        };

        Ok(Response::new(response))
    }
    async fn create_crdt_messages(
        &self,
        request: Request<CreateCrdtMessagesRequest>,
    ) -> Result<Response<CreateCrdtMessagesResponse>, Status> {
        // Implementation for CreateCrdtMessages method
        todo!()
    }

    async fn get_crdt_messages(
        &self,
        request: Request<GetCrdtMessagesRequest>,
    ) -> Result<Response<GetCrdtMessagesResponse>, Status> {
        // Implementation for GetCrdtMessages method
        todo!()
    }

    async fn update_crdt_messages(
        &self,
        request: Request<UpdateCrdtMessagesRequest>,
    ) -> Result<Response<UpdateCrdtMessagesResponse>, Status> {
        // Implementation for UpdateCrdtMessages method
        todo!()
    }

    async fn delete_crdt_messages(
        &self,
        request: Request<DeleteCrdtMessagesRequest>,
    ) -> Result<Response<DeleteCrdtMessagesResponse>, Status> {
        // Implementation for DeleteCrdtMessages method
        todo!()
    }

    async fn batch_insert_crdt_messages(
        &self,
        request: Request<BatchInsertCrdtMessagesRequest>,
    ) -> Result<Response<BatchInsertCrdtMessagesResponse>, Status> {
        // Implementation for DeleteItems method
        todo!()
    }

    async fn batch_update_crdt_messages(
        &self,
        request: Request<BatchUpdateCrdtMessagesRequest>,
    ) -> Result<Response<BatchUpdateCrdtMessagesResponse>, Status> {
        // Implementation for DeleteItems method
        todo!()
    }
   generate_batch_delete_method!(crdt_messages);

    async fn create_crdt_merkles(
        &self,
        request: Request<CreateCrdtMerklesRequest>,
    ) -> Result<Response<CreateCrdtMerklesResponse>, Status> {
        // Implementation for CreateCrdtMerkles method
        todo!()
    }

    async fn get_crdt_merkles(
        &self,
        request: Request<GetCrdtMerklesRequest>,
    ) -> Result<Response<GetCrdtMerklesResponse>, Status> {
        // Implementation for GetCrdtMerkles method
        todo!()
    }

    async fn update_crdt_merkles(
        &self,
        request: Request<UpdateCrdtMerklesRequest>,
    ) -> Result<Response<UpdateCrdtMerklesResponse>, Status> {
        // Implementation for UpdateCrdtMerkles method
        todo!()
    }

    async fn delete_crdt_merkles(
        &self,
        request: Request<DeleteCrdtMerklesRequest>,
    ) -> Result<Response<DeleteCrdtMerklesResponse>, Status> {
        // Implementation for DeleteCrdtMerkles method
        todo!()
    }

    async fn batch_insert_crdt_merkles(
        &self,
        request: Request<BatchInsertCrdtMerklesRequest>,
    ) -> Result<Response<BatchInsertCrdtMerklesResponse>, Status> {
        // Implementation for DeleteItems method
        todo!()
    }

    async fn batch_update_crdt_merkles(
        &self,
        request: Request<BatchUpdateCrdtMerklesRequest>,
    ) -> Result<Response<BatchUpdateCrdtMerklesResponse>, Status> {
        // Implementation for DeleteItems method
        todo!()
    }

    async fn batch_delete_crdt_merkles(
        &self,
        request: Request<BatchDeleteCrdtMerklesRequest>,
    ) -> Result<Response<BatchDeleteCrdtMerklesResponse>, Status> {
        // Implementation for DeleteItems method
        todo!()
    }

    async fn create_sync_endpoints(
        &self,
        request: Request<CreateSyncEndpointsRequest>,
    ) -> Result<Response<CreateSyncEndpointsResponse>, Status> {
        // Implementation for CreateSyncEndpoints method
        todo!()
    }

    async fn get_sync_endpoints(
        &self,
        request: Request<GetSyncEndpointsRequest>,
    ) -> Result<Response<GetSyncEndpointsResponse>, Status> {
        // Implementation for GetSyncEndpoints method
        todo!()
    }

    async fn update_sync_endpoints(
        &self,
        request: Request<UpdateSyncEndpointsRequest>,
    ) -> Result<Response<UpdateSyncEndpointsResponse>, Status> {
        // Implementation for UpdateSyncEndpoints method
        todo!()
    }

    async fn delete_sync_endpoints(
        &self,
        request: Request<DeleteSyncEndpointsRequest>,
    ) -> Result<Response<DeleteSyncEndpointsResponse>, Status> {
        // Implementation for DeleteSyncEndpoints method
        todo!()
    }

    async fn batch_insert_sync_endpoints(
        &self,
        request: Request<BatchInsertSyncEndpointsRequest>,
    ) -> Result<Response<BatchInsertSyncEndpointsResponse>, Status> {
        // Implementation for DeleteItems method
        todo!()
    }

    async fn batch_update_sync_endpoints(
        &self,
        request: Request<BatchUpdateSyncEndpointsRequest>,
    ) -> Result<Response<BatchUpdateSyncEndpointsResponse>, Status> {
        // Implementation for DeleteItems method
        todo!()
    }

    async fn batch_delete_sync_endpoints(
        &self,
        request: Request<BatchDeleteSyncEndpointsRequest>,
    ) -> Result<Response<BatchDeleteSyncEndpointsResponse>, Status> {
        // Implementation for DeleteItems method
        todo!()
    }

    async fn create_queues(
        &self,
        request: Request<CreateQueuesRequest>,
    ) -> Result<Response<CreateQueuesResponse>, Status> {
        // Implementation for CreateQueues method
        todo!()
    }

    async fn get_queues(
        &self,
        request: Request<GetQueuesRequest>,
    ) -> Result<Response<GetQueuesResponse>, Status> {
        // Implementation for GetQueues method
        todo!()
    }

    async fn update_queues(
        &self,
        request: Request<UpdateQueuesRequest>,
    ) -> Result<Response<UpdateQueuesResponse>, Status> {
        // Implementation for UpdateQueues method
        todo!()
    }

    async fn delete_queues(
        &self,
        request: Request<DeleteQueuesRequest>,
    ) -> Result<Response<DeleteQueuesResponse>, Status> {
        // Implementation for DeleteQueues method
        todo!()
    }

    async fn batch_insert_queues(
        &self,
        request: Request<BatchInsertQueuesRequest>,
    ) -> Result<Response<BatchInsertQueuesResponse>, Status> {
        // Implementation for DeleteItems method
        todo!()
    }

    async fn batch_update_queues(
        &self,
        request: Request<BatchUpdateQueuesRequest>,
    ) -> Result<Response<BatchUpdateQueuesResponse>, Status> {
        // Implementation for DeleteItems method
        todo!()
    }

    async fn batch_delete_queues(
        &self,
        request: Request<BatchDeleteQueuesRequest>,
    ) -> Result<Response<BatchDeleteQueuesResponse>, Status> {
        // Implementation for DeleteItems method
        todo!()
    }

    async fn create_queue_items(
        &self,
        request: Request<CreateQueueItemsRequest>,
    ) -> Result<Response<CreateQueueItemsResponse>, Status> {
        // Implementation for CreateQueueItems method
        todo!()
    }

    async fn get_queue_items(
        &self,
        request: Request<GetQueueItemsRequest>,
    ) -> Result<Response<GetQueueItemsResponse>, Status> {
        // Implementation for GetQueueItems method
        todo!()
    }

    async fn update_queue_items(
        &self,
        request: Request<UpdateQueueItemsRequest>,
    ) -> Result<Response<UpdateQueueItemsResponse>, Status> {
        // Implementation for UpdateQueueItems method
        todo!()
    }

    async fn delete_queue_items(
        &self,
        request: Request<DeleteQueueItemsRequest>,
    ) -> Result<Response<DeleteQueueItemsResponse>, Status> {
        // Implementation for DeleteQueueItems method
        todo!()
    }

    async fn batch_insert_queue_items(
        &self,
        request: Request<BatchInsertQueueItemsRequest>,
    ) -> Result<Response<BatchInsertQueueItemsResponse>, Status> {
        // Implementation for DeleteItems method
        todo!()
    }

    async fn batch_update_queue_items(
        &self,
        request: Request<BatchUpdateQueueItemsRequest>,
    ) -> Result<Response<BatchUpdateQueueItemsResponse>, Status> {
        // Implementation for DeleteItems method
        todo!()
    }

    async fn batch_delete_queue_items(
        &self,
        request: Request<BatchDeleteQueueItemsRequest>,
    ) -> Result<Response<BatchDeleteQueueItemsResponse>, Status> {
        // Implementation for DeleteItems method
        todo!()
    }

    async fn create_transactions(
        &self,
        request: Request<CreateTransactionsRequest>,
    ) -> Result<Response<CreateTransactionsResponse>, Status> {
        // Implementation for CreateTransactions method
        todo!()
    }

    async fn get_transactions(
        &self,
        request: Request<GetTransactionsRequest>,
    ) -> Result<Response<GetTransactionsResponse>, Status> {
        // Implementation for GetTransactions method
        todo!()
    }

    async fn update_transactions(
        &self,
        request: Request<UpdateTransactionsRequest>,
    ) -> Result<Response<UpdateTransactionsResponse>, Status> {
        // Implementation for UpdateTransactions method
        todo!()
    }

    async fn delete_transactions(
        &self,
        request: Request<DeleteTransactionsRequest>,
    ) -> Result<Response<DeleteTransactionsResponse>, Status> {
        // Implementation for DeleteTransactions method
        todo!()
    }

    async fn batch_insert_transactions(
        &self,
        request: Request<BatchInsertTransactionsRequest>,
    ) -> Result<Response<BatchInsertTransactionsResponse>, Status> {
        // Implementation for DeleteItems method
        todo!()
    }

    async fn batch_update_transactions(
        &self,
        request: Request<BatchUpdateTransactionsRequest>,
    ) -> Result<Response<BatchUpdateTransactionsResponse>, Status> {
        // Implementation for DeleteItems method
        todo!()
    }

    async fn batch_delete_transactions(
        &self,
        request: Request<BatchDeleteTransactionsRequest>,
    ) -> Result<Response<BatchDeleteTransactionsResponse>, Status> {
        // Implementation for DeleteItems method
        todo!()
    }
}

// You can add HTTP endpoints to configure or check gRPC status
pub async fn grpc_status() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({
        "status": "running",
        "message": "gRPC server is operational"
    }))
}

// Function to configure and register HTTP routes related to gRPC
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/api/grpc/status").route(web::get().to(grpc_status)));
}
