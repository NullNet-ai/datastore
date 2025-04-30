use crate::db;
use crate::generated::store::store_service_server::{StoreService, StoreServiceServer};
use actix_web::{HttpResponse, Responder, web};
use serde::Serialize;
use std::net::SocketAddr;
use tonic::{Request, Response, Status, transport::Server};
use crate::generated::store::{CreateCrdtMerklesRequest, CreateCrdtMerklesResponse, CreateCrdtMessagesRequest, CreateCrdtMessagesResponse, CreateItemsRequest, CreateItemsResponse, CreatePacketsRequest, CreatePacketsResponse, CreateQueueItemsRequest, CreateQueueItemsResponse, CreateQueuesRequest, CreateQueuesResponse, CreateSyncEndpointsRequest, CreateSyncEndpointsResponse, CreateTransactionsRequest, CreateTransactionsResponse};
// Add this import

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

impl StoreService for GrpcController {
    async fn create_items(&self, request: Request<CreateItemsRequest>) -> Result<Response<CreateItemsResponse>, Status> {
        // the code you add here is executed when the gRPC request is received
        todo!()
    }

    async fn create_packets(&self, request: Request<CreatePacketsRequest>) -> Result<Response<CreatePacketsResponse>, Status> {
        todo!()
    }

    async fn create_crdt_messages(&self, request: Request<CreateCrdtMessagesRequest>) -> Result<Response<CreateCrdtMessagesResponse>, Status> {
        todo!()
    }

    async fn create_crdt_merkles(&self, request: Request<CreateCrdtMerklesRequest>) -> Result<Response<CreateCrdtMerklesResponse>, Status> {
        todo!()
    }

    async fn create_sync_endpoints(&self, request: Request<CreateSyncEndpointsRequest>) -> Result<Response<CreateSyncEndpointsResponse>, Status> {
        todo!()
    }

    async fn create_queues(&self, request: Request<CreateQueuesRequest>) -> Result<Response<CreateQueuesResponse>, Status> {
        todo!()
    }

    async fn create_queue_items(&self, request: Request<CreateQueueItemsRequest>) -> Result<Response<CreateQueueItemsResponse>, Status> {
        todo!()
    }

    async fn create_transactions(&self, request: Request<CreateTransactionsRequest>) -> Result<Response<CreateTransactionsResponse>, Status> {
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
