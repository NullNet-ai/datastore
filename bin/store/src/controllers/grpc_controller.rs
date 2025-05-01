use crate::db;
use crate::generated::store::store_service_server::{StoreService, StoreServiceServer};
use actix_web::{HttpResponse, Responder, web};
use serde::Serialize;
use std::net::SocketAddr;
use tonic::{Request, Response, Status, transport::Server};
use crate::generated::store::{CreateItemsRequest, CreateItemsResponse, GetItemsRequest, GetItemsResponse, UpdateItemsRequest, UpdateItemsResponse, DeleteItemsRequest, DeleteItemsResponse, CreatePacketsRequest, CreatePacketsResponse, GetPacketsRequest, GetPacketsResponse, UpdatePacketsRequest, UpdatePacketsResponse, DeletePacketsRequest, DeletePacketsResponse, CreateCrdtMessagesRequest, CreateCrdtMessagesResponse, GetCrdtMessagesRequest, GetCrdtMessagesResponse, UpdateCrdtMessagesRequest, UpdateCrdtMessagesResponse, DeleteCrdtMessagesRequest, DeleteCrdtMessagesResponse, CreateCrdtMerklesRequest, CreateCrdtMerklesResponse, GetCrdtMerklesRequest, GetCrdtMerklesResponse, UpdateCrdtMerklesRequest, UpdateCrdtMerklesResponse, DeleteCrdtMerklesRequest, DeleteCrdtMerklesResponse, CreateSyncEndpointsRequest, CreateSyncEndpointsResponse, GetSyncEndpointsRequest, GetSyncEndpointsResponse, UpdateSyncEndpointsRequest, UpdateSyncEndpointsResponse, DeleteSyncEndpointsRequest, DeleteSyncEndpointsResponse, CreateQueuesRequest, CreateQueuesResponse, GetQueuesRequest, GetQueuesResponse, UpdateQueuesRequest, UpdateQueuesResponse, DeleteQueuesRequest, DeleteQueuesResponse, CreateQueueItemsRequest, CreateQueueItemsResponse, GetQueueItemsRequest, GetQueueItemsResponse, UpdateQueueItemsRequest, UpdateQueueItemsResponse, DeleteQueueItemsRequest, DeleteQueueItemsResponse, CreateTransactionsRequest, CreateTransactionsResponse, GetTransactionsRequest, GetTransactionsResponse, UpdateTransactionsRequest, UpdateTransactionsResponse, DeleteTransactionsRequest, DeleteTransactionsResponse};
use chrono::Utc;
use serde_json::json;
use uuid::Uuid as uuid_crate;
use crate::table_enum::Table;
use crate::sync::sync_service::insert;
use crate::generated::store::Packets;
use crate::structs::structs::CreateRequestBody;

// Define your gRPC service struct
pub struct GrpcController {}

impl GrpcController {
    pub fn new() -> Self {
        GrpcController {}
    }

    // Initialize the gRPC server
    pub async fn init(addr: &str) -> Result<(), Box<dyn std::error::Error>> {
        let addr: SocketAddr = addr.parse()?;  // Specify the type here
        let grpc_controller = GrpcController::new();

        println!("gRPC Server listening on {}", addr);

        Server::builder()
            .add_service(
                StoreServiceServer::new(grpc_controller)
                    .max_decoding_message_size(50 * 1024 * 1024),
            )
            .serve(addr)
            .await?;

        Ok(())
    }
}
#[tonic::async_trait]
impl StoreService for GrpcController {
    async fn create_items(&self, request: Request<CreateItemsRequest>) -> Result<Response<CreateItemsResponse>, Status> {
        // Implementation for CreateItems method
        todo!()
    }

    async fn get_items(&self, request: Request<GetItemsRequest>) -> Result<Response<GetItemsResponse>, Status> {
        // Implementation for GetItems method
        todo!()
    }

    async fn update_items(&self, request: Request<UpdateItemsRequest>) -> Result<Response<UpdateItemsResponse>, Status> {
        // Implementation for UpdateItems method
        todo!()
    }

