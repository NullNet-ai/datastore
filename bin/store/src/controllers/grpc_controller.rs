use crate::db::create_connection;
use actix_web::{HttpResponse, Responder, web};
use std::pin::Pin;
use std::net::SocketAddr;
use crate::table_enum::Table;
use crate::sync::sync_service::{insert, update};
use serde_json::Value;
use crate::structs::structs::RequestBody;
use tonic::{Request, Response, Status, transport::Server};
use crate::auth::auth_middleware::GrpcAuthInterceptor;
use super::common_controller::{perform_batch_update, process_record_for_insert, process_record_for_update, sanitize_updates, convert_json_to_csv, process_records, execute_copy};
use crate::generated::store::store_service_server::{StoreServiceServer, StoreService };
use crate::{ generate_batch_delete_method, generate_batch_insert_method, generate_batch_update_method, generate_create_method, generate_update_method, generate_get_method, generate_delete_method};
use crate::generated::store::{Items, CreateItemsRequest, CreateItemsResponse, GetItemsRequest, GetItemsResponse, UpdateItemsRequest, UpdateItemsResponse, DeleteItemsRequest, DeleteItemsResponse, BatchInsertItemsRequest, BatchInsertItemsResponse, BatchUpdateItemsRequest, BatchUpdateItemsResponse, BatchDeleteItemsRequest, BatchDeleteItemsResponse, Packets, CreatePacketsRequest, CreatePacketsResponse, GetPacketsRequest, GetPacketsResponse, UpdatePacketsRequest, UpdatePacketsResponse, DeletePacketsRequest, DeletePacketsResponse, BatchInsertPacketsRequest, BatchInsertPacketsResponse, BatchUpdatePacketsRequest, BatchUpdatePacketsResponse, BatchDeletePacketsRequest, BatchDeletePacketsResponse, Connections, CreateConnectionsRequest, CreateConnectionsResponse, GetConnectionsRequest, GetConnectionsResponse, UpdateConnectionsRequest, UpdateConnectionsResponse, DeleteConnectionsRequest, DeleteConnectionsResponse, BatchInsertConnectionsRequest, BatchInsertConnectionsResponse, BatchUpdateConnectionsRequest, BatchUpdateConnectionsResponse, BatchDeleteConnectionsRequest, BatchDeleteConnectionsResponse, CrdtMessages, CreateCrdtMessagesRequest, CreateCrdtMessagesResponse, GetCrdtMessagesRequest, GetCrdtMessagesResponse, UpdateCrdtMessagesRequest, UpdateCrdtMessagesResponse, DeleteCrdtMessagesRequest, DeleteCrdtMessagesResponse, BatchInsertCrdtMessagesRequest, BatchInsertCrdtMessagesResponse, BatchUpdateCrdtMessagesRequest, BatchUpdateCrdtMessagesResponse, BatchDeleteCrdtMessagesRequest, BatchDeleteCrdtMessagesResponse, CrdtMerkles, CreateCrdtMerklesRequest, CreateCrdtMerklesResponse, GetCrdtMerklesRequest, GetCrdtMerklesResponse, UpdateCrdtMerklesRequest, UpdateCrdtMerklesResponse, DeleteCrdtMerklesRequest, DeleteCrdtMerklesResponse, BatchInsertCrdtMerklesRequest, BatchInsertCrdtMerklesResponse, BatchUpdateCrdtMerklesRequest, BatchUpdateCrdtMerklesResponse, BatchDeleteCrdtMerklesRequest, BatchDeleteCrdtMerklesResponse, SyncEndpoints, CreateSyncEndpointsRequest, CreateSyncEndpointsResponse, GetSyncEndpointsRequest, GetSyncEndpointsResponse, UpdateSyncEndpointsRequest, UpdateSyncEndpointsResponse, DeleteSyncEndpointsRequest, DeleteSyncEndpointsResponse, BatchInsertSyncEndpointsRequest, BatchInsertSyncEndpointsResponse, BatchUpdateSyncEndpointsRequest, BatchUpdateSyncEndpointsResponse, BatchDeleteSyncEndpointsRequest, BatchDeleteSyncEndpointsResponse, Queues, CreateQueuesRequest, CreateQueuesResponse, GetQueuesRequest, GetQueuesResponse, UpdateQueuesRequest, UpdateQueuesResponse, DeleteQueuesRequest, DeleteQueuesResponse, BatchInsertQueuesRequest, BatchInsertQueuesResponse, BatchUpdateQueuesRequest, BatchUpdateQueuesResponse, BatchDeleteQueuesRequest, BatchDeleteQueuesResponse, QueueItems, CreateQueueItemsRequest, CreateQueueItemsResponse, GetQueueItemsRequest, GetQueueItemsResponse, UpdateQueueItemsRequest, UpdateQueueItemsResponse, DeleteQueueItemsRequest, DeleteQueueItemsResponse, BatchInsertQueueItemsRequest, BatchInsertQueueItemsResponse, BatchUpdateQueueItemsRequest, BatchUpdateQueueItemsResponse, BatchDeleteQueueItemsRequest, BatchDeleteQueueItemsResponse, Transactions, CreateTransactionsRequest, CreateTransactionsResponse, GetTransactionsRequest, GetTransactionsResponse, UpdateTransactionsRequest, UpdateTransactionsResponse, DeleteTransactionsRequest, DeleteTransactionsResponse, BatchInsertTransactionsRequest, BatchInsertTransactionsResponse, BatchUpdateTransactionsRequest, BatchUpdateTransactionsResponse, BatchDeleteTransactionsRequest, BatchDeleteTransactionsResponse};
pub struct GrpcController {}

