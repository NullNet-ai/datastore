use std::{future::Future, pin::Pin, rc::Rc};

use actix_http;
use actix_web::{
    cookie::{Cookie as ActixCookie, SameSite},
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    web::BytesMut,
    Error, HttpMessage,
};
use futures::future::{ok, Ready};
use futures::StreamExt;
use serde_json::Value;
use std::str::FromStr;
use tonic::service::Interceptor;
use tonic::{Request, Status};
use woothee::parser::Parser;

pub use super::session_core::prune_expired_sessions;
use super::session_core::{DeviceInfo, SessionManager};
use crate::structs::structs::{ApiResponse, Auth};
use crate::utils::utils::time_string_to_ms;
use crate::{
    providers::operations::auth::structs::{Claims, Origin},
    database::models::session_model::SessionModel,
};

pub struct SessionMiddleware;

struct HttpSessionConfig {
    session_manager: SessionManager,
    cookie_same_site: SameSite,
    cookie_secure: bool,
    cookie_http_only: bool,
}

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

    fn call(&self, mut req: ServiceRequest) -> Self::Future {
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

            let header_value = req
                .headers()
                .get(session_manager.session_header())
                .and_then(|h| h.to_str().ok());

            let cookie_value = req
                .cookie(session_manager.cookie_name())
                .map(|c| c.value().to_string());

            let session_id = header_value.map(|s| s.to_string()).or(cookie_value);

            // Check if this is a login route and handle account_id matching
            let is_login_route =
                path.contains("/auth") && req.method() == actix_web::http::Method::POST;

            // If route is not login and there is no session_id, reject the request
            if !is_login_route && session_id.is_none() {
                let error_response = ApiResponse {
                    success: false,
                    message: "Session ID is required for this request, please login first"
                        .to_string(),
                    count: 0,
                    data: vec![],
                };

                let json_error = actix_web::HttpResponse::Unauthorized()
                    .content_type("application/json")
                    .json(error_response);

                return Err(actix_web::error::InternalError::from_response(
                    "Unauthorized",
                    json_error,
                )
                .into());
            }
            let mut account_id_from_body: Option<String> = None;

            if is_login_route {
                // Extract and store the request body
                let mut payload = req.take_payload();
                let mut bytes = BytesMut::new();

                while let Some(chunk) = payload.next().await {
                    let chunk = chunk.map_err(|e| {
                        log::error!("Error reading request body: {}", e);
                        let error_response = ApiResponse {
                            success: false,
                            message: "Failed to read request body".to_string(),
                            count: 0,
                            data: vec![],
                        };
                        let json_error = actix_web::HttpResponse::BadRequest()
                            .content_type("application/json")
                            .json(error_response);
                        actix_web::error::InternalError::from_response("Bad Request", json_error)
                    })?;
                    bytes.extend_from_slice(&chunk);
                }

                let body_bytes = bytes.freeze();

                // Try to parse the body to extract account_id
                if let Ok(body_str) = std::str::from_utf8(&body_bytes) {
                    if let Ok(json_value) = serde_json::from_str::<Value>(body_str) {
                        // Extract account_id from the nested structure: data.account_id or data.email
                        if let Some(data) = json_value.get("data") {
                            account_id_from_body = data
                                .get("account_id")
                                .and_then(|v| v.as_str())
                                .map(|s| s.to_string())
                                .or_else(|| {
                                    data.get("email")
                                        .and_then(|v| v.as_str())
                                        .map(|s| s.to_string())
                                });
                        }
                    }
                }

                // Restore the body to the request so it can be consumed by the controller
                let (_, mut payload) = actix_http::h1::Payload::create(true);
                payload.unread_data(body_bytes);
                req.set_payload(payload.into());
            }

            // Extract IP address and location before session creation
            let location = req
                .headers()
                .get("x-forwarded-location")
                .and_then(|h| h.to_str().ok())
                .unwrap_or("Unknown");
            let authentication_method = req
                .headers()
                .get("x-authentication-method")
                .and_then(|h| h.to_str().ok())
                .unwrap_or("Unknown")
                .to_string();
            let ip_address = extract_client_ip(&req);

            // Extract device info from user agent
            let user_agent = req
                .headers()
                .get("user-agent")
                .and_then(|h| h.to_str().ok())
                .unwrap_or("Unknown");
            let device_info = parse_user_agent(user_agent);

            // Log device information
            log::info!(
                "Device Info - Browser: {}, OS: {}, Device: {}, User Agent: {}",
                device_info.browser_name,
                device_info.operating_system,
                device_info.device_name,
                user_agent
            );

            let (mut session, is_new_session) = if let Some(session_id) = session_id {
                // Try to load existing session first
                match session_manager.load_session(&session_id).await {
                    Ok(existing_session) => {
                        // For login routes, check if account_id matches existing session's user_account_id
                        if is_login_route {
                            if let (Some(body_account_id), Some(session_account_id)) =
                                (&account_id_from_body, &existing_session.user_account_id)
                            {
                                if body_account_id != session_account_id {
                                    // Account IDs don't match, create new session
                                    log::info!("Account ID mismatch for login route. Body: {}, Session: {}. Creating new session.", 
                                              body_account_id, session_account_id);
                                    let new_session_id =
                                        session_manager.extract_session_id(None, None);
                                    let new_session = session_manager
                                        .create_new_session(&new_session_id, "")
                                        .await;
                                    (new_session, true)
                                } else {
                                    // Account IDs match, use existing session
                                    log::info!("Account ID matches for login route. Reusing existing session.");
                                    (existing_session, false)
                                }
                            } else {
                                // Either no account_id in body or no user_account_id in session
                                // Use existing session but log the situation
                                if account_id_from_body.is_some()
                                    && existing_session.user_account_id.is_none()
                                {
                                    log::info!("Login route with account_id in body but no user_account_id in existing session. Using existing session.");
                                }
                                (existing_session, false)
                            }
                        } else {
                            // Not a login route, use existing session
                            (existing_session, false)
                        }
                    }
                    Err(_) => {
                        // Session doesn't exist, create new one
                        let new_session = session_manager.create_new_session(&session_id, "").await;
                        (new_session, true)
                    }
                }
            } else {
                // No session ID provided, create new session
                let new_session_id = session_manager.extract_session_id(None, None);
                let new_session = session_manager
                    .create_new_session(&new_session_id, "")
                    .await;
                (new_session, true)
            };

            // Update session with IP, location, and device info
            session.ip_address = Some(ip_address);
            session.location = Some(location.to_string());
            session.browser_name = Some(device_info.browser_name.clone());
            session.operating_system = Some(device_info.operating_system.clone());
            session.device_name = Some(device_info.device_name.clone());

            req.extensions_mut().insert(session.clone());

            let mut res = service.call(req).await?;

            let updated_session = res.request().extensions().get::<SessionModel>().cloned();

            if let Some(session) = updated_session {
                let auth = res.request().extensions().get::<Auth>().cloned();
                // Use account_profile_id from session if available, otherwise fall back to auth
                let account_organization_id = session
                    .account_organization_id
                    .clone()
                    .or_else(|| auth.as_ref().map(|a| a.account_organization_id.clone()));

                // Extract app_id from query parameters
                let app_id = res.request().query_string().split('&').find_map(|param| {
                    let mut parts = param.split('=');
                    if parts.next() == Some("app_id") {
                        parts.next().map(|s| s.to_string())
                    } else {
                        None
                    }
                });

                if let Err(e) = session_manager
                    .save_session(
                        &session,
                        account_organization_id,
                        Some(DeviceInfo {
                            device_name: device_info.device_name.clone(),
                            browser_name: device_info.browser_name.clone(),
                            operating_system: device_info.operating_system.clone(),
                            authentication_method: authentication_method.clone(),
                            location: session
                                .location
                                .clone()
                                .unwrap_or_else(|| "Unknown".to_string()),
                            ip_address: session
                                .ip_address
                                .clone()
                                .unwrap_or_else(|| "Unknown".to_string()),
                            remarks: None,
                        }),
                        auth.as_ref(),
                        app_id,
                        is_new_session,
                    )
                    .await
                {
                    log::error!("Failed to save session: {:?}", e);
                }

                let session_id = session.id.as_ref().ok_or_else(|| {
                    log::error!("Session ID doesn't exist");
                    let error_response = ApiResponse {
                        success: false,
                        message: "Session ID doesn't exist, please login first".to_string(),
                        count: 0,
                        data: vec![],
                    };
                    let json_error = actix_web::HttpResponse::InternalServerError()
                        .content_type("application/json")
                        .json(error_response);
                    actix_web::error::InternalError::from_response(
                        "Internal Server Error",
                        json_error,
                    )
                })?;

                let cookie = ActixCookie::build(session_manager.cookie_name(), session_id)
                    .path("/")
                    .same_site(cookie_same_site)
                    .secure(cookie_secure)
                    .http_only(cookie_http_only);

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

        let session_header_value = metadata
            .get(self.session_manager.session_header())
            .and_then(|v| v.to_str().ok());
        let session_id = self
            .session_manager
            .extract_session_id(session_header_value, None);

        request.extensions_mut().insert(session_id);

        Ok(request)
    }
}

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
        let mut first = self.first.clone();
        let request = first.call(request)?;

        let mut second = self.second.clone();
        second.call(request)
    }
}

