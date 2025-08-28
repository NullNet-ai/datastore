use crate::controllers::health_controller::HealthController;
use actix_web::web;
use actix_web::web::ServiceConfig;

/// Configure health check routes
///
/// This module provides various health check endpoints:
/// - `/health` - Basic health status
/// - `/health/detailed` - Comprehensive health information
/// - `/health/ready` - Kubernetes readiness probe
/// - `/health/live` - Kubernetes liveness probe
pub fn configure_health_routes(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .route("/health", web::get().to(HealthController::health_check))
            .route(
                "/health/detailed",
                web::get().to(HealthController::detailed_health_check),
            )
            .route(
                "/health/ready",
                web::get().to(HealthController::readiness_probe),
            )
            .route(
                "/health/live",
                web::get().to(HealthController::liveness_probe),
            )
             .route(
                "/health/phase",
                web::get().to(HealthController::get_current_phase),
            ),
    );
}