impl GrpcController {
    pub fn new() -> Self { GrpcController {} }

    pub async fn init(addr: &str) -> Result<(), Box<dyn std::error::Error>> {
        let addr: SocketAddr = addr.parse()?;
        let grpc_controller = GrpcController::new();
        println!("gRPC Server listening on {}", addr);
        Server::builder()
            .add_service(StoreServiceServer::with_interceptor(grpc_controller, GrpcAuthInterceptor))
            .serve(addr)
            .await?;
        Ok(())
    }
}

#[tonic::async_trait]
impl StoreService for GrpcController {
    // CRUD methods for items
    generate_create_method!(items);
    generate_update_method!(items, item);
    generate_batch_insert_method!(items);
    generate_batch_update_method!(items);
    generate_get_method!(items);
    generate_delete_method!(items);
    generate_batch_delete_method!(items);
    // CRUD methods for packets
    generate_create_method!(packets);
    generate_update_method!(packets, packet);
    generate_batch_insert_method!(packets);
    generate_batch_update_method!(packets);
    generate_get_method!(packets);
    generate_delete_method!(packets);
    generate_batch_delete_method!(packets);
    // CRUD methods for connections
    generate_create_method!(connections);
    generate_update_method!(connections, connection);
    generate_batch_insert_method!(connections);
    generate_batch_update_method!(connections);
    generate_get_method!(connections);
    generate_delete_method!(connections);
    generate_batch_delete_method!(connections);
    // CRUD methods for crdt_messages
    generate_create_method!(crdt_messages);
    generate_update_method!(crdt_messages, crdt_message);
    generate_batch_insert_method!(crdt_messages);
    generate_batch_update_method!(crdt_messages);
    generate_get_method!(crdt_messages);
    generate_delete_method!(crdt_messages);
    generate_batch_delete_method!(crdt_messages);
    // CRUD methods for crdt_merkles
    generate_create_method!(crdt_merkles);
    generate_update_method!(crdt_merkles, crdt_merkle);
    generate_batch_insert_method!(crdt_merkles);
    generate_batch_update_method!(crdt_merkles);
    generate_get_method!(crdt_merkles);
    generate_delete_method!(crdt_merkles);
    generate_batch_delete_method!(crdt_merkles);
    // CRUD methods for sync_endpoints
    generate_create_method!(sync_endpoints);
    generate_update_method!(sync_endpoints, sync_endpoint);
    generate_batch_insert_method!(sync_endpoints);
    generate_batch_update_method!(sync_endpoints);
    generate_get_method!(sync_endpoints);
    generate_delete_method!(sync_endpoints);
    generate_batch_delete_method!(sync_endpoints);
    // CRUD methods for queues
    generate_create_method!(queues);
    generate_update_method!(queues, queue);
    generate_batch_insert_method!(queues);
    generate_batch_update_method!(queues);
    generate_get_method!(queues);
    generate_delete_method!(queues);
    generate_batch_delete_method!(queues);
    // CRUD methods for queue_items
    generate_create_method!(queue_items);
    generate_update_method!(queue_items, queue_item);
    generate_batch_insert_method!(queue_items);
    generate_batch_update_method!(queue_items);
    generate_get_method!(queue_items);
    generate_delete_method!(queue_items);
    generate_batch_delete_method!(queue_items);
    // CRUD methods for transactions
    generate_create_method!(transactions);
    generate_update_method!(transactions, transaction);
    generate_batch_insert_method!(transactions);
    generate_batch_update_method!(transactions);
    generate_get_method!(transactions);
    generate_delete_method!(transactions);
    generate_batch_delete_method!(transactions);
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
