use super::common_controller::{
    convert_json_to_csv, execute_copy, perform_batch_update, perform_upsert,
    process_and_insert_record, process_and_update_record, process_record_for_insert,
    process_record_for_update, process_records, sanitize_updates,
};
use crate::db::create_connection;
use crate::generated::store::store_service_server::{StoreService, StoreServiceServer};
use crate::generated::store::{
    BatchDeleteConnectionsRequest, BatchDeleteConnectionsResponse, BatchDeleteDeviceSshKeysRequest,
    BatchDeleteDeviceSshKeysResponse, BatchDeletePacketsRequest, BatchDeletePacketsResponse,
    BatchInsertConnectionsRequest, BatchInsertConnectionsResponse, BatchInsertDeviceSshKeysRequest,
    BatchInsertDeviceSshKeysResponse, BatchInsertPacketsRequest, BatchInsertPacketsResponse,
    BatchUpdateConnectionsRequest, BatchUpdateConnectionsResponse, BatchUpdateDeviceSshKeysRequest,
    BatchUpdateDeviceSshKeysResponse, BatchUpdatePacketsRequest, BatchUpdatePacketsResponse,
    Connections, CreateConnectionsRequest, CreateConnectionsResponse, CreateDeviceSshKeysRequest,
    CreateDeviceSshKeysResponse, CreatePacketsRequest, CreatePacketsResponse,
    DeleteConnectionsRequest, DeleteConnectionsResponse, DeleteDeviceSshKeysRequest,
    DeleteDeviceSshKeysResponse, DeletePacketsRequest, DeletePacketsResponse, DeviceSshKeys,
    GetConnectionsRequest, GetConnectionsResponse, GetDeviceSshKeysRequest,
    GetDeviceSshKeysResponse, GetPacketsRequest, GetPacketsResponse, Packets,
    UpdateConnectionsRequest, UpdateConnectionsResponse, UpdateDeviceSshKeysRequest,
    UpdateDeviceSshKeysResponse, UpdatePacketsRequest, UpdatePacketsResponse,
    UpsertConnectionsRequest, UpsertConnectionsResponse, UpsertDeviceSshKeysRequest,
    UpsertDeviceSshKeysResponse, UpsertPacketsRequest, UpsertPacketsResponse,
};
use crate::middlewares::auth_middleware::GrpcAuthInterceptor;
use crate::structs::structs::Auth;
use crate::structs::structs::RequestBody;
use crate::sync::sync_service::{insert, update};
use crate::table_enum::Table;
use crate::utils::utils::table_exists;
use crate::{
    generate_batch_delete_method, generate_batch_insert_method, generate_batch_update_method,
    generate_create_method, generate_delete_method, generate_get_method, generate_update_method,
    generate_upsert_method,
};
use actix_web::{web, HttpResponse, Responder};
use serde_json::Value;
use std::net::SocketAddr;
use std::pin::Pin;
use tonic::{transport::Server, Request, Response, Status};
pub struct GrpcController {}

impl GrpcController {
    pub fn new() -> Self {
        GrpcController {}
    }

    pub async fn init(addr: &str) -> Result<(), Box<dyn std::error::Error>> {
        let addr: SocketAddr = addr.parse()?;
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
    // CRUD methods for packets
    generate_create_method!(packets);
    generate_update_method!(packets, packet);
    generate_batch_insert_method!(packets);
    generate_batch_update_method!(packets);
    generate_get_method!(packets);
    generate_delete_method!(packets);
    generate_batch_delete_method!(packets);
    generate_upsert_method!(packets);
    // CRUD methods for connections
    generate_create_method!(connections);
    generate_update_method!(connections, connection);
    generate_batch_insert_method!(connections);
    generate_batch_update_method!(connections);
    generate_get_method!(connections);
    generate_delete_method!(connections);
    generate_batch_delete_method!(connections);
    generate_upsert_method!(connections);
    // CRUD methods for device_ssh_keys
    generate_create_method!(device_ssh_keys);
    generate_update_method!(device_ssh_keys, device_ssh_key);
    generate_batch_insert_method!(device_ssh_keys);
    generate_batch_update_method!(device_ssh_keys);
    generate_get_method!(device_ssh_keys);
    generate_delete_method!(device_ssh_keys);
    generate_batch_delete_method!(device_ssh_keys);
    generate_upsert_method!(device_ssh_keys);
    // CRUD methods for device_group_settings
    generate_create_method!(device_group_settings);
    generate_update_method!(device_group_settings, device_group_setting);
    generate_batch_insert_method!(device_group_settings);
    generate_batch_update_method!(device_group_settings);
    generate_get_method!(device_group_settings);
    generate_delete_method!(device_group_settings);
    generate_batch_delete_method!(device_group_settings);
    generate_upsert_method!(device_group_settings);
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
