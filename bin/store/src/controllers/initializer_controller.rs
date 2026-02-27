use crate::initializers::system_initialization::init::{initialize, initialize_all};
use crate::initializers::system_initialization::structs::{EInitializer, InitializerParams};
use actix_web::{HttpResponse, Responder};
use serde_json::json;

pub async fn run_initializers() -> impl Responder {
    match initialize_all(None).await {
        Ok(_) => HttpResponse::Ok().json(json!({ "status": "ok" })),
        Err(e) => {
            log::error!("Failed to run initializers: {}", e);
            HttpResponse::InternalServerError().json(json!({ "error": format!("{}", e) }))
        }
    }
}

pub async fn run_system_code_config() -> impl Responder {
    match initialize(EInitializer::SYSTEM_CODE_CONFIG, None).await {
        Ok(_) => HttpResponse::Ok().json(json!({ "status": "ok" })),
        Err(e) => {
            log::error!("Failed to run system_code_config: {}", e);
            HttpResponse::InternalServerError().json(json!({ "error": format!("{}", e) }))
        }
    }
}

pub async fn run_root_account_config() -> impl Responder {
    let params = Some(InitializerParams {
        entity: "account_organizations".to_string(),
        ..Default::default()
    });
    match initialize(EInitializer::ROOT_ACCOUNT_CONFIG, params).await {
        Ok(_) => HttpResponse::Ok().json(json!({ "status": "ok" })),
        Err(e) => {
            log::error!("Failed to run root_account_config: {}", e);
            HttpResponse::InternalServerError().json(json!({ "error": format!("{}", e) }))
        }
    }
}

pub async fn run_global_organization_config() -> impl Responder {
    match initialize(EInitializer::GLOBAL_ORGANIZATION_CONFIG, None).await {
        Ok(_) => HttpResponse::Ok().json(json!({ "status": "ok" })),
        Err(e) => {
            log::error!("Failed to run global_organization_config: {}", e);
            HttpResponse::InternalServerError().json(json!({ "error": format!("{}", e) }))
        }
    }
}

pub async fn run_system_device_config() -> impl Responder {
    match initialize(EInitializer::SYSTEM_DEVICE_CONFIG, None).await {
        Ok(_) => HttpResponse::Ok().json(json!({ "status": "ok" })),
        Err(e) => {
            log::error!("Failed to run system_device_config: {}", e);
            HttpResponse::InternalServerError().json(json!({ "error": format!("{}", e) }))
        }
    }
}

pub async fn run_background_services_config() -> impl Responder {
    match initialize(EInitializer::BACKGROUND_SERVICES_CONFIG, None).await {
        Ok(_) => HttpResponse::Ok().json(json!({ "status": "ok" })),
        Err(e) => {
            log::error!("Failed to run background_services_config: {}", e);
            HttpResponse::InternalServerError().json(json!({ "error": format!("{}", e) }))
        }
    }
}

pub async fn run_initial_entity_data_config() -> impl Responder {
    match initialize(EInitializer::INITIAL_ENTITY_DATA_CONFIG, None).await {
        Ok(_) => HttpResponse::Ok().json(json!({ "status": "ok" })),
        Err(e) => {
            log::error!("Failed to run initial_entity_data_config: {}", e);
            HttpResponse::InternalServerError().json(json!({ "error": format!("{}", e) }))
        }
    }
}

pub async fn run_generate_schema_config() -> impl Responder {
    match initialize(EInitializer::GENERATE_SCHEMA_CONFIG, None).await {
        Ok(_) => HttpResponse::Ok().json(json!({ "status": "ok" })),
        Err(e) => {
            log::error!("Failed to run generate_schema_config: {}", e);
            HttpResponse::InternalServerError().json(json!({ "error": format!("{}", e) }))
        }
    }
}
