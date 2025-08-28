use log::{debug, info, warn};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant, SystemTime};
use tokio::sync::RwLock;
use tokio::time::interval;

/// Application lifecycle phases
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum LifecyclePhase {
    Initializing,
    Starting,
    Running,
    ShuttingDown,
    Stopped,
    Error(String),
}

/// Component status within the lifecycle
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ComponentStatus {
    NotStarted,
    Starting,
    Running,
    Stopping,
    Stopped,
    Failed(String),
}

/// Component information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentInfo {
    pub name: String,
    pub status: ComponentStatus,
    pub started_at: Option<SystemTime>,
    pub stopped_at: Option<SystemTime>,
    pub last_health_check: Option<SystemTime>,
    pub error_count: u64,
    pub restart_count: u64,
    pub metadata: HashMap<String, String>,
}

/// System health metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthMetrics {
    pub overall_status: ComponentStatus,
    pub uptime: Duration,
    pub memory_usage_mb: Option<u64>,
    pub cpu_usage_percent: Option<f64>,
    pub active_connections: u64,
    pub processed_requests: u64,
    pub error_rate: f64,
    pub last_updated: SystemTime,
}

/// Lifecycle state snapshot
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateSnapshot {
    pub phase: LifecyclePhase,
    pub components: HashMap<String, ComponentInfo>,
    pub health_metrics: HealthMetrics,
    pub created_at: SystemTime,
}

/// State change event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateChangeEvent {
    pub component: String,
    pub old_status: ComponentStatus,
    pub new_status: ComponentStatus,
    pub timestamp: SystemTime,
    pub message: Option<String>,
}

/// State manager for lifecycle tracking
pub struct StateManager {
    phase: Arc<RwLock<LifecyclePhase>>,
    components: Arc<RwLock<HashMap<String, ComponentInfo>>>,
    health_metrics: Arc<RwLock<HealthMetrics>>,
    start_time: Instant,
    event_history: Arc<RwLock<Vec<StateChangeEvent>>>,
    max_history_size: usize,
    monitoring_enabled: Arc<RwLock<bool>>,
    metrics_collection_interval: Duration,
}

impl StateManager {
    /// Create a new state manager
    pub fn new() -> Self {
        info!("[STATE] Initializing state manager");

        let health_metrics = HealthMetrics {
            overall_status: ComponentStatus::NotStarted,
            uptime: Duration::from_secs(0),
            memory_usage_mb: None,
            cpu_usage_percent: None,
            active_connections: 0,
            processed_requests: 0,
            error_rate: 0.0,
            last_updated: SystemTime::now(),
        };

        Self {
            phase: Arc::new(RwLock::new(LifecyclePhase::Initializing)),
            components: Arc::new(RwLock::new(HashMap::new())),
            health_metrics: Arc::new(RwLock::new(health_metrics)),
            start_time: Instant::now(),
            event_history: Arc::new(RwLock::new(Vec::new())),
            max_history_size: 1000,
            monitoring_enabled: Arc::new(RwLock::new(true)),
            metrics_collection_interval: Duration::from_secs(30),
        }
    }

    /// Set the current lifecycle phase
    pub async fn set_phase(&self, phase: LifecyclePhase) {
        let old_phase = {
            let mut current_phase = self.phase.write().await;
            let old = current_phase.clone();
            *current_phase = phase.clone();
            old
        };

        info!(
            "[STATE] Lifecycle phase changed: {:?} -> {:?}",
            old_phase, phase
        );

        // Update overall health status based on phase
        let overall_status = match &phase {
            LifecyclePhase::Initializing => ComponentStatus::Starting,
            LifecyclePhase::Starting => ComponentStatus::Starting,
            LifecyclePhase::Running => ComponentStatus::Running,
            LifecyclePhase::ShuttingDown => ComponentStatus::Stopping,
            LifecyclePhase::Stopped => ComponentStatus::Stopped,
            LifecyclePhase::Error(_) => ComponentStatus::Failed("Lifecycle error".to_string()),
        };

        self.update_health_metrics(|metrics| {
            metrics.overall_status = overall_status;
            metrics.uptime = self.start_time.elapsed();
            metrics.last_updated = SystemTime::now();
        })
        .await;
    }

