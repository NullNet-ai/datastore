use log::{debug, error, info, warn, LevelFilter};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::SystemTime;
use tokio::sync::RwLock;

/// Log levels for lifecycle events
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
    Critical,
}

/// Log categories for different system components
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum LogCategory {
    Lifecycle,
    Startup,
    Runtime,
    Shutdown,
    Health,
    Monitoring,
    State,
    Database,
    Network,
    Security,
    Performance,
    Custom(String),
}

/// Structured log entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogEntry {
    pub timestamp: SystemTime,
    pub level: LogLevel,
    pub category: LogCategory,
    pub component: String,
    pub message: String,
    pub metadata: HashMap<String, String>,
    pub correlation_id: Option<String>,
}

/// Log configuration
#[derive(Debug, Clone)]
pub struct LogConfig {
    pub level: LogLevel,
    pub enable_console: bool,
    pub enable_file: bool,
    pub file_path: Option<String>,
    pub enable_structured: bool,
    pub max_entries: usize,
}

impl Default for LogConfig {
    fn default() -> Self {
        Self {
            level: LogLevel::Info,
            enable_console: true,
            enable_file: false,
            file_path: None,
            enable_structured: true,
            max_entries: 10000,
        }
    }
}

/// Lifecycle logger
pub struct LifecycleLogger {
    config: LogConfig,
    entries: Arc<RwLock<Vec<LogEntry>>>,
    correlation_counter: Arc<RwLock<u64>>,
}

impl LifecycleLogger {
    /// Create a new lifecycle logger
    pub fn new(config: LogConfig) -> Self {
        info!(
            "[LOGGING] Initializing lifecycle logger with config: {:?}",
            config
        );

        Self {
            config,
            entries: Arc::new(RwLock::new(Vec::new())),
            correlation_counter: Arc::new(RwLock::new(0)),
        }
    }

    /// Initialize the logging system
    pub async fn initialize(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        info!("[LOGGING] Initializing logging system");

        // Configure log level
        let level_filter = match self.config.level {
            LogLevel::Trace => LevelFilter::Trace,
            LogLevel::Debug => LevelFilter::Debug,
            LogLevel::Info => LevelFilter::Info,
            LogLevel::Warn => LevelFilter::Warn,
            LogLevel::Error => LevelFilter::Error,
            LogLevel::Critical => LevelFilter::Error, // Map to Error as it's the highest in log crate
        };

        // Initialize env_logger if not already initialized
        if env_logger::try_init_from_env(
            env_logger::Env::default().default_filter_or(level_filter.to_string()),
        )
        .is_err()
        {
            debug!("[LOGGING] Logger already initialized");
        }

        info!(
            "[LOGGING] Logging system initialized with level: {:?}",
            self.config.level
        );
        Ok(())
    }

    /// Log a lifecycle event with automatic correlation ID generation
    pub async fn log(
        &self,
        level: LogLevel,
        category: LogCategory,
        component: &str,
        message: &str,
    ) {
        self.log_with_metadata(level, category, component, message, HashMap::new(), None)
            .await;
    }

    /// Log a lifecycle event with a specific correlation ID
    #[allow(dead_code)]
    pub async fn log_with_correlation(
        &self,
        level: LogLevel,
        category: LogCategory,
        component: &str,
        message: &str,
        correlation_id: String,
    ) {
        self.log_with_metadata(level, category, component, message, HashMap::new(), Some(correlation_id))
            .await;
    }

    /// Log a lifecycle event without correlation ID (for cases where correlation tracking is not needed)
    #[allow(dead_code)]
    pub async fn log_without_correlation(
        &self,
        level: LogLevel,
        category: LogCategory,
        component: &str,
        message: &str,
    ) {
        let entry = LogEntry {
            timestamp: SystemTime::now(),
            level: level.clone(),
            category: category.clone(),
            component: component.to_string(),
            message: message.to_string(),
            metadata: HashMap::new(),
            correlation_id: None,
        };

        // Store entry if structured logging is enabled
        if self.config.enable_structured {
            self.store_entry(entry.clone()).await;
        }

        // Output to console if enabled
        if self.config.enable_console {
            self.output_to_console(&entry).await;
        }

        // Output to file if enabled
        if self.config.enable_file {
            self.output_to_file(&entry).await;
        }
    }