    async fn delete_items(&self, request: Request<DeleteItemsRequest>) -> Result<Response<DeleteItemsResponse>, Status> {
        // Implementation for DeleteItems method
        todo!()
    }

    async fn create_packets(&self, request: Request<CreatePacketsRequest>) -> Result<Response<CreatePacketsResponse>, Status> {
        // Implementation for CreatePackets method
       
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
        let mut record = match request.packets {
            Some(r) => r,
            None => return Err(Status::invalid_argument("Record is required")),
        };
        record.hypertable_timestamp=record.timestamp.to_string(); // Set hypertable_timestamp to timestamp fiel
    
        // Convert protobuf message to serde_json::Value
        let mut processed_record = match serde_json::to_value(&record) {
            Ok(val) => val,
            Err(e) => {
                return Err(Status::internal(format!("Failed to process record: {}", e)));
            }
        };

        let mut request_body = CreateRequestBody { record: processed_record.clone() };
            request_body.process_record("create");
            processed_record = request_body.record;

            let record_value: serde_json::Value = match serde_json::from_value(processed_record) {
                Ok(val) => val,
                Err(e) => {
                    return Err(Status::internal(format!("Failed to process record: {}", e)));
                }
            };

        if let Err(e) = insert(&table_name.clone(), record_value.clone()).await {
            return Err(Status::internal(format!("Failed to insert record: {}", e)));
        }

        let pluck_fields: Vec<String> = query
        .pluck
        .split(',')
        .map(|s| s.trim().to_string())
        .collect();


        let packets: Packets=serde_json::from_value(record_value)
            .map_err(|e| Status::internal(format!("Failed to process record: {}", e)))?;

        //change plucked_record to CreatePacketsResponse
        let response = CreatePacketsResponse {
            data: Some(packets),
            message: format!("Record inserted into '{}'", &table_name.clone()),
            count:1,
            success:true,


        };
        Ok(Response::new(response))

    }

    async fn get_packets(&self, request: Request<GetPacketsRequest>) -> Result<Response<GetPacketsResponse>, Status> {
        // Implementation for GetPackets method
        todo!()
    }

    async fn update_packets(&self, request: Request<UpdatePacketsRequest>) -> Result<Response<UpdatePacketsResponse>, Status> {
        // Implementation for UpdatePackets method
        todo!()
    }

    async fn delete_packets(&self, request: Request<DeletePacketsRequest>) -> Result<Response<DeletePacketsResponse>, Status> {
        // Implementation for DeletePackets method
        todo!()
    }

    async fn create_crdt_messages(&self, request: Request<CreateCrdtMessagesRequest>) -> Result<Response<CreateCrdtMessagesResponse>, Status> {
        // Implementation for CreateCrdtMessages method
        todo!()
    }

    async fn get_crdt_messages(&self, request: Request<GetCrdtMessagesRequest>) -> Result<Response<GetCrdtMessagesResponse>, Status> {
        // Implementation for GetCrdtMessages method
        todo!()
    }

    async fn update_crdt_messages(&self, request: Request<UpdateCrdtMessagesRequest>) -> Result<Response<UpdateCrdtMessagesResponse>, Status> {
        // Implementation for UpdateCrdtMessages method
        todo!()
    }

    async fn delete_crdt_messages(&self, request: Request<DeleteCrdtMessagesRequest>) -> Result<Response<DeleteCrdtMessagesResponse>, Status> {
        // Implementation for DeleteCrdtMessages method
        todo!()
    }

    async fn create_crdt_merkles(&self, request: Request<CreateCrdtMerklesRequest>) -> Result<Response<CreateCrdtMerklesResponse>, Status> {
        // Implementation for CreateCrdtMerkles method
        todo!()
    }

    async fn get_crdt_merkles(&self, request: Request<GetCrdtMerklesRequest>) -> Result<Response<GetCrdtMerklesResponse>, Status> {
        // Implementation for GetCrdtMerkles method
        todo!()
    }