    /// Get the current lifecycle phase
    pub async fn get_phase(&self) -> LifecyclePhase {
        self.phase.read().await.clone()
    }

    /// Register a new component
    pub async fn register_component(&self, name: String) {
        let component_info = ComponentInfo {
            name: name.clone(),
            status: ComponentStatus::NotStarted,
            started_at: None,
            stopped_at: None,
            last_health_check: None,
            error_count: 0,
            restart_count: 0,
            metadata: HashMap::new(),
        };

        self.components
            .write()
            .await
            .insert(name.clone(), component_info);
        info!("[STATE] Component registered: {}", name);
    }

    /// Update component status
    pub async fn update_component_status(&self, name: &str, status: ComponentStatus) {
        let event = {
            let mut components = self.components.write().await;

            if let Some(component) = components.get_mut(name) {
                let old_status = component.status.clone();
                component.status = status.clone();

                // Update timestamps
                match &status {
                    ComponentStatus::Starting | ComponentStatus::Running => {
                        if component.started_at.is_none() {
                            component.started_at = Some(SystemTime::now());
                        }
                    }
                    ComponentStatus::Stopped => {
                        component.stopped_at = Some(SystemTime::now());
                    }
                    ComponentStatus::Failed(_) => {
                        component.error_count += 1;
                        component.stopped_at = Some(SystemTime::now());
                    }
                    _ => {}
                }

                Some(StateChangeEvent {
                    component: name.to_string(),
                    old_status,
                    new_status: status,
                    timestamp: SystemTime::now(),
                    message: None,
                })
            } else {
                warn!("[STATE] Attempted to update unknown component: {}", name);
                None
            }
        };

        if let Some(event) = event {
            info!(
                "[STATE] Component status changed: {} -> {:?}",
                name, event.new_status
            );
            self.add_event(event).await;
        }
    }

    /// Update component metadata
    pub async fn update_component_metadata(&self, name: &str, key: String, value: String) {
        let mut components = self.components.write().await;

        if let Some(component) = components.get_mut(name) {
            component.metadata.insert(key.clone(), value.clone());
            debug!(
                "[STATE] Component metadata updated: {} -> {}={}",
                name, key, value
            );
        } else {
            warn!(
                "[STATE] Attempted to update metadata for unknown component: {}",
                name
            );
        }
    }

    /// Record component health check
    pub async fn record_health_check(&self, name: &str, success: bool) {
        let mut components = self.components.write().await;

        if let Some(component) = components.get_mut(name) {
            component.last_health_check = Some(SystemTime::now());

            if !success {
                component.error_count += 1;
                warn!("[STATE] Health check failed for component: {}", name);
            } else {
                debug!("[STATE] Health check passed for component: {}", name);
            }
        }
    }

    /// Get component information
    pub async fn get_component(&self, name: &str) -> Option<ComponentInfo> {
        self.components.read().await.get(name).cloned()
    }

    /// Get all components
    pub async fn get_all_components(&self) -> HashMap<String, ComponentInfo> {
        self.components.read().await.clone()
    }

    /// Update health metrics
    pub async fn update_health_metrics<F>(&self, updater: F)
    where
        F: FnOnce(&mut HealthMetrics),
    {
        let mut metrics = self.health_metrics.write().await;
        updater(&mut *metrics);
        metrics.last_updated = SystemTime::now();
    }

    /// Get current health metrics
    pub async fn get_health_metrics(&self) -> HealthMetrics {
        let mut metrics = self.health_metrics.read().await.clone();
        metrics.uptime = self.start_time.elapsed();
        metrics
    }

