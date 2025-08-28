use crate::database::db;
use crate::initializers::init::initialize;
use crate::initializers::structs::EInitializer;
use crate::lifecycle::logging::{LogCategory, LogLevel};
use crate::providers::operations::batch_sync::batch_sync::BatchSyncService;
use crate::providers::operations::message_stream::pg_listener_service::PgListenerService;
use crate::providers::operations::sync::message_manager::{create_message_channel, SENDER};
use crate::providers::operations::sync::transactions::queue_service::QueueService;
use crate::providers::operations::sync::transactions::transaction_service::TransactionService;
use crate::providers::storage;
use crate::providers::storage::cache::cache_factory::CacheType;
use crate::providers::storage::cache::{cache, CacheConfig};
use log::{debug, error, info, warn};
use std::sync::Arc;
use std::time::Duration;
use std::time::Instant;

/// Startup configuration validation result
#[derive(Debug)]
pub struct StartupValidation {
    pub is_valid: bool,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
}

/// Startup manager for handling application initialization
pub struct StartupManager {
    start_time: Option<Instant>,
    pub pool: Option<crate::database::db::AsyncDbPool>,
    pub s3_client: Option<aws_sdk_s3::Client>,
    pub bucket_name: Option<String>,
    state_manager: Arc<crate::lifecycle::state::StateManager>,
    logger: Arc<crate::lifecycle::logging::LifecycleLogger>,
}

impl StartupManager {
    /// Create a new startup manager
    pub fn new(
        state_manager: Arc<crate::lifecycle::state::StateManager>,
        logger: Arc<crate::lifecycle::logging::LifecycleLogger>,
    ) -> Self {
        info!("[STARTUP] Initializing startup manager");
        Self {
            start_time: None,
            pool: None,
            s3_client: None,
            bucket_name: None,
            state_manager,
            logger,
        }
    }

    /// Execute the complete startup sequence
    pub async fn execute(&mut self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        self.start_time = Some(Instant::now());

        self.logger
            .log(
                LogLevel::Info,
                LogCategory::Startup,
                "StartupManager",
                "Beginning application startup sequence",
            )
            .await;

        // Phase 1: Configuration validation
        self.logger
            .log(
                LogLevel::Info,
                LogCategory::Startup,
                "StartupManager",
                "Phase 1: Configuration validation",
            )
            .await;
        self.validate_configuration().await?;

        // Phase 2: Core services initialization
        self.logger
            .log(
                LogLevel::Info,
                LogCategory::Startup,
                "StartupManager",
                "Phase 2: Core services initialization",
            )
            .await;
        self.initialize_core_services().await?;

        // Phase 3: Database and storage setup
        self.logger
            .log(
                LogLevel::Info,
                LogCategory::Startup,
                "StartupManager",
                "Phase 3: Database and storage setup",
            )
            .await;
        let (pool, s3_client, bucket_name) = self.setup_database_and_storage().await?;
        self.pool = Some(pool);
        self.s3_client = Some(s3_client);
        self.bucket_name = Some(bucket_name);

        // Phase 4: Background services initialization
        self.logger
            .log(
                LogLevel::Info,
                LogCategory::Startup,
                "StartupManager",
                "Phase 4: Background services initialization",
            )
            .await;
        self.initialize_background_services().await?;

        // Phase 5: Message and queue services
        self.logger
            .log(
                LogLevel::Info,
                LogCategory::Startup,
                "StartupManager",
                "Phase 5: Message and queue services",
            )
            .await;
        self.setup_messaging_services().await?;

        let elapsed = self.start_time.unwrap().elapsed();
        self.logger
            .log(
                LogLevel::Info,
                LogCategory::Startup,
                "StartupManager",
                &format!("Startup sequence completed successfully in {:?}", elapsed),
            )
            .await;

        Ok(())
    }

