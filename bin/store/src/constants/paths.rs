//! Path constants for the application
//!
//! This module contains all hardcoded file and directory paths used throughout
//! the application to ensure consistency and ease of maintenance.

/// Database-related path constants
pub mod database {
    /// Database initialization SQL file
    pub const INIT_SQL_FILE: &str = "src/database/schema/init.sql";

    /// Database cleanup SQL file
    pub const CLEANUP_SQL_FILE: &str = "src/database/cleanup.sql";
}
