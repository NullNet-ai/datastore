//! Path configuration for the store generator.
//!
//! Paths are resolved from (in order):
//! 1. `store-generator.toml` config file
//! 2. STORE_DIR environment variable
//! 3. Default "."

use crate::config::Config;

fn store_dir() -> String {
    Config::discover().store_dir()
}

pub const PROTO_FILE_NAME: &str = "store.proto";

lazy_static::lazy_static! {
    pub static ref GENERATED_DIR: String = format!("{}/src/generated", store_dir());
    pub static ref SCHEMA_FILE: String = format!("{}/src/generated/schema.rs", store_dir());
    pub static ref SCHEMA_TABLES_DIR: String = format!("{}/src/database/schema/tables", store_dir());
    pub static ref MIGRATIONS_DIR: String = format!("{}/migrations", store_dir());
    pub static ref MODELS_DIR: String = format!("{}/src/generated/models", store_dir());
    pub static ref MODELS_MOD_FILE: String = format!("{}/src/generated/models/mod.rs", store_dir());
    pub static ref TABLE_ENUM_FILE: String = format!("{}/src/generated/table_enum.rs", store_dir());
    pub static ref GRPC_CONTROLLER_FILE: String = format!("{}/src/generated/grpc_controller.rs", store_dir());
    pub static ref PROTO_SOURCE_FILE: String = format!("{}/src/generated/proto/store.proto", store_dir());
    pub static ref PROTO_OUTPUT_DIR: String = format!("{}/src/generated/proto", store_dir());
    pub static ref SYSTEM_FIELDS_FILE: String =
        format!("{}/src/macros/system_fields/system_fields.rs", store_dir());
    pub static ref HYPERTABLES_FILE: String = format!("{}/src/database/schema/hypertables.rs", store_dir());
    pub static ref SYSTEM_TABLES_FILE: String = format!("{}/src/database/schema/system_tables.rs", store_dir());
    pub static ref RESERVED_KEYWORDS_FILE: String =
        format!("{}/src/database/schema/reserved_keywords.rs", store_dir());
    pub static ref BUILD_SCRIPT: String = format!("{}/src/builders/generator/build_proto.rs", store_dir());
}

pub mod database {
    pub use super::{
        BUILD_SCRIPT, GENERATED_DIR, GRPC_CONTROLLER_FILE, HYPERTABLES_FILE, MIGRATIONS_DIR,
        MODELS_DIR, MODELS_MOD_FILE, PROTO_OUTPUT_DIR, PROTO_SOURCE_FILE, RESERVED_KEYWORDS_FILE,
        SCHEMA_FILE, SCHEMA_TABLES_DIR, SYSTEM_FIELDS_FILE, SYSTEM_TABLES_FILE, TABLE_ENUM_FILE,
    };

    pub const UP_SQL_FILE: &str = "up.sql";
}

pub mod grpc {
    pub use super::GRPC_CONTROLLER_FILE as CONTROLLER_FILE;
}

pub mod proto {
    pub use super::{
        BUILD_SCRIPT, GENERATED_DIR, PROTO_OUTPUT_DIR as OUTPUT_DIR,
        PROTO_SOURCE_FILE as SOURCE_FILE,
    };
}

pub mod templates {
    pub use super::PROTO_FILE_NAME;
}

pub fn schema_file() -> String {
    SCHEMA_FILE.clone()
}

pub fn proto_output_dir() -> String {
    PROTO_OUTPUT_DIR.clone()
}