    /// Validate application configuration
    async fn validate_configuration(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        debug!("[STARTUP] Validating environment configuration");

        let validation = self.perform_config_validation().await;

        // Log warnings
        for warning in &validation.warnings {
            warn!("[STARTUP] Configuration warning: {}", warning);
        }

        // Log errors and fail if invalid
        if !validation.is_valid {
            for error in &validation.errors {
                error!("[STARTUP] Configuration error: {}", error);
            }
            return Err(format!(
                "Configuration validation failed with {} errors",
                validation.errors.len()
            )
            .into());
        }

        info!("[STARTUP] Configuration validation passed");
        Ok(())
    }

    /// Perform detailed configuration validation
    async fn perform_config_validation(&self) -> StartupValidation {
        let mut validation = StartupValidation {
            is_valid: true,
            errors: Vec::new(),
            warnings: Vec::new(),
        };

        // Validate required environment variables
        self.validate_required_variables(&mut validation);

        // Validate port configurations
        self.validate_port_configurations(&mut validation);

        // Validate database configuration
        self.validate_database_configuration(&mut validation);

        // Validate cache configuration
        self.validate_cache_configuration(&mut validation);

        // Validate storage configuration
        self.validate_storage_configuration(&mut validation);

        // Validate security configuration
        self.validate_security_configuration(&mut validation);

        // Validate sync configuration
        self.validate_sync_configuration(&mut validation);

        // Validate organization configuration
        self.validate_organization_configuration(&mut validation);

        validation
    }

    /// Validate required environment variables
    fn validate_required_variables(&self, validation: &mut StartupValidation) {
        let required_vars = vec!["DATABASE_URL", "HOST", "PORT"];

        for var in required_vars {
            if std::env::var(var).is_err() {
                validation
                    .errors
                    .push(format!("Missing required environment variable: {}", var));
                validation.is_valid = false;
            }
        }

        // Check for recommended variables
        let recommended_vars = vec!["GRPC_PORT", "SOCKET_PORT", "CACHE_TYPE", "RUST_LOG"];

        for var in recommended_vars {
            if std::env::var(var).is_err() {
                validation.warnings.push(format!(
                    "Recommended environment variable not set: {} (using default)",
                    var
                ));
            }
        }
    }

    /// Validate port configurations
    fn validate_port_configurations(&self, validation: &mut StartupValidation) {
        // Validate main HTTP port
        if let Ok(port_str) = std::env::var("PORT") {
            match port_str.parse::<u16>() {
                Ok(port) => {
                    if port < 1024 {
                        validation.warnings.push(
                            "PORT is set to a privileged port (<1024), ensure proper permissions"
                                .to_string(),
                        );
                    }
                }
                Err(_) => {
                    validation
                        .errors
                        .push("PORT must be a valid integer".to_string());
                    validation.is_valid = false;
                }
            }
        }

        // Validate gRPC port
        if let Ok(grpc_port_str) = std::env::var("GRPC_PORT") {
            match grpc_port_str.parse::<u16>() {
                Ok(port) => {
                    if port < 1024 {
                        validation.warnings.push("GRPC_PORT is set to a privileged port (<1024), ensure proper permissions".to_string());
                    }

                    // Check for port conflicts
                    if let Ok(http_port_str) = std::env::var("PORT") {
                        if let Ok(http_port) = http_port_str.parse::<u16>() {
                            if port == http_port {
                                validation
                                    .errors
                                    .push("GRPC_PORT cannot be the same as PORT".to_string());
                                validation.is_valid = false;
                            }
                        }
                    }
                }
                Err(_) => {
                    validation
                        .errors
                        .push("GRPC_PORT must be a valid integer".to_string());
                    validation.is_valid = false;
                }
            }
        }

        // Validate socket port
        if let Ok(socket_port_str) = std::env::var("SOCKET_PORT") {
            match socket_port_str.parse::<u16>() {
                Ok(_port) => {
                    // Port is valid u16, no additional checks needed
                }
                Err(_) => {
                    validation
                        .errors
                        .push("SOCKET_PORT must be a valid integer".to_string());
                    validation.is_valid = false;
                }
            }
        }
    }

