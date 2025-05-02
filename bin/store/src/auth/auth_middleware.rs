use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    error::ErrorUnauthorized,
    http::header,
    Error,
};
use futures::future::{ok, Ready};
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use serde_json::Value;
use std::env;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

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
        let auth_header = req.headers().get(header::AUTHORIZATION);

        let auth_result = match auth_header {
            Some(header_value) => {
                let auth_str = header_value.to_str().unwrap_or("");
                if auth_str.starts_with("Bearer ") {
                    let token = &auth_str[7..]; // Skip "Bearer " prefix
                                                // Validate the token here
                    validate_token(token)
                } else {
                    false
                }
            }
            None => false,
        };

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
