use super::health_service::HealthService;
use super::logging::{LifecycleLogger, LogCategory, LogConfig, LogLevel};
use super::runtime::RuntimeManager;
use super::shutdown::ShutdownManager;
use super::startup::StartupManager;
use super::state::{ComponentStatus, LifecyclePhase, StateManager};
use log::info;
use std::sync::Arc;

/// Main lifecycle manager that orchestrates all phases
pub struct LifecycleManager {
    state_manager: Arc<StateManager>,
    logger: Arc<LifecycleLogger>,
    startup_manager: StartupManager,
    runtime_manager: RuntimeManager,
    shutdown_manager: ShutdownManager,
    health_service: Arc<HealthService>,
}

impl LifecycleManager {
    /// Create a new lifecycle manager
    pub fn new() -> Self {
        let state_manager = Arc::new(StateManager::new());
        let logger = Arc::new(LifecycleLogger::default());
        let health_service = Arc::new(HealthService::new());
        Self {
            state_manager: state_manager.clone(),
            logger: logger.clone(),
            startup_manager: StartupManager::new(state_manager.clone(), logger.clone()),
            runtime_manager: RuntimeManager::new(),
            shutdown_manager: ShutdownManager::new().with_logger(logger.clone()),
            health_service,
        }
    }

    /// Create a lifecycle manager with custom configuration
    pub fn with_config(log_config: LogConfig) -> Self {
        let state_manager = Arc::new(StateManager::new());
        let logger = Arc::new(LifecycleLogger::new(log_config));
        let health_service = Arc::new(HealthService::new());
        Self {
            state_manager: state_manager.clone(),
            logger: logger.clone(),
            startup_manager: StartupManager::new(state_manager.clone(), logger.clone()),
            runtime_manager: RuntimeManager::new().with_logger(logger.clone()),
            shutdown_manager: ShutdownManager::new().with_logger(logger.clone()),
            health_service,
        }
    }

    /// Execute the complete application lifecycle
    pub async fn execute(&mut self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        info!("[LIFECYCLE] Starting application lifecycle");

        // Initialize logging system
        self.logger.initialize().await?;

        // Log lifecycle start
        self.logger
            .log(
                LogLevel::Info,
                LogCategory::Lifecycle,
                "LifecycleManager",
                "Application lifecycle starting",
            )
            .await;

        // Set initial phase
        self.state_manager
            .set_phase(LifecyclePhase::Initializing)
            .await;

        // Execute startup phase
        self.update_health_service().await;
        match self.execute_startup().await {
            Ok(_) => {
                self.logger
                    .log(
                        LogLevel::Info,
                        LogCategory::Startup,
                        "LifecycleManager",
                        "Startup phase completed successfully",
                    )
                    .await;
            }
            Err(e) => {
                let error_msg = format!("Startup phase failed: {}", e);
                self.logger
                    .log(
                        LogLevel::Error,
                        LogCategory::Startup,
                        "LifecycleManager",
                        &error_msg,
                    )
                    .await;
                self.state_manager
                    .set_phase(LifecyclePhase::Error(error_msg.clone()))
                    .await;
                self.update_health_service().await;
                return Err(e);
            }
        }

        // Execute runtime phase
        info!("[LIFECYCLE] Transitioning to runtime phase");
        self.state_manager.set_phase(LifecyclePhase::Running).await;
        self.update_health_service().await;
        info!("[LIFECYCLE] About to call execute_runtime");
        match self.execute_runtime().await {
            Ok(_) => {
                self.logger
                    .log(
                        LogLevel::Info,
                        LogCategory::Runtime,
                        "LifecycleManager",
                        "Runtime phase completed successfully",
                    )
                    .await;
            }
            Err(e) => {
                let error_msg = format!("Runtime phase failed: {}", e);
                self.logger
                    .log(
                        LogLevel::Error,
                        LogCategory::Runtime,
                        "LifecycleManager",
                        &error_msg,
                    )
                    .await;
                self.state_manager
                    .set_phase(LifecyclePhase::Error(error_msg.clone()))
                    .await;
                self.update_health_service().await;
                // Continue to shutdown even if runtime failed
            }
        }

        // Execute shutdown phase
        self.state_manager
            .set_phase(LifecyclePhase::ShuttingDown)
            .await;
        self.update_health_service().await;
        match self.execute_shutdown().await {
            Ok(_) => {
                self.logger
                    .log(
                        LogLevel::Info,
                        LogCategory::Shutdown,
                        "LifecycleManager",
                        "Shutdown phase completed successfully",
                    )
                    .await;
                self.state_manager.set_phase(LifecyclePhase::Stopped).await;
                self.update_health_service().await;
            }
            Err(e) => {
                let error_msg = format!("Shutdown phase failed: {}", e);
                self.logger
                    .log(
                        LogLevel::Error,
                        LogCategory::Shutdown,
                        "LifecycleManager",
                        &error_msg,
                    )
                    .await;
                self.state_manager
                    .set_phase(LifecyclePhase::Error(error_msg))
                    .await;
                self.update_health_service().await;
                return Err(e);
            }
        }

        self.logger
            .log(
                LogLevel::Info,
                LogCategory::Lifecycle,
                "LifecycleManager",
                "Application lifecycle completed successfully",
            )
            .await;

        info!("[LIFECYCLE] Application lifecycle completed");
        Ok(())
    }