    /// Validate database configuration
    fn validate_database_configuration(&self, validation: &mut StartupValidation) {
        // Validate DATABASE_URL format
        if let Ok(db_url) = std::env::var("DATABASE_URL") {
            if !db_url.starts_with("postgres://") && !db_url.starts_with("postgresql://") {
                validation.errors.push("DATABASE_URL must be a valid PostgreSQL connection string starting with 'postgres://' or 'postgresql://'".to_string());
                validation.is_valid = false;
            }

            // Basic URL validation
            if !db_url.contains("@") || !db_url.contains("/") {
                validation.errors.push(
                    "DATABASE_URL appears to be malformed (missing @ or / characters)".to_string(),
                );
                validation.is_valid = false;
            }
        }

        // Validate individual PostgreSQL components if provided
        if let Ok(port_str) = std::env::var("POSTGRES_PORT") {
            match port_str.parse::<u16>() {
                Ok(port) => {
                    if port == 0 {
                        validation
                            .errors
                            .push("POSTGRES_PORT cannot be 0".to_string());
                        validation.is_valid = false;
                    }
                }
                Err(_) => {
                    validation
                        .errors
                        .push("POSTGRES_PORT must be a valid integer".to_string());
                    validation.is_valid = false;
                }
            }
        }
    }

    /// Validate cache configuration
    fn validate_cache_configuration(&self, validation: &mut StartupValidation) {
        let cache_type = std::env::var("CACHE_TYPE").unwrap_or_else(|_| "inmemory".to_string());

        // Validate cache type
        match cache_type.as_str() {
            "inmemory" | "redis" => {}
            _ => {
                validation.errors.push(format!(
                    "Invalid CACHE_TYPE '{}'. Must be 'inmemory' or 'redis'",
                    cache_type
                ));
                validation.is_valid = false;
            }
        }

        // Validate Redis configuration if using Redis cache
        if cache_type == "redis" {
            if std::env::var("REDIS_CONNECTION").is_err() {
                validation
                    .errors
                    .push("REDIS_CONNECTION required when CACHE_TYPE is 'redis'".to_string());
                validation.is_valid = false;
            } else if let Ok(redis_url) = std::env::var("REDIS_CONNECTION") {
                if !redis_url.starts_with("redis://") {
                    validation.errors.push(
                        "REDIS_CONNECTION must be a valid Redis URL starting with 'redis://'"
                            .to_string(),
                    );
                    validation.is_valid = false;
                }
            }
        }

        // Validate cache TTL
        if let Ok(ttl_str) = std::env::var("CACHE_TTL") {
            match ttl_str.parse::<u64>() {
                Ok(ttl) => {
                    if ttl == 0 {
                        validation.warnings.push(
                            "CACHE_TTL is set to 0, cache entries will expire immediately"
                                .to_string(),
                        );
                    }
                    if ttl > 86400 {
                        validation.warnings.push("CACHE_TTL is set to more than 24 hours, consider if this is intentional".to_string());
                    }
                }
                Err(_) => {
                    validation
                        .errors
                        .push("CACHE_TTL must be a valid integer (seconds)".to_string());
                    validation.is_valid = false;
                }
            }
        }
    }

