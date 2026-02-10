use crate::config::core::EnvConfig;
use crate::database::db;
use crate::initializers::system_initialization::init::initialize;
use crate::initializers::system_initialization::structs::EInitializer;
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
    config: Arc<EnvConfig>,
}

impl StartupManager {
    /// Create a new startup manager
    pub fn new(
        state_manager: Arc<crate::lifecycle::state::StateManager>,
        logger: Arc<crate::lifecycle::logging::LifecycleLogger>,
        config: Arc<EnvConfig>,
    ) -> Self {
        info!("[STARTUP] Initializing startup manager");
        Self {
            start_time: None,
            pool: None,
            s3_client: None,
            bucket_name: None,
            state_manager,
            logger,
            config,
        }
    }

    /// Execute the complete startup sequence
    pub async fn execute(&mut self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        self.start_time = Some(Instant::now());

        // Update StartupManager status to starting
        self.state_manager
            .update_component_status(
                "StartupManager",
                crate::lifecycle::state::ComponentStatus::Starting,
            )
            .await;

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

        // Phase 3.5: Schema generation (if enabled)

        self.logger
            .log(
                LogLevel::Info,
                LogCategory::Startup,
                "StartupManager",
                "Phase 3.5: Generating application schema",
            )
            .await;

        if let Err(e) = crate::initializers::system_initialization::init::initialize(
                crate::initializers::system_initialization::structs::EInitializer::GENERATE_SCHEMA_CONFIG,
                None,
            ).await {
                self.logger
                    .log(
                        LogLevel::Error,
                        LogCategory::Startup,
                        "StartupManager",
                        &format!("Schema generation failed: {}", e),
                    )
                    .await;
                return Err(e.into());
            }

        self.logger
            .log(
                LogLevel::Info,
                LogCategory::Startup,
                "StartupManager",
                "Schema generation completed successfully",
            )
            .await;

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

        // Update StartupManager status to running after successful completion
        self.state_manager
            .update_component_status(
                "StartupManager",
                crate::lifecycle::state::ComponentStatus::Running,
            )
            .await;

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
        // Check DATABASE_URL from centralized config
        let config = EnvConfig::default();
        if config.database_url.is_empty()
            && (config.postgres_user.is_empty()
                || config.postgres_password.is_empty()
                || config.postgres_host.is_empty()
                || config.postgres_port.is_empty()
                || config.postgres_db.is_empty())
        {
            validation.errors.push(
                "Missing required database configuration: DATABASE_URL or POSTGRES_* variables"
                    .to_string(),
            );
            validation.is_valid = false;
        }

        // Validate that config has required values (these are now guaranteed by EnvConfig)
        if self.config.host.is_empty() {
            validation
                .errors
                .push("HOST configuration is empty".to_string());
            validation.is_valid = false;
        }

        if self.config.port.is_empty() {
            validation
                .errors
                .push("PORT configuration is empty".to_string());
            validation.is_valid = false;
        }

        // Log configuration values being used
        info!("[STARTUP] Using HOST: {}", self.config.host);
        info!("[STARTUP] Using PORT: {}", self.config.port);
        info!("[STARTUP] Using GRPC_PORT: {}", self.config.grpc_port);
        info!("[STARTUP] Using SOCKET_PORT: {}", self.config.socket_port);
    }

