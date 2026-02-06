use crate::providers::storage::cache::cache_factory::CacheType;
use std::time::Duration;
/// Configuration structure for environment variables
#[allow(unused)]
pub struct EnvConfig {
    // Server Configuration
    pub host: String,
    pub port: String,
    pub grpc_port: String,
    pub grpc_url: String,
    pub socket_host: String,
    pub socket_port: String,
    pub rust_log: String,
    pub debug: bool,
    pub tz: String,

    // Database Configuration
    pub database_url: String,
    pub postgres_user: String,
    pub postgres_password: String,
    pub postgres_db: String,
    pub postgres_host: String,
    pub postgres_port: String,

    // Security
    pub jwt_secret: String,
    pub jwt_expires_in: String,
    pub cleanup_password: String,
    pub session_expires_in: String,
    pub default_sensitivity_level: u32,
    pub root_account_password: String,
    pub pgp_sym_key: String,

    // Organization Settings
    pub default_organization_id: String,
    pub default_organization_name: String,
    pub default_organization_admin_password: String,
    pub group_id: String,

    // Sync Configuration
    pub sync_enabled: bool,
    pub sync_timer_ms: u64,
    pub batch_sync_enabled: bool,
    pub batch_sync_type: String,
    pub batch_sync_size: u32,

    // Cache Configuration
    pub cache_type: CacheType,
    pub redis_connection: Option<String>,
    pub ttl: Option<Duration>,

    // Storage Configuration
    pub disable_storage: bool,
    pub storage_endpoint: String,
    pub storage_access_key: String,
    pub storage_secret_key: String,
    pub storage_bucket_name: String,
    pub storage_region: String,
    pub storage_disable_ssl_verification: bool,

    // Search suggestion
    pub default_search_pattern: String,
    pub search_suggestion_cache_ttl_ms: u64,

    // Log
    pub log_file_path: String,

    // Code Generation
    pub generate_proto: bool,
    pub generate_grpc: bool,
    pub generate_table_enum: bool,
    pub create_schema: bool,

    // Session Management
    pub session_cookie_name: String,
    pub session_cookie_max_age: u64,
    pub session_header_name: String,
    pub session_prune_interval: u64,
    pub session_cookie_secure: bool,

    // Device & Identity
    pub default_device_id: String,
    pub default_device_secret: String,
    pub default_organization_admin_email: String,
    pub super_admin_id: String,
    pub system_device_ulid: String,
    pub initialize_device: bool,

    // System Configuration
    pub experimental_permissions: bool,
    pub initialize_entity_data: bool,
    pub merkle_save_interval: u64,
    pub max_concurrent_flushes: u32,
    pub bucket_capacity: u32,

    // Test
    pub strict_validation: bool,
}