    /// Validate storage configuration
    fn validate_storage_configuration(&self, validation: &mut StartupValidation) {
        // Check if storage is disabled
        if let Ok(disable_storage) = std::env::var("DISABLE_STORAGE") {
            match disable_storage.to_lowercase().as_str() {
                "true" | "false" => {}
                _ => {
                    validation
                        .errors
                        .push("DISABLE_STORAGE must be 'true' or 'false'".to_string());
                    validation.is_valid = false;
                }
            }

            // If storage is enabled, validate storage configuration
            if disable_storage.to_lowercase() != "true" {
                let required_storage_vars = vec![
                    "STORAGE_ENDPOINT",
                    "STORAGE_ACCESS_KEY",
                    "STORAGE_SECRET_KEY",
                    "STORAGE_BUCKET_NAME",
                ];

                for var in required_storage_vars {
                    if std::env::var(var).is_err() {
                        validation.errors.push(format!("Missing required storage variable: {} (required when DISABLE_STORAGE is not 'true')", var));
                        validation.is_valid = false;
                    }
                }

                // Validate storage endpoint URL
                if let Ok(endpoint) = std::env::var("STORAGE_ENDPOINT") {
                    if !endpoint.starts_with("http://") && !endpoint.starts_with("https://") {
                        validation.errors.push("STORAGE_ENDPOINT must be a valid URL starting with 'http://' or 'https://'".to_string());
                        validation.is_valid = false;
                    }
                }

                // Validate SSL verification setting
                if let Ok(disable_ssl) = std::env::var("STORAGE_DISABLE_SSL_VERIFICATION") {
                    match disable_ssl.to_lowercase().as_str() {
                        "true" | "false" => {}
                        _ => {
                            validation.errors.push(
                                "STORAGE_DISABLE_SSL_VERIFICATION must be 'true' or 'false'"
                                    .to_string(),
                            );
                            validation.is_valid = false;
                        }
                    }

                    if disable_ssl.to_lowercase() == "true" {
                        validation.warnings.push("SSL verification is disabled for storage, this may be insecure in production".to_string());
                    }
                }
            }
        }
    }

    /// Validate security configuration
    fn validate_security_configuration(&self, validation: &mut StartupValidation) {
        // Validate JWT secret
        if let Ok(jwt_secret) = std::env::var("JWT_SECRET") {
            if jwt_secret.len() < 32 {
                validation.errors.push(
                    "JWT_SECRET must be at least 32 characters long for security".to_string(),
                );
                validation.is_valid = false;
            }

            // Check for common weak secrets
            let weak_secrets = vec!["secret", "password", "123456", "admin", "test"];
            if weak_secrets
                .iter()
                .any(|&weak| jwt_secret.to_lowercase().contains(weak))
            {
                validation.warnings.push("JWT_SECRET appears to contain common weak patterns, consider using a stronger secret".to_string());
            }
        } else {
            validation
                .errors
                .push("JWT_SECRET is required for authentication".to_string());
            validation.is_valid = false;
        }

        // Validate session expiration
        if let Ok(session_expires) = std::env::var("SESSION_EXPIRES_IN") {
            // Basic validation for session expiration format (should end with 'd', 'h', 'm', or 's')
            if !session_expires.ends_with('d')
                && !session_expires.ends_with('h')
                && !session_expires.ends_with('m')
                && !session_expires.ends_with('s')
            {
                validation.warnings.push("SESSION_EXPIRES_IN should end with 'd' (days), 'h' (hours), 'm' (minutes), or 's' (seconds)".to_string());
            }
        }

        // Validate default sensitivity level
        if let Ok(sensitivity_str) = std::env::var("DEFAULT_SENSITIVITY_LEVEL") {
            match sensitivity_str.parse::<u32>() {
                Ok(level) => {
                    if level == 0 {
                        validation.warnings.push("DEFAULT_SENSITIVITY_LEVEL is set to 0, this may allow unrestricted access".to_string());
                    }
                }
                Err(_) => {
                    validation
                        .errors
                        .push("DEFAULT_SENSITIVITY_LEVEL must be a valid integer".to_string());
                    validation.is_valid = false;
                }
            }
        }
    }