#[allow(dead_code)]
pub fn get_session_from_grpc_request<T>(request: &Request<T>) -> Option<SessionModel> {
    request.extensions().get::<SessionModel>().cloned()
}

#[allow(dead_code)]
pub fn update_session_in_grpc_request<T>(request: &mut Request<T>, session: SessionModel) {
    request.extensions_mut().insert(session);
}

pub async fn save_session_after_request(session: &SessionModel) -> Result<(), String> {
    let session_manager = SessionManager::with_default_config();
    let account_profile_id = Some("1".to_string());
    session_manager
        .save_session(session, account_profile_id, None, None, None, false)
        .await
        .map_err(|e| format!("Failed to save session: {:?}", e))
}

pub fn populate_session_with_auth_data(
    session: &mut SessionModel,
    _token: &str,
    claims: &Claims,
    origin: Origin,
    req: &ServiceRequest,
) {
    session.token = None;
    session.user_role_id = claims.account.role_id.clone();
    session.user_is_root_user = Some(claims.account.is_root_account);
    session.user_account_id = Some(claims.account.account_id.clone());

    session.origin_url = Some(origin.url);
    session.origin_host = Some(origin.host);
    session.origin_user_agent = origin.user_agent;

    let ip_address = extract_client_ip(req);
    let location = req
        .headers()
        .get("x-forwarded-location")
        .and_then(|h| h.to_str().ok())
        // .map(|s| s.to_string())
        .unwrap_or("Unknown");
    // let location = get_location_from_ip(&ip_address);

    // Extract device info from user agent
    let user_agent = req
        .headers()
        .get("user-agent")
        .and_then(|h| h.to_str().ok())
        .unwrap_or("Unknown");
    let device_info = parse_user_agent(user_agent);

    // Log device information for auth data population
    log::info!(
        "Auth Session Device Info - Browser: {}, OS: {}, Device: {}, User Agent: {}",
        device_info.browser_name,
        device_info.operating_system,
        device_info.device_name,
        user_agent
    );

    session.ip_address = Some(ip_address);
    session.location = Some(location.to_string());
    session.browser_name = Some(device_info.browser_name);
    session.operating_system = Some(device_info.operating_system);
    session.device_name = Some(device_info.device_name);
}

