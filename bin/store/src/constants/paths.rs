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

    /// Table enum file path
    pub const TABLE_ENUM_FILE: &str = const_format::concatcp!(GENERATED_DIR, "/table_enum.rs");

    /// Schema tables directory
    pub const SCHEMA_TABLES_DIR: &str = "src/database/schema/tables";

    /// System fields definition file
    pub const SYSTEM_FIELDS_FILE: &str = "src/builders/generator/system_fields.rs";

    /// Hypertables definition file
    pub const HYPERTABLES_FILE: &str = "src/database/schema/hypertables.rs";

    /// Database initialization SQL file
    pub const INIT_SQL_FILE: &str = "src/database/schema/init.sql";

    /// Models directory
    pub const MODELS_DIR: &str = const_format::concatcp!(GENERATED_DIR, "/models");

    /// Models module file
    pub const MODELS_MOD_FILE: &str = const_format::concatcp!(GENERATED_DIR, "/models/mod.rs");

    /// Database cleanup SQL file
    pub const CLEANUP_SQL_FILE: &str = "src/database/cleanup.sql";

    /// Migrations directory
    pub const MIGRATIONS_DIR: &str = "migrations";
}

/// Proto generation path constants
pub mod proto {
    use super::GENERATED_DIR;

    /// Proto source file
    pub const SOURCE_FILE: &str = const_format::concatcp!(GENERATED_DIR, "/proto/store.proto");
    /// Proto output directory
    pub const OUTPUT_DIR: &str = const_format::concatcp!(GENERATED_DIR, "/proto");
    /// Proto build script location
    pub const BUILD_SCRIPT: &str = "src/builders/generator/build_proto.rs";
}

/// gRPC controller path constants
pub mod grpc {
    use super::GENERATED_DIR;

    /// Generated gRPC controller file path
    pub const CONTROLLER_FILE: &str = const_format::concatcp!(GENERATED_DIR, "/grpc_controller.rs");
    
    /// gRPC struct converter file path
    pub const STRUCT_CONVERTER_FILE: &str = "grpc_struct_converter.rs";
}

/// Template-related path constants
pub mod templates {
    /// Proto file name constant
    pub const PROTO_FILE_NAME: &str = "store.proto";
    
    /// Template directory base path
    pub const TEMPLATES_DIR: &str = "src/builders/templates";
    
    /// gRPC controller template directory
    pub const GRPC_CONTROLLER_TEMPLATE_DIR: &str = const_format::concatcp!(TEMPLATES_DIR, "/grpc_controller");
    
    /// Table enum template directory
    pub const TABLE_ENUM_TEMPLATE_DIR: &str = const_format::concatcp!(TEMPLATES_DIR, "/table_enum");
}

/// Legacy schema file path (for backward compatibility)
pub const LEGACY_SCHEMA_FILE: &str = "schema.rs";
