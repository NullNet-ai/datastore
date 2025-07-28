use std::{future::Future, pin::Pin, rc::Rc};

use actix_web::{
    cookie::{Cookie as ActixCookie, SameSite},
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpMessage,
};
use futures::future::{ok, Ready};

use crate::auth::structs::Session;
use crate::utils::utils::time_string_to_ms;
use super::session_core::SessionManager;

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
                let cookie = if let Ok(max_age) = time_string_to_ms(session_manager.cookie_max_age()) {
                    cookie.max_age(actix_web::cookie::time::Duration::milliseconds(max_age as i64))
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
