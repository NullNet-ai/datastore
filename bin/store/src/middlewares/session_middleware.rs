use std::{future::Future, pin::Pin, rc::Rc};

use actix_web::{
    cookie::{Cookie as ActixCookie, SameSite},
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpMessage,
};
use futures::future::{ok, Ready};
use std::net::IpAddr;
use std::str::FromStr;
use tonic::service::Interceptor;
use tonic::{Request, Status};

use super::session_core::{DeviceInfo, SessionManager};
use crate::auth::structs::{Claims, Origin, Session};
use crate::structs::structs::Auth;
use crate::utils::utils::time_string_to_ms;
pub use super::session_core::prune_expired_sessions;


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

            let header_value = req
                .headers()
                .get(session_manager.session_header())
                .and_then(|h| h.to_str().ok());

            let cookie_value = req
                .cookie(session_manager.cookie_name())
                .map(|c| c.value().to_string());

            let session_id = header_value.map(|s| s.to_string()).or(cookie_value);

            let session = if let Some(session_id) = session_id {
                session_manager.get_or_create_session(&session_id, "").await
            } else {
                let new_session_id = session_manager.extract_session_id(None, None);
                session_manager
                    .get_or_create_session(&new_session_id, "")
                    .await
            };

            req.extensions_mut().insert(session.clone());

            let mut res = service.call(req).await?;

            let updated_session = res.request().extensions().get::<Session>().cloned();

            if let Some(session) = updated_session {
                let auth = res.request().extensions().get::<Auth>().cloned();
                let account_profile_id = auth
                    .as_ref()
                    .and_then(|a| a.account_organization_id.parse::<i32>().ok());
                
                // Extract app_id from query parameters
                let app_id = res.request().query_string()
                    .split('&')
                    .find_map(|param| {
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
                        account_profile_id,
                        Some(DeviceInfo {
                            device_name: "Unknown".to_string(),
                            browser_name: "Unknown".to_string(),
                            operating_system: "Unknown".to_string(),
                            authentication_method: "Unknown".to_string(),
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
                    )
                    .await
                {
                    log::error!("Failed to save session: {:?}", e);
                }

                let cookie = ActixCookie::build(session_manager.cookie_name(), session.session_id)
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


pub fn get_session(req: &ServiceRequest) -> Option<Session> {
    req.extensions().get::<Session>().cloned()
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
pub fn get_session_from_grpc_request<T>(request: &Request<T>) -> Option<Session> {
    request.extensions().get::<Session>().cloned()
}

#[allow(dead_code)]
pub fn update_session_in_grpc_request<T>(request: &mut Request<T>, session: Session) {
    request.extensions_mut().insert(session);
}

#[allow(dead_code)]
pub async fn save_session_after_request(session: &Session) -> Result<(), String> {
    let session_manager = SessionManager::with_default_config();
    let account_profile_id = Some(1);
    session_manager
        .save_session(session, account_profile_id, None, None, None)
        .await
        .map_err(|e| format!("Failed to save session: {:?}", e))
}

pub fn populate_session_with_auth_data(
    session: &mut Session,
    _token: &str,
    claims: &Claims,
    origin: Origin,
    req: &ServiceRequest,
) {
    session.token = "none".to_string();
    session.user.role_id = claims.account.role_id.clone().unwrap_or_default();
    session.user.is_root_user = claims.account.is_root_account;
    session.user.account_id = claims.account.account_id.clone();

    session.origin = Some(origin);

    let ip_address = extract_client_ip(req);

    let location = get_location_from_ip(&ip_address);

    session.ip_address = Some(ip_address);
    session.location = location;
}

fn extract_client_ip(req: &ServiceRequest) -> String {
    // First try to get IP directly from TCP connection
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

    // Fallback to headers for proxied requests
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

    // Final fallback
    req.connection_info()
        .realip_remote_addr()
        .unwrap_or("unknown")
        .to_string()
}

fn get_location_from_ip(ip_address: &str) -> Option<String> {
    if let Ok(ip) = IpAddr::from_str(ip_address) {
        match ip {
            IpAddr::V4(ipv4) => {
                if ipv4.is_loopback() || ipv4.is_private() {
                    return Some("Local".to_string());
                }
            }
            IpAddr::V6(ipv6) => {
                if ipv6.is_loopback() {
                    return Some("Local".to_string());
                }
            }
        }

        Some("Unknown Location".to_string())
    } else {
        None
    }
}

pub async fn load_and_populate_session_for_grpc<T>(request: &tonic::Request<T>) -> Option<Session> {
    let session_id = request.extensions().get::<String>().cloned()?;

    let session_manager = SessionManager::with_default_config();
    let mut session = session_manager.get_or_create_session(&session_id, "").await;

    if let (Some(auth_token), Some(claims)) = (
        request
            .extensions()
            .get::<crate::middlewares::auth_middleware::AuthToken>(),
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
    session: &mut Session,
    token: &str,
    claims: &Claims,
    origin: Origin,
) {
    session.token = token.to_string();

    session.user.role_id = claims.account.role_id.clone().unwrap_or_default();
    session.user.is_root_user = claims.account.is_root_account;
    session.user.account_id = claims.account.account_id.clone();

    session.origin = Some(origin);

    session.ip_address = Some("grpc-client".to_string());
    session.location = Some("gRPC".to_string());
}
