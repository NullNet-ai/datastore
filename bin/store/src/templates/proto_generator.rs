use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

use crate::utils::utils::{parse_tables, Field, Table};

pub fn generate_protos(schema_path: &str, output_dir: &str) {
    println!("Starting proto generation from schema: {}", schema_path);

    // Read schema file
    match fs::read_to_string(schema_path) {
        Ok(schema) => {
            println!("Successfully read schema file");
            let proto_file_path = Path::new(output_dir).join("store.proto");
            if proto_file_path.exists() {
                println!("Deleting existing proto file: {:?}", proto_file_path);
                if let Err(err) = fs::remove_file(&proto_file_path) {
                    eprintln!(
                        "Warning: Failed to delete file '{}': {}",
                        proto_file_path.display(),
                        err
                    );
                    // Continue despite deletion errors
                }
            }

            // Clean up existing proto files
            if let Err(err) = clean_output_directory(output_dir) {
                eprintln!("Warning: Failed to clean output directory: {}", err);
                // Continue despite cleanup errors
            }

            // Create output directory if it doesn't exist
            if let Err(err) = fs::create_dir_all(output_dir) {
                eprintln!(
                    "Failed to create output directory '{}': {}",
                    output_dir, err
                );
                return;
            }

            // Parse tables from schema
            let tables = parse_tables(&schema);
            if tables.is_empty() {
                eprintln!("Error: No tables found in schema");
                return;
            }

            println!("Successfully parsed {} tables from schema", tables.len());

            // Generate proto content
            let proto_content = generate_unified_proto(&tables);

            // Write proto file
            let file_path = Path::new(output_dir).join("store.proto");
            match File::create(&file_path) {
                Ok(mut file) => match file.write_all(proto_content.as_bytes()) {
                    Ok(_) => println!("Successfully wrote proto content to file"),
                    Err(err) => eprintln!("Failed to write proto content to file: {}", err),
                },
                Err(err) => eprintln!("Failed to create proto file: {}", err),
            }

            println!("Proto file generation completed!");
        }
        Err(err) => {
            eprintln!("Failed to read schema file '{}': {}", schema_path, err);
        }
    }
}

fn clean_output_directory(output_dir: &str) -> std::io::Result<()> {
    println!("Cleaning output directory: {}", output_dir);

    if let Ok(entries) = fs::read_dir(output_dir) {
        for entry in entries {
            match entry {
                Ok(entry) => {
                    let path = entry.path();
                    if path.is_file() && path.extension().map_or(false, |ext| ext == "proto") {
                        println!("Deleting existing proto file: {:?}", path);
                        if let Err(err) = fs::remove_file(&path) {
                            eprintln!(
                                "Warning: Failed to delete file '{}': {}",
                                path.display(),
                                err
                            );
                            // Continue despite deletion errors
                        }
                    }
                }
                Err(err) => {
                    eprintln!("Warning: Failed to read directory entry: {}", err);
                    // Continue despite read errors
                }
            }
        }
    }

    Ok(())
}