    /// Validate sync configuration
    fn validate_sync_configuration(&self, validation: &mut StartupValidation) {
        // Validate sync enabled flag
        if let Ok(sync_enabled) = std::env::var("SYNC_ENABLED") {
            match sync_enabled.to_lowercase().as_str() {
                "true" | "false" => {}
                _ => {
                    validation
                        .errors
                        .push("SYNC_ENABLED must be 'true' or 'false'".to_string());
                    validation.is_valid = false;
                }
            }
        }

        // Validate sync timer
        if let Ok(timer_str) = std::env::var("SYNC_TIMER_MS") {
            match timer_str.parse::<u64>() {
                Ok(timer) => {
                    if timer < 1000 {
                        validation.warnings.push(
                            "SYNC_TIMER_MS is less than 1000ms, this may cause high CPU usage"
                                .to_string(),
                        );
                    }
                    if timer > 3600000 {
                        validation.warnings.push(
                            "SYNC_TIMER_MS is greater than 1 hour, sync may be too infrequent"
                                .to_string(),
                        );
                    }
                }
                Err(_) => {
                    validation
                        .errors
                        .push("SYNC_TIMER_MS must be a valid integer (milliseconds)".to_string());
                    validation.is_valid = false;
                }
            }
        }

        // Validate batch sync configuration
        if let Ok(batch_enabled) = std::env::var("BATCH_SYNC_ENABLED") {
            match batch_enabled.to_lowercase().as_str() {
                "true" | "false" => {}
                _ => {
                    validation
                        .errors
                        .push("BATCH_SYNC_ENABLED must be 'true' or 'false'".to_string());
                    validation.is_valid = false;
                }
            }

            if batch_enabled.to_lowercase() == "true" {
                // Validate batch sync type
                if let Ok(batch_type) = std::env::var("BATCH_SYNC_TYPE") {
                    match batch_type.as_str() {
                        "round-robin" | "weighted-round-robin" => {}
                        _ => {
                            validation.errors.push(
                                "BATCH_SYNC_TYPE must be 'round-robin' or 'weighted-round-robin'"
                                    .to_string(),
                            );
                            validation.is_valid = false;
                        }
                    }
                }

                // Validate batch size
                if let Ok(size_str) = std::env::var("BATCH_SYNC_SIZE") {
                    match size_str.parse::<u32>() {
                        Ok(size) => {
                            if size == 0 {
                                validation
                                    .errors
                                    .push("BATCH_SYNC_SIZE cannot be 0".to_string());
                                validation.is_valid = false;
                            }
                            if size > 10000 {
                                validation.warnings.push(
                                    "BATCH_SYNC_SIZE is very large, this may cause memory issues"
                                        .to_string(),
                                );
                            }
                        }
                        Err(_) => {
                            validation
                                .errors
                                .push("BATCH_SYNC_SIZE must be a valid integer".to_string());
                            validation.is_valid = false;
                        }
                    }
                }
            }
        }
    }

    /// Validate organization configuration
    fn validate_organization_configuration(&self, validation: &mut StartupValidation) {
        // Validate default organization ID format (should be ULID)
        if let Ok(org_id) = std::env::var("DEFAULT_ORGANIZATION_ID") {
            if org_id.len() != 26 {
                validation.warnings.push(
                    "DEFAULT_ORGANIZATION_ID should be 26 characters long (ULID format)"
                        .to_string(),
                );
            }

            // Basic ULID character validation
            if !org_id.chars().all(|c| c.is_ascii_alphanumeric()) {
                validation.warnings.push("DEFAULT_ORGANIZATION_ID should only contain alphanumeric characters (ULID format)".to_string());
            }
        }

        // Validate organization name
        if let Ok(org_name) = std::env::var("DEFAULT_ORGANIZATION_NAME") {
            if org_name.is_empty() {
                validation
                    .errors
                    .push("DEFAULT_ORGANIZATION_NAME cannot be empty".to_string());
                validation.is_valid = false;
            }

            if org_name.len() > 100 {
                validation.warnings.push(
                    "DEFAULT_ORGANIZATION_NAME is very long, consider shortening it".to_string(),
                );
            }
        }

        // Validate organization admin password
        if let Ok(admin_password) = std::env::var("DEFAULT_ORGANIZATION_ADMIN_PASSWORD") {
            if admin_password.len() < 8 {
                validation.errors.push(
                    "DEFAULT_ORGANIZATION_ADMIN_PASSWORD must be at least 8 characters long"
                        .to_string(),
                );
                validation.is_valid = false;
            }

            // Check for common weak passwords
            let weak_passwords = vec!["password", "admin", "123456", "qwerty"];
            if weak_passwords
                .iter()
                .any(|&weak| admin_password.to_lowercase().contains(weak))
            {
                validation.warnings.push("DEFAULT_ORGANIZATION_ADMIN_PASSWORD appears to be weak, consider using a stronger password".to_string());
            }
        }

        // Validate group ID format (should be ULID)
        if let Ok(group_id) = std::env::var("GROUP_ID") {
            if group_id.len() != 26 {
                validation
                    .warnings
                    .push("GROUP_ID should be 26 characters long (ULID format)".to_string());
            }
        }
    }

