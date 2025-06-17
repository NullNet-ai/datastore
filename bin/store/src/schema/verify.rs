use regex::Regex;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
const SCHEMA_CONTENT: &str = include_str!("schema.rs");

/// Verifies if a field exists in a specified table by parsing the schema.rs file
///
/// # Arguments
///
/// * `table_name` - The name of the table to check
/// * `field_name` - The name of the field to verify
///
/// # Returns
///
/// * `bool` - True if the field exists in the table, false otherwise
///
pub fn field_exists_in_table(table_name: &str, field_name: &str) -> bool {
    // Path to schema.rs file
    let possible_paths = vec![Path::new("src/schema/schema.rs"), Path::new("schema.rs")];
    // Read the schema file
    let mut schema_content = String::new();
    for path in possible_paths {
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
    // This pattern is more flexible to match the actual format in schema.rs
    let table_pattern = format!(
        r"(?s)table!\s*\{{\s*{}\s*\([^)]*\)\s*\{{(.*?)\}}\s*\}}",
        regex::escape(table_name)
    );
    let table_regex = match Regex::new(&table_pattern) {
        Ok(re) => re,
        Err(e) => {
            log::error!("Failed to create table regex: {}", e);
            return false; // Return false if regex creation fails
        }
    };
    // Find the table definition
    if let Some(captures) = table_regex.captures(&schema_content) {
        if let Some(table_body) = captures.get(1) {
            // Get the table body content
            let table_content = table_body.as_str();

            // Create a regex pattern to find the field
            // This pattern matches field definitions more accurately
            let field_pattern = format!(r"(?m)^\s*{}\s*->\s*[^,]*", field_name);
            let field_regex = match Regex::new(&field_pattern) {
                Ok(re) => re,
                Err(e) => {
                    log::error!("Failed to create field regex: {}", e);
                    return false; // Return false if regex creation fails
                }
            };

            // Check if the field exists in the table
            let result = field_regex.is_match(table_content);
            return result;
        }
    }

    false // Return false if table not found
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FieldTypeInfo {
    pub is_array: bool,
    pub field_type: String,
    pub nullable: bool,
    pub is_json: bool,
}

pub fn field_type_in_table(table_name: &str, field_name: &str) -> Option<FieldTypeInfo> {
    // Path to schema.rs file
    let possible_paths = vec![Path::new("src/schema/schema.rs"), Path::new("schema.rs")];
    // Read the schema file
    let mut schema_content = String::new();
    for path in possible_paths {
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

            // Create a regex pattern to find the field and capture its type
            let field_pattern = format!(r"(?m)^\s*{}\s*->\s*([^,\s]+)", field_name);
            let field_regex = match Regex::new(&field_pattern) {
                Ok(re) => re,
                Err(e) => {
                    log::error!("Failed to create field regex: {}", e);
                    return None;
                }
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
                    let simplified_type = match processed_type.to_lowercase().as_str() {
                        "int4" | "integer" => "integer".to_string(),
                        "text" | "varchar" | "char" => "text".to_string(),
                        "float" | "float4" | "float8" | "double" => "float".to_string(),
                        "bool" => "bool".to_string(),
                        "timestamp" | "timestamptz" => "timestamp".to_string(),
                        "jsonb" | "json" => "json".to_string(),
                        _ => processed_type.to_lowercase(),
                    };

                    log::debug!("type_str: {}", type_str);
                    log::debug!("processed_type: {}", processed_type);
                    log::debug!("is_array: {}", is_array);
                    log::debug!("is_nullable: {}", is_nullable);
                    log::debug!("simplified_type: {}", simplified_type);

                    return Some(FieldTypeInfo {
                        is_array,
                        field_type: simplified_type,
                        nullable: is_nullable,
                        is_json,
                    });
                }
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_field_exists() {
        // Test with existing table and field
        assert!(field_exists_in_table("items", "name"));

        // Test with existing table but non-existing field
        assert!(!field_exists_in_table("items", "nonexistent_field"));

        // Test with non-existing table
        assert!(!field_exists_in_table("nonexistent_table", "name"));
    }
}