    /// Create a state snapshot
    pub async fn create_snapshot(&self) -> StateSnapshot {
        StateSnapshot {
            phase: self.get_phase().await,
            components: self.get_all_components().await,
            health_metrics: self.get_health_metrics().await,
            created_at: SystemTime::now(),
        }
    }

    /// Add state change event to history
    async fn add_event(&self, event: StateChangeEvent) {
        let mut history = self.event_history.write().await;

        history.push(event);

        // Trim history if it exceeds max size
        if history.len() > self.max_history_size {
            let excess_count = history.len() - self.max_history_size;
            history.drain(0..excess_count);
        }
    }

    /// Get recent state change events
    pub async fn get_recent_events(&self, limit: Option<usize>) -> Vec<StateChangeEvent> {
        let history = self.event_history.read().await;
        let limit = limit.unwrap_or(100).min(history.len());

        history.iter().rev().take(limit).cloned().collect()
    }

    /// Get system uptime
    pub fn get_uptime(&self) -> Duration {
        self.start_time.elapsed()
    }

    /// Check if system is healthy
    pub async fn is_healthy(&self) -> bool {
        let phase = self.get_phase().await;
        let components = self.get_all_components().await;

        // System is healthy if:
        // 1. Phase is Running
        // 2. All critical components are running
        // 3. No recent critical errors

        match phase {
            LifecyclePhase::Running => {
                // Check if any critical components have failed
                for (name, component) in components {
                    if let ComponentStatus::Failed(_) = component.status {
                        // Consider components with "server" or "database" in name as critical
                        if name.to_lowercase().contains("server")
                            || name.to_lowercase().contains("database")
                        {
                            return false;
                        }
                    }
                }
                true
            }
            LifecyclePhase::Error(_) => false,
            _ => false, // Not healthy during transitions
        }
    }

    /// Get component count by status
    pub async fn get_component_counts(&self) -> HashMap<ComponentStatus, usize> {
        let components = self.get_all_components().await;
        let mut counts = HashMap::new();

        for (_, component) in components {
            *counts.entry(component.status).or_insert(0) += 1;
        }

        counts
    }

    /// Generate health report
    pub async fn generate_health_report(&self) -> String {
        let phase = self.get_phase().await;
        let components = self.get_all_components().await;
        let health_metrics = self.get_health_metrics().await;
        let uptime = self.get_uptime();

        let mut report = String::new();
        report.push_str(&format!("=== System Health Report ===\n"));
        report.push_str(&format!("Phase: {:?}\n", phase));
        report.push_str(&format!("Uptime: {:?}\n", uptime));
        report.push_str(&format!(
            "Overall Status: {:?}\n",
            health_metrics.overall_status
        ));
        report.push_str(&format!(
            "Active Connections: {}\n",
            health_metrics.active_connections
        ));
        report.push_str(&format!(
            "Processed Requests: {}\n",
            health_metrics.processed_requests
        ));
        report.push_str(&format!(
            "Error Rate: {:.2}%\n",
            health_metrics.error_rate * 100.0
        ));

        if let Some(memory) = health_metrics.memory_usage_mb {
            report.push_str(&format!("Memory Usage: {} MB\n", memory));
        }

        if let Some(cpu) = health_metrics.cpu_usage_percent {
            report.push_str(&format!("CPU Usage: {:.1}%\n", cpu));
        }

        report.push_str("\n=== Components ===\n");
        for (name, info) in components {
            report.push_str(&format!("{}: {:?}\n", name, info.status));
        }

        report
    }

