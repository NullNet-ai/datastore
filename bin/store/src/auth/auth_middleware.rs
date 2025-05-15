use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    http::header,
    Error,
};
use futures::future::{ok, Ready};
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use serde_json::Value;
use std::env;
use std::future::Future;
use std::pin::Pin;
use tonic::{Request, Status};

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

    fn call(&self, req: ServiceRequest) -> Self::Future {
        // Extract the token from the Authorization header
        let auth_header = req
            .headers()
            .get(header::AUTHORIZATION)
            .and_then(|h| h.to_str().ok());

        let token = extract_token(auth_header);

        let auth_result = token.map(|t| validate_token(&t)).unwrap_or(false);

        if auth_result {
            // If authentication is successful, call the next service
            let fut = self.service.call(req);
            Box::pin(async move {
                let res = fut.await?;
                Ok(res)
            })
        } else {
            // If authentication fails, return a JSON error response
            let error_response = serde_json::json!({
                "success": false,
                "message": "Token verification failed: Invalid or expired token"
            });

            Box::pin(async move {
                // Create a proper JSON response with correct content type
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

// Function to validate the token
fn validate_token(token: &str) -> bool {
    // JWT secret - you should store this in an environment variable
    let jwt_secret = match env::var("JWT_SECRET") {
        Ok(secret) => secret,
        Err(_) => {
            println!("JWT_SECRET environment variable not set");
            return false;
        }
    };

    // Verify the JWT token
    let validation = Validation::new(Algorithm::HS256);

    match decode::<Value>(
        token,
        &DecodingKey::from_secret(jwt_secret.as_bytes()),
        &validation,
    ) {
        Ok(_token_data) => true,
        Err(err) => {
            // Token validation failed
            println!("Token validation error: {:?}", err);
            false
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
    fn call(&mut self, request: Request<()>) -> Result<Request<()>, Status> {
        // Try to get token from metadata
        let metadata = request.metadata();

        // First check for "authorization" metadata
        let auth_header = metadata.get("authorization").and_then(|v| v.to_str().ok());

        let token = extract_token(auth_header);

        // If no token in authorization header, check for "token" metadata
        let token = token.or_else(|| {
            metadata
                .get("token")
                .and_then(|v| v.to_str().ok())
                .map(|s| s.to_string())
        });

        match token {
            Some(t) if validate_token(&t) => Ok(request),
            _ => Err(Status::unauthenticated(
                "Invalid or missing authentication token",
            )),
        }
    }
}
