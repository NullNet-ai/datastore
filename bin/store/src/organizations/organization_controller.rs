use actix_web::{web, HttpRequest, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

use crate::organizations::organization_service::register;
use crate::organizations::structs::Register;
use crate::organizations::auth_service::{root_auth, auth};

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthDto {
    pub data: AuthData,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthData {
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
            return HttpResponse::BadRequest().json(serde_json::json!({"error": "Invalid Input"}));
        }

        // Set is_request to true as in the TypeScript example
        let is_request = Some(true);

        // Call the organization service register function
        match register(&data.0, is_request, None).await {
            Ok(result) => HttpResponse::Ok().json(result),
            Err(e) => {
                log::error!("Registration error: {}", e);
                HttpResponse::InternalServerError().json(serde_json::json!({"error": e}))
            }
        }
    }

    pub async fn reregister_existing_account(
        data: web::Json<Register>,
        id: web::Path<String>,
    ) -> impl Responder {
        // Check if data is valid
        if data.0.account_id.is_empty() || data.0.account_secret.is_empty() {
            return HttpResponse::BadRequest().json(serde_json::json!({"error": "Invalid Input"}));
        }

        // Set is_request to true as in the TypeScript example
        let is_request = Some(true);
        let param = id.to_string();

        // Call the organization service register function with the id parameter
        match register(&data.0, is_request, Some(param)).await {
            Ok(result) => HttpResponse::Ok().json(result),
            Err(e) => {
                log::error!("Registration error: {}", e);
                HttpResponse::InternalServerError().json(serde_json::json!({"error": e}))
            }
        }
    }

    pub async fn auth(data: web::Json<AuthDto>, req: HttpRequest) -> impl Responder {
        // Implementation remains the same, just remove the &self parameter
        let query_string = req.query_string();
        let query_params: Vec<(String, String)> = query_string
            .split('&')
            .filter(|s| !s.is_empty())
            .filter_map(|s| {
                let parts: Vec<&str> = s.split('=').collect();
                if parts.len() == 2 {
                    Some((parts[0].to_string(), parts[1].to_string()))
                } else {
                    None
                }
            })
            .collect();

        // Find is_root and t parameters
        let is_root = query_params
            .iter()
            .find(|(k, _)| k == "is_root")
            .map(|(_, v)| v == "true")
            .unwrap_or(false);

        let t = query_params
            .iter()
            .find(|(k, _)| k == "t")
            .map(|(_, v)| v.clone())
            .unwrap_or_default();

        // Get account_id and account_secret from the request body
        let account_id = data
            .data
            .account_id
            .clone()
            .unwrap_or_else(|| data.data.email.clone().unwrap_or_default());

        let account_secret = data
            .data
            .account_secret
            .clone()
            .unwrap_or_else(|| data.data.password.clone().unwrap_or_default());

        // Authenticate based on is_root parameter
        let result = if is_root {
            // Root authentication
            root_auth(
                &account_id,
                &account_secret,
                if !t.is_empty() { Some(&t) } else { None },
            )
            .await
            .map_err(|err| {
                log::error!("Root authentication error: {}", err);
                (err.message, None::<String>)
            })
        } else {
            // Regular authentication
           auth(
                &account_id,
                &account_secret,
                "", // Empty organization_id as it's not used in the auth function
            )
            .await
            .map_err(|err| {
                log::error!("Authentication error: {}", err);
                (err.message, None)
            })
        };

        // Handle the authentication result
        match result {
            Ok(login_response) => {
                if let Some(token) = login_response.token {
                    // Set cookie and return token
                    HttpResponse::Ok()
                        .cookie(
                            actix_web::cookie::Cookie::build("token", token.clone())
                                .path("/")
                                .finish(),
                        )
                        .json(serde_json::json!({ "token": token }))
                } else {
                    // Authentication failed but no error was thrown
                    HttpResponse::Forbidden().json(serde_json::json!({
                        "message": login_response.message
                    }))
                }
            }
            Err((message, _)) => {
                // Authentication error
                HttpResponse::Forbidden().json(serde_json::json!({
                    "message": message
                }))
            }
        }
    }

    pub async fn logout(req: HttpRequest, token_header: Option<String>) -> impl Responder {
        // Empty implementation
        HttpResponse::Ok().finish()
    }
}