pub fn generate_unified_proto(tables: &[Table]) -> String {
    let mut proto = String::new();

    // Header with more imports and documentation
    proto.push_str("syntax = \"proto3\";\n\n");
    proto.push_str("// Generated automatically from database schema\n");
    proto.push_str("// Do not edit manually\n\n");
    proto.push_str("package store;\n\n");

    // Import Google's well-known types if needed
    // proto.push_str("import \"google/protobuf/timestamp.proto\";\n\n");

    // Timestamp definition
    proto.push_str("// Standard timestamp representation\n");
    proto.push_str("message Timestamp {\n");
    proto.push_str("  int64 seconds = 1; // Seconds since epoch\n");
    proto.push_str("  int32 nanos = 2;   // Nanoseconds offset\n");
    proto.push_str("}\n\n");

    // Common structures for Create requests
    proto.push_str("// Common parameter structure for Create requests\n");
    proto.push_str("message CreateParams {\n");
    proto.push_str("  string table = 1; // Table name\n");
    proto.push_str("}\n\n");

    proto.push_str("// Common query structure for Create requests\n");
    proto.push_str("message CreateQuery {\n");
    proto.push_str("  string pluck = 1; // Field to pluck (e.g., \"id\")\n");
    proto.push_str("  string durability = 2; // Durability level (e.g., \"soft\")\n");
    proto.push_str("}\n\n");

    //Common structure for AdvanceFilter
    proto.push_str("// Common parameter structure for AdvanceFilter requests\n");
    proto.push_str("message AdvanceFilter {\n");
    proto.push_str("  string entity = 1; // Table name\n");
    proto.push_str("  string type = 2; // Filter type criteria or operator\n");
    proto.push_str("  string field = 3; // Column name\n");
    proto.push_str("  string operator = 4; // Equal, not equal etc\n");
    proto.push_str("  string values = 5; // JSON string of values\n");
    proto.push_str("}\n\n");

    // Add BatchUpdate common structures
    proto.push_str("// Common parameter structure for BatchUpdate requests\n");
    proto.push_str("message BatchUpdateParams {\n");
    proto.push_str("  string table = 1; // Table name\n");
    proto.push_str("}\n\n");

    // Add BatchDelete common structures
    proto.push_str("// Common parameter structure for BatchDelete requests\n");
    proto.push_str("message BatchDeleteParams {\n");
    proto.push_str("  string table = 1; // Table name\n");
    proto.push_str("}\n\n");

    // Common structures for Update requests
    proto.push_str("// Common parameter structure for Update requests\n");
    proto.push_str("message UpdateParams {\n");
    proto.push_str("  string id = 1; // Record ID\n");
    proto.push_str("  string table = 2; // Table name\n");
    proto.push_str("}\n\n");

    proto.push_str("// Common query structure for Update requests\n");
    proto.push_str("message UpdateQuery {\n");
    proto.push_str("  string pluck = 1; // Field to pluck (e.g., \"id,code\")\n");
    proto.push_str("}\n\n");

    // Add BatchInsert common structures
    proto.push_str("// Common parameter structure for BatchInsert requests\n");
    proto.push_str("message BatchInsertParams {\n");
    proto.push_str("  string table = 1; // Table name\n");
    proto.push_str("}\n\n");

    proto.push_str("// Common query structure for BatchInsert requests\n");
    proto.push_str("message BatchInsertQuery {\n");
    proto.push_str("  string pluck = 1; // Field to pluck (e.g., \"id\")\n");
    proto.push_str("}\n\n");

    // Add Upsert common structures
    proto.push_str("// Common parameter structure for Upsert requests\n");
    proto.push_str("message UpsertParams {\n");
    proto.push_str("  string table = 1; // Table name\n");
    proto.push_str("}\n\n");

    proto.push_str("// Common query structure for Upsert requests\n");
    proto.push_str("message UpsertQuery {\n");
    proto.push_str("  string pluck = 1; // Field to pluck (e.g., \"id,code,source_ip\")\n");
    proto.push_str("}\n\n");

    // Common query structure for Delete requests
proto.push_str("// Common query structure for Delete requests\n");
proto.push_str("message DeleteQuery {\n");
proto.push_str("  string is_permanent = 1; // Whether to perform permanent deletion\n");
proto.push_str("}\n\n");

proto.push_str("// Common parameter structure for Delete requests\n");
    proto.push_str("message DeleteParams {\n");
    proto.push_str("  string id = 1; // Record ID\n");
    proto.push_str("  string table = 2; // Table name\n");
    proto.push_str("}\n\n");

    // Generate all data messages first
    for table in tables {
        proto.push_str(&format!(
            "// {} entity definition\n",
            table.name.to_case(Case::Pascal)
        ));
        proto.push_str(&format!(
            "message {} {{\n",
            table.name.to_case(Case::Pascal)
        ));

        // Fields with comments
        for (i, field) in table.fields.iter().enumerate() {
            let field_number = i + 1;
            let type_prefix = if field.is_optional { "optional " } else { "" };
            let type_prefix = if field.is_array {
                "repeated "
            } else {
                type_prefix
            };

            proto.push_str(&format!(
                "  {}{} {} = {};\n",
                type_prefix,
                field.proto_type,
                field.name.to_case(Case::Snake),
                field_number
            ));
        }
        proto.push_str("}\n\n");
    }

    // Generate request/response messages for CRUD operations
    for table in tables {
        let pascal_name = table.name.to_case(Case::Pascal);
        let snake_name = table.name.to_case(Case::Snake);
        let singular_name = to_singular(&table.name);

        // Create operation
        proto.push_str(&format!("// Create {} request\n", pascal_name));
        proto.push_str(&format!("message Create{}Request {{\n", pascal_name));
        proto.push_str(&format!("  {} {} = 1;\n", pascal_name, snake_name));
        proto.push_str("  CreateParams params = 2;\n");
        proto.push_str("  CreateQuery query = 3;\n");
        proto.push_str("  string entity_prefix = 4; // Entity prefix code\n");
        proto.push_str("}\n\n");

        // Create response
        proto.push_str(&format!("// Create {} response\n", pascal_name));
        proto.push_str(&format!("message Create{}Response {{\n", pascal_name));
        proto.push_str("  bool success = 1;\n");
        proto.push_str("  int32 count = 2;\n");
        proto.push_str("  string message = 3;\n");
        proto.push_str(&format!("  {} data = 4;\n", pascal_name));
        proto.push_str("}\n\n");

        // Get operation
        proto.push_str(&format!("// Get {} request\n", pascal_name));
        proto.push_str(&format!("message Get{}Request {{\n", pascal_name));
        proto.push_str("  string id = 1;\n");
        proto.push_str("}\n\n");

        proto.push_str(&format!("// Get {} response\n", pascal_name));
        proto.push_str(&format!("message Get{}Response {{\n", pascal_name));
        proto.push_str("  bool success = 1;\n");
        proto.push_str("  string message = 2;\n");
        proto.push_str(&format!("  {} data = 3;\n", pascal_name));
        proto.push_str("}\n\n");

        // Update operation
        proto.push_str(&format!("// Update {} request\n", pascal_name));
        proto.push_str(&format!("message Update{}Request {{\n", pascal_name));
        proto.push_str(&format!("  {} {} = 1;\n", pascal_name, singular_name));
        proto.push_str("  UpdateParams params = 2;\n");
        proto.push_str("  UpdateQuery query = 3;\n");
        proto.push_str("}\n\n");

        proto.push_str(&format!("// Update {} response\n", pascal_name));
        proto.push_str(&format!("message Update{}Response {{\n", pascal_name));
        proto.push_str("  bool success = 1;\n");
        proto.push_str("  int32 count = 2;\n");
        proto.push_str("  string message = 3;\n");
        proto.push_str(&format!("  {} data = 4;\n", pascal_name));
        proto.push_str("}\n\n");

        // BatchUpdate operation
        proto.push_str(&format!("// BatchUpdate {} request\n", pascal_name));
        proto.push_str(&format!("message BatchUpdate{}Request {{\n", pascal_name));
        proto.push_str("  BatchUpdateParams params = 1;\n");
        proto.push_str("  message BatchUpdateBody {\n");
        proto.push_str("    repeated AdvanceFilter advance_filters = 1;\n");
        proto.push_str(&format!("  {} updates = 2;\n", pascal_name));
        proto.push_str("  }\n");
        proto.push_str("  BatchUpdateBody body = 2;\n");
        proto.push_str("}\n\n");

        proto.push_str(&format!("// BatchUpdate {} response\n", pascal_name));
        proto.push_str(&format!("message BatchUpdate{}Response {{\n", pascal_name));
        proto.push_str("  bool success = 1;\n");
        proto.push_str("  string message = 2;\n");
        proto.push_str("  int32 count = 3;\n");
        proto.push_str(&format!("  repeated {} data = 4;\n", pascal_name));
        proto.push_str("}\n\n");

        // BatchDelete operation
        proto.push_str(&format!("// BatchDelete {} request\n", pascal_name));
        proto.push_str(&format!("message BatchDelete{}Request {{\n", pascal_name));
        proto.push_str("  BatchDeleteParams params = 1;\n");
        proto.push_str("  message BatchDeleteBody {\n");
        proto.push_str("    repeated AdvanceFilter advance_filters = 1;\n");
        proto.push_str("  }\n");
        proto.push_str("  BatchDeleteBody body = 2;\n");
        proto.push_str("}\n\n");

        proto.push_str(&format!("// BatchDelete {} response\n", pascal_name));
        proto.push_str(&format!("message BatchDelete{}Response {{\n", pascal_name));
        proto.push_str("  bool success = 1;\n");
        proto.push_str("  string message = 2;\n");
        proto.push_str("  int32 count = 3;\n");
        proto.push_str(&format!("  {} data = 4;\n", pascal_name));
        proto.push_str("}\n\n");

        // Delete operation
        proto.push_str(&format!("// Delete {} request\n", pascal_name));
          proto.push_str(&format!("message Delete{}Request {{\n", pascal_name));
          proto.push_str("  DeleteParams params = 1;\n");
          proto.push_str("  DeleteQuery query = 2;\n");
          proto.push_str("}\n\n");

        proto.push_str(&format!("// Delete {} response\n", pascal_name));
        proto.push_str(&format!("message Delete{}Response {{\n", pascal_name));
        proto.push_str("  bool success = 1;\n");
        proto.push_str("  string message = 2;\n");
        proto.push_str("  int32 count = 3;\n");
        proto.push_str(&format!("  {} data = 4;\n", pascal_name));
        proto.push_str("}\n\n");

        // BatchInsert operation
        proto.push_str(&format!("// BatchInsert {} request\n", pascal_name));
        proto.push_str(&format!("message BatchInsert{}Request {{\n", pascal_name));
        proto.push_str("  BatchInsertParams params = 1;\n");
        proto.push_str("  BatchInsertQuery query = 2;\n");
        proto.push_str("  message BatchBody {\n");
        proto.push_str("    string entity_prefix = 1;\n");
        proto.push_str(&format!(
            "    repeated {} {} = 2;\n",
            pascal_name, snake_name
        ));
        proto.push_str("  }\n");
        proto.push_str("  BatchBody body = 3;\n");
        proto.push_str("}\n\n");

        // BatchInsert response
        proto.push_str(&format!("// BatchInsert {} response\n", pascal_name));
        proto.push_str(&format!("message BatchInsert{}Response {{\n", pascal_name));
        proto.push_str("  bool success = 1;\n");
        proto.push_str("  string message = 2;\n");
        proto.push_str("  int32 count = 3;\n");
        proto.push_str(&format!("  repeated {} data = 4;\n", pascal_name));
        proto.push_str("}\n\n");

          // Upsert operation
          proto.push_str(&format!("// Upsert {} request\n", pascal_name));
          proto.push_str(&format!("message Upsert{}Request {{\n", pascal_name));
          proto.push_str("  UpsertParams params = 1;\n");
          proto.push_str("  UpsertQuery query = 2;\n");
          proto.push_str("  message UpsertBody {\n");
          proto.push_str("    repeated string conflict_columns = 1;\n");
          proto.push_str(&format!("    {} data = 2;\n", pascal_name));
          proto.push_str("    string entity_prefix = 3;\n");
          proto.push_str("  }\n");
          proto.push_str("  UpsertBody body = 3;\n");
          proto.push_str("}\n\n");
  
          proto.push_str(&format!("// Upsert {} response\n", pascal_name));
          proto.push_str(&format!("message Upsert{}Response {{\n", pascal_name));
          proto.push_str("  bool success = 1;\n");
          proto.push_str("  string message = 2;\n");
          proto.push_str("  int32 count = 3;\n");
          proto.push_str(&format!("  repeated {} data = 4;\n", pascal_name));
          proto.push_str("}\n\n");

          
    }

    // Generate batch operations for multiple records - only once

    // Generate unified service with full CRUD operations
    proto.push_str("// Store service definition with CRUD operations\n");
    proto.push_str("service StoreService {\n");

    for table in tables {
        let pascal_name = table.name.to_case(Case::Pascal);

        // Create
        proto.push_str(&format!("  // Create a new {}\n", pascal_name));
        proto.push_str(&format!(
            "  rpc Create{}(Create{}Request) returns (Create{}Response);\n\n",
            pascal_name, pascal_name, pascal_name
        ));

        // Get
        proto.push_str(&format!("  // Get a {} by ID\n", pascal_name));
        proto.push_str(&format!(
            "  rpc Get{}(Get{}Request) returns (Get{}Response);\n\n",
            pascal_name, pascal_name, pascal_name
        ));

        // Update
        proto.push_str(&format!("  // Update an existing {}\n", pascal_name));
        proto.push_str(&format!(
            "  rpc Update{}(Update{}Request) returns (Update{}Response);\n\n",
            pascal_name, pascal_name, pascal_name
        ));

        // Delete
        proto.push_str(&format!("  // Delete a {} by ID\n", pascal_name));
        proto.push_str(&format!(
            "  rpc Delete{}(Delete{}Request) returns (Delete{}Response);\n\n",
            pascal_name, pascal_name, pascal_name
        ));

        // BatchInsert
        proto.push_str(&format!("  // Batch insert multiple {}s\n", pascal_name));
        proto.push_str(&format!(
            "  rpc BatchInsert{}(BatchInsert{}Request) returns (BatchInsert{}Response);\n\n",
            pascal_name, pascal_name, pascal_name
        ));

        // BatchUpdate
        proto.push_str(&format!(
            "  // Batch update multiple {}s based on filters\n",
            pascal_name
        ));
        proto.push_str(&format!(
            "  rpc BatchUpdate{}(BatchUpdate{}Request) returns (BatchUpdate{}Response);\n\n",
            pascal_name, pascal_name, pascal_name
        ));

        // BatchDelete
        proto.push_str(&format!(
            "  // Batch delete multiple {}s based on filters\n",
            pascal_name
        ));
        proto.push_str(&format!(
            "  rpc BatchDelete{}(BatchDelete{}Request) returns (BatchDelete{}Response);\n\n",
            pascal_name, pascal_name, pascal_name
        ));

         //Upsert
         proto.push_str(&format!("  // Upsert a {} (create if not exists, update if exists)\n", pascal_name));
         proto.push_str(&format!(
             "  rpc Upsert{}(Upsert{}Request) returns (Upsert{}Response);\n\n",
             pascal_name, pascal_name, pascal_name
         ));

    }

    proto.push_str("}\n");

    proto
}

