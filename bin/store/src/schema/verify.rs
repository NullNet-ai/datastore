use regex::Regex;
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
