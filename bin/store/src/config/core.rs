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
    pub database_pool_size: usize,
    pub db_pool_wait_timeout_ms: u64,
    pub db_pool_create_timeout_ms: u64,
    pub db_pool_recycle_timeout_ms: u64,
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
    pub temporary_file_ttl_secs: u64,

    // Search suggestion
    pub default_search_pattern: String,
    pub search_suggestion_cache_ttl_ms: u64,
    pub find_cache_ttl_ms: u64,

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
    pub prometheus_base_url: String,
    pub server_keep_alive_secs: u64,
    pub server_workers: Option<usize>,
    pub insert_queue_capacity: usize,
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
            database_pool_size: std::env::var("DATABASE_POOL_SIZE")
                .unwrap_or_else(|_| "20".to_string())
                .parse()
                .unwrap_or(20),
            db_pool_wait_timeout_ms: std::env::var("DB_POOL_WAIT_TIMEOUT_MS")
                .unwrap_or_else(|_| "15000".to_string())
                .parse()
                .unwrap_or(15_000),
            db_pool_create_timeout_ms: std::env::var("DB_POOL_CREATE_TIMEOUT_MS")
                .unwrap_or_else(|_| "10000".to_string())
                .parse()
                .unwrap_or(10_000),
            db_pool_recycle_timeout_ms: std::env::var("DB_POOL_RECYCLE_TIMEOUT_MS")
                .unwrap_or_else(|_| "5000".to_string())
                .parse()
                .unwrap_or(5_000),
            postgres_user: std::env::var("POSTGRES_USER").unwrap_or_else(|_| "admin".to_string()),
            postgres_password: std::env::var("POSTGRES_PASSWORD")
                .unwrap_or_else(|_| "admin".to_string()),
            postgres_db: std::env::var("POSTGRES_DB").unwrap_or_else(|_| "store".to_string()),
            postgres_host: std::env::var("POSTGRES_HOST")
                .unwrap_or_else(|_| "localhost".to_string()),
            postgres_port: std::env::var("POSTGRES_PORT").unwrap_or_else(|_| "5432".to_string()),

            // Security
            // ! JWT webtoken does not allow '$' in secret
            jwt_secret: std::env::var("JWT_SECRET")
                .unwrap_or_else(|_| "Zt7mK#9xL4nP@2vR8wY5hB3jF6qN9cX7A1B".to_string()),
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
                .unwrap_or_else(|_| "30000".to_string())
                .parse()
                .unwrap_or(30000),
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
            temporary_file_ttl_secs: std::env::var("TEMPORARY_FILE_TTL_IN_SECS")
                .unwrap_or_else(|_| "300".to_string())
                .parse()
                .unwrap_or(300),

            // Search suggestion
            default_search_pattern: std::env::var("DEFAULT_SEARCH_PATTERN")
                .unwrap_or_else(|_| "contains".to_string()),
            search_suggestion_cache_ttl_ms: std::env::var("SEARCH_SUGGESTION_CACHE_TTL_MS")
                .unwrap_or_else(|_| "30000".to_string())
                .parse()
                .unwrap_or(30000),
            find_cache_ttl_ms: std::env::var("FIND_CACHE_TTL_MS")
                .unwrap_or_else(|_| "300000".to_string())
                .parse()
                .unwrap_or(300000),

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
            super_admin_id: std::env::var("SUPER_ADMIN_ID").unwrap_or_else(|_| "".to_string()),
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
                .unwrap_or_else(|_| "30000".to_string())
                .parse()
                .unwrap_or(30000),
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
            prometheus_base_url: std::env::var("PROMETHEUS_BASE_URL")
                .unwrap_or_else(|_| "http://prometheus:9090".to_string()),
            server_keep_alive_secs: std::env::var("KEEP_ALIVE_SECS")
                .unwrap_or_else(|_| "30".to_string())
                .parse()
                .unwrap_or(30),
            server_workers: std::env::var("WORKERS")
                .ok()
                .and_then(|s| s.parse::<usize>().ok()),
            insert_queue_capacity: std::env::var("INSERT_QUEUE_CAPACITY")
                .unwrap_or_else(|_| "100".to_string())
                .parse()
                .unwrap_or(100),
        }
    }
}

