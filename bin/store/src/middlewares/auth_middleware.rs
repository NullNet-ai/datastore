use crate::auth::auth_service::verify;
use crate::auth::structs::Claims;
use crate::structs::structs::{ApiResponse, Auth};
use actix_web::HttpMessage;
use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    http::header,
    Error,
};
use futures::future::{ok, Ready};
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use std::env;
use std::future::Future;
use std::pin::Pin;
use tonic::{Request, Status};

#[derive(Debug)]
pub struct AuthFailedMarker;

pub struct Authentication;

// Middleware factory implementation
impl<S, B> Transform<S, ServiceRequest> for Authentication
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = AuthenticationMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(AuthenticationMiddleware { service })
    }
}

// Middleware service implementation
pub struct AuthenticationMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for AuthenticationMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    forward_ready!(service);

    fn call(&self, auth: ServiceRequest) -> Self::Future {
        // Extract the token from the Authorization header
        let auth_header = auth
            .headers()
            .get(header::AUTHORIZATION)
            .and_then(|h| h.to_str().ok());

        let token = extract_token(auth_header);

        // Check if this is a root request
        let is_root_request = auth.path().contains("/root/");

        match token {
            Some(t) => {
                match verify(&t) {
                    Ok(claims) => {
                        // Check if this is a root request but the account is not a root account
                        if is_root_request && !claims.account.is_root_account {
                            let error_response = ApiResponse {
                                success: false,
                                message: "Access denied: Root access required".to_string(),
                                count: 0,
                                data: vec![],
                            };

                            return Box::pin(async move {
                                let json_error = actix_web::HttpResponse::Forbidden()
                                    .content_type("application/json")
                                    .json(error_response);
                                Err(actix_web::error::InternalError::from_response(
                                    "Forbidden",
                                    json_error,
                                )
                                .into())
                            });
                        }

                        // Check if this is not a root request but the account is a root account
                        if !is_root_request && claims.account.is_root_account {
                            let error_response = ApiResponse {
                        success: false,
                        message: "Invalid Authorization: Using Root Account on a non-root request".to_string(),
                        count: 0,
                        data: vec![],
                    };

                            return Box::pin(async move {
                                let json_error = actix_web::HttpResponse::Forbidden()
                                    .content_type("application/json")
                                    .json(error_response);
                                Err(actix_web::error::InternalError::from_response(
                                    "Forbidden",
                                    json_error,
                                )
                                .into())
                            });
                        }

                        let auth_data = Auth {
                            organization_id: claims.account.organization_id.clone(),
                            responsible_account: claims.account.account_organization_id.clone(),
                            sensitivity_level: claims.sensitivity_level,
                            role_name: claims.role_name.clone(),
                            is_root_account: claims.account.is_root_account,
                        };

                        // Store the Auth object in request extensions
                        auth.extensions_mut().insert(auth_data);

                        let fut = self.service.call(auth);
                        Box::pin(async move {
                            let res = fut.await?;
                            Ok(res)
                        })
                    }
                    Err(err) => {
                        // Get the actual error message from the error
                        let err_message = err.to_string();

                        let error_message = if is_root_request {
                            format!("Root token verification failed: {}", err_message)
                        } else {
                            format!("Token verification failed: {}", err_message)
                        };

                        let error_response = ApiResponse {
                            success: false,
                            message: error_message,
                            count: 0,
                            data: vec![],
                        };

                        Box::pin(async move {
                            let json_error = actix_web::HttpResponse::Unauthorized()
                                .content_type("application/json")
                                .json(error_response);
                            let error: actix_web::Error =
                                actix_web::error::InternalError::from_response(
                                    "Unauthorized",
                                    json_error,
                                )
                                .into();

                            // Store the error in request extensions
                            auth.extensions_mut().insert(AuthFailedMarker);

                            // Return the error
                            Err(error)
                        })
                    }
                }
            }
            None => {
                let error_message = if is_root_request {
                    "Root authorization required"
                } else {
                    "Authorization required"
                };

                let error_response = ApiResponse {
                    success: false,
                    message: error_message.to_string(),
                    count: 0,
                    data: vec![],
                };

                Box::pin(async move {
                    let json_error = actix_web::HttpResponse::Unauthorized()
                        .content_type("application/json")
                        .json(error_response);
                    let error: actix_web::Error =
                        actix_web::error::InternalError::from_response("Unauthorized", json_error)
                            .into();

                    // Store a marker in request extensions to indicate authentication failed
                    auth.extensions_mut().insert(AuthFailedMarker);

                    // Return the error
                    Err(error)
                })
            }
        }
    }
}

// Extract token from various sources
pub fn extract_token(auth_header: Option<&str>) -> Option<String> {
    match auth_header {
        Some(auth_str) if auth_str.starts_with("Bearer ") => {
            Some(auth_str[7..].to_string()) // Skip "Bearer " prefix
        }
        _ => None,
    }
}

use tonic::service::Interceptor;

#[derive(Clone)]
pub struct GrpcAuthInterceptor;

impl Interceptor for GrpcAuthInterceptor {
    fn call(&mut self, mut request: Request<()>) -> Result<Request<()>, Status> {
        let metadata = request.metadata();

        let auth_header = metadata.get("authorization").and_then(|v| v.to_str().ok());
        let token = extract_token(auth_header).or_else(|| {
            metadata
                .get("token")
                .and_then(|v| v.to_str().ok())
                .map(|s| s.to_string())
        });

        match token {
            Some(t) => match verify(&t) {
                Ok(claims) => {
                    // Create Auth object with both IDs
                    let auth_data = Auth {
                        organization_id: claims.account.organization_id.clone(),
                        responsible_account: claims.account.account_organization_id.clone(),
                        sensitivity_level: claims.sensitivity_level,
                        role_name: claims.role_name.clone(),
                        is_root_account: claims.account.is_root_account,
                    };

                    // Store the Auth object in request extensions
                    request.extensions_mut().insert(auth_data);
                    Ok(request)
                }
                Err(_) => Err(Status::unauthenticated("Invalid token")),
            },
            None => Err(Status::unauthenticated("Missing authentication token")),
        }
    }
}
