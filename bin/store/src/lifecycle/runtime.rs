use crate::{
    lifecycle::logging::{LogCategory, LogLevel},
    providers::operations::sync::sync_service::bg_sync,
};
use log::{debug, error, info, warn};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::signal::unix::{signal, SignalKind};
use tokio::sync::RwLock;

/// Runtime health status
#[derive(Debug, Clone, PartialEq)]
pub enum HealthStatus {
    Healthy,
    Degraded(String),
    Unhealthy(String),
}

/// Runtime metrics
#[derive(Debug, Clone)]
pub struct RuntimeMetrics {
    pub uptime: Duration,
    pub health_status: HealthStatus,
    pub active_connections: u64,
    pub processed_requests: u64,
    pub last_health_check: Instant,
}

/// Runtime manager responsible for application execution phase
pub struct RuntimeManager {
    start_time: Option<Instant>,
    metrics: Arc<RwLock<RuntimeMetrics>>,
    shutdown_requested: Arc<RwLock<bool>>,
    health_check_interval: Duration,
    logger: Option<Arc<crate::lifecycle::logging::LifecycleLogger>>,
    health_service: Option<Arc<crate::lifecycle::health_service::HealthService>>,
}

impl RuntimeManager {
    /// Create a new runtime manager
    pub fn new() -> Self {
        info!("[RUNTIME] Initializing runtime manager");

        let metrics = RuntimeMetrics {
            uptime: Duration::from_secs(0),
            health_status: HealthStatus::Healthy,
            active_connections: 0,
            processed_requests: 0,
            last_health_check: Instant::now(),
        };

        Self {
            start_time: None,
            metrics: Arc::new(RwLock::new(metrics)),
            shutdown_requested: Arc::new(RwLock::new(false)),
            health_check_interval: Duration::from_secs(30),
            logger: None,
            health_service: None,
        }
    }

    /// Set the logger for structured logging
    pub fn with_logger(mut self, logger: Arc<crate::lifecycle::logging::LifecycleLogger>) -> Self {
        self.logger = Some(logger);
        self
    }

    pub fn with_health_service(
        mut self,
        health_service: Arc<crate::lifecycle::health_service::HealthService>,
    ) -> Self {
        self.health_service = Some(health_service);
        self
    }

    /// Execute the runtime phase with actual services
    pub async fn execute(
        &mut self,
        pool: crate::database::db::AsyncDbPool,
        s3_client: aws_sdk_s3::Client,
        bucket_name: String,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        self.start_time = Some(Instant::now());

        if let Some(logger) = &self.logger {
            logger
                .log(
                    LogLevel::Info,
                    LogCategory::Runtime,
                    "RuntimeManager",
                    "Starting application runtime phase",
                )
                .await;
        }

        // Start health monitoring
        self.start_health_monitoring().await;

        // Setup signal handlers
        self.setup_signal_handlers().await?;

        // Start background services
        self.start_background_services().await?;

        // Start HTTP server and run main loop
        self.run_main_loop_with_server(pool, s3_client, bucket_name)
            .await?;

        if let Some(logger) = &self.logger {
            logger
                .log(
                    LogLevel::Info,
                    LogCategory::Runtime,
                    "RuntimeManager",
                    "Runtime phase completed",
                )
                .await;
        }
        Ok(())
    }

    /// Start health monitoring background task
    async fn start_health_monitoring(&self) {
        let metrics = self.metrics.clone();
        let shutdown_requested = self.shutdown_requested.clone();
        let interval = self.health_check_interval;

        tokio::spawn(async move {
            let mut health_interval = tokio::time::interval(interval);

            loop {
                health_interval.tick().await;

                // Check if shutdown was requested
                if *shutdown_requested.read().await {
                    debug!("[RUNTIME] Health monitoring stopping due to shutdown request");
                    break;
                }

                // Perform health check
                let health_status = Self::perform_health_check().await;

                // Update metrics
                {
                    let mut metrics_guard = metrics.write().await;
                    metrics_guard.health_status = health_status.clone();
                    metrics_guard.last_health_check = Instant::now();
                }

                // Log health status changes
                match health_status {
                    HealthStatus::Healthy => {
                        debug!("[RUNTIME] Health check passed");
                    }
                    HealthStatus::Degraded(reason) => {
                        warn!("[RUNTIME] System degraded: {}", reason);
                    }
                    HealthStatus::Unhealthy(reason) => {
                        error!("[RUNTIME] System unhealthy: {}", reason);
                    }
                }
            }

            info!("[RUNTIME] Health monitoring stopped");
        });

        info!(
            "[RUNTIME] Health monitoring started with interval: {:?}",
            interval
        );
    }