impl EnvConfig {
    pub fn render_loaded_env_table(&self) -> String {
        let categories: Vec<(&str, Vec<(&str, String)>)> = vec![
            (
                "Server Configuration",
                vec![
                    ("HOST", self.host.clone()),
                    ("PORT", self.port.clone()),
                    ("GRPC_PORT", self.grpc_port.clone()),
                    ("GRPC_URL", self.grpc_url.clone()),
                    ("SOCKET_HOST", self.socket_host.clone()),
                    ("SOCKET_PORT", self.socket_port.clone()),
                    ("RUST_LOG", self.rust_log.clone()),
                    ("DEBUG", self.debug.to_string()),
                    ("TZ", self.tz.clone()),
                    ("KEEP_ALIVE_SECS", self.server_keep_alive_secs.to_string()),
                    (
                        "WORKERS",
                        self.server_workers
                            .map(|workers| workers.to_string())
                            .unwrap_or_else(|| "<unset>".to_string()),
                    ),
                ],
            ),
            (
                "Database Configuration",
                vec![
                    ("DATABASE_URL", Self::mask_secret(&self.database_url)),
                    ("DATABASE_POOL_SIZE", self.database_pool_size.to_string()),
                    (
                        "DB_POOL_WAIT_TIMEOUT_MS",
                        self.db_pool_wait_timeout_ms.to_string(),
                    ),
                    (
                        "DB_POOL_CREATE_TIMEOUT_MS",
                        self.db_pool_create_timeout_ms.to_string(),
                    ),
                    (
                        "DB_POOL_RECYCLE_TIMEOUT_MS",
                        self.db_pool_recycle_timeout_ms.to_string(),
                    ),
                    ("POSTGRES_USER", self.postgres_user.clone()),
                    ("POSTGRES_PASSWORD", Self::mask_secret(&self.postgres_password)),
                    ("POSTGRES_DB", self.postgres_db.clone()),
                    ("POSTGRES_HOST", self.postgres_host.clone()),
                    ("POSTGRES_PORT", self.postgres_port.clone()),
                ],
            ),
            (
                "Security",
                vec![
                    ("JWT_SECRET", Self::mask_secret(&self.jwt_secret)),
                    ("JWT_EXPIRES_IN", self.jwt_expires_in.clone()),
                    ("CLEANUP_PASSWORD", Self::mask_secret(&self.cleanup_password)),
                    ("SESSION_EXPIRES_IN", self.session_expires_in.clone()),
                    (
                        "DEFAULT_SENSITIVITY_LEVEL",
                        self.default_sensitivity_level.to_string(),
                    ),
                    (
                        "ROOT_ACCOUNT_PASSWORD",
                        Self::mask_secret(&self.root_account_password),
                    ),
                    ("PGP_SYM_KEY", Self::mask_secret(&self.pgp_sym_key)),
                ],
            ),
            (
                "Organization Settings",
                vec![
                    (
                        "DEFAULT_ORGANIZATION_ID",
                        self.default_organization_id.clone(),
                    ),
                    (
                        "DEFAULT_ORGANIZATION_NAME",
                        self.default_organization_name.clone(),
                    ),
                    (
                        "DEFAULT_ORGANIZATION_ADMIN_PASSWORD",
                        Self::mask_secret(&self.default_organization_admin_password),
                    ),
                    ("GROUP_ID", self.group_id.clone()),
                ],
            ),
            (
                "Sync Configuration",
                vec![
                    ("SYNC_ENABLED", self.sync_enabled.to_string()),
                    ("SYNC_TIMER_MS", self.sync_timer_ms.to_string()),
                    ("BATCH_SYNC_ENABLED", self.batch_sync_enabled.to_string()),
                    ("BATCH_SYNC_TYPE", self.batch_sync_type.clone()),
                    ("BATCH_SYNC_SIZE", self.batch_sync_size.to_string()),
                ],
            ),
            (
                "Cache Configuration",
                vec![
                    ("CACHE_TYPE", format!("{:?}", self.cache_type)),
                    (
                        "REDIS_CONNECTION",
                        self.redis_connection
                            .as_deref()
                            .map(Self::mask_secret)
                            .unwrap_or_else(|| "<unset>".to_string()),
                    ),
                    (
                        "CACHE_TTL",
                        self.ttl
                            .map(|d| format!("{}s", d.as_secs()))
                            .unwrap_or_else(|| "<unset>".to_string()),
                    ),
                ],
            ),
            (
                "Storage Configuration",
                vec![
                    ("DISABLE_STORAGE", self.disable_storage.to_string()),
                    ("STORAGE_ENDPOINT", self.storage_endpoint.clone()),
                    (
                        "STORAGE_ACCESS_KEY",
                        Self::mask_secret(&self.storage_access_key),
                    ),
                    (
                        "STORAGE_SECRET_KEY",
                        Self::mask_secret(&self.storage_secret_key),
                    ),
                    ("STORAGE_BUCKET_NAME", self.storage_bucket_name.clone()),
                    ("STORAGE_REGION", self.storage_region.clone()),
                    (
                        "STORAGE_DISABLE_SSL_VERIFICATION",
                        self.storage_disable_ssl_verification.to_string(),
                    ),
                    (
                        "TEMPORARY_FILE_TTL_IN_SECS",
                        self.temporary_file_ttl_secs.to_string(),
                    ),
                ],
            ),
            (
                "Search Suggestion",
                vec![
                    ("DEFAULT_SEARCH_PATTERN", self.default_search_pattern.clone()),
                    (
                        "SEARCH_SUGGESTION_CACHE_TTL_MS",
                        self.search_suggestion_cache_ttl_ms.to_string(),
                    ),
                    ("FIND_CACHE_TTL_MS", self.find_cache_ttl_ms.to_string()),
                ],
            ),
            ("Log", vec![("LOG_FILE_PATH", self.log_file_path.clone())]),
            (
                "Code Generation",
                vec![
                    ("GENERATE_PROTO", self.generate_proto.to_string()),
                    ("GENERATE_GRPC", self.generate_grpc.to_string()),
                    ("GENERATE_TABLE_ENUM", self.generate_table_enum.to_string()),
                    ("CREATE_SCHEMA", self.create_schema.to_string()),
                ],
            ),
            (
                "Session Management",
                vec![
                    ("SESSION_COOKIE_NAME", self.session_cookie_name.clone()),
                    (
                        "SESSION_COOKIE_MAX_AGE",
                        self.session_cookie_max_age.to_string(),
                    ),
                    ("SESSION_HEADER_NAME", self.session_header_name.clone()),
                    (
                        "SESSION_PRUNE_INTERVAL",
                        self.session_prune_interval.to_string(),
                    ),
                    ("SESSION_COOKIE_SECURE", self.session_cookie_secure.to_string()),
                ],
            ),
            (
                "Device & Identity",
                vec![
                    ("DEFAULT_DEVICE_ID", self.default_device_id.clone()),
                    (
                        "DEFAULT_DEVICE_SECRET",
                        Self::mask_secret(&self.default_device_secret),
                    ),
                    (
                        "DEFAULT_ORGANIZATION_ADMIN_EMAIL",
                        self.default_organization_admin_email.clone(),
                    ),
                    ("SUPER_ADMIN_ID", Self::mask_secret(&self.super_admin_id)),
                    ("SYSTEM_DEVICE_ULID", self.system_device_ulid.clone()),
                    ("INITIALIZE_DEVICE", self.initialize_device.to_string()),
                ],
            ),
            (
                "System Configuration",
                vec![
                    (
                        "EXPERIMENTAL_PERMISSIONS",
                        self.experimental_permissions.to_string(),
                    ),
                    (
                        "INITIALIZE_ENTITY_DATA",
                        self.initialize_entity_data.to_string(),
                    ),
                    ("MERKLE_SAVE_INTERVAL", self.merkle_save_interval.to_string()),
                    (
                        "MAX_CONCURRENT_FLUSHES",
                        self.max_concurrent_flushes.to_string(),
                    ),
                    ("BUCKET_CAPACITY", self.bucket_capacity.to_string()),
                    ("INSERT_QUEUE_CAPACITY", self.insert_queue_capacity.to_string()),
                ],
            ),
            (
                "Test & Monitoring",
                vec![
                    ("STRICT_VALIDATION", self.strict_validation.to_string()),
                    ("PROMETHEUS_BASE_URL", self.prometheus_base_url.clone()),
                ],
            ),
        ];

        categories
            .into_iter()
            .map(|(name, rows)| {
                Self::render_table(&format!("Loaded Environment Configuration - {name}"), &rows)
            })
            .collect::<Vec<_>>()
            .join("\n\n")
    }

