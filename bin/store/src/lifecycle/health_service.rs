use crate::lifecycle::state::{ComponentInfo, HealthMetrics, LifecyclePhase};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Health report structure
#[derive(Debug, Clone)]
pub struct HealthReport {
    pub metrics: HealthMetrics,
    pub components: HashMap<String, ComponentInfo>,
}

/// Shared health service for communication between LifecycleManager and RuntimeManager
pub struct HealthService {
    phase: Arc<RwLock<LifecyclePhase>>,
    is_healthy: Arc<RwLock<bool>>,
    health_report: Arc<RwLock<Option<HealthReport>>>,
}

impl HealthService {
    pub fn new() -> Self {
        Self {
            phase: Arc::new(RwLock::new(LifecyclePhase::Initializing)),
            is_healthy: Arc::new(RwLock::new(false)),
            health_report: Arc::new(RwLock::new(None)),
        }
    }

    pub async fn update_phase(&self, phase: LifecyclePhase) {
        *self.phase.write().await = phase;
    }

    pub async fn update_health_status(&self, is_healthy: bool) {
        *self.is_healthy.write().await = is_healthy;
    }

    pub async fn update_health_report(&self, report: HealthReport) {
        *self.health_report.write().await = Some(report);
    }

    pub async fn get_phase(&self) -> LifecyclePhase {
        self.phase.read().await.clone()
    }

    pub async fn is_healthy(&self) -> bool {
        *self.is_healthy.read().await
    }

    pub async fn generate_health_report(&self) -> HealthReport {
        self.health_report.read().await.clone().unwrap_or_else(|| {
            // Return default health report if none exists
            HealthReport {
                metrics: HealthMetrics {
                    overall_status: crate::lifecycle::state::ComponentStatus::NotStarted,
                    uptime: std::time::Duration::from_secs(0),
                    memory_usage_mb: None,
                    cpu_usage_percent: None,
                    active_connections: 0,
                    processed_requests: 0,
                    error_rate: 0.0,
                    last_updated: std::time::SystemTime::now(),
                },
                components: HashMap::new(),
            }
        })
    }
}

impl Default for HealthService {
    fn default() -> Self {
        Self::new()
    }
}
