use crate::db;
use actix_web::{web, HttpResponse, Responder};
use serde::Serialize;
use tonic::{transport::Server, Request, Response, Status};
use std::net::SocketAddr;  // Add this import

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

        // Here you would register your gRPC services
        // For example:
        // Server::builder()
        //     .add_service(YourServiceServer::new(grpc_controller))
        //     .serve(addr)
        //     .await?;

        Ok(())
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