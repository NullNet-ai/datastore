use regex::Regex;
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::Path;
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
    writeln!(file, "use crate::db;")?;
    writeln!(
        file,
        "use crate::generated::store::store_service_server::{{StoreService, StoreServiceServer}};"
    )?;
    writeln!(file, "use actix_web::{{HttpResponse, Responder, web}};")?;
    writeln!(file, "use serde::Serialize;")?;
    writeln!(file, "use std::net::SocketAddr;")?;
    writeln!(
        file,
        "use tonic::{{Request, Response, Status, transport::Server}};"
    )?;

    // Import request and response types
    write!(file, "use crate::generated::store::{{")?;
    for (i, (_, request_type, response_type)) in rpc_methods.iter().enumerate() {
        if i > 0 {
            write!(file, ", ")?;
        }
        write!(file, "{}, {}", request_type, response_type)?;
    }
    writeln!(file, "}};")?;

    // Define controller struct
    writeln!(file, "\n// Define your gRPC service struct")?;
    writeln!(file, "pub struct GrpcController {{}}")?;

    // Implement basic methods
    writeln!(file, "\nimpl GrpcController {{")?;
    writeln!(file, "    pub fn new() -> Self {{")?;
    writeln!(file, "        GrpcController {{}}")?;
    writeln!(file, "    }}")?;

    // Initialize method
    writeln!(file, "\n    // Initialize the gRPC server")?;
    writeln!(
        file,
        "    pub async fn init(addr: &str) -> Result<(), Box<dyn std::error::Error>> {{"
    )?;
    writeln!(
        file,
        "        let addr: SocketAddr = addr.parse()?;  // Specify the type here"
    )?;
    writeln!(file, "        let grpc_controller = GrpcController::new();")?;
    writeln!(
        file,
        "\n        println!(\"gRPC Server listening on {{}}\", addr);"
    )?;
    writeln!(file, "\n        Server::builder()")?;
    writeln!(file, "            .add_service(")?;
    writeln!(
        file,
        "                {}Server::new(grpc_controller)",
        service_name
    )?;
    writeln!(
        file,
        "                    .max_decoding_message_size(50 * 1024 * 1024),"
    )?;
    writeln!(file, "            )")?;
    writeln!(file, "            .serve(addr)")?;
    writeln!(file, "            .await?;")?;
    writeln!(file, "\n        Ok(())")?;
    writeln!(file, "    }}")?;
    writeln!(file, "}}")?;

    // Implement service trait
    writeln!(file, "#[tonic::async_trait]")?;
    writeln!(file, "impl {} for GrpcController {{", service_name)?;

    // Implement each RPC method
    for (method_name, request_type, response_type) in rpc_methods {
        let snake_method = to_snake_case(&method_name);
        writeln!(
            file,
            "    async fn {}(&self, request: Request<{}>) -> Result<Response<{}>, Status> {{",
            snake_method, request_type, response_type
        )?;
        writeln!(file, "        // Implementation for {} method", method_name)?;
        writeln!(file, "        todo!()")?;
        writeln!(file, "    }}\n")?;
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
