use crate::templates::proto_generator::{Case, CaseConvert};
use crate::utils::utils::{parse_tables, to_singular};
use regex::Regex;
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::Path;
use std::process;
use std::process::Command;

pub fn generate_grpc_controller(proto_path: &str, output_path: &str) -> io::Result<()> {
    println!("Generating gRPC controller from proto file: {}", proto_path);

    // Read the proto file
    let proto_content = fs::read_to_string(proto_path)?;

    // Extract service definition
    let service_regex = Regex::new(r"service\s+(\w+)\s*\{([\s\S]*?)\}").unwrap();
    let rpc_regex =
        Regex::new(r"rpc\s+(\w+)\s*\(\s*(\w+)\s*\)\s*returns\s*\(\s*(\w+)\s*\)").unwrap();

    let mut service_name = String::new();
    let mut rpc_methods = Vec::new();
    let schema_path = "src/schema/schema.rs";
    let schema = match fs::read_to_string(schema_path) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Error reading schema file: {}", e);
            process::exit(1);
        }
    };
    let tables = parse_tables(&schema);

    if let Some(captures) = service_regex.captures(&proto_content) {
        service_name = captures.get(1).unwrap().as_str().to_string();
        let service_body = captures.get(2).unwrap().as_str();
        for cap in rpc_regex.captures_iter(service_body) {
            let method_name = cap.get(1).unwrap().as_str().to_string();
            let request_type = cap.get(2).unwrap().as_str().to_string();
            let response_type = cap.get(3).unwrap().as_str().to_string();

            rpc_methods.push((method_name, request_type, response_type));
        }
    } else {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "No service definition found in proto file",
        ));
    }

    // Create output directory if it doesn't exist
    if let Some(parent) = Path::new(output_path).parent() {
        fs::create_dir_all(parent)?;
    }

    // Generate the controller file
    let mut file = File::create(output_path)?;

    // Write imports
    writeln!(file, "use crate::db::create_connection;")?;

    writeln!(file, "use actix_web::{{HttpResponse, Responder, web}};")?;
    writeln!(file, "use std::pin::Pin;")?;
    writeln!(file, "use std::net::SocketAddr;")?;
    writeln!(file, "use crate::table_enum::Table;")?;
    writeln!(file, "use crate::sync::sync_service::{{insert, update}};")?;
    writeln!(file, "use serde_json::Value;")?;
    writeln!(file, "use crate::structs::structs::RequestBody;")?;
    writeln!(
        file,
        "use tonic::{{Request, Response, Status, transport::Server}};"
    )?;
    writeln!(
        file,
        "use crate::auth::auth_middleware::GrpcAuthInterceptor;"
    )?;
    writeln!(file, "use super::common_controller::{{perform_batch_update, process_record_for_insert, process_record_for_update, sanitize_updates, convert_json_to_csv, process_records, execute_copy}};")?;
    writeln!(
        file,
        "use crate::generated::store::store_service_server::{{{}Server, {} }};",
        service_name, service_name
    )?;
    writeln!(file, "use crate::{{ generate_batch_delete_method, generate_batch_insert_method, generate_batch_update_method, generate_create_method, generate_update_method, generate_get_method, generate_delete_method}};")?;

    // Import request and response types
    write!(file, "use crate::generated::store::{{")?;
    for (i, table) in tables.iter().enumerate() {
        if i > 0 {
            write!(file, ", ")?;
        }
        // Import the main type
        write!(file, "{}", table.name.to_case(Case::Pascal))?;

        // Import all related request/response types
        write!(
            file,
            ", Create{}Request, Create{}Response",
            table.name.to_case(Case::Pascal),
            table.name.to_case(Case::Pascal)
        )?;
        write!(
            file,
            ", Get{}Request, Get{}Response",
            table.name.to_case(Case::Pascal),
            table.name.to_case(Case::Pascal)
        )?;
        write!(
            file,
            ", Update{}Request, Update{}Response",
            table.name.to_case(Case::Pascal),
            table.name.to_case(Case::Pascal)
        )?;
        write!(
            file,
            ", Delete{}Request, Delete{}Response",
            table.name.to_case(Case::Pascal),
            table.name.to_case(Case::Pascal)
        )?;
        write!(
            file,
            ", BatchInsert{}Request, BatchInsert{}Response",
            table.name.to_case(Case::Pascal),
            table.name.to_case(Case::Pascal)
        )?;
        write!(
            file,
            ", BatchUpdate{}Request, BatchUpdate{}Response",
            table.name.to_case(Case::Pascal),
            table.name.to_case(Case::Pascal)
        )?;
        write!(
            file,
            ", BatchDelete{}Request, BatchDelete{}Response",
            table.name.to_case(Case::Pascal),
            table.name.to_case(Case::Pascal)
        )?;
    }
    writeln!(file, "}};")?;

    // Initialize method
    writeln!(file, "pub struct GrpcController {{}}\n")?;
    writeln!(file, "impl GrpcController {{")?;
    writeln!(file, "    pub fn new() -> Self {{ GrpcController {{}} }}")?;
    writeln!(
        file,
        "\n    pub async fn init(addr: &str) -> Result<(), Box<dyn std::error::Error>> {{"
    )?;
    writeln!(file, "        let addr: SocketAddr = addr.parse()?;")?;
    writeln!(file, "        let grpc_controller = GrpcController::new();")?;
    writeln!(
        file,
        "        println!(\"gRPC Server listening on {{}}\", addr);"
    )?;
    writeln!(
        file,
        "        Server::builder()
            .add_service({}Server::with_interceptor(grpc_controller, GrpcAuthInterceptor))
            .serve(addr)
            .await?;",
        service_name
    )?;
    writeln!(file, "        Ok(())")?;
    writeln!(file, "    }}")?;
    writeln!(file, "}}\n")?;

    // Implement service trait
    writeln!(file, "#[tonic::async_trait]")?;
    writeln!(file, "impl {} for GrpcController {{", service_name)?;

    // Implement each RPC method
    for table in &tables {
        let singular_name = to_singular(&table.name);
        writeln!(file, "    // CRUD methods for {}", table.name)?;

        writeln!(file, "    generate_create_method!({});", table.name)?;
        writeln!(
            file,
            "    generate_update_method!({}, {});",
            table.name, singular_name
        )?;
        writeln!(file, "    generate_batch_insert_method!({});", table.name)?;
        writeln!(file, "    generate_batch_update_method!({});", table.name)?;
        writeln!(file, "    generate_get_method!({});", table.name)?;
        writeln!(file, "    generate_delete_method!({});", table.name)?;
        writeln!(file, "    generate_batch_delete_method!({});", table.name)?;
    }
    writeln!(file, "}}")?;

    // Add HTTP endpoints
    writeln!(
        file,
        "\n// You can add HTTP endpoints to configure or check gRPC status"
    )?;
    writeln!(file, "pub async fn grpc_status() -> impl Responder {{")?;
    writeln!(file, "    HttpResponse::Ok().json(serde_json::json!({{")?;
    writeln!(file, "        \"status\": \"running\",")?;
    writeln!(file, "        \"message\": \"gRPC server is operational\"")?;
    writeln!(file, "    }}))")?;
    writeln!(file, "}}")?;

    // Add configuration function
    writeln!(
        file,
        "\n// Function to configure and register HTTP routes related to gRPC"
    )?;
    writeln!(file, "pub fn configure(cfg: &mut web::ServiceConfig) {{")?;
    writeln!(
        file,
        "    cfg.service(web::resource(\"/api/grpc/status\").route(web::get().to(grpc_status)));"
    )?;
    writeln!(file, "}}")?;

    println!("Successfully generated gRPC controller at: {}", output_path);
    Ok(())
}

fn to_snake_case(name: &str) -> String {
    let mut snake = String::new();
    for (i, c) in name.chars().enumerate() {
        if c.is_uppercase() {
            if i != 0 {
                snake.push('_');
            }
            snake.extend(c.to_lowercase());
        } else {
            snake.push(c);
        }
    }
    snake
}

// Main function to run the generator as a standalone script
pub fn run_generator() -> io::Result<()> {
    println!("Starting gRPC controller generator");

    // Default paths
    let proto_path = "src/proto/store.proto";
    let output_path = "src/controllers/grpc_controller.rs";

    // Generate the controller
    match generate_grpc_controller(proto_path, output_path) {
        Ok(_) => {
            println!("Successfully generated gRPC controller");

            // Format the generated code with rustfmt
            println!("Formatting generated code...");
            match Command::new("rustfmt").arg(output_path).status() {
                Ok(_) => println!("Code formatting completed"),
                Err(e) => println!("Warning: Failed to format code: {}", e),
            }

            Ok(())
        }
        Err(e) => {
            eprintln!("Error generating gRPC controller: {}", e);
            Err(e)
        }
    }
}
