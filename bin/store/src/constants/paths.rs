//! Path constants for the application
//!
//! This module contains all hardcoded file and directory paths used throughout
//! the application to ensure consistency and ease of maintenance.

/// Database-related path constants
pub mod database {
    /// Main schema file path
    pub const SCHEMA_FILE: &str = "src/generated/schema.rs";

    /// Table enum file path
    pub const TABLE_ENUM_FILE: &str = "src/generated/table_enum.rs";

    /// Schema tables directory
    pub const SCHEMA_TABLES_DIR: &str = "src/database/schema/tables";

    /// System fields definition file
    pub const SYSTEM_FIELDS_FILE: &str = "src/database/schema/generator/system_fields.rs";

    /// Hypertables definition file
    pub const HYPERTABLES_FILE: &str = "src/database/schema/hypertables.rs";

    /// Database initialization SQL file
    pub const INIT_SQL_FILE: &str = "src/database/schema/init.sql";

    /// Models directory
    pub const MODELS_DIR: &str = "src/database/models";

    /// Models module file
    pub const MODELS_MOD_FILE: &str = "src/database/models/mod.rs";

    /// Database cleanup SQL file
    pub const CLEANUP_SQL_FILE: &str = "src/database/cleanup.sql";
}

/// Proto generation path constants
pub mod proto {
    /// Proto source file
    pub const SOURCE_FILE: &str = "src/proto/store.proto";
    /// Proto output directory
    pub const OUTPUT_DIR: &str = "src/proto";
    /// Generated proto output directory
    pub const GENERATED_DIR: &str = "src/generated";
    /// Proto build script location
    pub const BUILD_SCRIPT: &str = "src/builders/generator/build_proto.rs";
}

/// Legacy schema file path (for backward compatibility)
pub const LEGACY_SCHEMA_FILE: &str = "schema.rs";
