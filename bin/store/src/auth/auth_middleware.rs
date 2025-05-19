use crate::structs::structs::Auth;
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

#[derive(Debug, Serialize, Deserialize)]
struct Account {
    organization_id: String,
    account_id: String,
    organization_account_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    account: Account,
    exp: usize,
    iat: usize,
}

// Authentication middleware struct
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

        match token {
            Some(t) => match validate_token(&t) {
                Ok(claims) => {
                    let auth_data = Auth {
                        organization_id: claims.account.organization_id.clone(),
                        responsible_account: claims.account.organization_account_id.clone(),
                    };

                    // Store the Auth object in request extensions
                    auth.extensions_mut().insert(auth_data);

                    let fut = self.service.call(auth);
                    Box::pin(async move {
                        let res = fut.await?;
                        Ok(res)
                    })
                }
                Err(_) => {
                    let error_response = serde_json::json!({
                        "success": false,
                        "message": "Token verification failed: Invalid or expired token"
                    });

                    Box::pin(async move {
                        let json_error = actix_web::HttpResponse::Unauthorized()
                            .content_type("application/json")
                            .json(error_response);
                        Err(actix_web::error::InternalError::from_response(
                            "Unauthorized",
                            json_error,
                        )
                        .into())
                    })
                }
            },
            None => {
                let error_response = serde_json::json!({
                    "success": false,
                    "message": "No authorization token provided"
                });

                Box::pin(async move {
                    let json_error = actix_web::HttpResponse::Unauthorized()
                        .content_type("application/json")
                        .json(error_response);
                    Err(
                        actix_web::error::InternalError::from_response("Unauthorized", json_error)
                            .into(),
                    )
                })
            }
        }
    }
}

// Function to validate the token
fn validate_token(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let jwt_secret = match env::var("JWT_SECRET") {
        Ok(secret) => secret,
        Err(_) => {
            println!("JWT_SECRET environment variable not set");
            return Err(jsonwebtoken::errors::Error::from(
                jsonwebtoken::errors::ErrorKind::InvalidToken,
            ));
        }
    };

    // Verify the JWT token
    let validation = Validation::new(Algorithm::HS256);
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(jwt_secret.as_bytes()),
        &validation,
    )?;

    Ok(token_data.claims)
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
            Some(t) => match validate_token(&t) {
                Ok(claims) => {
                    // Create Auth object with both IDs
                    let auth_data = Auth {
                        organization_id: claims.account.organization_id,
                        responsible_account: claims.account.organization_account_id,
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
