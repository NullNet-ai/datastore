use crate::controllers::health_controller::{
    health_check, detailed_health_check, readiness_probe, liveness_probe
};
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
        web::scope("")
            // Basic health check - no authentication required
            .service(health_check)
            // Detailed health information
            .service(detailed_health_check)
            // Kubernetes-style probes
            .service(readiness_probe)
            .service(liveness_probe)
    );
}