    fn render_table(title: &str, rows: &[(&str, String)]) -> String {
        let key_col_width = rows
            .iter()
            .map(|(key, _)| key.len())
            .max()
            .unwrap_or(3)
            .max("ENV".len());

        let value_col_width = rows
            .iter()
            .map(|(_, value)| value.len())
            .max()
            .unwrap_or(5)
            .max("VALUE".len())
            .min(100);

        let mut out = String::new();
        out.push_str(title);
        out.push('\n');
        out.push_str(&format!(
            "+-{:-<key$}-+-{:-<val$}-+\n",
            "",
            "",
            key = key_col_width,
            val = value_col_width
        ));
        out.push_str(&format!(
            "| {:<key$} | {:<val$} |\n",
            "ENV",
            "VALUE",
            key = key_col_width,
            val = value_col_width
        ));
        out.push_str(&format!(
            "+-{:-<key$}-+-{:-<val$}-+\n",
            "",
            "",
            key = key_col_width,
            val = value_col_width
        ));

        for (key, value) in rows {
            let printable = if value.len() > value_col_width {
                format!("{}...", &value[..value_col_width.saturating_sub(3)])
            } else {
                value.clone()
            };
            out.push_str(&format!(
                "| {:<key$} | {:<val$} |\n",
                key,
                printable,
                key = key_col_width,
                val = value_col_width
            ));
        }

        out.push_str(&format!(
            "+-{:-<key$}-+-{:-<val$}-+",
            "",
            "",
            key = key_col_width,
            val = value_col_width
        ));
        out
    }

    fn mask_secret(value: &str) -> String {
        if value.is_empty() {
            return "<empty>".to_string();
        }
        if value.len() <= 4 {
            return "****".to_string();
        }
        format!("{}***{}", &value[..2], &value[value.len() - 2..])
    }
}