fn extract_client_ip(req: &ServiceRequest) -> String {
    // First try to headers for proxied requests
    if let Some(forwarded_for) = req.headers().get("x-forwarded-for") {
        if let Ok(forwarded_str) = forwarded_for.to_str() {
            if let Some(first_ip) = forwarded_str.split(',').next() {
                return first_ip.trim().to_string();
            }
        }
    }

    if let Some(real_ip) = req.headers().get("x-real-ip") {
        if let Ok(ip_str) = real_ip.to_str() {
            return ip_str.to_string();
        }
    }

    // Fallback to get IP directly from TCP connection
    if let Some(peer_addr) = req.connection_info().peer_addr() {
        // Extract just the IP part (remove port if present)
        if let Some(ip_part) = peer_addr.split(':').next() {
            let ip_str = ip_part.trim();
            // Validate it's a proper IP address
            if std::net::IpAddr::from_str(ip_str).is_ok() {
                return ip_str.to_string();
            }
        }
    }

    // Final fallback
    req.connection_info()
        .realip_remote_addr()
        .unwrap_or("unknown")
        .to_string()
}

fn parse_user_agent(user_agent: &str) -> DeviceInfo {
    let parser = Parser::new();

    log::info!("Parsing User Agent: {}", user_agent);

    match parser.parse(user_agent) {
        Some(result) => {
            let browser_name = format!("{} {}", result.name, result.version);
            let operating_system = format!("{} {}", result.os, result.os_version);

            // Determine device type based on category
            let device_name = match result.category {
                "smartphone" => "Mobile".to_string(),
                "mobilephone" => "Mobile".to_string(),
                "tablet" => "Tablet".to_string(),
                "pc" => "Desktop".to_string(),
                "appliance" => "Smart Device".to_string(),
                "crawler" => "Bot/Crawler".to_string(),
                _ => "Unknown Device".to_string(),
            };

            log::info!(
                "Parsed device info - Browser: {}, OS: {}, Device: {}, Category: {}",
                browser_name,
                operating_system,
                device_name,
                result.category
            );

            DeviceInfo {
                device_name,
                browser_name,
                operating_system,
                authentication_method: "Unknown".to_string(),
                location: "Unknown".to_string(),
                ip_address: "Unknown".to_string(),
                remarks: None,
            }
        }
        None => {
            log::warn!("Failed to parse user agent: {}", user_agent);
            DeviceInfo {
                device_name: "Unknown Device".to_string(),
                browser_name: "Unknown Browser".to_string(),
                operating_system: "Unknown OS".to_string(),
                authentication_method: "Unknown".to_string(),
                location: "Unknown".to_string(),
                ip_address: "Unknown".to_string(),
                remarks: None,
            }
        }
    }
}

