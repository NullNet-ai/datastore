use crate::lifecycle::health_service::HealthService;
use crate::lifecycle::runtime::check_cache_health;
use crate::lifecycle::runtime::RuntimeManager;
use crate::lifecycle::state::{
    ComponentStatus, HealthMetrics, LifecyclePhase, StateChangeEvent, StateManager,
};
use actix_web::{web, HttpResponse, Responder};
use chrono;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
/// Health check response structure
#[derive(Serialize, Debug)]
pub struct HealthResponse {
    pub status: String,
    pub timestamp: String,
    pub uptime_seconds: u64,
    pub components: HashMap<String, ComponentHealthStatus>,
    pub metrics: SystemMetrics,
}

/// Component health status for API response
#[derive(Serialize, Debug)]
pub struct ComponentHealthStatus {
    pub status: String,
    pub last_check: Option<String>,
    pub uptime_seconds: Option<u64>,
}

/// System metrics for health response
#[derive(Serialize, Debug)]
pub struct SystemMetrics {
    pub memory_usage_mb: f64,
    pub cpu_usage_percent: f64,
    pub active_connections: u64,
    pub processed_requests: u64,
    pub shutdown_stage: Option<String>,
    pub shutdown_elapsed_seconds: Option<f64>,
}

/// Detailed health check response
#[derive(Serialize, Debug)]
pub struct DetailedHealthResponse {
    pub status: String,
    pub timestamp: String,
    pub uptime_seconds: u64,
    pub components: HashMap<String, ComponentHealthStatus>,
    pub metrics: SystemMetrics,
    pub checks: HashMap<String, CheckResult>,
}

/// Individual check result
#[derive(Serialize, Debug)]
pub struct CheckResult {
    pub status: String,
    pub message: String,
    pub duration_ms: u64,
}

/// Readiness probe response
#[derive(Serialize, Debug)]
pub struct ReadinessResponse {
    pub ready: bool,
    pub components: HashMap<String, bool>,
}

/// Liveness probe response
#[derive(Serialize, Debug)]
pub struct LivenessResponse {
    pub alive: bool,
    pub uptime_seconds: u64,
}

/// Component metadata update request
#[derive(Deserialize, Serialize, Debug)]
pub struct ComponentMetadataRequest {
    pub key: String,
    pub value: String,
}

/// Health check record request
#[derive(Deserialize, Serialize, Debug)]
pub struct HealthCheckRequest {
    pub success: bool,
}

/// Monitoring configuration request
#[derive(Deserialize, Serialize, Debug)]
pub struct MonitoringConfigRequest {
    pub interval_seconds: u64,
}

/// Component statistics response
#[derive(Serialize, Debug)]
pub struct ComponentStatisticsResponse {
    pub statistics: HashMap<String, HashMap<String, u64>>,
    pub timestamp: String,
}

/// Recent events response
#[derive(Serialize, Debug)]
pub struct RecentEventsResponse {
    pub events: Vec<StateChangeEvent>,
    pub total_count: usize,
    pub timestamp: String,
}

/// Component counts response
#[derive(Serialize, Debug)]
pub struct ComponentCountsResponse {
    pub counts: HashMap<ComponentStatus, usize>,
    pub timestamp: String,
}

/// Monitoring status response
#[derive(Serialize, Debug)]
pub struct MonitoringStatusResponse {
    pub enabled: bool,
    pub timestamp: String,
}

pub struct HealthController;

impl HealthController {
    /// Basic health check endpoint
    /// Returns 200 OK if the service is healthy, 503 Service Unavailable otherwise
    pub async fn health_check(health_service: web::Data<Arc<HealthService>>) -> impl Responder {
        let is_healthy = health_service.is_healthy().await;
        let health_report = health_service.generate_health_report().await;

        // Get system metrics from health report
        let health_metrics = &health_report.metrics;
        let components = &health_report.components;

        // Convert components to API format
        let mut component_statuses = HashMap::new();
        for (name, info) in components {
            let status_str = match info.status {
                ComponentStatus::NotStarted => "not_started",
                ComponentStatus::Starting => "starting",
                ComponentStatus::Running => "running",
                ComponentStatus::Stopping => "stopping",
                ComponentStatus::Stopped => "stopped",
                ComponentStatus::Failed(_) => "failed",
            };

            component_statuses.insert(
                name.clone(),
                ComponentHealthStatus {
                    status: status_str.to_string(),
                    last_check: info.last_health_check.map(|t| {
                        chrono::DateTime::<chrono::Utc>::from(t)
                            .format("%Y-%m-%dT%H:%M:%S%.3fZ")
                            .to_string()
                    }),
                    uptime_seconds: info
                        .started_at
                        .map(|t| t.elapsed().unwrap_or_default().as_secs()),
                },
            );
        }

        let response = HealthResponse {
            status: if is_healthy {
                "healthy".to_string()
            } else {
                "unhealthy".to_string()
            },
            timestamp: chrono::Utc::now()
                .format("%Y-%m-%dT%H:%M:%S%.3fZ")
                .to_string(),
            uptime_seconds: health_metrics.uptime.as_secs(),
            components: component_statuses,
            metrics: SystemMetrics {
                memory_usage_mb: health_metrics.memory_usage_mb.unwrap_or(0) as f64,
                cpu_usage_percent: health_metrics.cpu_usage_percent.unwrap_or(0.0),
                active_connections: health_metrics.active_connections,
                processed_requests: health_metrics.processed_requests,
                shutdown_stage: health_metrics
                    .shutdown_stage
                    .as_ref()
                    .map(|stage| format!("{:?}", stage)),
                shutdown_elapsed_seconds: health_metrics
                    .shutdown_elapsed_time
                    .map(|duration| duration.as_secs_f64()),
            },
        };

        if is_healthy {
            HttpResponse::Ok().json(response)
        } else {
            HttpResponse::ServiceUnavailable().json(response)
        }
    }