    /// Perform comprehensive health check
    async fn perform_health_check() -> HealthStatus {
        // Check database connectivity
        if let Err(e) = Self::check_database_health().await {
            return HealthStatus::Unhealthy(format!("Database connectivity issue: {}", e));
        }

        // Check cache system
        if let Err(e) = check_cache_health().await {
            return HealthStatus::Degraded(format!("Cache system issue: {}", e));
        }

        // Check memory usage
        if let Some(warning) = Self::check_memory_usage().await {
            return HealthStatus::Degraded(warning);
        }

        HealthStatus::Healthy
    }

    /// Check database health
    pub async fn check_database_health() -> Result<(), String> {
        use crate::database::db::create_connection;
        use std::time::Duration;

        debug!("[RUNTIME] Checking database health");

        // Perform actual database health check with timeout
        match tokio::time::timeout(Duration::from_secs(5), async {
            // Create database connection
            let client = create_connection()
                .await
                .map_err(|e| format!("Failed to create database connection: {}", e))?;

            // Execute simple health check query
            let rows = client
                .query("SELECT 1 as health_check", &[])
                .await
                .map_err(|e| format!("Health check query failed: {}", e))?;

            // Verify we got expected result
            if rows.is_empty() {
                return Err("Health check query returned no rows".to_string());
            }

            let health_value: i32 = rows[0].get(0);
            if health_value != 1 {
                return Err(format!(
                    "Health check query returned unexpected value: {}",
                    health_value
                ));
            }

            debug!("[RUNTIME] Database health check successful");
            Ok(())
        })
        .await
        {
            Ok(Ok(())) => {
                info!("[RUNTIME] Database health check passed");
                Ok(())
            }
            Ok(Err(e)) => {
                error!("[RUNTIME] Database health check failed: {}", e);
                Err(e)
            }
            Err(_) => {
                let error_msg = "Database health check timed out after 5 seconds";
                error!("[RUNTIME] {}", error_msg);
                Err(error_msg.to_string())
            }
        }
    }

    /// Check memory usage
    async fn check_memory_usage() -> Option<String> {
        debug!("[RUNTIME] Checking memory usage");

        // TODO: Implement actual memory usage check
        // This would typically check RSS, heap usage, etc.

        None
    }

    /// Setup signal handlers for graceful shutdown
    async fn setup_signal_handlers(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let shutdown_requested = self.shutdown_requested.clone();

        tokio::spawn(async move {
            let mut sigint =
                signal(SignalKind::interrupt()).expect("Failed to create SIGINT handler");
            let mut sigterm =
                signal(SignalKind::terminate()).expect("Failed to create SIGTERM handler");

            tokio::select! {
                _ = sigint.recv() => {
                    info!("[RUNTIME] SIGINT received, requesting graceful shutdown");
                }
                _ = sigterm.recv() => {
                    info!("[RUNTIME] SIGTERM received, requesting graceful shutdown");
                }
            }

            *shutdown_requested.write().await = true;
        });

        info!("[RUNTIME] Signal handlers configured");
        Ok(())
    }