    async fn update_crdt_merkles(&self, request: Request<UpdateCrdtMerklesRequest>) -> Result<Response<UpdateCrdtMerklesResponse>, Status> {
        // Implementation for UpdateCrdtMerkles method
        todo!()
    }

    async fn delete_crdt_merkles(&self, request: Request<DeleteCrdtMerklesRequest>) -> Result<Response<DeleteCrdtMerklesResponse>, Status> {
        // Implementation for DeleteCrdtMerkles method
        todo!()
    }

    async fn create_sync_endpoints(&self, request: Request<CreateSyncEndpointsRequest>) -> Result<Response<CreateSyncEndpointsResponse>, Status> {
        // Implementation for CreateSyncEndpoints method
        todo!()
    }

    async fn get_sync_endpoints(&self, request: Request<GetSyncEndpointsRequest>) -> Result<Response<GetSyncEndpointsResponse>, Status> {
        // Implementation for GetSyncEndpoints method
        todo!()
    }

    async fn update_sync_endpoints(&self, request: Request<UpdateSyncEndpointsRequest>) -> Result<Response<UpdateSyncEndpointsResponse>, Status> {
        // Implementation for UpdateSyncEndpoints method
        todo!()
    }

    async fn delete_sync_endpoints(&self, request: Request<DeleteSyncEndpointsRequest>) -> Result<Response<DeleteSyncEndpointsResponse>, Status> {
        // Implementation for DeleteSyncEndpoints method
        todo!()
    }

    async fn create_queues(&self, request: Request<CreateQueuesRequest>) -> Result<Response<CreateQueuesResponse>, Status> {
        // Implementation for CreateQueues method
        todo!()
    }

    async fn get_queues(&self, request: Request<GetQueuesRequest>) -> Result<Response<GetQueuesResponse>, Status> {
        // Implementation for GetQueues method
        todo!()
    }

    async fn update_queues(&self, request: Request<UpdateQueuesRequest>) -> Result<Response<UpdateQueuesResponse>, Status> {
        // Implementation for UpdateQueues method
        todo!()
    }

    async fn delete_queues(&self, request: Request<DeleteQueuesRequest>) -> Result<Response<DeleteQueuesResponse>, Status> {
        // Implementation for DeleteQueues method
        todo!()
    }

    async fn create_queue_items(&self, request: Request<CreateQueueItemsRequest>) -> Result<Response<CreateQueueItemsResponse>, Status> {
        // Implementation for CreateQueueItems method
        todo!()
    }

    async fn get_queue_items(&self, request: Request<GetQueueItemsRequest>) -> Result<Response<GetQueueItemsResponse>, Status> {
        // Implementation for GetQueueItems method
        todo!()
    }

    async fn update_queue_items(&self, request: Request<UpdateQueueItemsRequest>) -> Result<Response<UpdateQueueItemsResponse>, Status> {
        // Implementation for UpdateQueueItems method
        todo!()
    }

    async fn delete_queue_items(&self, request: Request<DeleteQueueItemsRequest>) -> Result<Response<DeleteQueueItemsResponse>, Status> {
        // Implementation for DeleteQueueItems method
        todo!()
    }

    async fn create_transactions(&self, request: Request<CreateTransactionsRequest>) -> Result<Response<CreateTransactionsResponse>, Status> {
        // Implementation for CreateTransactions method
        todo!()
    }

    async fn get_transactions(&self, request: Request<GetTransactionsRequest>) -> Result<Response<GetTransactionsResponse>, Status> {
        // Implementation for GetTransactions method
        todo!()
    }

    async fn update_transactions(&self, request: Request<UpdateTransactionsRequest>) -> Result<Response<UpdateTransactionsResponse>, Status> {
        // Implementation for UpdateTransactions method
        todo!()
    }

    async fn delete_transactions(&self, request: Request<DeleteTransactionsRequest>) -> Result<Response<DeleteTransactionsResponse>, Status> {
        // Implementation for DeleteTransactions method
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