    /// Detailed health check endpoint with comprehensive system information
    pub async fn detailed_health_check(
        health_service: web::Data<Arc<HealthService>>,
    ) -> impl Responder {
        let is_healthy = health_service.is_healthy().await;
        let health_report = health_service.generate_health_report().await;

        // Get system metrics from health report
        let health_metrics = &health_report.metrics;
        let components = &health_report.components;

        // Convert components to API format
        let mut component_statuses = HashMap::new();
        for (name, info) in components {
            let status_str = match info.status {
                ComponentStatus::NotStarted => "not_started",
                ComponentStatus::Starting => "starting",
                ComponentStatus::Running => "running",
                ComponentStatus::Stopping => "stopping",
                ComponentStatus::Stopped => "stopped",
                ComponentStatus::Failed(_) => "failed",
            };

            component_statuses.insert(
                name.clone(),
                ComponentHealthStatus {
                    status: status_str.to_string(),
                    last_check: info.last_health_check.map(|t| {
                        chrono::DateTime::<chrono::Utc>::from(t)
                            .format("%Y-%m-%dT%H:%M:%S%.3fZ")
                            .to_string()
                    }),
                    uptime_seconds: info
                        .started_at
                        .map(|t| t.elapsed().unwrap_or_default().as_secs()),
                },
            );
        }

        // Perform detailed checks
        let mut checks = HashMap::new();

        // Database connectivity check
        let db_check_start = std::time::Instant::now();
        let db_check = perform_database_check().await;
        checks.insert(
            "database".to_string(),
            CheckResult {
                status: if db_check.is_ok() {
                    "pass".to_string()
                } else {
                    "fail".to_string()
                },
                message: db_check.unwrap_or_else(|e| e),
                duration_ms: db_check_start.elapsed().as_millis() as u64,
            },
        );

        // Memory check
        let memory_check_start = std::time::Instant::now();
        let memory_check = perform_memory_check(&health_metrics).await;
        checks.insert(
            "memory".to_string(),
            CheckResult {
                status: if memory_check.is_ok() {
                    "pass".to_string()
                } else {
                    "warn".to_string()
                },
                message: memory_check.unwrap_or_else(|e| e),
                duration_ms: memory_check_start.elapsed().as_millis() as u64,
            },
        );

        // Cache connectivity check
        let cache_check_start = std::time::Instant::now();
        let cache_check = perform_cache_check().await;
        checks.insert(
            "cache".to_string(),
            CheckResult {
                status: if cache_check.is_ok() {
                    "pass".to_string()
                } else {
                    "fail".to_string()
                },
                message: cache_check.unwrap_or_else(|e| e),
                duration_ms: cache_check_start.elapsed().as_millis() as u64,
            },
        );

        let response = DetailedHealthResponse {
            status: if is_healthy {
                "healthy".to_string()
            } else {
                "unhealthy".to_string()
            },
            timestamp: chrono::Utc::now()
                .format("%Y-%m-%dT%H:%M:%S%.3fZ")
                .to_string(),
            uptime_seconds: health_metrics.uptime.as_secs(),
            components: component_statuses,
            metrics: SystemMetrics {
                memory_usage_mb: health_metrics.memory_usage_mb.unwrap_or(0) as f64,
                cpu_usage_percent: health_metrics.cpu_usage_percent.unwrap_or(0.0),
                active_connections: health_metrics.active_connections,
                processed_requests: health_metrics.processed_requests,
                shutdown_stage: health_metrics
                    .shutdown_stage
                    .as_ref()
                    .map(|stage| format!("{:?}", stage)),
                shutdown_elapsed_seconds: health_metrics
                    .shutdown_elapsed_time
                    .map(|duration| duration.as_secs_f64()),
            },
            checks,
        };

        if is_healthy {
            HttpResponse::Ok().json(response)
        } else {
            HttpResponse::ServiceUnavailable().json(response)
        }
    }

