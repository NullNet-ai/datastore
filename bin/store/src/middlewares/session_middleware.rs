use std::{future::Future, pin::Pin, rc::Rc};

use actix_web::{
    cookie::{Cookie as ActixCookie, SameSite},
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpMessage,
};
use futures::future::{ok, Ready};
use tonic::service::Interceptor;
use tonic::{Request, Status};

use super::session_core::SessionManager;
use crate::auth::structs::{Claims, Origin, Session};
use crate::utils::utils::time_string_to_ms;

// Just a marker type
pub struct SessionMiddleware;

// Configuration holder used internally
#[allow(warnings)]
struct HttpSessionConfig {
    session_manager: SessionManager,
    cookie_same_site: SameSite,
    cookie_secure: bool,
    cookie_http_only: bool,
}

// Internal service
pub struct SessionMiddlewareService<S> {
    service: Rc<S>,
    config: HttpSessionConfig,
}

impl Default for SessionMiddleware {
    fn default() -> Self {
        Self
    }
}

impl<S, B> Transform<S, ServiceRequest> for SessionMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = SessionMiddlewareService<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        let session_manager = SessionManager::with_default_config();
        let config = HttpSessionConfig {
            session_manager,
            cookie_same_site: SameSite::Lax,
            cookie_secure: std::env::var("SESSION_COOKIE_SECURE")
                .map(|v| v == "true")
                .unwrap_or(false),
            cookie_http_only: true,
        };

        ok(SessionMiddlewareService {
            service: Rc::new(service),
            config,
        })
    }
}

impl<S, B> Service<ServiceRequest> for SessionMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let service = Rc::clone(&self.service);
        let session_manager = self.config.session_manager.clone();
        let cookie_same_site = self.config.cookie_same_site;
        let cookie_secure = self.config.cookie_secure;
        let cookie_http_only = self.config.cookie_http_only;

        let path = req.path().to_string();
        let skip_session = session_manager.should_skip_session(&path);

        Box::pin(async move {
            if skip_session {
                return service.call(req).await;
            }

            // Extract session ID from header or cookie
            let header_value = req
                .headers()
                .get(session_manager.session_header())
                .and_then(|h| h.to_str().ok());

            let cookie_value = req
                .cookie(session_manager.cookie_name())
                .map(|c| c.value().to_string());

            let session_id = header_value.map(|s| s.to_string()).or(cookie_value);

            // Load or create session
            let session = if let Some(session_id) = session_id {
                session_manager.get_or_create_session(&session_id).await
            } else {
                let new_session_id = session_manager.extract_session_id(None, None);
                session_manager.get_or_create_session(&new_session_id).await
            };

            // Store session in request extensions
            req.extensions_mut().insert(session.clone());

            // Call the next service
            let mut res = service.call(req).await?;

            // Get the possibly modified session from extensions
            let updated_session = res.request().extensions().get::<Session>().cloned();

            if let Some(session) = updated_session {
                // Save the session to the database
                if let Err(err) = session_manager.save_session(&session).await {
                    log::error!("Failed to save session: {:?}", err);
                }

                // Set the session cookie in the response
                let cookie = ActixCookie::build(session_manager.cookie_name(), session.session_id)
                    .path("/")
                    .same_site(cookie_same_site)
                    .secure(cookie_secure)
                    .http_only(cookie_http_only);

                // Set cookie expiration if max_age is provided
                let cookie =
                    if let Ok(max_age) = time_string_to_ms(session_manager.cookie_max_age()) {
                        cookie.max_age(actix_web::cookie::time::Duration::milliseconds(
                            max_age as i64,
                        ))
                    } else {
                        cookie
                    };

                res.response_mut().add_cookie(&cookie.finish())?;
            }

            Ok(res)
        })
    }
}

// Re-export the shared session management functions
pub use super::session_core::prune_expired_sessions;

// Helper function to get session from request
#[allow(warnings)]
pub fn get_session(req: &ServiceRequest) -> Option<Session> {
    req.extensions().get::<Session>().cloned()
}

// ============================================================================
// gRPC Session Management
// ============================================================================

/// gRPC Session Interceptor that reuses the core session logic
#[derive(Clone)]
pub struct GrpcSessionInterceptor {
    session_manager: SessionManager,
}

impl GrpcSessionInterceptor {
    pub fn new() -> Self {
        Self {
            session_manager: SessionManager::with_default_config(),
        }
    }

