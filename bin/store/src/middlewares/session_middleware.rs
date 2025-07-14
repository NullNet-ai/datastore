use std::{future::Future, pin::Pin, rc::Rc};

use actix_web::{
    cookie::{Cookie as ActixCookie, SameSite},
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpMessage,
};
use chrono::{Duration, Utc};
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use futures::future::{ok, Ready};
use serde_json::json;
use ulid::Ulid;

use crate::db;
use crate::models::session_model::SessionModel;
use crate::schema::schema::sessions;
use crate::{
    auth::structs::{Session, User},
    utils::utils::time_string_to_ms,
};

// Just a marker type
pub struct SessionMiddleware;

// Configuration holder used internally
struct SessionConfig {
    cookie_name: String,
    cookie_max_age: String,
    session_header: String,
    cookie_same_site: SameSite,
    cookie_secure: bool,
    cookie_http_only: bool,
    secret: String,
}

// Internal service
pub struct SessionMiddlewareService<S> {
    service: Rc<S>,
    config: SessionConfig,
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
        let config = SessionConfig {
            cookie_name: std::env::var("SESSION_COOKIE_NAME")
                .unwrap_or_else(|_| "SessionCookie".to_string()),
            cookie_max_age: std::env::var("SESSION_COOKIE_MAX_AGE")
                .unwrap_or_else(|_| "1d".to_string()),
            session_header: std::env::var("SESSION_HEADER_NAME")
                .unwrap_or_else(|_| "x-session-id".to_string()),
            cookie_same_site: SameSite::Lax,
            cookie_secure: std::env::var("SESSION_COOKIE_SECURE")
                .map(|v| v == "true")
                .unwrap_or(false),
            cookie_http_only: true,
            secret: std::env::var("PGP_SYM_KEY").unwrap_or_default(),
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
        let config = &self.config;
        let cookie_name = config.cookie_name.clone();
        let cookie_max_age = config.cookie_max_age.clone();
        let session_header = config.session_header.clone();
        let cookie_same_site = config.cookie_same_site;
        let cookie_secure = config.cookie_secure;
        let cookie_http_only = config.cookie_http_only;

        let path = req.path().to_string();
        let skip_session = path.contains("logout") || !path.contains("/api/");

        Box::pin(async move {
            if skip_session {
                return service.call(req).await;
            }

            // Extract session ID from header or cookie or generate a new one
            let session_id = req
                .headers()
                .get(&session_header)
                .and_then(|h| h.to_str().ok())
                .map(|s| s.to_string())
                .or_else(|| {
                    // Try to get session ID from cookie
                    req.cookie(&cookie_name).map(|c| c.value().to_string())
                })
                .unwrap_or_else(|| {
                    // Generate new session ID if none exists
                    let new_id = Ulid::new().to_string();
                    log::info!("Generated new SessionId {}", new_id);
                    new_id
                });

            // Try to load existing session
            let mut conn = db::get_async_connection().await;

            let session_result = sessions::table
                .filter(sessions::sid.eq(&session_id))
                .first::<SessionModel>(&mut conn)
                .await;

            let session = match session_result {
                Ok(session_model) => {
                    // Parse the session data from JSON
                    match serde_json::from_value::<Session>(session_model.sess) {
                        Ok(session) => session,
                        Err(err) => {
                            // Create new session if parsing fails
                            log::error!(
                                "Error parsing session to json, in session middleware, {:?}",
                                err
                            );
                            create_new_session(&session_id, &cookie_max_age, "".to_string())
                        }
                    }
                }
                Err(err) => {
                    // log an error and print it
                    log::error!("Error loading session, in session middleware, {:?}", err);
                    create_new_session(&session_id, &cookie_max_age, "".to_string())
                }
            };

            // Store session in request extensions
            req.extensions_mut().insert(session.clone());

            // Call the next service
            let mut res = service.call(req).await?;

            // Get the possibly modified session from extensions
            let updated_session = res.request().extensions().get::<Session>().cloned();

            if let Some(session) = updated_session {
                // Save the session to the database
                if let Err(err) = save_session(&session).await {
                    log::error!("Failed to save session: {:?}", err);
                }

                // Set the session cookie in the response
                let cookie = ActixCookie::build(cookie_name, session.session_id)
                    .path("/")
                    .same_site(cookie_same_site)
                    .secure(cookie_secure)
                    .http_only(cookie_http_only);

                // Set cookie expiration if max_age is provided
                let cookie = if let Ok(max_age) = time_string_to_ms(&cookie_max_age) {
                    cookie.max_age(time::Duration::milliseconds(max_age as i64))
                } else {
                    cookie
                };

                res.response_mut().add_cookie(&cookie.finish())?;
            }

            Ok(res)
        })
    }
}

// Helper function to create a new session
fn create_new_session(session_id: &str, cookie_max_age: &str, token: String) -> Session {
    let cookie_expiry_res = time_string_to_ms(cookie_max_age);
    let cookie_exp = match cookie_expiry_res {
        Ok(expiry) => expiry,
        Err(err) => {
            log::error!("Error converting cookie expiry time '{}' to milliseconds in session middleware: {}", cookie_max_age, err);
            86400000 // Default to 1 day (86400000 ms) on error
        }
    };

    let cookie = crate::auth::structs::Cookie {
        path: "/".to_string(),
        expires: Utc::now()
            .checked_add_signed(Duration::milliseconds(cookie_exp as i64))
            .unwrap_or(Utc::now())
            .to_rfc3339(),
        originalMaxAge: cookie_exp as i64,
        httpOnly: true,
    };

    // Create a new session with default User and extracted Origin
    Session {
        user: User::default(),
        session_id: session_id.to_string(),
        origin: None,
        token,
        cookie,
        ..Default::default()
    }
}

// Helper function to save session data
pub async fn save_session(session: &Session) -> Result<(), diesel::result::Error> {
    let mut conn = db::get_async_connection().await;

    let session_expires = std::env::var("SESSION_EXPIRES_IN").unwrap_or_else(|_| "1d".to_string());
    let expiry_ms = match time_string_to_ms(&session_expires) {
        Ok(ms) => ms,
        Err(err) => {
            log::error!("Error converting session expiry time '{}' to milliseconds in session middleware: {}", session_expires, err);
            86400000 // Default to 1 day (86400000 ms) on error
        }
    };

    let expires = Utc::now().naive_utc() + Duration::milliseconds(expiry_ms as i64); // Default expiry of 1 day

    let session_model = SessionModel {
        sid: session.session_id.clone(),
        sess: json!(session),
        expire: expires,
    };

    diesel::insert_into(sessions::table)
        .values(&session_model)
        .on_conflict(sessions::sid)
        .do_update()
        .set(&session_model)
        .execute(&mut conn)
        .await
        .map(|_| ())
}

// Helper function to get session from request
pub fn get_session(req: &ServiceRequest) -> Option<Session> {
    req.extensions().get::<Session>().cloned()
}

// Function to prune expired sessions
pub async fn prune_expired_sessions() -> Result<usize, diesel::result::Error> {
    let mut conn = db::get_async_connection().await;
    let now = Utc::now().naive_utc();

    diesel::delete(sessions::table)
        .filter(sessions::expire.lt(now))
        .execute(&mut conn)
        .await
}