    /// Start background services
    async fn start_background_services(
        &self,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        if let Some(logger) = &self.logger {
            logger
                .log(
                    LogLevel::Info,
                    LogCategory::Runtime,
                    "RuntimeManager",
                    "Starting background services",
                )
                .await;
        }

        // Parse environment variables for service configuration
        let grpc_url = std::env::var("GRPC_URL").unwrap_or_else(|_| "127.0.0.1".to_string());
        let grpc_port = std::env::var("GRPC_PORT").unwrap_or_else(|_| "50051".to_string());
        let socket_host = std::env::var("SOCKET_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
        let socket_port = std::env::var("SOCKET_PORT").unwrap_or_else(|_| "3001".to_string());

        let grpc_addr = format!("{}:{}", grpc_url, grpc_port);

        // Start gRPC server
        tokio::spawn(async move {
            use crate::generated::grpc_controller::GrpcController;
            match GrpcController::init(&grpc_addr).await {
                Ok(_) => info!("gRPC server started successfully on {}", grpc_addr),
                Err(e) => error!("Failed to start gRPC server: {}", e),
            }
        });

        // Start background sync service
        tokio::spawn(async {
            use crate::providers::operations::batch_sync::background_sync::BackgroundSyncService;
            match BackgroundSyncService::new().await {
                Ok(service) => {
                    if let Err(e) = service.init().await {
                        error!("Error in background sync service: {}", e);
                    }
                }
                Err(e) => error!("Failed to initialize BackgroundSyncService: {}", e),
            }
        });

        // Start Socket.IO server
        tokio::spawn(async move {
            use crate::providers::operations::message_stream::gateway::{
                create_socket_io, set_streaming_service,
            };
            use crate::providers::operations::message_stream::streaming_service::MessageStreamingService;
            use axum::Router;

            // Create Socket.IO layer and instance
            let (layer, io) = create_socket_io();

            // Initialize the MessageStreamingService
            let streaming_service = MessageStreamingService::new(io);

            // Set the streaming service reference
            set_streaming_service(streaming_service.clone());

            // Initialize the streaming service (starts broker and routing)
            if let Err(e) = streaming_service.initialize().await {
                error!("Failed to initialize MessageStreamingService: {}", e);
            } else {
                info!("MessageStreamingService initialized successfully");
            }

            // Create and run the Socket.IO server
            let app = Router::new().layer(layer);
            let listener =
                match tokio::net::TcpListener::bind(format!("{}:{}", socket_host, socket_port))
                    .await
                {
                    Ok(listener) => listener,
                    Err(e) => {
                        error!("Failed to bind Socket.IO server: {}", e);
                        return;
                    }
                };

            info!(
                "Socket.IO server listening on {}:{}",
                socket_host, socket_port
            );
            if let Err(e) = axum::serve(listener, app).await {
                error!("Socket.IO server error: {}", e);
            }
        });

        // start running background sync
        tokio::spawn(async {
            if let Err(e) = bg_sync().await {
                log::error!("Error starting background sync: {}", e);
            }
        });

        info!("[RUNTIME] Background services started");
        Ok(())
    }

    /// Run the main application loop with HTTP server
    async fn run_main_loop_with_server(
        &self,
        pool: crate::database::db::AsyncDbPool,
        s3_client: aws_sdk_s3::Client,
        bucket_name: String,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        info!("[RUNTIME] Starting HTTP server and main loop");

        // Get server configuration from environment
        let host = std::env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
        let port = std::env::var("PORT").unwrap_or_else(|_| "5000".to_string());
        let bind_address = format!("{}:{}", host, port);

        // Create HTTP server
        let server = self.create_http_server(pool, s3_client, bucket_name, bind_address.clone())?;

        info!(
            "[RUNTIME] HTTP server successfully bound to {}",
            bind_address
        );

        // Run server with shutdown handling
        tokio::select! {
            result = server => {
                match result {
                    Ok(_) => info!("[RUNTIME] HTTP server completed successfully"),
                    Err(e) => {
                        error!("[RUNTIME] HTTP server error: {}", e);
                        return Err(Box::new(e));
                    }
                }
            },
            _ = self.wait_for_shutdown() => {
                info!("[RUNTIME] Shutdown signal received, stopping HTTP server");
            }
        }

        Ok(())
    }

    /// Create and configure the HTTP server
    fn create_http_server(
        &self,
        pool: crate::database::db::AsyncDbPool,
        s3_client: aws_sdk_s3::Client,
        bucket_name: String,
        bind_address: String,
    ) -> Result<actix_web::dev::Server, Box<dyn std::error::Error + Send + Sync>> {
        use crate::providers::storage::AppState;
        use crate::routers::*;
        use actix_web::{middleware::Logger, web, App, HttpServer};

        info!("[RUNTIME] Configuring HTTP server for {}", bind_address);

        let health_service = self.health_service.clone();

        // Validate bind address format
        if !bind_address.contains(':') {
            return Err(format!("Invalid bind address format: {}", bind_address).into());
        }

        let server = HttpServer::new(move || {
            let app_state = AppState {
                s3_client: s3_client.clone(),
                bucket_name: bucket_name.clone(),
            };

            let mut app = App::new()
                .wrap(Logger::default())
                .app_data(web::Data::new(pool.clone()))
                .configure(sync_router::configure_sync_routes)
                .configure(organizations_router::configure_organizations_routes)
                .configure(organizations_router::configure_token_routes)
                .configure(root_store_router::configure_root_store_routes)
                .configure(|cfg| store_router::configure_store_routes(cfg, app_state.clone()))
                .configure(listener_router::configure_listener_routes)
                .configure(|cfg| file_router::configure_file_routes(cfg, app_state.clone()))
                // TODO: not sure what happens here if the order is set at above
                // order issue
                .configure(health_router::configure_health_routes);

            if let Some(hs) = &health_service {
                app = app.app_data(web::Data::new(hs.clone()));
            }

            app
        })
        .workers(1)
        .disable_signals()
        .bind(&bind_address)
        .map_err(|e| {
            error!(
                "[RUNTIME] Failed to bind HTTP server to {}: {}",
                bind_address, e
            );
            e
        })?
        .run();

        info!(
            "[RUNTIME] HTTP server configured and ready to start on {}",
            bind_address
        );
        Ok(server)
    }

    /// Wait for shutdown signal
    async fn wait_for_shutdown(&self) {
        loop {
            if *self.shutdown_requested.read().await {
                break;
            }
            tokio::time::sleep(Duration::from_millis(100)).await;
        }
    }

    /// Get current runtime metrics
    pub async fn get_metrics(&self) -> RuntimeMetrics {
        let mut metrics = self.metrics.read().await.clone();

        // Update uptime if we have a start time
        if let Some(start_time) = self.start_time {
            metrics.uptime = start_time.elapsed();
        }

        metrics
    }

    /// Request shutdown
    pub async fn request_shutdown(&self) {
        info!("[RUNTIME] Shutdown requested");
        *self.shutdown_requested.write().await = true;
    }

    /// Check if shutdown was requested
    pub async fn is_shutdown_requested(&self) -> bool {
        *self.shutdown_requested.read().await
    }
}

impl Default for RuntimeManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Check cache system health
pub async fn check_cache_health() -> Result<String, String> {
    debug!("[RUNTIME] Checking cache health");

    use crate::providers::storage::cache::{cache, CacheConfig};
    use std::time::Duration;
    use tokio::time::timeout;

    // Get current cache configuration
    let cache_config = CacheConfig::global();
    let cache_type = cache_config.cache_type;

    debug!("[RUNTIME] Cache type: {:?}", cache_type);

    // Test cache operations with timeout
    let health_check = async {
        let test_key = "__health_check__".to_string();
        let test_value = serde_json::Value::String("test_value".to_string());

        // Test insert operation
        cache.insert(test_key.clone(), test_value.clone());

        // Test get operation
        match cache.get(&test_key) {
            Some(retrieved_value) => {
                if retrieved_value == test_value {
                    debug!("[RUNTIME] Cache read/write test successful");

                    // Clean up test key
                    cache.remove(&test_key);

                    Ok(())
                } else {
                    Err("Cache returned incorrect value".to_string())
                }
            }
            None => Err("Cache failed to retrieve test value".to_string()),
        }
    };

    // Apply timeout to prevent hanging on Redis connection issues
    match timeout(Duration::from_secs(5), health_check).await {
        Ok(result) => match result {
            Ok(_) => {
                info!("[RUNTIME] Cache health check passed for {:?}", cache_type);
                Ok(format!("Cache ({:?}) connectivity verified", cache_type))
            }
            Err(e) => {
                error!("[RUNTIME] Cache health check failed: {}", e);
                Err(format!("Cache health check failed: {}", e))
            }
        },
        Err(_) => {
            error!("[RUNTIME] Cache health check timed out");
            Err("Cache health check timed out after 5 seconds".to_string())
        }
    }
}
