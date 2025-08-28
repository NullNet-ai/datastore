use crate::lifecycle::logging::{LogCategory, LogLevel};
use actix_web::dev::ServerHandle;
use log::{error, info, warn};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::mpsc;
use tokio::sync::RwLock;

/// Shutdown phase stages
#[derive(Debug, Clone, PartialEq)]
pub enum ShutdownStage {
    NotStarted,
    StoppingHttpServer,
    DrainConnections,
    StoppingBackgroundServices,
    CleanupResources,
    Completed,
    Failed(String),
}

/// Shutdown configuration
#[derive(Debug, Clone)]
pub struct ShutdownConfig {
    pub graceful_timeout: Duration,
    pub force_timeout: Duration,
    pub drain_timeout: Duration,
}

impl Default for ShutdownConfig {
    fn default() -> Self {
        Self {
            graceful_timeout: Duration::from_secs(30),
            force_timeout: Duration::from_secs(60),
            drain_timeout: Duration::from_secs(10),
        }
    }
}

/// Shutdown manager responsible for graceful application termination
pub struct ShutdownManager {
    config: ShutdownConfig,
    stage: Arc<RwLock<ShutdownStage>>,
    start_time: Option<Instant>,
    services: Vec<Box<dyn ShutdownService + Send + Sync>>,
    logger: Option<Arc<crate::lifecycle::logging::LifecycleLogger>>,
}

/// Trait for services that need graceful shutdown
#[async_trait::async_trait]
pub trait ShutdownService {
    /// Service name for logging
    fn name(&self) -> &str;

    /// Gracefully shutdown the service
    async fn shutdown(&mut self) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;

    /// Force shutdown if graceful shutdown fails
    async fn force_shutdown(&mut self) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;
}

/// HTTP Server shutdown wrapper
pub struct HttpServerShutdown {
    server_handle: Option<ServerHandle>,
}

impl HttpServerShutdown {
    pub fn new(server_handle: ServerHandle) -> Self {
        Self {
            server_handle: Some(server_handle),
        }
    }
}

#[async_trait::async_trait]
impl ShutdownService for HttpServerShutdown {
    fn name(&self) -> &str {
        "HTTP Server"
    }

    async fn shutdown(&mut self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        if let Some(server_handle) = self.server_handle.take() {
            info!("[SHUTDOWN] Stopping HTTP server gracefully");
            server_handle.stop(true).await;
            info!("[SHUTDOWN] HTTP server stopped");
        }
        Ok(())
    }

    async fn force_shutdown(&mut self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        if let Some(server_handle) = self.server_handle.take() {
            warn!("[SHUTDOWN] Force stopping HTTP server");
            server_handle.stop(false).await;
            warn!("[SHUTDOWN] HTTP server force stopped");
        }
        Ok(())
    }
}

/// Background service shutdown wrapper
pub struct BackgroundServiceShutdown {
    name: String,
    shutdown_tx: Option<mpsc::Sender<()>>,
}

impl BackgroundServiceShutdown {
    pub fn new(name: String, shutdown_tx: mpsc::Sender<()>) -> Self {
        Self {
            name,
            shutdown_tx: Some(shutdown_tx),
        }
    }
}

#[async_trait::async_trait]
impl ShutdownService for BackgroundServiceShutdown {
    fn name(&self) -> &str {
        &self.name
    }

    async fn shutdown(&mut self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        if let Some(tx) = self.shutdown_tx.take() {
            info!("[SHUTDOWN] Stopping background service: {}", self.name);
            if let Err(e) = tx.send(()).await {
                warn!(
                    "[SHUTDOWN] Failed to send shutdown signal to {}: {}",
                    self.name, e
                );
            } else {
                info!("[SHUTDOWN] Shutdown signal sent to {}", self.name);
            }
        }
        Ok(())
    }

    async fn force_shutdown(&mut self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // For background services, force shutdown is the same as graceful
        self.shutdown().await
    }
}