    /// Kubernetes-style readiness probe
    /// Returns 200 if the service is ready to accept traffic
    pub async fn readiness_probe(health_service: web::Data<Arc<HealthService>>) -> impl Responder {
        let health_report = health_service.generate_health_report().await;
        let components = &health_report.components;
        let mut component_ready = HashMap::new();
        let mut all_ready = true;

        for (name, info) in components {
            let is_ready = matches!(info.status, ComponentStatus::Running);
            component_ready.insert(name.clone(), is_ready);
            if !is_ready {
                all_ready = false;
            }
        }

        let response = ReadinessResponse {
            ready: all_ready,
            components: component_ready,
        };

        if all_ready {
            HttpResponse::Ok().json(response)
        } else {
            HttpResponse::ServiceUnavailable().json(response)
        }
    }

    /// Kubernetes-style liveness probe
    /// Returns 200 if the service is alive (basic functionality)
    pub async fn liveness_probe(health_service: web::Data<Arc<HealthService>>) -> impl Responder {
        let health_report = health_service.generate_health_report().await;
        let health_metrics = &health_report.metrics;

        let response = LivenessResponse {
            alive: true, // If we can respond, we're alive
            uptime_seconds: health_metrics.uptime.as_secs(),
        };

        HttpResponse::Ok().json(response)
    }

    pub async fn get_current_phase(
        health_service: web::Data<Arc<HealthService>>,
    ) -> impl Responder {
        let phase = health_service.get_phase().await;
        match phase {
            LifecyclePhase::Initializing => HttpResponse::Ok()
                .json(serde_json::json!({"phase": "Initializing", "status": "starting"})),
            LifecyclePhase::Starting => HttpResponse::Ok()
                .json(serde_json::json!({"phase": "Starting", "status": "starting"})),
            LifecyclePhase::Running => HttpResponse::Ok()
                .json(serde_json::json!({"phase": "Running", "status": "healthy"})),
            LifecyclePhase::ShuttingDown => HttpResponse::Ok()
                .json(serde_json::json!({"phase": "ShuttingDown", "status": "stopping"})),
            LifecyclePhase::Stopped => HttpResponse::Ok()
                .json(serde_json::json!({"phase": "Stopped", "status": "stopped"})),
            LifecyclePhase::Error(error_msg) => HttpResponse::InternalServerError().json(
                serde_json::json!({"phase": "Error", "status": "error", "message": error_msg}),
            ),
        }
    }

    /// Update component metadata endpoint
    /// PUT /health/components/{component_name}/metadata
    pub async fn update_component_metadata(
        state_manager: web::Data<Arc<StateManager>>,
        path: web::Path<String>,
        req: web::Json<ComponentMetadataRequest>,
    ) -> impl Responder {
        let component_name = path.into_inner();

        state_manager
            .update_component_metadata(&component_name, req.key.clone(), req.value.clone())
            .await;

        HttpResponse::Ok().json(serde_json::json!({
            "status": "success",
            "message": format!("Metadata updated for component: {}", component_name),
            "component": component_name,
            "key": req.key,
            "value": req.value,
            "timestamp": chrono::Utc::now().format("%Y-%m-%dT%H:%M:%S%.3fZ").to_string()
        }))
    }

    /// Record health check endpoint
    /// POST /health/components/{component_name}/health-check
    pub async fn record_health_check(
        state_manager: web::Data<Arc<StateManager>>,
        path: web::Path<String>,
        req: web::Json<HealthCheckRequest>,
    ) -> impl Responder {
        let component_name = path.into_inner();

        state_manager
            .record_health_check(&component_name, req.success)
            .await;

        HttpResponse::Ok().json(serde_json::json!({
            "status": "success",
            "message": format!("Health check recorded for component: {}", component_name),
            "component": component_name,
            "success": req.success,
            "timestamp": chrono::Utc::now().format("%Y-%m-%dT%H:%M:%S%.3fZ").to_string()
        }))
    }

    /// Get component information endpoint
    /// GET /health/components/{component_name}
    pub async fn get_component(
        state_manager: web::Data<Arc<StateManager>>,
        path: web::Path<String>,
    ) -> impl Responder {
        let component_name = path.into_inner();

        match state_manager.get_component(&component_name).await {
            Some(component) => HttpResponse::Ok().json(component),
            None => HttpResponse::NotFound().json(serde_json::json!({
                "error": "Component not found",
                "component": component_name,
                "timestamp": chrono::Utc::now().format("%Y-%m-%dT%H:%M:%S%.3fZ").to_string()
            })),
        }
    }