    /// Initialize core application services
    async fn initialize_core_services(
        &self,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        debug!("[STARTUP] Initializing logging and cache systems");

        // Initialize cache configuration
        let cache_type_str = std::env::var("CACHE_TYPE").unwrap_or_else(|_| "inmemory".to_string());
        let cache_type = CacheType::from_str(&cache_type_str).unwrap_or(CacheType::InMemory);
        let redis_connection = std::env::var("REDIS_CONNECTION").ok();
        let ttl = std::env::var("CACHE_TTL")
            .ok()
            .and_then(|ttl_str| ttl_str.parse::<u64>().ok())
            .map(Duration::from_secs);

        CacheConfig::init(cache_type.clone(), redis_connection, ttl);

        info!(
            "[STARTUP] Cache initialized with type: {:?}, TTL: {:?}",
            cache_type, ttl
        );

        // Verify cache is working
        let _ = cache.cache_type();
        debug!("[STARTUP] Cache system verified");

        Ok(())
    }

    /// Setup database and storage systems
    async fn setup_database_and_storage(
        &self,
    ) -> Result<
        (crate::database::db::AsyncDbPool, aws_sdk_s3::Client, String),
        Box<dyn std::error::Error + Send + Sync>,
    > {
        debug!("[STARTUP] Setting up database connection pool");

        // Initialize database pool
        let pool = db::establish_async_pool();
        info!("[STARTUP] Database connection pool established");

        debug!("[STARTUP] Initializing S3 storage");

        // Initialize S3 storage
        let (s3_client, bucket_name) = storage::initialize().await.map_err(|e| {
            error!("[STARTUP] Failed to initialize S3 storage: {}", e);
            e
        })?;

        info!(
            "[STARTUP] S3 storage initialized with bucket: {}",
            bucket_name
        );

        Ok((pool, s3_client, bucket_name))
    }

    /// Initialize background services
    async fn initialize_background_services(
        &self,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        debug!("[STARTUP] Initializing background services configuration");

        if let Err(e) = initialize(EInitializer::BACKGROUND_SERVICES_CONFIG, None).await {
            error!("[STARTUP] Failed to initialize background services: {}", e);
            return Err(e.into());
        }

        info!("[STARTUP] Background services configuration initialized");

        debug!("[STARTUP] Initializing transaction service");
        TransactionService::initialize().await;
        info!("[STARTUP] Transaction service initialized");

        debug!("[STARTUP] Initializing batch sync service");
        if let Err(e) = BatchSyncService::init().await {
            error!("[STARTUP] Failed to initialize batch sync service: {}", e);
            return Err(e.into());
        }
        info!("[STARTUP] Batch sync service initialized");

        debug!("[STARTUP] Initializing queue service");
        if let Err(e) = QueueService::init().await {
            error!("[STARTUP] Failed to initialize queue service: {}", e);
            return Err(e.into());
        }
        info!("[STARTUP] Queue service initialized");

        Ok(())
    }

    /// Setup messaging and listener services
    async fn setup_messaging_services(
        &self,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        debug!("[STARTUP] Initializing PostgreSQL listener service");

        if let Err(e) = PgListenerService::initialize().await {
            error!("[STARTUP] Failed to initialize PgListenerService: {}", e);
            return Err(e.into());
        }
        info!("[STARTUP] PostgreSQL listener service initialized");

        debug!("[STARTUP] Setting up message channel");

        // Initialize message sender
        let sender = create_message_channel();
        let arc_sender = Arc::new(sender);
        SENDER
            .set(arc_sender)
            .map_err(|_| "Failed to initialize message sender")?;

        info!("[STARTUP] Message channel established");

        Ok(())
    }
}

// Note: Default implementation removed as StartupManager requires parameters
