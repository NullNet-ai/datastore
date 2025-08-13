use chrono::{Duration, Utc};
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use std::env;

use ulid::Ulid;

use crate::auth::structs::{Cookie, Session, User};
use crate::db;
use crate::models::session_model::SessionModel;
use crate::schema::schema::{account_organizations, sessions};
use crate::structs::structs::{Auth, RequestBody};
use crate::utils::utils::time_string_to_ms;
use serde_json::json;

#[derive(Clone, Debug)]
pub struct DeviceInfo {
    pub device_name: String,
    pub browser_name: String,
    pub operating_system: String,
    pub authentication_method: String,
    pub location: String,
    pub ip_address: String,
    pub remarks: Option<String>,
}

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
    pub async fn get_or_create_session(&self, session_id: &str, token: &str) -> Session {
        match self.load_session(session_id).await {
            Ok(session) => session,
            Err(err) => {
                log::warn!(
                    "Error loading session, will create a new session: {:?}",
                    err
                );
                self.create_new_session(session_id, token)
            }
        }
    }

    /// Load existing session from database
    async fn load_session(&self, session_id: &str) -> Result<Session, diesel::result::Error> {
        let mut conn = db::get_async_connection().await;

        let session_model = sessions::table
            .filter(sessions::id.eq(session_id))
            .filter(sessions::status.eq("Active"))
            .filter(sessions::tombstone.eq(0))
            .first::<SessionModel>(&mut conn)
            .await?;

        // Update last_accessed timestamp
        diesel::update(sessions::table.filter(sessions::id.eq(session_id)))
            .set(sessions::last_accessed.eq(Some(Utc::now().naive_utc())))
            .execute(&mut conn)
            .await
            .ok(); // Ignore errors for this update

        // Convert SessionModel back to Session struct
        Ok(Session {
            user: User {
                role_id: session_model.user_role_id.unwrap_or_default(),
                account_id: session_model.user_account_id.unwrap_or_default(),
                is_root_user: session_model.user_is_root_user.unwrap_or(false),
            },
            session_id: session_model.id.unwrap_or_default(),
            origin: if session_model.origin_url.is_some()
                || session_model.origin_host.is_some()
                || session_model.origin_user_agent.is_some()
            {
                Some(crate::auth::structs::Origin {
                    url: session_model.origin_url.unwrap_or_default(),
                    host: session_model.origin_host.unwrap_or_default(),
                    user_agent: session_model.origin_user_agent,
                })
            } else {
                None
            },
            token: session_model.token.unwrap_or_default(),
            cookie: Cookie {
                path: session_model.cookie_path.unwrap_or("/".to_string()),
                expires: session_model.cookie_expires.unwrap_or_default(),
                originalMaxAge: session_model.cookie_original_max_age.unwrap_or(86400000),
                httpOnly: session_model.cookie_http_only.unwrap_or(true),
            },
            valid_pass_keys: session_model
                .valid_pass_key
                .and_then(|v| serde_json::from_str(&v).ok()),
            role_permissions: session_model
                .role_permission
                .and_then(|v| serde_json::from_str(&v).ok()),
            field_permissions: session_model
                .field_permission
                .and_then(|v| serde_json::from_str(&v).ok()),
            record_permissions: session_model
                .record_permission
                .and_then(|v| serde_json::from_str(&v).ok()),
            ip_address: session_model.ip_address,
            location: session_model.location,
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
    pub async fn save_session(
        &self,
        session: &Session,
        account_profile_id: Option<i32>,
        device_info: Option<DeviceInfo>,
        auth: Option<&Auth>,
        app_id: Option<String>,
    ) -> Result<(), diesel::result::Error> {
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

        // Check if session already exists to determine if this is create or update
        let existing_session = {
            let mut conn = db::get_async_connection().await;
            sessions::table
                .filter(sessions::id.eq(&session.session_id))
                .first::<SessionModel>(&mut conn)
                .await
                .optional()
                .map_err(|e| diesel::result::Error::DatabaseError(
                    diesel::result::DatabaseErrorKind::Unknown,
                    Box::new(format!("Failed to query existing session: {}", e)),
                ))?
        };

        let is_update = existing_session.is_some();

        // Create base session model data
        let mut session_json = json!({
            "id": session.session_id.clone(),
            "sensitivity_level": 1000,
            "is_batch": false,
            "account_profile_id": account_profile_id,
            "device_name": device_info.as_ref().map(|d| d.device_name.clone()),
            "browser_name": device_info.as_ref().map(|d| d.browser_name.clone()),
            "operating_system": device_info.as_ref().map(|d| d.operating_system.clone()),
            "authentication_method": device_info.as_ref().map(|d| d.authentication_method.clone()),
        });

        // If updating, preserve fields that shouldn't change
        if is_update {
            if let Some(existing) = &existing_session {
                // Preserve fields that shouldn't change during update
                if let Some(created_date) = &existing.created_date {
                    session_json["created_date"] = json!(created_date);
                }
                if let Some(created_time) = &existing.created_time {
                    session_json["created_time"] = json!(created_time);
                }
                if let Some(expire) = &existing.expire {
                    session_json["expire"] = json!(expire.format("%Y-%m-%d %H:%M:%S%.f").to_string());
                }
                if let Some(code) = &existing.code {
                    session_json["code"] = json!(code);
                }
                if let Some(sensitivity_level) = existing.sensitivity_level {
                    session_json["sensitivity_level"] = json!(sensitivity_level);
                }
                if let Some(timestamp) = &existing.timestamp {
                    session_json["timestamp"] = json!(timestamp.format("%Y-%m-%d %H:%M:%S%.f").to_string());
                }
                if let Some(session_started) = &existing.session_started {
                    session_json["session_started"] = json!(session_started.format("%Y-%m-%d %H:%M:%S%.f").to_string());
                }
                if let Some(cookie_original_max_age) = existing.cookie_original_max_age {
                    session_json["cookie_original_max_age"] = json!(cookie_original_max_age);
                }
                if let Some(cookie_expires) = &existing.cookie_expires {
                    session_json["cookie_expires"] = json!(cookie_expires);
                }
            }
        } else {
            // For new sessions, set expire field
            session_json["expire"] = json!(expires.format("%Y-%m-%d %H:%M:%S%.f").to_string());
        }

        // Apply common fields using the add_common_fields pattern if auth is available
        if let Some(auth_data) = auth {
            let mut request_body = RequestBody {
                record: session_json.clone(),
            };

            // Use "update" for existing sessions, "create" for new ones
            let operation = if is_update { "update" } else { "create" };
            request_body.process_record(operation, auth_data, auth_data.is_root_account, "sessions");
            session_json = request_body.record;
        }

        // Extract values back to SessionModel
        let session_model = SessionModel {
            id: session_json["id"].as_str().map(|s| s.to_string()),
            tombstone: session_json["tombstone"].as_i64().map(|v| v as i32),
            status: session_json["status"].as_str().map(|s| s.to_string()),
            previous_status: session_json["previous_status"]
                .as_str()
                .map(|s| s.to_string()),
            version: session_json["version"].as_i64().map(|v| v as i32),
            created_date: session_json["created_date"].as_str().map(|s| s.to_string()),
            created_time: session_json["created_time"].as_str().map(|s| s.to_string()),
            updated_date: session_json["updated_date"].as_str().map(|s| s.to_string()),
            updated_time: session_json["updated_time"].as_str().map(|s| s.to_string()),
            organization_id: session_json["organization_id"]
                .as_str()
                .map(|s| s.to_string()),
            created_by: session_json["created_by"].as_str().map(|s| s.to_string()),
            updated_by: session_json["updated_by"].as_str().map(|s| s.to_string()),
            deleted_by: session_json["deleted_by"].as_str().map(|s| s.to_string()),
            requested_by: session_json["requested_by"].as_str().map(|s| s.to_string()),
            timestamp: Some(Utc::now().naive_utc()),
            tags: session_json["tags"].as_array().map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect()
            }),
            categories: session_json["categories"].as_array().map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect()
            }),
            code: session_json["code"].as_str().map(|s| s.to_string()),
            sensitivity_level: session_json["sensitivity_level"].as_i64().map(|v| v as i32),
            sync_status: session_json["sync_status"].as_str().map(|s| s.to_string()),
            is_batch: session_json["is_batch"].as_bool(),
            account_profile_id: session_json["account_profile_id"]
                .as_i64()
                .map(|v| v as i32),
            device_name: session_json["device_name"].as_str().map(|s| s.to_string()),
            browser_name: session_json["browser_name"].as_str().map(|s| s.to_string()),
            operating_system: session_json["operating_system"]
                .as_str()
                .map(|s| s.to_string()),
            authentication_method: session_json["authentication_method"]
                .as_str()
                .map(|s| s.to_string()),
            location: device_info.as_ref().map(|d| d.location.clone()),
            ip_address: device_info.as_ref().map(|d| d.ip_address.clone()),
            session_started: if is_update {
                // Preserve existing session_started for updates
                session_json["session_started"].as_str()
                    .and_then(|s| chrono::NaiveDateTime::parse_from_str(s, "%Y-%m-%d %H:%M:%S%.f").ok())
                    .or_else(|| existing_session.as_ref().and_then(|e| e.session_started))
            } else {
                Some(Utc::now().naive_utc())
            },
            remarks: device_info.as_ref().and_then(|d| d.remarks.clone()),

            user_role_id: Some(session.user.role_id.clone()),
            user_account_id: Some(session.user.account_id.clone()),
            user_is_root_user: Some(session.user.is_root_user),
            token: Some(session.token.clone()),
            cookie_path: Some(session.cookie.path.clone()),
            cookie_expires: if is_update {
                // Preserve existing cookie_expires for updates
                session_json["cookie_expires"].as_str().map(|s| s.to_string())
                    .or_else(|| existing_session.as_ref().and_then(|e| e.cookie_expires.clone()))
            } else {
                Some(session.cookie.expires.clone())
            },
            cookie_http_only: Some(session.cookie.httpOnly),
            cookie_original_max_age: if is_update {
                // Preserve existing cookie_original_max_age for updates
                session_json["cookie_original_max_age"].as_i64()
                    .or_else(|| existing_session.as_ref().and_then(|e| e.cookie_original_max_age))
            } else {
                Some(session.cookie.originalMaxAge)
            },
            origin_url: session.origin.as_ref().map(|o| o.url.clone()),
            origin_host: session.origin.as_ref().map(|o| o.host.clone()),
            origin_user_agent: session.origin.as_ref().and_then(|o| o.user_agent.clone()),
            valid_pass_key: session
                .valid_pass_keys
                .as_ref()
                .and_then(|v| serde_json::to_string(v).ok()),
            role_permission: session
                .role_permissions
                .as_ref()
                .and_then(|v| serde_json::to_string(v).ok()),
            field_permission: session
                .field_permissions
                .as_ref()
                .and_then(|v| serde_json::to_string(v).ok()),
            record_permission: session
                .record_permissions
                .as_ref()
                .and_then(|v| serde_json::to_string(v).ok()),
            expire: if is_update {
                // Use preserved expire value from session_json for updates
                session_json["expire"].as_str()
                    .and_then(|s| chrono::NaiveDateTime::parse_from_str(s, "%Y-%m-%d %H:%M:%S%.f").ok())
                    .or_else(|| existing_session.as_ref().and_then(|e| e.expire))
            } else {
                Some(expires)
            },
            application_accessed: app_id,
            last_accessed: Some(Utc::now().naive_utc()),
        };

        // Convert session model to JSON for sync service
        let session_json = serde_json::to_value(&session_model)
            .map_err(|e| {
                diesel::result::Error::DatabaseError(
                    diesel::result::DatabaseErrorKind::Unknown,
                    Box::new(format!("Failed to serialize session model: {}", e)),
                )
            })?;

        // Use sync service to insert/update session for synchronization
        crate::sync::sync_service::insert(&"sessions".to_string(), session_json)
            .await?;

        Ok(())
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

    // Find expired sessions to soft delete
    let expired_sessions: Vec<SessionModel> = sessions::table
        .filter(sessions::expire.lt(now).and(sessions::expire.is_not_null()))
        .filter(sessions::status.ne("Archived"))
        .filter(sessions::tombstone.eq(0))
        .load::<SessionModel>(&mut conn)
        .await?;

    let mut updated_count = 0;

    // Soft delete each expired session using sync service
    for session in expired_sessions {
        if let Some(_session_id) = &session.id {
            // Create updated session data for soft deletion
            let updated_session = serde_json::to_value(&session)
                .map_err(|e| {
                    diesel::result::Error::DatabaseError(
                        diesel::result::DatabaseErrorKind::Unknown,
                        Box::new(format!("Failed to serialize session: {}", e)),
                    )
                })?;

            // Create a RequestBody wrapper to use process_record
            let mut request_body = crate::structs::structs::RequestBody {
                record: updated_session,
            };

            // Create a dummy Auth struct for the soft deletion
            // Get environment variables
            let default_organization_id = env::var("DEFAULT_ORGANIZATION_ID")
                .unwrap_or_else(|_| "01JBHKXHYSKPP247HZZWHA3JCT".to_string());

            // Query for responsible account with category "Root"
            let mut conn = db::get_async_connection().await;
            let responsible_account_id = account_organizations::table
                .filter(account_organizations::categories.contains(vec!["Root".to_string()]))
                .filter(account_organizations::tombstone.eq(0))
                .select(account_organizations::id)
                .first::<Option<String>>(&mut conn)
                .await
                .unwrap_or_default()
                .unwrap_or_else(|| "system".to_string());

            let auth = crate::structs::structs::Auth {
                organization_id: default_organization_id,
                responsible_account: responsible_account_id,
                sensitivity_level: 0,
                role_name: "super_admin".to_string(),
                account_organization_id: "system".to_string(),
                role_id: "super_admin".to_string(),
                is_root_account: true,
                account_id: "system".to_string(),
            };

            // Use process_record for consistent soft deletion
            request_body.process_record("delete", &auth, true, "sessions");

            // Use sync service to update the session
            crate::sync::sync_service::insert(&"sessions".to_string(), request_body.record)
                .await?;

            updated_count += 1;
        }
    }

    Ok(updated_count)
}