    /// Validate port configurations
    fn validate_port_configurations(&self, validation: &mut StartupValidation) {
        // Validate main HTTP port
        match self.config.port.parse::<u16>() {
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

        // Validate gRPC port
        match self.config.grpc_port.parse::<u16>() {
            Ok(grpc_port) => {
                if grpc_port < 1024 {
                    validation.warnings.push(
                        "GRPC_PORT is set to a privileged port (<1024), ensure proper permissions"
                            .to_string(),
                    );
                }

                // Check for port conflicts
                if let Ok(http_port) = self.config.port.parse::<u16>() {
                    if grpc_port == http_port {
                        validation
                            .errors
                            .push("GRPC_PORT cannot be the same as PORT".to_string());
                        validation.is_valid = false;
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

        // Validate socket port
        match self.config.socket_port.parse::<u16>() {
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

    /// Validate database configuration
    fn validate_database_configuration(&self, validation: &mut StartupValidation) {
        // Validate DATABASE_URL format
        if !self.config.database_url.is_empty() {
            if !self.config.database_url.starts_with("postgres://")
                && !self.config.database_url.starts_with("postgresql://")
            {
                validation.errors.push("DATABASE_URL must be a valid PostgreSQL connection string starting with 'postgres://' or 'postgresql://'".to_string());
                validation.is_valid = false;
            }

            // Basic URL validation
            if !self.config.database_url.contains("@") || !self.config.database_url.contains("/") {
                validation.errors.push(
                    "DATABASE_URL appears to be malformed (missing @ or / characters)".to_string(),
                );
                validation.is_valid = false;
            }
        }

        // Validate individual PostgreSQL components if provided
        match self.config.postgres_port.parse::<u16>() {
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

    /// Validate cache configuration
    fn validate_cache_configuration(&self, validation: &mut StartupValidation) {
        // Validate Redis configuration if using Redis cache
        match self.config.cache_type {
            CacheType::Redis => {
                if self.config.redis_connection.is_none() {
                    validation
                        .errors
                        .push("REDIS_CONNECTION required when CACHE_TYPE is 'redis'".to_string());
                    validation.is_valid = false;
                } else if let Some(ref redis_url) = self.config.redis_connection {
                    if !redis_url.starts_with("redis://") {
                        validation.errors.push(
                            "REDIS_CONNECTION must be a valid Redis URL starting with 'redis://'"
                                .to_string(),
                        );
                        validation.is_valid = false;
                    }
                }
            }
            CacheType::InMemory => {
                // InMemory cache doesn't require additional validation
            }
        }

        // Validate cache TTL
        if let Some(ttl) = self.config.ttl {
            let ttl_secs = ttl.as_secs();
            if ttl_secs == 0 {
                validation.warnings.push(
                    "CACHE_TTL is set to 0, cache entries will expire immediately".to_string(),
                );
            }
            if ttl_secs > 86400 {
                validation.warnings.push(
                    "CACHE_TTL is set to more than 24 hours, consider if this is intentional"
                        .to_string(),
                );
            }
        }
    }

    /// Validate storage configuration
    fn validate_storage_configuration(&self, validation: &mut StartupValidation) {
        // Check if storage is disabled
        if !self.config.disable_storage {
            // Storage is enabled, validate storage configuration
            let storage_vars = [
                ("STORAGE_ENDPOINT", &self.config.storage_endpoint),
                ("STORAGE_ACCESS_KEY", &self.config.storage_access_key),
                ("STORAGE_SECRET_KEY", &self.config.storage_secret_key),
                ("STORAGE_BUCKET_NAME", &self.config.storage_bucket_name),
            ];

            for (var_name, var_value) in storage_vars {
                if var_value.is_empty() {
                    validation.errors.push(format!("Missing required storage variable: {} (required when DISABLE_STORAGE is not 'true')", var_name));
                    validation.is_valid = false;
                }
            }

            // Validate storage endpoint URL
            if !self.config.storage_endpoint.is_empty() {
                if !self.config.storage_endpoint.starts_with("http://")
                    && !self.config.storage_endpoint.starts_with("https://")
                {
                    validation.errors.push("STORAGE_ENDPOINT must be a valid URL starting with 'http://' or 'https://'".to_string());
                    validation.is_valid = false;
                }
            }

            // Validate SSL verification setting
            if self.config.storage_disable_ssl_verification {
                validation.warnings.push(
                    "SSL verification is disabled for storage, this may be insecure in production"
                        .to_string(),
                );
            }
        }
    }

    /// Validate security configuration
    fn validate_security_configuration(&self, validation: &mut StartupValidation) {
        // Validate JWT secret
        if !self.config.jwt_secret.is_empty() {
            if self.config.jwt_secret.len() < 32 {
                validation.warnings.push(
                    "JWT_SECRET must be at least 32 characters long for security".to_string(),
                );
            }

            // Check for common weak secrets
            let weak_secrets = vec!["secret", "password", "123456", "admin", "test"];
            if weak_secrets
                .iter()
                .any(|&weak| self.config.jwt_secret.to_lowercase().contains(weak))
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
        if !self.config.session_expires_in.is_empty() {
            // Basic validation for session expiration format (should end with 'd', 'h', 'm', or 's')
            if !self.config.session_expires_in.ends_with('d')
                && !self.config.session_expires_in.ends_with('h')
                && !self.config.session_expires_in.ends_with('m')
                && !self.config.session_expires_in.ends_with('s')
            {
                validation.warnings.push("SESSION_EXPIRES_IN should end with 'd' (days), 'h' (hours), 'm' (minutes), or 's' (seconds)".to_string());
            }
        }

        // Validate default sensitivity level
        if self.config.default_sensitivity_level == 0 {
            validation.warnings.push(
                "DEFAULT_SENSITIVITY_LEVEL is set to 0, this may allow unrestricted access"
                    .to_string(),
            );
        }
    }

    /// Validate sync configuration
    fn validate_sync_configuration(&self, validation: &mut StartupValidation) {
        // Validate sync timer
        let timer = self.config.sync_timer_ms;
        if timer < 1000 {
            validation.warnings.push(
                "SYNC_TIMER_MS is less than 1000ms, this may cause high CPU usage".to_string(),
            );
        }
        if timer > 3600000 {
            validation.warnings.push(
                "SYNC_TIMER_MS is greater than 1 hour, sync may be too infrequent".to_string(),
            );
        }

        // Validate batch sync configuration
        if self.config.batch_sync_enabled {
            // Validate batch sync type
            if !self.config.batch_sync_type.is_empty() {
                match self.config.batch_sync_type.as_str() {
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
            let size = self.config.batch_sync_size;
            if size == 0 {
                validation
                    .errors
                    .push("BATCH_SYNC_SIZE cannot be 0".to_string());
                validation.is_valid = false;
            }
            if size > 10000 {
                validation.warnings.push(
                    "BATCH_SYNC_SIZE is very large, this may cause memory issues".to_string(),
                );
            }
        }
    }

    /// Validate organization configuration
    fn validate_organization_configuration(&self, validation: &mut StartupValidation) {
        // Validate default organization ID format (should be ULID)
        if !self.config.default_organization_id.is_empty() {
            if self.config.default_organization_id.len() != 26 {
                validation.warnings.push(
                    "DEFAULT_ORGANIZATION_ID should be 26 characters long (ULID format)"
                        .to_string(),
                );
            }

            // Basic ULID character validation
            if !self
                .config
                .default_organization_id
                .chars()
                .all(|c| c.is_ascii_alphanumeric())
            {
                validation.warnings.push("DEFAULT_ORGANIZATION_ID should only contain alphanumeric characters (ULID format)".to_string());
            }
        }

        // Validate organization name
        if !self.config.default_organization_name.is_empty() {
            if self.config.default_organization_name.len() > 100 {
                validation.warnings.push(
                    "DEFAULT_ORGANIZATION_NAME is very long, consider shortening it".to_string(),
                );
            }
        }

        // Validate organization admin password
        if !self.config.default_organization_admin_password.is_empty() {
            if self.config.default_organization_admin_password.len() < 8 {
                validation.errors.push(
                    "DEFAULT_ORGANIZATION_ADMIN_PASSWORD must be at least 8 characters long"
                        .to_string(),
                );
                validation.is_valid = false;
            }

            // Check for common weak passwords
            let weak_passwords = vec!["password", "admin", "123456", "qwerty"];
            if weak_passwords.iter().any(|&weak| {
                self.config
                    .default_organization_admin_password
                    .to_lowercase()
                    .contains(weak)
            }) {
                validation.warnings.push("DEFAULT_ORGANIZATION_ADMIN_PASSWORD appears to be weak, consider using a stronger password".to_string());
            }
        }

        // Validate group ID format (should be ULID)
        if !self.config.group_id.is_empty() {
            if self.config.group_id.len() != 26 {
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

        // Update StartupManager status to starting
        self.state_manager
            .update_component_status(
                "StartupManager",
                crate::lifecycle::state::ComponentStatus::Starting,
            )
            .await;

        // Initialize cache configuration
        let cache_type = self.config.cache_type.clone();
        let redis_connection = self.config.redis_connection.clone();
        let ttl = self.config.ttl;

        CacheConfig::init(cache_type.clone(), redis_connection, ttl);

        info!(
            "[STARTUP] Cache initialized with type: {:?}, TTL: {:?}",
            cache_type, ttl
        );

        // Verify cache is working
        let _ = cache.cache_type();
        debug!("[STARTUP] Cache system verified");

        // Update StartupManager status to running
        self.state_manager
            .update_component_status(
                "StartupManager",
                crate::lifecycle::state::ComponentStatus::Running,
            )
            .await;

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

        // Update DatabasePool status to starting
        self.state_manager
            .update_component_status(
                "DatabasePool",
                crate::lifecycle::state::ComponentStatus::Starting,
            )
            .await;

        // Initialize database pool
        let pool = db::establish_async_pool();
        info!("[STARTUP] Database connection pool established");

        // Update DatabasePool status to running
        self.state_manager
            .update_component_status(
                "DatabasePool",
                crate::lifecycle::state::ComponentStatus::Running,
            )
            .await;

        debug!("[STARTUP] Initializing S3 storage");

        // Update S3Client status to starting
        self.state_manager
            .update_component_status(
                "S3Client",
                crate::lifecycle::state::ComponentStatus::Starting,
            )
            .await;

        // Initialize S3 storage
        let (s3_client, bucket_name) = storage::initialize().await.map_err(|e| {
            error!("[STARTUP] Failed to initialize S3 storage: {}", e);
            e
        })?;

        info!(
            "[STARTUP] S3 storage initialized with bucket: {}",
            bucket_name
        );

        // Update S3Client status to running
        self.state_manager
            .update_component_status(
                "S3Client",
                crate::lifecycle::state::ComponentStatus::Running,
            )
            .await;

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
