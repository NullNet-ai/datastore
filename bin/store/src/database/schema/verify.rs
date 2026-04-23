use crate::constants::paths;
use crate::database::schema::reserved_keywords;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::path::PathBuf;
const SCHEMA_CONTENT: &str = include_str!("../../generated/schema.rs");

pub fn field_exists_in_table(table_name: &str, field_name: &str) -> bool {
    // Use get_table_fields which handles #[sql_name] mapping (e.g. columns_data -> columns)
    get_table_fields(table_name).map_or(false, |fields| fields.contains(&field_name.to_string()))
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FieldTypeInfo {
    pub is_array: bool,
    pub field_type: String,
    pub nullable: bool,
    pub is_json: bool,
    /// True when the DB type is timestamptz (with time zone). Used to serialize for DateTime<Utc> vs NaiveDateTime.
    pub is_timestamptz: bool,
}

pub fn field_type_in_table(table_name: &str, field_name: &str) -> Option<FieldTypeInfo> {
    let mut possible_paths: Vec<PathBuf> = Vec::new();
    possible_paths.push(PathBuf::from(paths::database::schema_file()));
    possible_paths.push(PathBuf::from(paths::legacy_schema_file()));
    if let Ok(exe) = env::current_exe() {
        if let Some(dir) = exe.parent() {
            possible_paths.push(dir.join(paths::database::schema_file()));
            possible_paths.push(dir.join(paths::legacy_schema_file()));
        }
    }
    let mut schema_content = String::new();
    for path in possible_paths.iter() {
        if let Ok(content) = fs::read_to_string(&path) {
            schema_content = content;
            break;
        }
    }

    if schema_content.is_empty() {
        log::error!("Could not find schema file");
        schema_content = SCHEMA_CONTENT.to_string();
    }

    // Create a regex pattern to find the table definition
    let table_pattern = format!(
        r"(?s)table!\s*\{{\s*{}\s*\([^)]*\)\s*\{{(.*?)\}}\s*\}}",
        regex::escape(table_name)
    );
    let table_regex = match Regex::new(&table_pattern) {
        Ok(re) => re,
        Err(e) => {
            log::error!("Failed to create table regex: {}", e);
            return None;
        }
    };

    // Find the table definition
    if let Some(captures) = table_regex.captures(&schema_content) {
        if let Some(table_body) = captures.get(1) {
            // Get the table body content
            let table_content = table_body.as_str();

            // Try field_name first; for reserved keywords also try rust_identifier (schema renames it)
            let rust_identifiers: Vec<String> = if reserved_keywords::is_reserved(field_name) {
                vec![
                    field_name.to_string(),
                    reserved_keywords::rust_identifier(field_name),
                ]
            } else {
                vec![field_name.to_string()]
            };

            for rust_id in &rust_identifiers {
                let field_pattern = format!(r"(?m)^\s*{}\s*->\s*([^,\s]+)", regex::escape(rust_id));
                let field_regex = match Regex::new(&field_pattern) {
                    Ok(re) => re,
                    Err(_) => continue,
                };

                // Extract the field type if found
                if let Some(field_captures) = field_regex.captures(table_content) {
                    if let Some(field_type) = field_captures.get(1) {
                        let type_str = field_type.as_str().to_string();

                        // Parse the type string to extract information
                        let is_array = type_str.to_lowercase().contains("array");
                        let is_nullable = type_str.to_lowercase().contains("nullable");
                        let is_json = type_str.to_lowercase().contains("json")
                            || type_str.to_lowercase().contains("jsonb");

                        // Extract the base type - process the type string step by step
                        let mut processed_type = type_str.clone();

                        // First, handle array if present
                        if is_array {
                            let re = Regex::new(r"(?i)array<([^>]+)>?").unwrap();
                            if let Some(caps) = re.captures(&processed_type) {
                                if let Some(inner_type) = caps.get(1) {
                                    processed_type = inner_type.as_str().to_string();
                                }
                            }
                        }

                        // Then, handle nullable if present
                        if is_nullable {
                            let re = Regex::new(r"(?i)nullable<([^>]+)>?").unwrap();
                            if let Some(caps) = re.captures(&processed_type) {
                                if let Some(inner_type) = caps.get(1) {
                                    processed_type = inner_type.as_str().to_string();
                                }
                            }
                        }

                        // Simplify the base type
                        let lower = processed_type.to_lowercase();
                        let is_timestamptz = lower == "timestamptz";
                        let simplified_type = match lower.as_str() {
                            "int4" | "integer" => "integer".to_string(),
                            "text" | "varchar" | "char" => "text".to_string(),
                            "float" | "float4" | "float8" | "double" => "float".to_string(),
                            "bool" => "bool".to_string(),
                            "timestamp" | "timestamptz" => "timestamp".to_string(),
                            "jsonb" | "json" => "json".to_string(),
                            "inet" | "cidr" => "inet".to_string(),
                            "uuid" => "uuid".to_string(),
                            "bytea" => "bytea".to_string(),
                            "numeric" | "decimal" => "numeric".to_string(),
                            _ => processed_type.to_lowercase(),
                        };

                        return Some(FieldTypeInfo {
                            is_array,
                            field_type: simplified_type,
                            nullable: is_nullable,
                            is_json,
                            is_timestamptz,
                        });
                    }
                }
            }
        }
    }

    None
}

/// Gets all field names for a specified table by parsing the schema.rs file
///
/// # Arguments
///
/// * `table_name` - The name of the table to get fields for
///
/// # Returns
///
/// * `Option<Vec<String>>` - Vector of field names if table exists, None otherwise
///
pub fn get_table_fields(table_name: &str) -> Option<Vec<String>> {
    let mut possible_paths: Vec<PathBuf> = Vec::new();
    possible_paths.push(PathBuf::from(paths::database::schema_file()));
    possible_paths.push(PathBuf::from(paths::legacy_schema_file()));
    if let Ok(exe) = env::current_exe() {
        if let Some(dir) = exe.parent() {
            possible_paths.push(dir.join(paths::database::schema_file()));
            possible_paths.push(dir.join(paths::legacy_schema_file()));
        }
    }
    let mut schema_content = String::new();
    for path in possible_paths.iter() {
        if let Ok(content) = fs::read_to_string(&path) {
            schema_content = content;
            break;
        }
    }

    if schema_content.is_empty() {
        log::error!("Could not find schema file");
        schema_content = SCHEMA_CONTENT.to_string();
    }

    // Create a regex pattern to find the table definition
    let table_pattern = format!(
        r"(?s)table!\s*\{{\s*{}\s*\([^)]*\)\s*\{{(.*?)\}}\s*\}}",
        regex::escape(table_name)
    );
    let table_regex = match Regex::new(&table_pattern) {
        Ok(re) => re,
        Err(e) => {
            log::error!("Failed to create table regex: {}", e);
            return None;
        }
    };

    // Find the table definition
    if let Some(captures) = table_regex.captures(&schema_content) {
        if let Some(table_body) = captures.get(1) {
            // Get the table body content
            let table_content = table_body.as_str();

            // Match field definitions, including #[sql_name = "X"] for reserved names like "columns"
            let field_pattern = r#"(?ms)(?:#\[sql_name\s*=\s*"([^"]+)"\]\s*\n)?\s*([a-zA-Z_][a-zA-Z0-9_]*)\s*->\s*[^,]*"#;
            let field_regex = match Regex::new(field_pattern) {
                Ok(re) => re,
                Err(e) => {
                    log::error!("Failed to create field regex: {}", e);
                    return None;
                }
            };

            // Extract all field names (use sql_name when present, else Rust identifier)
            let mut fields = Vec::new();
            for captures in field_regex.captures_iter(table_content) {
                let field_name = if let Some(sql_name) = captures.get(1) {
                    sql_name.as_str().to_string()
                } else if let Some(rust_name) = captures.get(2) {
                    rust_name.as_str().to_string()
                } else {
                    continue;
                };
                fields.push(field_name);
            }

            if !fields.is_empty() {
                return Some(fields);
            }
        }
    }

    None
}