    /// Create state snapshot endpoint
    /// GET /health/snapshot
    pub async fn create_snapshot(state_manager: web::Data<Arc<StateManager>>) -> impl Responder {
        let snapshot = state_manager.create_snapshot().await;
        HttpResponse::Ok().json(snapshot)
    }

    /// Get recent events endpoint
    /// GET /health/events?limit=100
    pub async fn get_recent_events(
        state_manager: web::Data<Arc<StateManager>>,
        query: web::Query<HashMap<String, String>>,
    ) -> impl Responder {
        let limit = query
            .get("limit")
            .and_then(|s| s.parse::<usize>().ok())
            .unwrap_or(100);

        let events = state_manager.get_recent_events(Some(limit)).await;
        let total_count = events.len();

        let response = RecentEventsResponse {
            events,
            total_count,
            timestamp: chrono::Utc::now()
                .format("%Y-%m-%dT%H:%M:%S%.3fZ")
                .to_string(),
        };

        HttpResponse::Ok().json(response)
    }

    /// Get component counts endpoint
    /// GET /health/components/counts
    pub async fn get_component_counts(
        state_manager: web::Data<Arc<StateManager>>,
    ) -> impl Responder {
        let counts = state_manager.get_component_counts().await;

        let response = ComponentCountsResponse {
            counts,
            timestamp: chrono::Utc::now()
                .format("%Y-%m-%dT%H:%M:%S%.3fZ")
                .to_string(),
        };

        HttpResponse::Ok().json(response)
    }

    /// Get monitoring status endpoint
    /// GET /health/monitoring/status
    pub async fn get_monitoring_status(
        state_manager: web::Data<Arc<StateManager>>,
    ) -> impl Responder {
        let enabled = state_manager.is_monitoring_enabled().await;

        let response = MonitoringStatusResponse {
            enabled,
            timestamp: chrono::Utc::now()
                .format("%Y-%m-%dT%H:%M:%S%.3fZ")
                .to_string(),
        };

        HttpResponse::Ok().json(response)
    }

    /// Set metrics collection interval endpoint
    /// PUT /health/monitoring/interval
    pub async fn set_metrics_interval(
        state_manager: web::Data<Arc<StateManager>>,
        req: web::Json<MonitoringConfigRequest>,
    ) -> impl Responder {
        let interval = Duration::from_secs(req.interval_seconds);
        state_manager.set_metrics_interval(interval).await;
        
        HttpResponse::Ok().json(serde_json::json!({
            "status": "success",
            "message": "Metrics collection interval updated successfully",
            "interval_seconds": req.interval_seconds,
            "timestamp": chrono::Utc::now().format("%Y-%m-%dT%H:%M:%S%.3fZ").to_string()
        }))
    }

    /// Get component statistics endpoint
    /// GET /health/components/statistics
    pub async fn get_component_statistics(
        state_manager: web::Data<Arc<StateManager>>,
    ) -> impl Responder {
        let statistics = state_manager.get_component_statistics().await;

        let response = ComponentStatisticsResponse {
            statistics,
            timestamp: chrono::Utc::now()
                .format("%Y-%m-%dT%H:%M:%S%.3fZ")
                .to_string(),
        };

        HttpResponse::Ok().json(response)
    }
}

/// Perform database connectivity check
async fn perform_database_check() -> Result<String, String> {
    // Reuse the existing database health check from runtime module
    match RuntimeManager::check_database_health().await {
        Ok(()) => Ok("Database connection successful".to_string()),
        Err(e) => Err(e),
    }
}

/// Perform memory usage check
async fn perform_memory_check(metrics: &HealthMetrics) -> Result<String, String> {
    const MEMORY_WARNING_THRESHOLD: u64 = 80; // 80 MB memory usage
    const MEMORY_CRITICAL_THRESHOLD: u64 = 95; // 95 MB memory usage

    let memory_usage = metrics.memory_usage_mb.unwrap_or(0);

    if memory_usage > MEMORY_CRITICAL_THRESHOLD {
        Err(format!("Critical memory usage: {} MB", memory_usage))
    } else if memory_usage > MEMORY_WARNING_THRESHOLD {
        Err(format!("High memory usage: {} MB", memory_usage))
    } else {
        Ok(format!("Memory usage normal: {} MB", memory_usage))
    }
}

/// Performs cache health check by delegating to the runtime module
async fn perform_cache_check() -> Result<String, String> {
    match check_cache_health().await {
        Ok(message) => Ok(message),
        Err(e) => Err(format!("Cache health check failed: {}", e)),
    }
}
