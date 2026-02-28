//! Path constants for the application
//!
//! This module contains all hardcoded file and directory paths used throughout
//! the application to ensure consistency and ease of maintenance.

/// Generated directory constant
pub const GENERATED_DIR: &str = "src/generated";

/// Database-related path constants
pub mod database {
    use super::GENERATED_DIR;

    /// Main schema file path
    pub const SCHEMA_FILE: &str = const_format::concatcp!(GENERATED_DIR, "/schema.rs");

    /// Database initialization SQL file
    pub const INIT_SQL_FILE: &str = "src/database/schema/init.sql";

    /// Database cleanup SQL file
    pub const CLEANUP_SQL_FILE: &str = "src/database/cleanup.sql";
}

/// Legacy schema file path (for backward compatibility)
pub const LEGACY_SCHEMA_FILE: &str = "schema.rs";