    /// Log with metadata
    pub async fn log_with_metadata(
        &self,
        level: LogLevel,
        category: LogCategory,
        component: &str,
        message: &str,
        metadata: HashMap<String, String>,
        correlation_id: Option<String>,
    ) {
        // Generate correlation ID if none provided
        let correlation_id = match correlation_id {
            Some(id) => Some(id),
            None => Some(self.generate_correlation_id().await),
        };

        let entry = LogEntry {
            timestamp: SystemTime::now(),
            level: level.clone(),
            category: category.clone(),
            component: component.to_string(),
            message: message.to_string(),
            metadata,
            correlation_id,
        };

        // Store entry if structured logging is enabled
        if self.config.enable_structured {
            self.store_entry(entry.clone()).await;
        }

        // Output to console if enabled
        if self.config.enable_console {
            self.output_to_console(&entry).await;
        }

        // Output to file if enabled
        if self.config.enable_file {
            self.output_to_file(&entry).await;
        }
    }

    /// Store log entry in memory
    async fn store_entry(&self, entry: LogEntry) {
        let mut entries = self.entries.write().await;

        entries.push(entry);

        // Trim entries if we exceed max size
        if entries.len() > self.config.max_entries {
            let excess_count = entries.len() - self.config.max_entries;
            entries.drain(0..excess_count);
        }
    }

    /// Output log entry to console
    async fn output_to_console(&self, entry: &LogEntry) {
        let formatted = self.format_entry(entry, false);

        match entry.level {
            LogLevel::Trace => debug!("{}", formatted),
            LogLevel::Debug => debug!("{}", formatted),
            LogLevel::Info => info!("{}", formatted),
            LogLevel::Warn => warn!("{}", formatted),
            LogLevel::Error | LogLevel::Critical => error!("{}", formatted),
        }
    }

    /// Output log entry to file
    async fn output_to_file(&self, entry: &LogEntry) {
        if let Some(file_path) = &self.config.file_path {
            let formatted = self.format_entry(entry, true);

            // TODO: Implement actual file writing
            // This would typically involve:
            // 1. Opening/creating the log file
            // 2. Writing the formatted entry
            // 3. Handling file rotation if needed

            debug!("[LOGGING] Would write to file {}: {}", file_path, formatted);
        }
    }

    /// Format log entry for output
    fn format_entry(&self, entry: &LogEntry, include_metadata: bool) -> String {
        let timestamp = entry
            .timestamp
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        let level_str = match entry.level {
            LogLevel::Trace => "TRACE",
            LogLevel::Debug => "DEBUG",
            LogLevel::Info => "INFO",
            LogLevel::Warn => "WARN",
            LogLevel::Error => "ERROR",
            LogLevel::Critical => "CRITICAL",
        };

        let category_str = match &entry.category {
            LogCategory::Lifecycle => "LIFECYCLE",
            LogCategory::Startup => "STARTUP",
            LogCategory::Runtime => "RUNTIME",
            LogCategory::Shutdown => "SHUTDOWN",
            LogCategory::Health => "HEALTH",
            LogCategory::Monitoring => "MONITORING",
            LogCategory::State => "STATE",
            LogCategory::Database => "DATABASE",
            LogCategory::Network => "NETWORK",
            LogCategory::Security => "SECURITY",
            LogCategory::Performance => "PERFORMANCE",
            LogCategory::Custom(name) => name,
        };

        let mut formatted = format!(
            "[{}] [{}] [{}] [{}] {}",
            timestamp, level_str, category_str, entry.component, entry.message
        );

        if include_metadata && !entry.metadata.is_empty() {
            let metadata_str: Vec<String> = entry
                .metadata
                .iter()
                .map(|(k, v)| format!("{}={}", k, v))
                .collect();
            formatted.push_str(&format!(" [{}]", metadata_str.join(", ")));
        }

        if let Some(correlation_id) = &entry.correlation_id {
            formatted.push_str(&format!(" [correlation_id={}]", correlation_id));
        }

        formatted
    }

    /// Generate correlation ID
    pub async fn generate_correlation_id(&self) -> String {
        let mut counter = self.correlation_counter.write().await;
        *counter += 1;
        format!(
            "lc-{:08x}-{:08x}",
            SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
            *counter
        )
    }

    /// Get recent log entries
    pub async fn get_recent_entries(&self, limit: Option<usize>) -> Vec<LogEntry> {
        let entries = self.entries.read().await;
        let limit = limit.unwrap_or(100).min(entries.len());

        entries.iter().rev().take(limit).cloned().collect()
    }

    /// Get entries by category
    pub async fn get_entries_by_category(
        &self,
        category: LogCategory,
        limit: Option<usize>,
    ) -> Vec<LogEntry> {
        let entries = self.entries.read().await;
        let limit = limit.unwrap_or(100);

        entries
            .iter()
            .rev()
            .filter(|entry| entry.category == category)
            .take(limit)
            .cloned()
            .collect()
    }

    /// Get entries by component
    pub async fn get_entries_by_component(
        &self,
        component: &str,
        limit: Option<usize>,
    ) -> Vec<LogEntry> {
        let entries = self.entries.read().await;
        let limit = limit.unwrap_or(100);

        entries
            .iter()
            .rev()
            .filter(|entry| entry.component == component)
            .take(limit)
            .cloned()
            .collect()
    }