    #[allow(dead_code)]
    pub fn with_config(config: super::session_core::SessionConfig) -> Self {
        Self {
            session_manager: SessionManager::new(config),
        }
    }
}

impl Default for GrpcSessionInterceptor {
    fn default() -> Self {
        Self::new()
    }
}

impl Interceptor for GrpcSessionInterceptor {
    fn call(&mut self, mut request: Request<()>) -> Result<Request<()>, Status> {
        let metadata = request.metadata();

        // Log all headers for debugging
        log::debug!("gRPC Request headers: {:?}", metadata);

        // Extract session ID from header
        let session_header_value = metadata
            .get(self.session_manager.session_header())
            .and_then(|v| v.to_str().ok());

        log::debug!(
            "Session header '{}' value: {:?}",
            self.session_manager.session_header(),
            session_header_value
        );

        // For gRPC, we don't have cookies, so we only check headers
        let session_id = self
            .session_manager
            .extract_session_id(session_header_value, None);

        log::debug!("Extracted/Generated session ID: {}", session_id);

        // Store session ID in request extensions for later async loading
        // This avoids the runtime panic from calling block_on in an async context
        request.extensions_mut().insert(session_id);

        Ok(request)
    }
}

/// A chain of two interceptors that applies them in sequence
#[derive(Clone)]
pub struct InterceptorChain<A, B> {
    first: A,
    second: B,
}

impl<A, B> InterceptorChain<A, B> {
    pub fn new(first: A, second: B) -> Self {
        Self { first, second }
    }
}

impl<A, B> Interceptor for InterceptorChain<A, B>
where
    A: Interceptor + Clone,
    B: Interceptor + Clone,
{
    fn call(&mut self, request: Request<()>) -> Result<Request<()>, Status> {
        // Apply the first interceptor
        let mut first = self.first.clone();
        let request = first.call(request)?;

        // Apply the second interceptor
        let mut second = self.second.clone();
        second.call(request)
    }
}

/// Helper function to extract session from gRPC request
#[allow(dead_code)]
pub fn get_session_from_grpc_request<T>(request: &Request<T>) -> Option<Session> {
    request.extensions().get::<Session>().cloned()
}

/// Helper function to update session in gRPC request
#[allow(dead_code)]
pub fn update_session_in_grpc_request<T>(request: &mut Request<T>, session: Session) {
    request.extensions_mut().insert(session);
}

/// Async helper to save session after gRPC request processing
#[allow(dead_code)]
pub async fn save_session_after_request(session: &Session) -> Result<(), String> {
    let session_manager = SessionManager::with_default_config();
    session_manager
        .save_session(session)
        .await
        .map_err(|e| format!("Failed to save session: {:?}", e))
}

/// Common function to populate session with authentication data
/// Used by both HTTP and gRPC authentication flows
pub fn populate_session_with_auth_data(
    session: &mut Session,
    token: &str,
    claims: &Claims,
    origin: Origin,
) {
    // Update session with token
    session.token = token.to_string();

    // Update session with user data from claims
    session.user.role_id = claims.account.role_id.clone().unwrap_or_default();
    session.user.is_root_user = claims.account.is_root_account;
    session.user.account_id = claims.account.account_id.clone();

    // Set origin
    session.origin = Some(origin);
}

/// Load and populate session with auth data for gRPC requests
/// This function centralizes session management logic similar to HTTP middleware
pub async fn load_and_populate_session_for_grpc<T>(request: &tonic::Request<T>) -> Option<Session> {
    // Extract session ID from interceptor
    let session_id = request.extensions().get::<String>().cloned()?;

    let session_manager = SessionManager::with_default_config();
    let mut session = session_manager.get_or_create_session(&session_id).await;

    // Update session with auth data if available (similar to HTTP middleware)
    if let (Some(auth_token), Some(claims)) = (
        request
            .extensions()
            .get::<crate::middlewares::auth_middleware::AuthToken>(),
        request.extensions().get::<Claims>(),
    ) {
        // Create gRPC-specific origin
        let origin = Origin {
            user_agent: Some("gRPC-client".to_string()),
            host: "grpc".to_string(),
            url: "grpc://".to_string(),
        };

        // Use common function to populate session
        populate_session_with_auth_data(&mut session, &auth_token.0, claims, origin);
    }

    Some(session)
}