impl Default for EnvConfig {
    fn default() -> Self {
        let cache_type = std::env::var("CACHE_TYPE")
            .unwrap_or_else(|_| "inmemory".to_string())
            .trim()
            .to_lowercase();

        let cache_type = match cache_type.as_str() {
            "redis" => CacheType::Redis,
            _ => CacheType::InMemory,
        };

        let redis_connection = std::env::var("REDIS_CONNECTION")
            .ok()
            .filter(|s| !s.trim().is_empty());

        let ttl = std::env::var("CACHE_TTL")
            .ok()
            .and_then(|s| s.parse::<u64>().ok())
            .map(Duration::from_secs);

        Self {
            // Server Configuration
            host: std::env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string()),
            port: std::env::var("PORT").unwrap_or_else(|_| "5000".to_string()),
            grpc_port: std::env::var("GRPC_PORT").unwrap_or_else(|_| "9000".to_string()),
            grpc_url: std::env::var("GRPC_URL").unwrap_or_else(|_| "0.0.0.0".to_string()),
            socket_host: std::env::var("SOCKET_HOST").unwrap_or_else(|_| "0.0.0.0".to_string()),
            socket_port: std::env::var("SOCKET_PORT").unwrap_or_else(|_| "3001".to_string()),
            rust_log: std::env::var("RUST_LOG").unwrap_or_else(|_| "info".to_string()),
            debug: std::env::var("DEBUG")
                .unwrap_or_else(|_| "false".to_string())
                .parse()
                .unwrap_or(false),
            tz: std::env::var("TZ").unwrap_or_else(|_| "UTC".to_string()),

            // Database Configuration
            database_url: std::env::var("DATABASE_URL")
                .unwrap_or_else(|_| "postgres://admin:admin@localhost:5432/datastore".to_string()),
            postgres_user: std::env::var("POSTGRES_USER")
                .unwrap_or_else(|_| "admin".to_string()),
            postgres_password: std::env::var("POSTGRES_PASSWORD")
                .unwrap_or_else(|_| "admin".to_string()),
            postgres_db: std::env::var("POSTGRES_DB").unwrap_or_else(|_| "store".to_string()),
            postgres_host: std::env::var("POSTGRES_HOST")
                .unwrap_or_else(|_| "localhost".to_string()),
            postgres_port: std::env::var("POSTGRES_PORT").unwrap_or_else(|_| "5432".to_string()),

            // Security
            jwt_secret: std::env::var("JWT_SECRET")
                .unwrap_or_else(|_| "Zt7mK#9xL$4nP@2vR8wY5hB3jF6qN9cX7A1B".to_string()),
            jwt_expires_in: std::env::var("JWT_EXPIRES_IN").unwrap_or_else(|_| "24h".to_string()),
            cleanup_password: std::env::var("CLEANUP_PASSWORD")
                .unwrap_or_else(|_| "admin".to_string()),
            session_expires_in: std::env::var("SESSION_EXPIRES_IN")
                .unwrap_or_else(|_| "7d".to_string()),
            default_sensitivity_level: std::env::var("DEFAULT_SENSITIVITY_LEVEL")
                .unwrap_or_else(|_| "1000".to_string())
                .parse()
                .unwrap_or(1000),
            root_account_password: std::env::var("ROOT_ACCOUNT_PASSWORD")
                .unwrap_or_else(|_| "pl3@s3ch@ng3m3!!".to_string()),
            pgp_sym_key: std::env::var("PGP_SYM_KEY").unwrap_or_else(|_| "".to_string()),

            // Organization Settings
            default_organization_id: std::env::var("DEFAULT_ORGANIZATION_ID")
                .unwrap_or_else(|_| "01JBHKXHYSKPP247HZZWHA3JCT".to_string()),
            default_organization_name: std::env::var("DEFAULT_ORGANIZATION_NAME")
                .unwrap_or_else(|_| "global-organization".to_string()),
            default_organization_admin_password: std::env::var(
                "DEFAULT_ORGANIZATION_ADMIN_PASSWORD",
            )
            .unwrap_or_else(|_| "ch@ng3m3Pl3@s3!!".to_string()),
            group_id: std::env::var("GROUP_ID")
                .unwrap_or_else(|_| "01JBHKXHYSKPP247HZZWHA3JBT".to_string()),

            // Sync Configuration
            sync_enabled: std::env::var("SYNC_ENABLED")
                .unwrap_or_else(|_| "false".to_string())
                .parse()
                .unwrap_or(false),
            sync_timer_ms: std::env::var("SYNC_TIMER_MS")
                .unwrap_or_else(|_| "60000".to_string())
                .parse()
                .unwrap_or(60000),
            batch_sync_enabled: std::env::var("BATCH_SYNC_ENABLED")
                .unwrap_or_else(|_| "false".to_string())
                .parse()
                .unwrap_or(false),
            batch_sync_type: std::env::var("BATCH_SYNC_TYPE")
                .unwrap_or_else(|_| "round-robin".to_string()),
            batch_sync_size: std::env::var("BATCH_SYNC_SIZE")
                .unwrap_or_else(|_| "100".to_string())
                .parse()
                .unwrap_or(100),

            // Cache Configuration
            cache_type,
            redis_connection,
            ttl,

            // Storage Configuration
            disable_storage: std::env::var("DISABLE_STORAGE")
                .unwrap_or_else(|_| "false".to_string())
                .parse()
                .unwrap_or(false),
            storage_endpoint: std::env::var("STORAGE_ENDPOINT")
                .unwrap_or_else(|_| "https://mastra-minio-api.app.dnaqa.net".to_string()),
            storage_access_key: std::env::var("STORAGE_ACCESS_KEY")
                .unwrap_or_else(|_| "minioadmin".to_string()),
            storage_secret_key: std::env::var("STORAGE_SECRET_KEY")
                .unwrap_or_else(|_| "uA633fr9F9hQ".to_string()),
            storage_bucket_name: std::env::var("STORAGE_BUCKET_NAME")
                .unwrap_or_else(|_| "global-organization".to_string()),
            storage_region: std::env::var("STORAGE_REGION")
                .unwrap_or_else(|_| "us-east-1".to_string()),
            storage_disable_ssl_verification: std::env::var("STORAGE_DISABLE_SSL_VERIFICATION")
                .unwrap_or_else(|_| "true".to_string())
                .parse()
                .unwrap_or(true),

            // Search suggestion
            default_search_pattern: std::env::var("DEFAULT_SEARCH_PATTERN")
                .unwrap_or_else(|_| "contains".to_string()),
            search_suggestion_cache_ttl_ms: std::env::var("SEARCH_SUGGESTION_CACHE_TTL_MS")
                .unwrap_or_else(|_| "30000".to_string())
                .parse()
                .unwrap_or(30000),

            // Log
            log_file_path: std::env::var("LOG_FILE_PATH")
                .unwrap_or_else(|_| "logs/lifecycle.log".to_string()),

            // Code Generation Configuration
            generate_proto: std::env::var("GENERATE_PROTO").unwrap_or_else(|_| "false".to_string())
                == "true",
            generate_grpc: std::env::var("GENERATE_GRPC").unwrap_or_else(|_| "false".to_string())
                == "true",
            generate_table_enum: std::env::var("GENERATE_TABLE_ENUM")
                .unwrap_or_else(|_| "false".to_string())
                == "true",
            create_schema: std::env::var("CREATE_SCHEMA").unwrap_or_else(|_| "false".to_string())
                == "true",

            // Session Management
            session_cookie_name: std::env::var("SESSION_COOKIE_NAME")
                .unwrap_or_else(|_| "session_id".to_string()),
            session_cookie_max_age: std::env::var("SESSION_COOKIE_MAX_AGE")
                .unwrap_or_else(|_| "86400".to_string())
                .parse()
                .unwrap_or(86400),
            session_header_name: std::env::var("SESSION_HEADER_NAME")
                .unwrap_or_else(|_| "X-Session-ID".to_string()),
            session_prune_interval: std::env::var("SESSION_PRUNE_INTERVAL")
                .unwrap_or_else(|_| "3600".to_string())
                .parse()
                .unwrap_or(3600),
            session_cookie_secure: std::env::var("SESSION_COOKIE_SECURE")
                .unwrap_or_else(|_| "false".to_string())
                .parse()
                .unwrap_or(false),

            // Device & Identity
            default_device_id: std::env::var("DEFAULT_DEVICE_ID")
                .unwrap_or_else(|_| "default_device".to_string()),
            default_device_secret: std::env::var("DEFAULT_DEVICE_SECRET")
                .unwrap_or_else(|_| "default_secret".to_string()),
            default_organization_admin_email: std::env::var("DEFAULT_ORGANIZATION_ADMIN_EMAIL")
                .unwrap_or_else(|_| "admin@example.com".to_string()),
            super_admin_id: std::env::var("SUPER_ADMIN_ID")
                .unwrap_or_else(|_| "super_admin".to_string()),
            system_device_ulid: std::env::var("SYSTEM_DEVICE_ULID")
                .unwrap_or_else(|_| "01ARZ3NDEKTSV4RRFFQ69G5FAV".to_string()),
            initialize_device: std::env::var("INITIALIZE_DEVICE")
                .unwrap_or_else(|_| "false".to_string())
                .parse()
                .unwrap_or(false),

            // System Configuration
            experimental_permissions: std::env::var("EXPERIMENTAL_PERMISSIONS")
                .unwrap_or_else(|_| "false".to_string())
                .parse()
                .unwrap_or(false),
            initialize_entity_data: std::env::var("INITIALIZE_ENTITY_DATA")
                .unwrap_or_else(|_| "false".to_string())
                .parse()
                .unwrap_or(false),
            merkle_save_interval: std::env::var("MERKLE_SAVE_INTERVAL")
                .unwrap_or_else(|_| "300".to_string())
                .parse()
                .unwrap_or(300),
            max_concurrent_flushes: std::env::var("MAX_CONCURRENT_FLUSHES")
                .unwrap_or_else(|_| "10".to_string())
                .parse()
                .unwrap_or(10),
            bucket_capacity: std::env::var("BUCKET_CAPACITY")
                .unwrap_or_else(|_| "1000".to_string())
                .parse()
                .unwrap_or(1000),

            // Test
            strict_validation: std::env::var("STRICT_VALIDATION")
                .unwrap_or_else(|_| "false".to_string())
                .parse()
                .unwrap_or(false),
        }
    }
}