pub async fn load_and_populate_session_for_grpc<T>(
    request: &tonic::Request<T>,
) -> Option<SessionModel> {
    let session_id = request.extensions().get::<String>().cloned()?;

    let session_manager = SessionManager::with_default_config();
    let mut session = session_manager.get_or_create_session(&session_id, "").await;

    if let (Some(auth_token), Some(claims)) = (
        request
            .extensions()
            .get::<crate::middleware::auth_middleware::AuthToken>(),
        request.extensions().get::<Claims>(),
    ) {
        let origin = Origin {
            user_agent: Some("gRPC-client".to_string()),
            host: "grpc".to_string(),
            url: "grpc://".to_string(),
        };

        populate_session_with_auth_data_grpc(&mut session, &auth_token.0, claims, origin);
    }

    Some(session)
}

pub fn populate_session_with_auth_data_grpc(
    session: &mut SessionModel,
    _token: &str,
    claims: &Claims,
    origin: Origin,
) {
    session.token = Some("".to_string());

    session.user_role_id = claims.account.role_id.clone();
    session.user_is_root_user = Some(claims.account.is_root_account);
    session.user_account_id = Some(claims.account.account_id.clone());

    session.origin_url = Some(origin.url);
    session.origin_host = Some(origin.host);
    session.origin_user_agent = origin.user_agent;

    session.ip_address = Some("grpc-client".to_string());
    session.location = Some("gRPC".to_string());
}
