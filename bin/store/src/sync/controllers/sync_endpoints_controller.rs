use crate::db;
use crate::models::sync_endpoint_model::SyncEndpointModel;
use crate::sync::sync_endpoints_service;
use crate::sync::transport::transport_driver::PostOpts;
use actix_web::{get, post, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct ResponsePackage {
    data: Vec<PostOpts>,
}

#[derive(Deserialize)]
pub struct EndpointRequest {
    endpoint: SyncEndpointModel,
}

#[get("/api/sync_endpoints")]
pub async fn get_sync_endpoints() -> impl Responder {
    let mut conn = db::get_async_connection().await;

    match sync_endpoints_service::get_sync_endpoints(&mut conn).await {
        Ok(endpoints) => {
            let response = ResponsePackage { data: endpoints };
            HttpResponse::Ok().json(response)
        }
        Err(e) => {
            log::error!("Failed to get sync endpoints: {}", e);
            HttpResponse::InternalServerError().body("Failed to get sync endpoints")
        }
    }
}
#[post("/api/sync_endpoints")]
pub async fn create_endpoint(endpoint_req: web::Json<EndpointRequest>) -> impl Responder {
    let mut conn = db::get_async_connection().await;

    match sync_endpoints_service::create_endpoint(&mut conn, &endpoint_req.endpoint).await {
        Ok(_) => HttpResponse::Ok().json(serde_json::json!({
            "message": "ok"
        })),
        Err(e) => {
            log::error!("Failed to create sync endpoint: {}", e);
            HttpResponse::InternalServerError().body("Failed to create sync endpoint")
        }
    }
}

// Function to configure and register the controller routes
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(get_sync_endpoints).service(create_endpoint);
}
