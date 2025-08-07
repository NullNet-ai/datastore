use chrono::{Duration, Utc};
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use serde_json::json;
use ulid::Ulid;

use crate::auth::structs::{Cookie, Session, User};
use crate::db;
use crate::models::session_model::SessionModel;
use crate::schema::schema::sessions;
use crate::utils::utils::time_string_to_ms;

#[derive(Clone)]
pub struct SessionConfig {
    pub cookie_name: String,
    pub cookie_max_age: String,
    pub session_header: String,
    #[allow(dead_code)]
    pub secret: String,
}

impl Default for SessionConfig {
    fn default() -> Self {
        Self {
            cookie_name: std::env::var("SESSION_COOKIE_NAME")
                .unwrap_or_else(|_| "SessionCookie".to_string()),
            cookie_max_age: std::env::var("SESSION_COOKIE_MAX_AGE")
                .unwrap_or_else(|_| "1d".to_string()),
            session_header: std::env::var("SESSION_HEADER_NAME")
                .unwrap_or_else(|_| "x-session-id".to_string()),
            secret: std::env::var("PGP_SYM_KEY").unwrap_or_default(),
        }
    }
}

/// Core session management functions that can be shared between HTTP and gRPC
#[derive(Clone)]
pub struct SessionManager {
    config: SessionConfig,
}

impl SessionManager {
    pub fn new(config: SessionConfig) -> Self {
        Self { config }
    }

    pub fn with_default_config() -> Self {
        Self::new(SessionConfig::default())
    }

    /// Extract session ID from various sources (header, cookie, etc.)
    pub fn extract_session_id(
        &self,
        header_value: Option<&str>,
        cookie_value: Option<&str>,
    ) -> String {
        header_value
            .map(|s| s.to_string())
            .or_else(|| cookie_value.map(|s| s.to_string()))
            .unwrap_or_else(|| {
                let new_id = Ulid::new().to_string();
                log::info!("Generated new SessionId {}", new_id);
                new_id
            })
    }

    /// Load or create a session
    pub async fn get_or_create_session(&self, session_id: &str) -> Session {
        match self.load_session(session_id).await {
            Ok(session) => session,
            Err(err) => {
                log::warn!(
                    "Error loading session, will create a new session: {:?}",
                    err
                );
                self.create_new_session(session_id, "")
            }
        }
    }

    /// Load existing session from database
    async fn load_session(&self, session_id: &str) -> Result<Session, diesel::result::Error> {
        let mut conn = db::get_async_connection().await;

        let session_model = sessions::table
            .filter(sessions::sid.eq(session_id))
            .first::<SessionModel>(&mut conn)
            .await?;

        // Parse the session data from JSON
        serde_json::from_value::<Session>(session_model.sess).map_err(|err| {
            log::error!("Error parsing session to json: {:?}", err);
            diesel::result::Error::DeserializationError(Box::new(err))
        })
    }

    pub fn create_new_session(&self, session_id: &str, token: &str) -> Session {
        let cookie_expiry_res = time_string_to_ms(&self.config.cookie_max_age);
        let cookie_exp = match cookie_expiry_res {
            Ok(expiry) => expiry,
            Err(err) => {
                log::error!(
                    "Error converting cookie expiry time '{}' to milliseconds: {}",
                    self.config.cookie_max_age,
                    err
                );
                86400000 // Default to 1 day (86400000 ms) on error
            }
        };

        let cookie = Cookie {
            path: "/".to_string(),
            expires: Utc::now()
                .checked_add_signed(Duration::milliseconds(cookie_exp as i64))
                .unwrap_or(Utc::now())
                .to_rfc3339(),
            originalMaxAge: cookie_exp as i64,
            httpOnly: true,
        };

        Session {
            user: User::default(),
            session_id: session_id.to_string(),
            origin: None,
            token: token.to_string(),
            cookie,
            ..Default::default()
        }
    }

    /// Save session to database
    pub async fn save_session(&self, session: &Session) -> Result<(), diesel::result::Error> {
        let mut conn = db::get_async_connection().await;

        let session_expires =
            std::env::var("SESSION_EXPIRES_IN").unwrap_or_else(|_| "1d".to_string());
        let expiry_ms = match time_string_to_ms(&session_expires) {
            Ok(ms) => ms,
            Err(err) => {
                log::warn!(
                    "Error converting session expiry time '{}' to milliseconds: {}",
                    session_expires,
                    err
                );
                86400000 // Default to 1 day (86400000 ms) on error
            }
        };

        let expires = Utc::now().naive_utc() + Duration::milliseconds(expiry_ms as i64);

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

    pub fn should_skip_session(&self, path: &str) -> bool {
        path.contains("logout") || !path.contains("/api/")
    }

    /// Get session header name
    pub fn session_header(&self) -> &str {
        &self.config.session_header
    }

    /// Get cookie name
    pub fn cookie_name(&self) -> &str {
        &self.config.cookie_name
    }

    /// Get cookie max age
    pub fn cookie_max_age(&self) -> &str {
        &self.config.cookie_max_age
    }
}

/// Function to prune expired sessions (shared utility)
pub async fn prune_expired_sessions() -> Result<usize, diesel::result::Error> {
    let mut conn = db::get_async_connection().await;
    let now = Utc::now().naive_utc();

    diesel::delete(sessions::table)
        .filter(sessions::expire.lt(now))
        .execute(&mut conn)
        .await
}