pub fn generate_build_file(proto_dir: &str) -> std::io::Result<()> {
    // Get all proto files in the directory
    let mut proto_files = Vec::new();

    if let Ok(entries) = fs::read_dir(proto_dir) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.is_file() && path.extension().map_or(false, |ext| ext == "proto") {
                    if let Some(file_name) = path.file_name() {
                        if let Some(file_str) = file_name.to_str() {
                            // Fix the path format
                            let relative_path = format!("{}/{}", proto_dir, file_str);
                            proto_files.push(relative_path);
                        }
                    }
                }
            }
        }
    }

    // Create build.rs content
    let mut build_content = String::new();
    build_content.push_str("fn main() -> Result<(), Box<dyn std::error::Error>> {\n");

    // Add rerun-if-changed for all proto files
    for proto_file in &proto_files {
        build_content.push_str(&format!(
            "    println!(\"cargo:rerun-if-changed={}\");\n",
            proto_file
        ));
    }

    build_content.push_str("\n    tonic_build::configure()\n");
    build_content.push_str("        .build_server(true)   // Enable server code (default)\n");
    build_content.push_str("        .build_client(false)   // Enable client code (default)\n");
    build_content.push_str("        .out_dir(\"src/generated\") // Custom output directory\n");
    build_content.push_str("        .compile_protos(\n");

    // Add all proto files to the compile function
    build_content.push_str("            &[\n");
    for proto_file in &proto_files {
        build_content.push_str(&format!("                \"{}\",\n", proto_file));
    }
    build_content.push_str("            ],\n");
    build_content.push_str("            &[\"src\"],\n");
    build_content.push_str("        )?;\n\n");

    build_content.push_str("    println!(\"cargo:warning=Successfully compiled proto files\");\n");
    build_content.push_str("    Ok(())\n");
    build_content.push_str("}\n");

    // Write to build.rs file
    let build_path = Path::new("build.rs");
    let mut file = File::create(build_path)?;
    file.write_all(build_content.as_bytes())?;

    println!(
        "Generated build.rs file with {} proto files",
        proto_files.len()
    );
    Ok(())
}

