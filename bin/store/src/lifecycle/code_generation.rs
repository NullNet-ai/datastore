//! Code generation delegated to store-generator.
//! Use: make store-generator-all or STORE_DIR=bin/store cargo run -p store-generator

use crate::structs::core::CommandArgs;
use log::info;
use std::process;

/// Handle code generation. All generation (proto, gRPC, table_enum, schema) uses store-generator.
pub async fn handle_code_generation(args: &CommandArgs) {
    if args.generate_proto || args.generate_grpc || args.generate_table_enum || args.create_schema {
        info!(
            "Code generation: use store-generator. Run: make store-generator-all \
             or STORE_DIR=bin/store CREATE_SCHEMA=true GENERATE_PROTO=true \
             cargo run -p store-generator"
        );
        process::exit(0);
    }
}
