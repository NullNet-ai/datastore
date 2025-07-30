use crate::schema::generator::field_definition::{ParsedField, TableDefinition};
use std::fs;
use std::path::Path;

pub struct ModelGenerator;

impl ModelGenerator {
    /// Generate a Rust model file for the given table definition
    pub fn generate_model(table_def: &TableDefinition) -> Result<String, String> {
        let mut parsed_fields = Vec::new();
        
        // Parse all fields
        for field in &table_def.fields {
            match field.parse() {
                Ok(parsed) => parsed_fields.push(parsed),
                Err(e) => return Err(format!("Error parsing field {}: {}", field.field_name, e)),
            }
        }
        
        // Generate the model content
        let model_content = Self::generate_model_content(&table_def.table_name, &parsed_fields)?;
        Ok(model_content)
    }
    
    /// Generate the actual model file content
    fn generate_model_content(table_name: &str, fields: &[ParsedField]) -> Result<String, String> {
        let singular_name = Self::to_singular(table_name);
        let struct_name = Self::to_pascal_case(&singular_name);
        
        let mut content = String::new();
        
        // Add imports
        content.push_str(&format!("use crate::schema::schema::{};", table_name));
        content.push_str("\nuse diesel::prelude::*;");
        content.push_str("\nuse serde::{Deserialize, Serialize};");
        
        // Add additional imports based on field types
        let mut needs_chrono = false;
        let mut needs_serde_json = false;
        let mut needs_std_net = false;
        
        for field in fields {
            if field.rust_type.contains("chrono::") {
                needs_chrono = true;
            }
            if field.rust_type.contains("serde_json::") {
                needs_serde_json = true;
            }
            if field.rust_type.contains("std::net::") {
                needs_std_net = true;
            }
        }
        
        if needs_chrono {
            content.push_str("\nuse chrono::{DateTime, Utc};");
        }
        if needs_serde_json {
            content.push_str("\nuse serde_json::Value;");
        }
        if needs_std_net {
            content.push_str("\nuse std::net::IpAddr;");
        }
        
        content.push_str("\n\n");
        
        // Generate the struct
        content.push_str(&format!("#[derive(Queryable, Insertable, Serialize, Deserialize, Debug, Clone)]\n"));
        content.push_str(&format!("#[diesel(table_name = {})]\n", table_name));
        content.push_str(&format!("pub struct {} {{\n", struct_name));
        
        // Add fields
        for field in fields {
            content.push_str(&format!("    pub {}: {},\n", field.name, field.rust_type));
        }
        
        content.push_str("}\n");
        
        Ok(content)
    }
    
    /// Convert table name to singular form (simple implementation)
    fn to_singular(table_name: &str) -> String {
        if table_name.ends_with("ies") {
            format!("{}y", &table_name[..table_name.len()-3])
        } else if table_name.ends_with("s") && !table_name.ends_with("ss") {
            table_name[..table_name.len()-1].to_string()
        } else {
            table_name.to_string()
        }
    }
    
    /// Convert snake_case to PascalCase
    fn to_pascal_case(snake_str: &str) -> String {
        snake_str
            .split('_')
            .map(|word| {
                let mut chars = word.chars();
                match chars.next() {
                    None => String::new(),
                    Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
                }
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::schema::generator::field_definition::FieldDefinition;

    #[test]
    fn test_to_singular() {
        assert_eq!(ModelGenerator::to_singular("connections"), "connection");
        assert_eq!(ModelGenerator::to_singular("categories"), "category");
        assert_eq!(ModelGenerator::to_singular("address"), "address");
    }

    #[test]
    fn test_to_pascal_case() {
        assert_eq!(ModelGenerator::to_pascal_case("connection"), "Connection");
        assert_eq!(ModelGenerator::to_pascal_case("user_role"), "UserRole");
    }

    #[test]
    fn test_generate_model() {
        let table_def = TableDefinition {
            table_name: "users".to_string(),
            fields: vec![
                FieldDefinition {
                    field_name: "id".to_string(),
                    field_type: "Int4".to_string(),
                    is_index: true,
                    joins_with: None,
                    default_value: None,
                },
                FieldDefinition {
                    field_name: "name".to_string(),
                    field_type: "Text".to_string(),
                    is_index: false,
                    joins_with: None,
                    default_value: None,
                },
            ],
        };

        let result = ModelGenerator::generate_model(&table_def);
        assert!(result.is_ok());
        let content = result.unwrap();
        assert!(content.contains("pub struct User"));
        assert!(content.contains("pub id: i32"));
        assert!(content.contains("pub name: String"));
    }
}