use actix_web::{web, HttpRequest, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

use crate::organizations::organization_service::register;
use crate::organizations::structs::Register;

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthDto {
    pub account_id: Option<String>,
    pub account_secret: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
}

pub struct OrganizationsController;

impl OrganizationsController {
    pub fn new() -> Self {
        Self
    }

    pub async fn register(data: web::Json<Register>) -> impl Responder {
        // Check if data is valid
        if data.0.account_id.is_empty() || data.0.account_secret.is_empty() {
            return HttpResponse::BadRequest()
                .json({ serde_json::json!({"error": "Invalid Input"}) });
        }

        // Set is_request to true as in the TypeScript example
        let is_request = Some(true);

        // Call the organization service register function
        match register(&data.0, is_request, None).await {
            Ok(result) => HttpResponse::Ok().json(result),
            Err(e) => {
                log::error!("Registration error: {}", e);
                HttpResponse::InternalServerError().json({ serde_json::json!({"error": e}) })
            }
        }
    }

    pub async fn reregister_existing_account(
        data: web::Json<Register>,
        id: web::Path<String>,
    ) -> impl Responder {
        // Check if data is valid
        if data.0.account_id.is_empty() || data.0.account_secret.is_empty() {
            return HttpResponse::BadRequest()
                .json({ serde_json::json!({"error": "Invalid Input"}) });
        }

        // Set is_request to true as in the TypeScript example
        let is_request = Some(true);
        let param = id.to_string();

        // Call the organization service register function with the id parameter
        match register(&data.0, is_request, Some(param)).await {
            Ok(result) => HttpResponse::Ok().json(result),
            Err(e) => {
                log::error!("Registration error: {}", e);
                HttpResponse::InternalServerError().json({ serde_json::json!({"error": e}) })
            }
        }
    }

    pub async fn auth(data: web::Json<AuthDto>, req: HttpRequest) -> impl Responder {
        // Implementation remains the same, just remove the &self parameter
        HttpResponse::Ok().finish()
    }

    pub async fn logout(req: HttpRequest, token_header: Option<String>) -> impl Responder {
        // Empty implementation
        HttpResponse::Ok().finish()
    }
}