impl ShutdownManager {
    /// Create a new shutdown manager
    pub fn new() -> Self {
        Self::with_config(ShutdownConfig::default())
    }

    /// Set the logger for structured logging
    pub fn with_logger(mut self, logger: Arc<crate::lifecycle::logging::LifecycleLogger>) -> Self {
        self.logger = Some(logger);
        self
    }

    /// Create a new shutdown manager with custom configuration
    pub fn with_config(config: ShutdownConfig) -> Self {
        info!(
            "[SHUTDOWN] Initializing shutdown manager with config: {:?}",
            config
        );

        Self {
            config,
            stage: Arc::new(RwLock::new(ShutdownStage::NotStarted)),
            start_time: None,
            services: Vec::new(),
            logger: None,
        }
    }

    /// Register a service for shutdown management
    pub fn register_service(&mut self, service: Box<dyn ShutdownService + Send + Sync>) {
        info!(
            "[SHUTDOWN] Registering service for shutdown: {}",
            service.name()
        );
        self.services.push(service);
    }

    /// Execute graceful shutdown
    pub async fn execute(&mut self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        self.start_time = Some(Instant::now());

        if let Some(logger) = &self.logger {
            logger
                .log(
                    LogLevel::Info,
                    LogCategory::Shutdown,
                    "ShutdownManager",
                    "Starting graceful shutdown sequence",
                )
                .await;
        } else {
            info!("[SHUTDOWN] Starting graceful shutdown sequence");
        }

        // Set initial stage
        *self.stage.write().await = ShutdownStage::StoppingHttpServer;

        // Execute shutdown stages
        if let Err(e) = self.execute_shutdown_stages().await {
            if let Some(logger) = &self.logger {
                logger
                    .log(
                        LogLevel::Error,
                        LogCategory::Shutdown,
                        "ShutdownManager",
                        &format!("Shutdown failed: {}", e),
                    )
                    .await;
            } else {
                error!("[SHUTDOWN] Shutdown failed: {}", e);
            }
            *self.stage.write().await = ShutdownStage::Failed(e.to_string());
            return Err(e);
        }

        *self.stage.write().await = ShutdownStage::Completed;

        let elapsed = self.start_time.unwrap().elapsed();
        if let Some(logger) = &self.logger {
            logger
                .log(
                    LogLevel::Info,
                    LogCategory::Shutdown,
                    "ShutdownManager",
                    &format!("Graceful shutdown completed in {:?}", elapsed),
                )
                .await;
        } else {
            info!("[SHUTDOWN] Graceful shutdown completed in {:?}", elapsed);
        }

        Ok(())
    }

    /// Execute all shutdown stages
    async fn execute_shutdown_stages(
        &mut self,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let total_timeout = self.config.graceful_timeout;
        let start_time = Instant::now();

        // Stage 1: Stop HTTP server
        self.stop_http_servers().await?;

        // Stage 2: Drain connections
        *self.stage.write().await = ShutdownStage::DrainConnections;
        self.drain_connections().await?;

        // Stage 3: Stop background services
        *self.stage.write().await = ShutdownStage::StoppingBackgroundServices;
        self.stop_background_services().await?;

        // Stage 4: Cleanup resources
        *self.stage.write().await = ShutdownStage::CleanupResources;
        self.cleanup_resources().await?;

        // Check if we exceeded the total timeout
        let elapsed = start_time.elapsed();
        if elapsed > total_timeout {
            warn!(
                "[SHUTDOWN] Shutdown took longer than expected: {:?} > {:?}",
                elapsed, total_timeout
            );
        }

        Ok(())
    }

