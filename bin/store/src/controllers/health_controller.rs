use crate::lifecycle::manager::LifecycleManager;
use crate::lifecycle::state::{ComponentStatus, HealthMetrics};
use actix_web::{get, web, HttpResponse, Responder};
use chrono;
use serde::Serialize;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

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

/// Basic health check endpoint
/// Returns 200 OK if the service is healthy, 503 Service Unavailable otherwise
#[get("/health")]
pub async fn health_check(
    lifecycle_manager: web::Data<Arc<RwLock<LifecycleManager>>>,
) -> impl Responder {
    let manager = lifecycle_manager.read().await;

    let is_healthy = manager.is_healthy().await;
    let state_manager = manager.state_manager();

    // Get system metrics
    let health_metrics = state_manager.get_health_metrics().await;
    let components = state_manager.get_all_components().await;

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
            name,
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
        },
    };

    if is_healthy {
        HttpResponse::Ok().json(response)
    } else {
        HttpResponse::ServiceUnavailable().json(response)
    }
}

/// Detailed health check endpoint with comprehensive system information
#[get("/health/detailed")]
pub async fn detailed_health_check(
    lifecycle_manager: web::Data<Arc<RwLock<LifecycleManager>>>,
) -> impl Responder {
    let manager = lifecycle_manager.read().await;

    let is_healthy = manager.is_healthy().await;
    let state_manager = manager.state_manager();

    // Get system metrics
    let health_metrics = state_manager.get_health_metrics().await;
    let components = state_manager.get_all_components().await;

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
            name,
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
#[get("/health/ready")]
pub async fn readiness_probe(
    lifecycle_manager: web::Data<Arc<RwLock<LifecycleManager>>>,
) -> impl Responder {
    let manager = lifecycle_manager.read().await;
    let state_manager = manager.state_manager();

    let components = state_manager.get_all_components().await;
    let mut component_ready = HashMap::new();
    let mut all_ready = true;

    for (name, info) in components {
        let is_ready = matches!(info.status, ComponentStatus::Running);
        component_ready.insert(name, is_ready);
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
#[get("/health/live")]
pub async fn liveness_probe(
    lifecycle_manager: web::Data<Arc<RwLock<LifecycleManager>>>,
) -> impl Responder {
    let manager = lifecycle_manager.read().await;
    let state_manager = manager.state_manager();

    let health_metrics = state_manager.get_health_metrics().await;

    let response = LivenessResponse {
        alive: true, // If we can respond, we're alive
        uptime_seconds: health_metrics.uptime.as_secs(),
    };

    HttpResponse::Ok().json(response)
}

/// Perform database connectivity check
async fn perform_database_check() -> Result<String, String> {
    // TODO: Implement actual database connectivity check
    // This would typically involve a simple SELECT 1 query
    match tokio::time::timeout(std::time::Duration::from_secs(5), async {
        // Simulate database check
        tokio::time::sleep(std::time::Duration::from_millis(10)).await;
        Result::<(), &str>::Ok(())
    })
    .await
    {
        Ok(Ok(())) => Ok("Database connection successful".to_string()),
        Ok(Err(_)) => Err("Database connection failed".to_string()),
        Err(_) => Err("Database connection timeout".to_string()),
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
