use crate::constants::paths;
use paths::database::SCHEMA_FILE;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
const SCHEMA_CONTENT: &str = SCHEMA_FILE;

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
    let possible_paths = vec![
        Path::new(paths::database::SCHEMA_FILE),
        Path::new(paths::LEGACY_SCHEMA_FILE),
    ];
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
    let possible_paths = vec![
        Path::new(paths::database::SCHEMA_FILE),
        Path::new(paths::LEGACY_SCHEMA_FILE),
    ];
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

            if let Some(captures) = field_regex.captures(table_content) {
                if let Some(type_match) = captures.get(1) {
                    let field_type_str = type_match.as_str();
                    log::debug!("Found field type: {}", field_type_str);

                    // Parse the type to extract information
                    let is_nullable = field_type_str.starts_with("Nullable<");
                    let is_array = field_type_str.contains("Array<");
                    let is_json =
                        field_type_str.contains("Jsonb") || field_type_str.contains("Json");

                    // Extract the base type
                    let mut simplified_type = field_type_str.to_string();

                    // Remove Nullable wrapper
                    if is_nullable {
                        simplified_type = simplified_type
                            .strip_prefix("Nullable<")
                            .unwrap_or(&simplified_type)
                            .strip_suffix(">")
                            .unwrap_or(&simplified_type)
                            .to_string();
                    }

                    // Remove Array wrapper
                    if is_array {
                        simplified_type = simplified_type
                            .strip_prefix("Array<")
                            .unwrap_or(&simplified_type)
                            .strip_suffix(">")
                            .unwrap_or(&simplified_type)
                            .to_string();
                    }

                    // Convert diesel types to more readable names
                    simplified_type = match simplified_type.as_str() {
                        "Text" => "String".to_string(),
                        "Int4" => "i32".to_string(),
                        "Int8" => "i64".to_string(),
                        "Bool" => "bool".to_string(),
                        "Timestamp" => "NaiveDateTime".to_string(),
                        "Timestamptz" => "DateTime<Utc>".to_string(),
                        "Jsonb" => "serde_json::Value".to_string(),
                        "Json" => "serde_json::Value".to_string(),
                        "Inet" => "ipnetwork::IpNetwork".to_string(),
                        _ => simplified_type,
                    };

                    let is_nullable = is_nullable;
                    log::debug!("is_array: {}", is_array);
                    log::debug!("is_json: {}", is_json);
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
    // Path to schema.rs file
    let possible_paths = vec![
        Path::new(paths::database::SCHEMA_FILE),
        Path::new(paths::LEGACY_SCHEMA_FILE),
    ];
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

            // Create a regex pattern to find all field definitions
            let field_pattern = r"(?m)^\s*([a-zA-Z_][a-zA-Z0-9_]*)\s*->";
            let field_regex = match Regex::new(field_pattern) {
                Ok(re) => re,
                Err(e) => {
                    log::error!("Failed to create field regex: {}", e);
                    return None;
                }
            };

            // Collect all field names
            let mut fields = Vec::new();
            for captures in field_regex.captures_iter(table_content) {
                if let Some(field_match) = captures.get(1) {
                    fields.push(field_match.as_str().to_string());
                }
            }

            if !fields.is_empty() {
                return Some(fields);
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

    #[test]
    fn test_get_table_fields() {
        // Test with existing table
        if let Some(fields) = get_table_fields("items") {
            assert!(!fields.is_empty());
            // Check that common fields exist
            assert!(fields.contains(&"id".to_string()));
        }

        // Test with non-existing table
        assert!(get_table_fields("nonexistent_table").is_none());
    }
}