    /// Stop HTTP servers
    async fn stop_http_servers(&mut self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        info!("[SHUTDOWN] Stopping HTTP servers");

        let mut http_services = Vec::new();
        let mut other_services = Vec::new();

        // Separate HTTP services from others
        for service in self.services.drain(..) {
            if service.name().contains("HTTP") || service.name().contains("Server") {
                http_services.push(service);
            } else {
                other_services.push(service);
            }
        }

        // Shutdown HTTP services with timeout
        let shutdown_future = async {
            for mut service in http_services {
                if let Err(e) = service.shutdown().await {
                    error!("[SHUTDOWN] Failed to shutdown {}: {}", service.name(), e);
                    // Try force shutdown
                    if let Err(fe) = service.force_shutdown().await {
                        error!(
                            "[SHUTDOWN] Force shutdown also failed for {}: {}",
                            service.name(),
                            fe
                        );
                    }
                }
            }
        };

        // Apply timeout
        if let Err(_) = tokio::time::timeout(self.config.graceful_timeout, shutdown_future).await {
            warn!("[SHUTDOWN] HTTP server shutdown timed out, proceeding anyway");
        }

        // Restore non-HTTP services
        self.services = other_services;

        info!("[SHUTDOWN] HTTP servers stopped");
        Ok(())
    }

    /// Drain active connections
    async fn drain_connections(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        info!("[SHUTDOWN] Draining active connections");

        // Wait for connections to drain naturally
        tokio::time::sleep(self.config.drain_timeout).await;

        // TODO: Implement actual connection monitoring and draining
        // This would typically involve:
        // 1. Monitoring active connection count
        // 2. Waiting for connections to complete naturally
        // 3. Force closing connections after timeout

        info!("[SHUTDOWN] Connection draining completed");
        Ok(())
    }

    /// Stop background services
    async fn stop_background_services(
        &mut self,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        info!("[SHUTDOWN] Stopping background services");

        let shutdown_future = async {
            for mut service in self.services.drain(..) {
                if let Err(e) = service.shutdown().await {
                    error!("[SHUTDOWN] Failed to shutdown {}: {}", service.name(), e);
                    // Try force shutdown
                    if let Err(fe) = service.force_shutdown().await {
                        error!(
                            "[SHUTDOWN] Force shutdown also failed for {}: {}",
                            service.name(),
                            fe
                        );
                    }
                }
            }
        };

        // Apply timeout
        if let Err(_) = tokio::time::timeout(self.config.graceful_timeout, shutdown_future).await {
            warn!("[SHUTDOWN] Background services shutdown timed out");
        }

        info!("[SHUTDOWN] Background services stopped");
        Ok(())
    }

    /// Cleanup resources
    async fn cleanup_resources(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        info!("[SHUTDOWN] Cleaning up resources");

        // TODO: Implement resource cleanup
        // This would typically involve:
        // 1. Closing database connections
        // 2. Flushing caches
        // 3. Cleaning up temporary files
        // 4. Releasing locks

        // Simulate cleanup time
        tokio::time::sleep(Duration::from_millis(100)).await;

        info!("[SHUTDOWN] Resource cleanup completed");
        Ok(())
    }

    /// Get current shutdown stage
    pub async fn get_stage(&self) -> ShutdownStage {
        self.stage.read().await.clone()
    }

    /// Get shutdown elapsed time
    pub fn get_elapsed_time(&self) -> Option<Duration> {
        self.start_time.map(|start| start.elapsed())
    }

    /// Force shutdown (emergency)
    pub async fn force_shutdown(&mut self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        warn!("[SHUTDOWN] Initiating force shutdown");

        let force_future = async {
            for mut service in self.services.drain(..) {
                if let Err(e) = service.force_shutdown().await {
                    error!(
                        "[SHUTDOWN] Force shutdown failed for {}: {}",
                        service.name(),
                        e
                    );
                }
            }
        };

        // Apply force timeout
        if let Err(_) = tokio::time::timeout(self.config.force_timeout, force_future).await {
            error!("[SHUTDOWN] Force shutdown timed out, terminating anyway");
        }

        warn!("[SHUTDOWN] Force shutdown completed");
        Ok(())
    }
}

impl Default for ShutdownManager {
    fn default() -> Self {
        Self::new()
    }
}