    /// Execute startup phase
    async fn execute_startup(&mut self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // Start state monitoring
        self.state_manager.start_monitoring().await;

        // Register core components for monitoring
        self.state_manager
            .register_component("StartupManager".to_string())
            .await;
        self.state_manager
            .register_component("RuntimeManager".to_string())
            .await;
        self.state_manager
            .register_component("ShutdownManager".to_string())
            .await;
        self.state_manager
            .register_component("DatabasePool".to_string())
            .await;
        self.state_manager
            .register_component("S3Client".to_string())
            .await;

        // Execute startup
        self.startup_manager.execute().await
    }

    /// Execute runtime phase
    async fn execute_runtime(&mut self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        info!("[LIFECYCLE] execute_runtime method called");
        // Get the initialized services from startup manager
        let pool = self
            .startup_manager
            .pool
            .as_ref()
            .ok_or("Database pool not initialized")?;
        let s3_client = self
            .startup_manager
            .s3_client
            .as_ref()
            .ok_or("S3 client not initialized")?;
        let bucket_name = self
            .startup_manager
            .bucket_name
            .as_ref()
            .ok_or("Bucket name not initialized")?;

        // Update RuntimeManager status to starting
        self.state_manager
            .update_component_status("RuntimeManager", ComponentStatus::Starting)
            .await;

        // Configure the runtime manager with the logger and health service
        self.runtime_manager = RuntimeManager::new()
            .with_logger(self.logger.clone())
            .with_health_service(self.health_service.clone());

        // Update RuntimeManager status to running
        self.state_manager
            .update_component_status("RuntimeManager", ComponentStatus::Running)
            .await;

        // Execute runtime with required parameters
        // This will run until a shutdown signal is received
        match self.runtime_manager
            .execute(pool.clone(), s3_client.clone(), bucket_name.clone())
            .await {
            Ok(_) => {
                // Update RuntimeManager status to stopped when gracefully shut down
                self.state_manager
                    .update_component_status("RuntimeManager", ComponentStatus::Stopped)
                    .await;
            }
            Err(e) => {
                // Update RuntimeManager status to failed on error
                self.state_manager
                    .update_component_status("RuntimeManager", ComponentStatus::Failed(e.to_string()))
                    .await;
                return Err(e);
            }
        }

        // When runtime exits (due to shutdown signal), transition to shutdown phase
        self.logger
            .log(
                LogLevel::Info,
                LogCategory::Runtime,
                "LifecycleManager",
                "Runtime phase completed, initiating shutdown",
            )
            .await;

        Ok(())
    }

    /// Execute shutdown phase
    async fn execute_shutdown(&mut self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // Update component statuses before shutdown
        self.state_manager
            .update_component_status("StartupManager", ComponentStatus::Stopping)
            .await;
        self.state_manager
            .update_component_status("RuntimeManager", ComponentStatus::Stopping)
            .await;
        
        // Update ShutdownManager status to starting
        self.state_manager
            .update_component_status("ShutdownManager", ComponentStatus::Starting)
            .await;

        // Update ShutdownManager status to running
        self.state_manager
            .update_component_status("ShutdownManager", ComponentStatus::Running)
            .await;

        // Execute shutdown
        let result = match self.shutdown_manager.execute().await {
            Ok(_) => {
                // Update ShutdownManager status to stopped when completed
                self.state_manager
                    .update_component_status("ShutdownManager", ComponentStatus::Stopped)
                    .await;
                Ok(())
            }
            Err(e) => {
                // Update ShutdownManager status to failed on error
                self.state_manager
                    .update_component_status("ShutdownManager", ComponentStatus::Failed(e.to_string()))
                    .await;
                Err(e)
            }
        };

        // Stop monitoring and collect final metrics
        self.state_manager.stop_monitoring().await;

        // Update final component statuses
        self.state_manager
            .update_component_status("StartupManager", ComponentStatus::Stopped)
            .await;
        self.state_manager
            .update_component_status("RuntimeManager", ComponentStatus::Stopped)
            .await;

        result
    }

    /// Get state manager reference
    pub fn state_manager(&self) -> &Arc<StateManager> {
        &self.state_manager
    }

    /// Get logger reference
    pub fn logger(&self) -> &Arc<LifecycleLogger> {
        &self.logger
    }

    /// Get current lifecycle phase
    pub async fn get_phase(&self) -> LifecyclePhase {
        self.state_manager.get_phase().await
    }

    /// Check if system is healthy
    pub async fn is_healthy(&self) -> bool {
        let is_healthy = self.state_manager.is_healthy().await;
        self.health_service.update_health_status(is_healthy).await;
        is_healthy
    }

    /// Generate a comprehensive health report
    pub async fn generate_health_report(&self) -> String {
        self.state_manager.generate_health_report().await
    }

    /// Update health service with current state
    pub async fn update_health_service(&self) {
        let phase = self.state_manager.get_phase().await;
        let is_healthy = self.state_manager.is_healthy().await;
        let metrics = self.state_manager.get_health_metrics().await;
        let components = self.state_manager.get_all_components().await;
        
        self.health_service.update_phase(phase).await;
        self.health_service.update_health_status(is_healthy).await;
        
        let health_report = super::health_service::HealthReport {
            metrics,
            components,
        };
        
        self.health_service.update_health_report(health_report).await;
    }

    /// Request graceful shutdown
    pub async fn request_shutdown(&self) {
        self.runtime_manager.request_shutdown().await;

        self.logger
            .log(
                LogLevel::Info,
                LogCategory::Lifecycle,
                "LifecycleManager",
                "Graceful shutdown requested",
            )
            .await;
    }
}

impl Default for LifecycleManager {
    fn default() -> Self {
        Self::new()
    }
}
