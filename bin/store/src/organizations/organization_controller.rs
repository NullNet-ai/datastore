use actix_web::{web, HttpMessage, HttpRequest, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

use crate::auth::auth_service::verify;
use crate::auth::structs::{Origin, User};
use crate::middlewares::auth_middleware::extract_token;
use crate::organizations::auth_service::{auth, root_auth};
use crate::organizations::organization_service::register;
use crate::organizations::structs::Register;
use crate::structs::structs::ApiResponse;

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
    // pub fn new() -> Self {
    //     Self
    // }

    pub async fn register(data: web::Json<Register>) -> impl Responder {
        if data.0.account_id.is_empty() || data.0.account_secret.is_empty() {
            return HttpResponse::BadRequest().json(serde_json::json!({"error": "Invalid Input"}));
        }

        let is_request = Some(true);

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
        if data.0.account_id.is_empty() || data.0.account_secret.is_empty() {
            return HttpResponse::BadRequest().json(serde_json::json!({"error": "Invalid Input"}));
        }

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
        let query_string = req.query_string();
        let session_option = req
            .extensions()
            .get::<crate::auth::structs::Session>()
            .cloned();
        let session = match session_option {
            Some(session) => session,
            None => {
                return HttpResponse::Unauthorized().json(crate::structs::structs::ApiResponse {
                    success: false,
                    message: "Session doesn't exist in the login request".to_string(),
                    count: 0,
                    data: vec![],
                })
            }
        };

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
                session.session_id.clone(),
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
                session.session_id.clone(),
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
                    let updated = crate::auth::structs::Session {
                        token: token.clone(),
                        origin: Some(Origin {
                            user_agent: req
                                .headers()
                                .get("user-agent")
                                .map(|v| v.to_str().unwrap_or_default().to_string()),
                            host: req.connection_info().host().to_string(),
                            url: req.path().to_string(),
                        }),
                        user: User {
                            role_id: login_response.role_id,
                            is_root_user: is_root,
                            account_id,
                        },
                        ..session
                    };
                    req.extensions_mut().insert(updated);

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

    pub async fn logout(_req: HttpRequest, _token_header: Option<String>) -> impl Responder {
        // Empty implementation
        HttpResponse::Ok().finish()
    }

    pub async fn verify_token(
        req: HttpRequest,
        query: web::Query<std::collections::HashMap<String, String>>,
    ) -> impl Responder {
        // Extract token from Authorization header or query parameter
        let auth_header = req
            .headers()
            .get("authorization")
            .and_then(|h| h.to_str().ok());

        let token_from_header = extract_token(auth_header);
        let token_from_query = query.get("t").cloned();

        let token = token_from_query.or(token_from_header);

        match token {
            Some(t) => match verify(&t) {
                Ok(result) => {
                    let success_response = ApiResponse {
                        success: true,
                        message: "Token Verified".to_string(),
                        count: 1,
                        data: vec![serde_json::to_value(result).unwrap_or_default()],
                    };
                    HttpResponse::Ok().json(success_response)
                }
                Err(err) => {
                    let error_response = ApiResponse {
                        success: false,
                        message: format!("Token Verification Failed: {}", err),
                        count: 0,
                        data: vec![],
                    };
                    HttpResponse::BadRequest().json(error_response)
                }
            },
            None => {
                let error_response = ApiResponse {
                    success: false,
                    message: "Token Verification Failed: Missing token".to_string(),
                    count: 0,
                    data: vec![],
                };
                HttpResponse::BadRequest().json(error_response)
            }
        }
    }
}
