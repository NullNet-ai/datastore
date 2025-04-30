use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

pub fn generate_protos(schema_path: &str, output_dir: &str) -> std::io::Result<()> {
    let schema = fs::read_to_string(schema_path)?;
    println!("reached here");
    if let Ok(entries) = fs::read_dir(output_dir) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.is_file() && path.extension().map_or(false, |ext| ext == "proto") {
                    println!("Deleting existing proto file: {:?}", path);
                    fs::remove_file(path)?;
                }
            }
        }
    }
    
    fs::create_dir_all(output_dir)?;

    let tables = parse_tables(&schema);
    
    // Generate a single proto file containing all messages and services
    let proto_content = generate_unified_proto(&tables);
    let file_path = Path::new(output_dir).join("store.proto");
    let mut file = File::create(file_path)?;
    file.write_all(proto_content.as_bytes())?;
    println!("Generated unified proto file");

    // generate_build_file(output_dir)?;

    println!("Proto file generation complete!");
    Ok(())
}

fn parse_tables(schema: &str) -> Vec<Table> {
    let mut tables = Vec::new();
    let mut current_table: Option<Table> = None;
    let mut bracket_depth = 0;
    let mut in_table_def = false;
    let mut table_name = String::new();

    for line in schema.lines() {
        let line = line.trim();
        
        // Skip empty lines and comments
        if line.is_empty() || line.starts_with("//") {
            continue;
        }
        
        // Start of table definition
        if line.starts_with("table!") {
            in_table_def = true;
            bracket_depth = 0;
        }
        
        if in_table_def {
            // Count brackets for nesting level
            bracket_depth += line.chars().filter(|&c| c == '{').count();
            bracket_depth -= line.chars().filter(|&c| c == '}').count();
            
            // Extract table name from the line after "table!"
            if table_name.is_empty() && line.contains("(") && !line.starts_with("table!") {
                let name_part = line.split('(').next().unwrap_or("").trim();
                if !name_part.is_empty() {
                    table_name = name_part.to_string();
                    println!("Found table: {}", table_name);
                    current_table = Some(Table {
                        name: table_name.clone(),
                        fields: Vec::new(),
                    });
                }
            }
            
            // Parse field definitions
            if bracket_depth > 0 && line.contains("->") {
                if let Some(table) = &mut current_table {
                    let parts: Vec<&str> = line.split("->").collect();
                    if parts.len() == 2 {
                        let field_name = parts[0].trim().trim_end_matches(',');
                        let field_type = parts[1].trim().trim_end_matches(',');
                        
                        println!("Found field: {} -> {}", field_name, field_type);
                        
                        table.fields.push(Field {
                            name: field_name.to_string(),
                            proto_type: diesel_type_to_proto(field_type),
                            is_optional: field_type.contains("Nullable"),
                            is_array: field_type.contains("Array"),
                        });
                    }
                }
            }
            
            // End of table definition
            if bracket_depth == 0 && !table_name.is_empty() {
                if let Some(table) = current_table.take() {
                    if !table.fields.is_empty() {
                        tables.push(table.clone());
                        println!("Added table: {} with {} fields", table_name, table.fields.len());
                    }
                }
                table_name = String::new();
                in_table_def = false;
            }
        }
    }

    // Add any remaining table
    if let Some(table) = current_table {
        if !table.fields.is_empty() {
            tables.push(table);
        }
    }

    tables
}

pub fn generate_unified_proto(tables: &[Table]) -> String {
    let mut proto = String::new();
    
    // Header
    proto.push_str("syntax = \"proto3\";\n");
    proto.push_str("\n");
    proto.push_str("package store;\n");
    proto.push_str("\n");
    
    // Timestamp definition
    proto.push_str("message Timestamp {\n");
    proto.push_str("  int64 seconds = 1;\n");
    proto.push_str("  int32 nanos = 2;\n");
    proto.push_str("}\n\n");

    // Generate all messages first
    for table in tables {
        proto.push_str(&format!("message {} {{\n", table.name.to_case(Case::Pascal)));

        // Fields
        for (i, field) in table.fields.iter().enumerate() {
            let field_number = i + 1;
            let type_prefix = if field.is_optional { "optional " } else { "" };
            let type_prefix = if field.is_array { "repeated " } else { type_prefix };
            
            proto.push_str(&format!("  {}{} {} = {};\n", 
                type_prefix,
                field.proto_type,
                field.name.to_case(Case::Snake),
                field_number
            ));
        }
        proto.push_str("}\n\n");

        // CRUD messages
        proto.push_str(&format!("message Create{}Request {{\n", table.name.to_case(Case::Pascal)));
        proto.push_str(&format!("  {} {} = 1;\n", table.name.to_case(Case::Pascal), table.name.to_case(Case::Snake)));
        proto.push_str("}\n\n");

        proto.push_str(&format!("message Create{}Response {{\n", table.name.to_case(Case::Pascal)));
        proto.push_str("  bool success = 1;\n");
        proto.push_str("  string message = 2;\n");
        proto.push_str(&format!("  {} data = 3;\n", table.name.to_case(Case::Pascal)));
        proto.push_str("}\n\n");
    }

    // Generate unified service
    proto.push_str("service StoreService {\n");
    for table in tables {
        proto.push_str(&format!("  rpc Create{}(Create{}Request) returns (Create{}Response);\n",
            table.name.to_case(Case::Pascal),
            table.name.to_case(Case::Pascal),
            table.name.to_case(Case::Pascal)
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
        build_content.push_str(&format!("    println!(\"cargo:rerun-if-changed={}\");\n", proto_file));
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
    
    println!("Generated build.rs file with {} proto files", proto_files.len());
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
        t if t.contains("Timestamp") => "Timestamp",
        t if t.contains("Array") => "string", // For arrays of basic types
        _ => "string", // Default fallback
    }
}

#[derive(Clone)]
pub struct Table {
    name: String,
    fields: Vec<Field>,
}
#[derive(Clone)]
pub struct Field {
    name: String,
    proto_type: &'static str,
    is_optional: bool,
    is_array: bool,
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
