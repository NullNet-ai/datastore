use tonic::service::Interceptor;
use tonic::{Request, Status};

use crate::auth::structs::Session;
use super::session_core::{SessionConfig, SessionManager};

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
    pub fn with_config(config: SessionConfig) -> Self {
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
        
        log::debug!("Session header '{}' value: {:?}", self.session_manager.session_header(), session_header_value);
        
        // For gRPC, we don't have cookies, so we only check headers
        let session_id = self.session_manager.extract_session_id(session_header_value, None);
        
        log::debug!("Extracted/Generated session ID: {}", session_id);
        
        // Store session ID in request extensions for later async loading
        // This avoids the runtime panic from calling block_on in an async context
        request.extensions_mut().insert(session_id);
        
        Ok(request)
    }
}

/// Helper function to extract session from gRPC request
#[allow(dead_code)]
pub fn get_session_from_request<T>(request: &Request<T>) -> Option<Session> {
    request.extensions().get::<Session>().cloned()
}

/// Helper function to update session in gRPC request
#[allow(dead_code)]
pub fn update_session_in_request<T>(request: &mut Request<T>, session: Session) {
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