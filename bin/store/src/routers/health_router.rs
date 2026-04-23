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
/// - `/health/phase` - Current deployment phase
/// - `/health/components/{name}/metadata` - Update component metadata
/// - `/health/components/{name}/health-check` - Record health check
/// - `/health/components/{name}` - Get component information
/// - `/health/snapshot` - Create state snapshot
/// - `/health/events` - Get recent events
/// - `/health/components/counts` - Get component counts
/// - `/health/monitoring/status` - Get monitoring status
/// - `/health/monitoring/interval` - Set metrics interval
/// - `/health/components/statistics` - Get component statistics
pub fn configure_health_routes(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .route("/health", web::get().to(HealthController::health_check))
            .route("/metrics", web::get().to(HealthController::metrics))
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
            )
            // Monitoring endpoints - specific routes must come before parameterized routes
            .route(
                "/health/components/counts",
                web::get().to(HealthController::get_component_counts),
            )
            .route(
                "/health/components/statistics",
                web::get().to(HealthController::get_component_statistics),
            )
            .route(
                "/health/components/{component_name}/metadata",
                web::put().to(HealthController::update_component_metadata),
            )
            .route(
                "/health/components/{component_name}/health-check",
                web::post().to(HealthController::record_health_check),
            )
            .route(
                "/health/components/{component_name}",
                web::get().to(HealthController::get_component),
            )
            .route(
                "/health/snapshot",
                web::get().to(HealthController::create_snapshot),
            )
            .route(
                "/health/events",
                web::get().to(HealthController::get_recent_events),
            )
            .route(
                "/health/monitoring/status",
                web::get().to(HealthController::get_monitoring_status),
            )
            .route(
                "/health/monitoring/interval",
                web::put().to(HealthController::set_metrics_interval),
            ),
    );
}