pub fn diesel_type_to_proto(diesel_type: &str) -> &'static str {
    match diesel_type {
        t if t.contains("Int4") => "int32",
        t if t.contains("Integer") => "int32",
        t if t.contains("BigInt") => "int64",
        t if t.contains("Uuid") => "string",
        t if t.contains("Text") => "string",
        t if t.contains("Varchar") => "string",
        t if t.contains("Timestamp") => "string",
        t if t.contains("Array") => "string", // For arrays of basic types
        t if t.contains("Inet") => "string",
        _ => "string", // Default fallback
    }
}

// Add case conversion trait and implementation
pub trait CaseConvert {
    fn to_case(&self, case: Case) -> String;
}

impl CaseConvert for str {
    fn to_case(&self, case: Case) -> String {
        match case {
            Case::Snake => self.to_string(),
            Case::Pascal => {
                let mut result = String::new();
                let mut capitalize = true;

                for c in self.chars() {
                    if c == '_' {
                        capitalize = true;
                    } else {
                        if capitalize {
                            result.push(c.to_ascii_uppercase());
                            capitalize = false;
                        } else {
                            result.push(c);
                        }
                    }
                }

                result
            }
        }
    }
}

pub enum Case {
    Snake,
    Pascal,
}

fn to_singular(name: &str) -> String {
    let name = name.to_lowercase();

    // Handle common plural endings
    if name.ends_with("ies") {
        return format!("{}y", &name[0..name.len() - 3]);
    } else if name.ends_with("ses") {
        return format!("{}s", &name[0..name.len() - 2]);
    } else if name.ends_with("s") && !name.ends_with("ss") {
        return name[0..name.len() - 1].to_string();
    }

    // Return original if no plural pattern matched
    name
}
