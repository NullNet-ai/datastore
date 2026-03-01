//! Path constants for the application
//!
//! This module contains file and directory paths used throughout
//! the application. Paths are conditional based on the --init-db flag
//! and the execution context (root binary vs cargo run vs cargo build).

use std::sync::OnceLock;
use std::env;
use std::path::PathBuf;

/// Global configuration for path selection
static PATH_CONFIG: OnceLock<PathConfig> = OnceLock::new();

/// Execution context types
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ExecutionContext {
    /// Running from root binary (e.g., store-mac-os)
    RootBinary,
    /// Running from cargo run/build in bin/store
    CargoRun,
    /// Running from cargo build release
    CargoBuild,
}

/// Path configuration that determines which paths to use
#[derive(Debug, Clone)]
pub struct PathConfig {
    pub use_init_db_paths: bool,
    pub execution_context: ExecutionContext,
}

/// Detect the execution context based on current directory and executable path
fn detect_execution_context() -> ExecutionContext {
    let current_dir = env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    
    // Check if we're in a target directory (cargo build scenario)
    if current_dir.to_string_lossy().contains("/target/") {
        return ExecutionContext::CargoBuild;
    }
    
    // Check if we have a src directory (cargo run/build scenario)
    if current_dir.join("src").exists() {
        return ExecutionContext::CargoRun;
    }
    
    // Check if we have a bin directory (root binary scenario)
    if current_dir.join("bin").exists() {
        return ExecutionContext::RootBinary;
    }
    
    // Default to cargo run if we can't determine
    ExecutionContext::CargoRun
}

/// Initialize path configuration based on command line arguments
pub fn init_path_config(args: &[String]) {
    let use_init_db_paths = args.contains(&"--init-db".to_string());
    let execution_context = detect_execution_context();
    
    let config = PathConfig {
        use_init_db_paths,
        execution_context,
    };
    
    PATH_CONFIG
        .set(config)
        .expect("Path configuration should only be initialized once");
}

/// Get the current path configuration
fn get_path_config() -> &'static PathConfig {
    PATH_CONFIG.get().expect("Path configuration not initialized")
}

/// Database-related path constants
pub mod database {
    use super::{get_path_config, ExecutionContext};

    /// Get the appropriate schema file path based on execution context and --init-db flag
    pub fn schema_file() -> &'static str {
        let config = get_path_config();
        
        match (config.execution_context, config.use_init_db_paths) {
            (ExecutionContext::RootBinary, true) => "src/generated/schema.rs",
            (ExecutionContext::RootBinary, false) => "bin/store/src/generated/schema.rs",
            (ExecutionContext::CargoRun, true) => "src/generated/schema.rs",
            (ExecutionContext::CargoRun, false) => "src/generated/schema.rs",
            (ExecutionContext::CargoBuild, true) => "src/generated/schema.rs",
            (ExecutionContext::CargoBuild, false) => "src/generated/schema.rs",
        }
    }

    /// Get the database initialization SQL file path
    pub fn init_sql_file() -> &'static str {
        let config = get_path_config();
        
        match (config.execution_context, config.use_init_db_paths) {
            (ExecutionContext::RootBinary, true) => "src/database/schema/init.sql",
            (ExecutionContext::RootBinary, false) => "bin/store/src/database/schema/init.sql",
            (ExecutionContext::CargoRun, true) => "src/database/schema/init.sql",
            (ExecutionContext::CargoRun, false) => "src/database/schema/init.sql",
            (ExecutionContext::CargoBuild, true) => "src/database/schema/init.sql",
            (ExecutionContext::CargoBuild, false) => "src/database/schema/init.sql",
        }
    }

    /// Get the database initialization SQL content
    pub fn init_sql_content() -> &'static str {
        include_str!("../database/schema/init.sql")
    }

    /// Get the database cleanup SQL file path
    pub fn cleanup_sql_file() -> &'static str {
        let config = get_path_config();
        
        match (config.execution_context, config.use_init_db_paths) {
            (ExecutionContext::RootBinary, true) => "src/database/cleanup.sql",
            (ExecutionContext::RootBinary, false) => "bin/store/src/database/cleanup.sql",
            (ExecutionContext::CargoRun, true) => "src/database/cleanup.sql",
            (ExecutionContext::CargoRun, false) => "src/database/cleanup.sql",
            (ExecutionContext::CargoBuild, true) => "src/database/cleanup.sql",
            (ExecutionContext::CargoBuild, false) => "src/database/cleanup.sql",
        }
    }

    /// Get the database cleanup SQL content
    pub fn cleanup_sql_content() -> &'static str {
        include_str!("../database/cleanup.sql")
    }
}

/// Legacy schema file path (for backward compatibility)
pub fn legacy_schema_file() -> &'static str {
    let config = get_path_config();
    
    match (config.execution_context, config.use_init_db_paths) {
        (ExecutionContext::RootBinary, true) => "schema.rs",
        (ExecutionContext::RootBinary, false) => "bin/store/src/generated/schema.rs",
        (ExecutionContext::CargoRun, true) => "schema.rs",
        (ExecutionContext::CargoRun, false) => "schema.rs",
        (ExecutionContext::CargoBuild, true) => "schema.rs",
        (ExecutionContext::CargoBuild, false) => "schema.rs",
    }
}