    /// Start background monitoring task
    pub async fn start_monitoring(&self) {
        let monitoring_enabled = self.monitoring_enabled.clone();
        let health_metrics = self.health_metrics.clone();
        let components = self.components.clone();
        let interval_duration = self.metrics_collection_interval;

        *monitoring_enabled.write().await = true;

        tokio::spawn(async move {
            let mut interval_timer = interval(interval_duration);

            loop {
                interval_timer.tick().await;

                // Check if monitoring is still enabled
                if !*monitoring_enabled.read().await {
                    debug!("[STATE] Monitoring disabled, stopping background task");
                    break;
                }

                // Collect system metrics
                Self::collect_system_metrics(&health_metrics).await;

                // Update component health checks
                Self::update_component_health_checks(&components).await;

                debug!("[STATE] Metrics collection completed");
            }
        });

        info!(
            "[STATE] Background monitoring started with interval: {:?}",
            interval_duration
        );
    }

    /// Stop background monitoring
    pub async fn stop_monitoring(&self) {
        *self.monitoring_enabled.write().await = false;
        info!("[STATE] Background monitoring stopped");
    }

    /// Collect system metrics (CPU, memory, etc.)
    async fn collect_system_metrics(health_metrics: &Arc<RwLock<HealthMetrics>>) {
        let mut metrics = health_metrics.write().await;

        // Update timestamp
        metrics.last_updated = SystemTime::now();

        // TODO: Implement actual system metrics collection
        // This would typically use system APIs or crates like `sysinfo`
        // For now, we'll simulate some basic metrics

        // Simulate memory usage (in a real implementation, use sysinfo crate)
        if let Ok(memory_info) = Self::get_memory_usage() {
            metrics.memory_usage_mb = Some(memory_info);
        }

        // Simulate CPU usage
        if let Ok(cpu_usage) = Self::get_cpu_usage() {
            metrics.cpu_usage_percent = Some(cpu_usage);
        }

        debug!("[STATE] System metrics updated");
    }

    /// Update health checks for all components
    async fn update_component_health_checks(
        components: &Arc<RwLock<HashMap<String, ComponentInfo>>>,
    ) {
        let mut components_guard = components.write().await;
        let now = SystemTime::now();

        for (name, component) in components_guard.iter_mut() {
            // Update last health check time for running components
            if component.status == ComponentStatus::Running {
                component.last_health_check = Some(now);
                debug!("[STATE] Updated health check for component: {}", name);
            }
        }
    }

    /// Get memory usage (placeholder implementation)
    fn get_memory_usage() -> Result<u64, Box<dyn std::error::Error + Send + Sync>> {
        // TODO: Implement actual memory usage collection
        // This is a placeholder that would be replaced with actual system calls
        Ok(128) // Simulated 128 MB usage
    }

    /// Get CPU usage (placeholder implementation)
    fn get_cpu_usage() -> Result<f64, Box<dyn std::error::Error + Send + Sync>> {
        // TODO: Implement actual CPU usage collection
        // This is a placeholder that would be replaced with actual system calls
        Ok(15.5) // Simulated 15.5% CPU usage
    }

    /// Get monitoring status
    pub async fn is_monitoring_enabled(&self) -> bool {
        *self.monitoring_enabled.read().await
    }

    /// Set metrics collection interval
    pub async fn set_metrics_interval(&mut self, interval: Duration) {
        self.metrics_collection_interval = interval;
        info!(
            "[STATE] Metrics collection interval updated to: {:?}",
            interval
        );
    }

    /// Get detailed component statistics
    pub async fn get_component_statistics(&self) -> HashMap<String, HashMap<String, u64>> {
        let components = self.get_all_components().await;
        let mut stats = HashMap::new();

        for (name, info) in components {
            let mut component_stats = HashMap::new();
            component_stats.insert("error_count".to_string(), info.error_count);
            component_stats.insert("restart_count".to_string(), info.restart_count);

            if let Some(started_at) = info.started_at {
                if let Ok(uptime) = SystemTime::now().duration_since(started_at) {
                    component_stats.insert("uptime_seconds".to_string(), uptime.as_secs());
                }
            }

            stats.insert(name, component_stats);
        }

        stats
    }
}

impl Default for StateManager {
    fn default() -> Self {
        Self::new()
    }
}
