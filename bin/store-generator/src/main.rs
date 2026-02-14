//! Standalone store code generator.
//!
//! Run from workspace root with STORE_DIR=bin/store (default when unset).
//! Example: `cd bin/store && cargo run -p store-generator` (paths relative to store)
//! Or: `STORE_DIR=bin/store cargo run -p store-generator` (from workspace root)

use log::{error, info};
use std::env;
use std::process;

fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    let create_schema = env::var("CREATE_SCHEMA")
        .unwrap_or_default()
        .to_lowercase()
        == "true";
    let generate_proto = env::var("GENERATE_PROTO")
        .unwrap_or_default()
        .to_lowercase()
        == "true";
    let generate_grpc = env::var("GENERATE_GRPC")
        .unwrap_or_default()
        .to_lowercase()
        == "true";
    let generate_table_enum = env::var("GENERATE_TABLE_ENUM")
        .unwrap_or_default()
        .to_lowercase()
        == "true";

    if create_schema || generate_proto || generate_grpc || generate_table_enum {
        info!("Starting standalone code generation...");

        // Schema generation (includes models, migrations)
        if create_schema {
            info!("Running schema generator");
            if let Err(e) = store_generator::generator::run_schema() {
                error!("Schema generation failed: {}", e);
                process::exit(1);
            }
        }

        // Proto generation
        if generate_proto {
            info!("Generating proto files");
            store_generator::templates::proto_generator::generate_protos(
                &store_generator::paths::schema_file(),
                &store_generator::paths::proto_output_dir(),
            );
        }

        // gRPC controller generation
        if generate_grpc {
            info!("Generating gRPC controllers");
            if let Err(e) = store_generator::templates::grpc_controller::run_generator() {
                error!("gRPC controller generation failed: {}", e);
                process::exit(1);
            }
        }

        // Table enum generation
        if generate_table_enum {
            info!("Generating table enum");
            if let Err(e) = store_generator::templates::table_enum::run_generator() {
                error!("Table enum generation failed: {}", e);
                process::exit(1);
            }
        }

        info!("Code generation completed successfully!");
        process::exit(0);
    } else {
        info!("No generation flags set. Use CREATE_SCHEMA=true, GENERATE_PROTO=true, GENERATE_GRPC=true, GENERATE_TABLE_ENUM=true");
    }
}
