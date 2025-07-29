use crate::auth::auth_service::verify;
use crate::auth::structs::{Origin, Session, Claims};
use crate::structs::structs::{ApiResponse, Auth};
use actix_web::HttpMessage;
use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    http::header,
    Error,
};
use futures::future::{ok, Ready};
use std::env;
use std::future::Future;
use std::pin::Pin;
use tonic::{Request, Status};

#[derive(Debug, Clone)]
pub struct AuthFailedMarker;

// Wrapper type for authentication token to avoid conflicts with session ID
#[derive(Debug, Clone)]
pub struct AuthToken(pub String);

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

        // Use unified authentication with path context
        let auth_result = authenticate_with_context(token, None, Some(auth.path()));
        
        match auth_result {
            AuthResult::Success { auth_data, token: t, claims } => {

                        // Store the Auth object in request extensions
                        auth.extensions_mut().insert(auth_data);

                        // Early return if EXPERIMENTAL_PERMISSIONS is not enabled
                        if env::var("EXPERIMENTAL_PERMISSIONS")
                            .unwrap_or_else(|_| "false".to_string())
                            != "true"
                        {
                            let fut = self.service.call(auth);
                            return Box::pin(async move {
                                let res = fut.await?;
                                Ok(res)
                            });
                        }
                        let maybe_session = auth.extensions().get::<Session>().cloned();
                        if let Some(mut session) = maybe_session {
                            // Create HTTP-specific origin
                            let host = auth.connection_info().host().to_string();
                            let url = auth.uri().to_string();
                            let user_agent = auth
                                .headers()
                                .get(header::USER_AGENT)
                                .and_then(|h| h.to_str().ok())
                                .map(|s| s.to_string());

                            let origin = Origin {
                                user_agent,
                                host,
                                url,
                            };

                            // Use common function to populate session
                            crate::middlewares::session_middleware::populate_session_with_auth_data(
                                &mut session,
                                &t,
                                &claims,
                                origin,
                            );

                            auth.extensions_mut().insert(session);
                        }

                let fut = self.service.call(auth);
                Box::pin(async move {
                    let res = fut.await?;
                    Ok(res)
                })
            }
            AuthResult::RootAccessDenied => {
                let error_response = ApiResponse {
                    success: false,
                    message: "Access denied: Root access required".to_string(),
                    count: 0,
                    data: vec![],
                };

                Box::pin(async move {
                    let json_error = actix_web::HttpResponse::Forbidden()
                        .content_type("application/json")
                        .json(error_response);
                    Err(actix_web::error::InternalError::from_response(
                        "Forbidden",
                        json_error,
                    )
                    .into())
                })
            }
            AuthResult::InvalidRootUsage => {
                let error_response = ApiResponse {
                    success: false,
                    message: "Invalid Authorization: Using Root Account on a non-root request".to_string(),
                    count: 0,
                    data: vec![],
                };

                Box::pin(async move {
                    let json_error = actix_web::HttpResponse::Forbidden()
                        .content_type("application/json")
                        .json(error_response);
                    Err(actix_web::error::InternalError::from_response(
                        "Forbidden",
                        json_error,
                    )
                    .into())
                })
            }
            AuthResult::TokenVerificationFailed(error_message) => {
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
            AuthResult::MissingToken => {
                let is_root_request = determine_root_request(None, Some(auth.path()));
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

// Common authentication result
#[derive(Debug, Clone)]
pub enum AuthResult {
    Success {
        auth_data: Auth,
        token: String,
        claims: Claims,
    },
    RootAccessDenied,
    InvalidRootUsage,
    TokenVerificationFailed(String),
    MissingToken,
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

// Common authentication logic for both HTTP and gRPC
pub fn authenticate_request(token: Option<String>, is_root_request: bool) -> AuthResult {
    match token {
        Some(t) => match verify(&t) {
            Ok(claims) => {
                // Check if this is a root request but the account is not a root account
                if is_root_request && !claims.account.is_root_account {
                    return AuthResult::RootAccessDenied;
                }

                // Check if this is not a root request but the account is a root account
                if !is_root_request && claims.account.is_root_account {
                    return AuthResult::InvalidRootUsage;
                }

                let auth_data = Auth {
                    organization_id: claims.account.organization_id.clone(),
                    responsible_account: claims.account.account_organization_id.clone(),
                    sensitivity_level: claims.sensitivity_level.unwrap_or(1000),
                    role_name: claims.role_name.clone().unwrap_or_default(),
                    account_organization_id: claims.account.organization_id.clone(),
                    role_id: claims.account.role_id.clone().unwrap_or_default(),
                    is_root_account: claims.account.is_root_account,
                    account_id: claims.account.account_id.clone(),
                };

                AuthResult::Success {
                    auth_data,
                    token: t,
                    claims,
                }
            }
            Err(err) => {
                let err_message = err.to_string();
                let error_message = if is_root_request {
                    format!("Root token verification failed: {}", err_message)
                } else {
                    format!("Token verification failed: {}", err_message)
                };
                AuthResult::TokenVerificationFailed(error_message)
            }
        },
        None => AuthResult::MissingToken,
    }
}

// Common function to determine if a request is a root request
pub fn determine_root_request(request_type: Option<&str>, path: Option<&str>) -> bool {
    // For gRPC: check the type parameter
    if let Some(req_type) = request_type {
        return req_type == "root";
    }
    
    // For HTTP: check the path
    if let Some(p) = path {
        return p.contains("/root/");
    }
    
    false
}

// Unified authentication function that handles both HTTP and gRPC
pub fn authenticate_with_context(token: Option<String>, request_type: Option<&str>, path: Option<&str>) -> AuthResult {
    let is_root_request = determine_root_request(request_type, path);
    authenticate_request(token, is_root_request)
}

// Simplified authentication for gRPC interceptor (token validation only)
pub fn authenticate_token_only(token: Option<String>) -> AuthResult {
    match token {
        Some(t) => match verify(&t) {
            Ok(claims) => {
                let auth_data = Auth {
                    organization_id: claims.account.organization_id.clone(),
                    responsible_account: claims.account.account_organization_id.clone(),
                    sensitivity_level: claims.sensitivity_level.unwrap_or(1000),
                    role_name: claims.role_name.clone().unwrap_or_default(),
                    account_organization_id: claims.account.organization_id.clone(),
                    role_id: claims.account.role_id.clone().unwrap_or_default(),
                    is_root_account: claims.account.is_root_account,
                    account_id: claims.account.account_id.clone(),
                };

                AuthResult::Success {
                    auth_data,
                    token: t,
                    claims,
                }
            }
            Err(e) => AuthResult::TokenVerificationFailed(e.to_string()),
        },
        None => AuthResult::MissingToken,
    }
}

// Root access validation function for use in gRPC macros
pub fn validate_root_access(claims: &Claims, is_root_request: bool) -> Result<(), AuthResult> {
    // Check if this is a root request but the account is not a root account
    if is_root_request && !claims.account.is_root_account {
        return Err(AuthResult::RootAccessDenied);
    }

    // Check if this is not a root request but the account is a root account
    if !is_root_request && claims.account.is_root_account {
        return Err(AuthResult::InvalidRootUsage);
    }

    Ok(())
}

// Common gRPC authentication and root validation logic
pub fn validate_grpc_request_with_root_access<T>(
    request: &tonic::Request<T>,
    request_type: &str,
) -> Result<(crate::structs::structs::Auth, Claims), tonic::Status> {
    // Get auth data
    let auth_data = match request.extensions().get::<crate::structs::structs::Auth>() {
        Some(data) => data.clone(),
        None => {
            return Err(tonic::Status::internal(
                "Authentication information not available",
            ));
        }
    };

    // Get claims for root access validation
    let claims = match request.extensions().get::<crate::auth::structs::Claims>() {
        Some(claims) => claims.clone(),
        None => {
            return Err(tonic::Status::internal(
                "Claims not available",
            ));
        }
    };

    // Determine if this is a root request and validate access
    let is_root_request = determine_root_request(Some(request_type), None);
    
    if let Err(auth_error) = validate_root_access(&claims, is_root_request) {
        return match auth_error {
            AuthResult::RootAccessDenied => {
                Err(tonic::Status::permission_denied("Access denied: Root access required"))
            }
            AuthResult::InvalidRootUsage => {
                Err(tonic::Status::permission_denied("Invalid Authorization: Using Root Account on a non-root request"))
            }
            _ => Err(tonic::Status::internal("Unexpected authentication error"))
        };
    }

    Ok((auth_data, claims))
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

        // Only validate token at interceptor level
        // Root access validation will be handled in gRPC macros where request params are available
        match authenticate_token_only(token) {
            AuthResult::Success { auth_data, token: t, claims } => {
                // Insert auth data into request extensions
                request.extensions_mut().insert(auth_data);
                request.extensions_mut().insert(AuthToken(t));
                request.extensions_mut().insert(claims);

                // Handle EXPERIMENTAL_PERMISSIONS if enabled
                if std::env::var("EXPERIMENTAL_PERMISSIONS").unwrap_or_default() == "true" {
                    // Session updates are handled in gRPC macros
                }

                Ok(request)
            }
            AuthResult::TokenVerificationFailed(error_message) => {
                // Store the error marker in request extensions
                request.extensions_mut().insert(AuthFailedMarker);
                
                Err(Status::unauthenticated(error_message))
            }
            AuthResult::MissingToken => {
                // Store the error marker in request extensions
                request.extensions_mut().insert(AuthFailedMarker);
                
                Err(Status::unauthenticated("Authorization required"))
            }
            // Root access errors should not occur at interceptor level
            AuthResult::RootAccessDenied | AuthResult::InvalidRootUsage => {
                Err(Status::internal("Unexpected root access validation at interceptor level"))
            }
        }
    }
}