    /// Get entries by level
    pub async fn get_entries_by_level(
        &self,
        level: LogLevel,
        limit: Option<usize>,
    ) -> Vec<LogEntry> {
        let entries = self.entries.read().await;
        let limit = limit.unwrap_or(100);

        entries
            .iter()
            .rev()
            .filter(|entry| entry.level == level)
            .take(limit)
            .cloned()
            .collect()
    }

    /// Clear all stored entries
    pub async fn clear_entries(&self) {
        let mut entries = self.entries.write().await;
        entries.clear();
        info!("[LOGGING] Log entries cleared");
    }

    /// Get log statistics
    pub async fn get_statistics(&self) -> HashMap<String, u64> {
        let entries = self.entries.read().await;
        let mut stats = HashMap::new();

        stats.insert("total_entries".to_string(), entries.len() as u64);

        // Count by level
        let mut level_counts = HashMap::new();
        for entry in entries.iter() {
            let level_key = format!("{:?}", entry.level).to_lowercase();
            *level_counts.entry(level_key).or_insert(0) += 1;
        }

        for (level, count) in level_counts {
            stats.insert(format!("level_{}", level), count);
        }

        // Count by category
        let mut category_counts = HashMap::new();
        for entry in entries.iter() {
            let category_key = match &entry.category {
                LogCategory::Custom(name) => name.clone(),
                other => format!("{:?}", other).to_lowercase(),
            };
            *category_counts.entry(category_key).or_insert(0) += 1;
        }

        for (category, count) in category_counts {
            stats.insert(format!("category_{}", category), count);
        }

        stats
    }
}

/// Convenience macros for lifecycle logging
#[macro_export]
macro_rules! lifecycle_log {
    ($logger:expr, $level:expr, $category:expr, $component:expr, $($arg:tt)*) => {
        $logger.log($level, $category, $component, &format!($($arg)*)).await
    };
}

/// Macro for logging with a specific correlation ID
#[macro_export]
macro_rules! lifecycle_log_with_correlation {
    ($logger:expr, $level:expr, $category:expr, $component:expr, $correlation_id:expr, $($arg:tt)*) => {
        $logger.log_with_correlation($level, $category, $component, &format!($($arg)*), $correlation_id).await
    };
}

/// Macro for logging without correlation ID
#[macro_export]
macro_rules! lifecycle_log_without_correlation {
    ($logger:expr, $level:expr, $category:expr, $component:expr, $($arg:tt)*) => {
        $logger.log_without_correlation($level, $category, $component, &format!($($arg)*)).await
    };
}

#[macro_export]
macro_rules! lifecycle_info {
    ($logger:expr, $category:expr, $component:expr, $($arg:tt)*) => {
        lifecycle_log!($logger, crate::lifecycle::logging::LogLevel::Info, $category, $component, $($arg)*)
    };
}

#[macro_export]
macro_rules! lifecycle_info_with_correlation {
    ($logger:expr, $category:expr, $component:expr, $correlation_id:expr, $($arg:tt)*) => {
        lifecycle_log_with_correlation!($logger, crate::lifecycle::logging::LogLevel::Info, $category, $component, $correlation_id, $($arg)*)
    };
}

#[macro_export]
macro_rules! lifecycle_warn {
    ($logger:expr, $category:expr, $component:expr, $($arg:tt)*) => {
        lifecycle_log!($logger, crate::lifecycle::logging::LogLevel::Warn, $category, $component, $($arg)*)
    };
}

#[macro_export]
macro_rules! lifecycle_warn_with_correlation {
    ($logger:expr, $category:expr, $component:expr, $correlation_id:expr, $($arg:tt)*) => {
        lifecycle_log_with_correlation!($logger, crate::lifecycle::logging::LogLevel::Warn, $category, $component, $correlation_id, $($arg)*)
    };
}

#[macro_export]
macro_rules! lifecycle_error {
    ($logger:expr, $category:expr, $component:expr, $($arg:tt)*) => {
        lifecycle_log!($logger, crate::lifecycle::logging::LogLevel::Error, $category, $component, $($arg)*)
    };
}

#[macro_export]
macro_rules! lifecycle_error_with_correlation {
    ($logger:expr, $category:expr, $component:expr, $correlation_id:expr, $($arg:tt)*) => {
        lifecycle_log_with_correlation!($logger, crate::lifecycle::logging::LogLevel::Error, $category, $component, $correlation_id, $($arg)*)
    };
}

impl Default for LifecycleLogger {
    fn default() -> Self {
        Self::new(LogConfig::default())
    }
}