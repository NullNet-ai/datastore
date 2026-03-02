use crate::{
    config::core::EnvConfig,
    lifecycle::logging::{LogCategory, LogLevel},
    providers::operations::sync::sync_service::bg_sync_with_shutdown_check,
    utils::helpers::parse_command_args,
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

// RuntimeMetrics removed - using HealthService/HealthMetrics instead

/// Runtime manager responsible for application execution phase
pub struct RuntimeManager {
    start_time: Option<Instant>,
    shutdown_requested: Arc<RwLock<bool>>,
    health_check_interval: Duration,
    logger: Option<Arc<crate::lifecycle::logging::LifecycleLogger>>,
    health_service: Option<Arc<crate::lifecycle::health_service::HealthService>>,
    state_manager: Option<Arc<crate::lifecycle::state::StateManager>>,
    post_startup_callback: Option<
        Box<
            dyn Fn() -> std::pin::Pin<Box<dyn std::future::Future<Output = ()> + Send>>
                + Send
                + Sync,
        >,
    >,
    shutdown_manager: Option<*mut crate::lifecycle::shutdown::ShutdownManager>,
    shutdown_callback: Option<
        Arc<
            dyn Fn() -> std::pin::Pin<Box<dyn std::future::Future<Output = ()> + Send>>
                + Send
                + Sync,
        >,
    >,
    config: Arc<EnvConfig>,
}

impl RuntimeManager {
    /// Create a new runtime manager
    pub fn new(config: Arc<EnvConfig>) -> Self {
        info!("[RUNTIME] Initializing runtime manager");

        Self {
            start_time: None,
            shutdown_requested: Arc::new(RwLock::new(false)),
            health_check_interval: Duration::from_secs(30),
            logger: None,
            health_service: None,
            state_manager: None,
            post_startup_callback: None,
            shutdown_manager: None,
            shutdown_callback: None,
            config,
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

    pub fn with_state_manager(
        mut self,
        state_manager: Arc<crate::lifecycle::state::StateManager>,
    ) -> Self {
        self.state_manager = Some(state_manager);
        self
    }

    /// Set post-startup callback for lifecycle hooks
    pub fn with_post_startup_callback<F, Fut>(mut self, callback: F) -> Self
    where
        F: Fn() -> Fut + Send + Sync + 'static,
        Fut: std::future::Future<Output = ()> + Send + 'static,
    {
        self.post_startup_callback = Some(Box::new(move || Box::pin(callback())));
        self
    }

    /// Set the shutdown manager for service registration
    pub fn with_shutdown_manager(
        mut self,
        shutdown_manager: &mut crate::lifecycle::shutdown::ShutdownManager,
    ) -> Self {
        self.shutdown_manager = Some(shutdown_manager as *mut _);
        self
    }

    /// Set shutdown callback function
    pub fn with_shutdown_callback<F, Fut>(mut self, callback: F) -> Self
    where
        F: Fn() -> Fut + Send + Sync + 'static,
        Fut: std::future::Future<Output = ()> + Send + 'static,
    {
        self.shutdown_callback = Some(Arc::new(move || Box::pin(callback())));
        self
    }

    /// Execute the runtime phase with actual services
    pub async fn execute(
        &mut self,
        pool: crate::database::db::AsyncDbPool,
        s3_client: aws_sdk_s3::Client,
        bucket_name: String,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        info!("[RUNTIME] ===== ENTERING RuntimeManager::execute =====");
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

        let args = parse_command_args();
        println!("args: {:?}", &args);
        // // Handle database operations
        // db::handle_database_operations(&args).await;

        // Start HTTP server and run main loop
        info!("[RUNTIME] About to call run_main_loop_with_server");
        self.run_main_loop_with_server(pool, s3_client, bucket_name)
            .await?;
        info!("[RUNTIME] run_main_loop_with_server completed");

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
        let shutdown_requested = self.shutdown_requested.clone();
        let health_service = self.health_service.clone();
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

                // Update health service if available
                if let Some(health_service) = &health_service {
                    let is_healthy = matches!(health_status, HealthStatus::Healthy);
                    health_service.update_health_status(is_healthy).await;
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

    /// Check database health using the connection pool (reuses connections instead of opening new ones)
    pub async fn check_database_health() -> Result<(), String> {
        use crate::database::db::get_async_connection;
        use diesel_async::RunQueryDsl;
        use std::time::Duration;

        debug!("[RUNTIME] Checking database health");

        // Perform actual database health check with timeout using pooled connection
        match tokio::time::timeout(Duration::from_secs(5), async {
            let mut conn = get_async_connection().await;
            diesel::sql_query("SELECT 1 as health_check")
                .execute(&mut *conn)
                .await
                .map_err(|e| format!("Health check query failed: {}", e))?;
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
        let shutdown_callback = self.shutdown_callback.clone();
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

            // Call shutdown callback if available, otherwise set flag directly
            if let Some(callback) = shutdown_callback {
                info!("[RUNTIME] Calling shutdown callback");
                callback().await;
            } else {
                info!("[RUNTIME] No shutdown callback, setting flag directly");
                *shutdown_requested.write().await = true;
            }
            info!("[RUNTIME] Shutdown request processed, runtime will terminate gracefully");
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

        // Parse configuration for service configuration
        let grpc_url = self.config.grpc_url.clone();
        let grpc_port = self.config.grpc_port.clone();
        let socket_host = self.config.socket_host.clone();
        let socket_port = self.config.socket_port.clone();

        let grpc_addr = format!("{}:{}", grpc_url, grpc_port);

        // Create shutdown channels for each service
        let (grpc_shutdown_tx, mut grpc_shutdown_rx) = tokio::sync::mpsc::channel::<()>(1);
        let (batch_sync_shutdown_tx, mut batch_sync_shutdown_rx) =
            tokio::sync::mpsc::channel::<()>(1);
        let (socket_shutdown_tx, mut socket_shutdown_rx) = tokio::sync::mpsc::channel::<()>(1);
        let (bg_sync_shutdown_tx, mut bg_sync_shutdown_rx) = tokio::sync::mpsc::channel::<()>(1);

        // Register services with shutdown manager if available
        info!("[RUNTIME] Checking if shutdown manager is available for service registration");
        if let Some(shutdown_manager_ptr) = self.shutdown_manager {
            info!("[RUNTIME] Shutdown manager found, registering background services");
            use crate::lifecycle::shutdown::BackgroundServiceShutdown;

            unsafe {
                let shutdown_manager = &mut *shutdown_manager_ptr;

                // Register gRPC service
                let grpc_service =
                    BackgroundServiceShutdown::new("grpc-server".to_string(), grpc_shutdown_tx);
                shutdown_manager.register_service(Box::new(grpc_service));

                // Register batch sync service
                let batch_sync_service = BackgroundServiceShutdown::new(
                    "batch-sync-service".to_string(),
                    batch_sync_shutdown_tx,
                );
                shutdown_manager.register_service(Box::new(batch_sync_service));

                // Register Socket.IO service
                let socket_service = BackgroundServiceShutdown::new(
                    "socket-io-server".to_string(),
                    socket_shutdown_tx,
                );
                shutdown_manager.register_service(Box::new(socket_service));

                // Register background sync service
                let bg_sync_service = BackgroundServiceShutdown::new(
                    "background-sync".to_string(),
                    bg_sync_shutdown_tx,
                );
                shutdown_manager.register_service(Box::new(bg_sync_service));

                info!(
                    "[RUNTIME] Registered {} background services with shutdown manager",
                    4
                );
            }
        } else {
            info!("[RUNTIME] No shutdown manager available, skipping service registration");
        }

        // Start gRPC server
        tokio::spawn(async move {
            use crate::generated::grpc_controller::GrpcController;

            tokio::select! {
                result = GrpcController::init(&grpc_addr) => {
                    match result {
                        Ok(_) => info!("gRPC server started successfully on {}", grpc_addr),
                        Err(e) => error!("Failed to start gRPC server: {}", e),
                    }
                }
                _ = grpc_shutdown_rx.recv() => {
                    info!("gRPC server received shutdown signal");
                }
            }
        });

        // Start background batch sync service
        tokio::spawn(async move {
            use crate::providers::operations::batch_sync::background_sync::BackgroundSyncService;

            tokio::select! {
                _ = async {
                    match BackgroundSyncService::new().await {
                        Ok(service) => {
                            if let Err(e) = service.init().await {
                                error!("Error in background batch sync service: {}", e);
                            }
                        }
                        Err(e) => error!("Failed to initialize BackgroundSyncService: {}", e),
                    }
                } => {}
                _ = batch_sync_shutdown_rx.recv() => {
                    info!("Background batch sync service received shutdown signal");
                }
            }
        });

        // Start Socket.IO server
        tokio::spawn(async move {
            use crate::providers::operations::message_stream::gateway::{
                create_socket_io, set_streaming_service,
            };
            use crate::providers::operations::message_stream::streaming_service::MessageStreamingService;
            use axum::Router;

            tokio::select! {
                _ = async {
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
                } => {}
                _ = socket_shutdown_rx.recv() => {
                    info!("Socket.IO server received shutdown signal");
                }
            }
        });

        // start running background synced data to the sync server (bin/server)
        let shutdown_flag = self.shutdown_requested.clone();
        tokio::spawn(async move {
            tokio::select! {
                _ = async {
                    if let Err(e) = bg_sync_with_shutdown_check(move || {
                        let shutdown_flag = shutdown_flag.clone();
                        async move {
                            *shutdown_flag.read().await
                        }
                    }).await {
                        log::error!("Error starting background sync: {}", e);
                    }
                } => {}
                _ = bg_sync_shutdown_rx.recv() => {
                    info!("Background sync received shutdown signal");
                }
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

        // Get server configuration from config
        let bind_address = format!("{}:{}", self.config.host, self.config.port);

        // Create HTTP server
        let server = self.create_http_server(pool, s3_client, bucket_name, bind_address.clone())?;

        // Keep handle to stop server on Ctrl+C so the process exits and releases the port.
        // We do not register HttpServerShutdown with the shutdown manager so we can stop
        // and await the server here; otherwise the Server future would be dropped and the port stays bound.
        let server_handle = server.handle();
        let mut server_join = tokio::spawn(server);

        info!(
            "[RUNTIME] HTTP server successfully bound to {}",
            bind_address
        );

        // Call post-startup hooks after server is successfully bound
        info!("[RUNTIME] About to call post-startup hooks...");
        self.call_post_startup_hooks().await;
        info!("[RUNTIME] Post-startup hooks call completed");

        // Run server with shutdown handling. When Ctrl+C wins, stop the server and await
        // its completion so the process exits and the port is released.
        tokio::select! {
            result = &mut server_join => {
                match result {
                    Ok(Ok(_)) => info!("[RUNTIME] HTTP server completed successfully"),
                    Ok(Err(e)) => {
                        error!("[RUNTIME] HTTP server error: {}", e);
                        return Err(Box::new(e));
                    }
                    Err(e) => {
                        error!("[RUNTIME] HTTP server task join error: {}", e);
                        return Err(Box::new(e));
                    }
                }
            },
            _ = self.wait_for_shutdown() => {
                info!("[RUNTIME] Shutdown signal received, stopping HTTP server");
                server_handle.stop(true).await;
                if let Ok(join_result) = server_join.await {
                    match join_result {
                        Ok(_) => info!("[RUNTIME] HTTP server stopped, port released"),
                        Err(e) => error!("[RUNTIME] HTTP server error during shutdown: {}", e),
                    }
                }
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
        let state_manager = self.state_manager.clone();

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
                .configure(system_router::configure_system_routes)
                .configure(|cfg| store_router::configure_store_routes(cfg, app_state.clone()))
                .configure(listener_router::configure_listener_routes)
                .configure(|cfg| file_router::configure_file_routes(cfg, app_state.clone()))
                // TODO: not sure what happens here if the order is set at above
                // order issue
                .configure(health_router::configure_health_routes);

            if let Some(hs) = &health_service {
                app = app.app_data(web::Data::new(hs.clone()));
            }

            if let Some(sm) = &state_manager {
                app = app.app_data(web::Data::new(sm.clone()));
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

    /// Call post-startup hooks using the provided callback
    async fn call_post_startup_hooks(&self) {
        info!("[RUNTIME] ===== ENTERING call_post_startup_hooks =====");

        // Log server startup completion
        if let Some(logger) = &self.logger {
            info!("[RUNTIME] Using lifecycle logger for post-startup hooks");
            logger
                .log(
                    LogLevel::Info,
                    LogCategory::Runtime,
                    "RuntimeManager",
                    "HTTP server started successfully, executing post-startup hooks",
                )
                .await;
        } else {
            info!("[RUNTIME] HTTP server started successfully, executing post-startup hooks (no logger)");
        }

        // Execute the post-startup callback if provided
        if let Some(callback) = &self.post_startup_callback {
            info!("[RUNTIME] Post-startup callback found, executing...");
            if let Some(logger) = &self.logger {
                logger
                    .log(
                        LogLevel::Info,
                        LogCategory::Runtime,
                        "RuntimeManager",
                        "Executing post-startup callback",
                    )
                    .await;
            } else {
                info!("[RUNTIME] Executing post-startup callback (no logger)");
            }
            let future = callback();
            future.await;
            if let Some(logger) = &self.logger {
                logger
                    .log(
                        LogLevel::Info,
                        LogCategory::Runtime,
                        "RuntimeManager",
                        "Post-startup callback completed",
                    )
                    .await;
            } else {
                info!("[RUNTIME] Post-startup callback completed (no logger)");
            }
        } else {
            info!("[RUNTIME] No post-startup callback found!");
            if let Some(logger) = &self.logger {
                logger
                    .log(
                        LogLevel::Info,
                        LogCategory::Runtime,
                        "RuntimeManager",
                        "No post-startup callback configured, skipping hooks",
                    )
                    .await;
            } else {
                info!("[RUNTIME] No post-startup callback configured, skipping hooks (no logger)");
            }
        }

        info!("[RUNTIME] ===== EXITING call_post_startup_hooks =====");
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

    /// Get shutdown flag for external management
    pub fn get_shutdown_flag(&self) -> Arc<RwLock<bool>> {
        self.shutdown_requested.clone()
    }
}

// RuntimeManager no longer implements Default since it requires config parameter

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
