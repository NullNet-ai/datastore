use crate::schema::generator::field_definition::{ParsedField, TableDefinition};

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
        let mut import_sets = std::collections::HashSet::new();
        
        for field in fields {
            let type_dependencies = Self::extract_type_dependencies(&field.rust_type);
            import_sets.extend(type_dependencies);
        }
        
        // Add imports in a consistent order
        let mut imports: Vec<_> = import_sets.into_iter().collect();
        imports.sort();
        
        for import in imports {
            content.push_str(&format!("\n{}", import));
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
    
    fn extract_type_dependencies(rust_type: &str) -> Vec<String> {
        let mut dependencies = Vec::new();
        
        // Handle chrono types - be more specific about what's needed
        let mut chrono_imports = Vec::new();
        if rust_type.contains("DateTime") || rust_type.contains("chrono::DateTime") {
            chrono_imports.push("DateTime");
        }
        if rust_type.contains("NaiveDateTime") || rust_type.contains("chrono::NaiveDateTime") {
            chrono_imports.push("NaiveDateTime");
        }
        if rust_type.contains("Utc") || rust_type.contains("chrono::Utc") {
            chrono_imports.push("Utc");
        }
        if rust_type.contains("NaiveDate") || rust_type.contains("chrono::NaiveDate") {
            chrono_imports.push("NaiveDate");
        }
        if rust_type.contains("NaiveTime") || rust_type.contains("chrono::NaiveTime") {
            chrono_imports.push("NaiveTime");
        }
        
        if !chrono_imports.is_empty() {
            dependencies.push(format!("use chrono::{{{}}};", chrono_imports.join(", ")));
        }
        
        // Handle serde_json types
        if rust_type.contains("Value") || rust_type.contains("serde_json::") {
            dependencies.push("use serde_json::Value;".to_string());
        }
        
        // Handle std::net types
        if rust_type.contains("IpAddr") || rust_type.contains("std::net::") {
            dependencies.push("use std::net::IpAddr;".to_string());
        }
        
        // Handle UUID types
        if rust_type.contains("Uuid") || rust_type.contains("uuid::") {
            dependencies.push("use uuid::Uuid;".to_string());
        }
        
        // Handle BigDecimal types
        if rust_type.contains("BigDecimal") || rust_type.contains("bigdecimal::") {
            dependencies.push("use bigdecimal::BigDecimal;".to_string());
        }
        
        // Handle collections
        if rust_type.contains("HashMap") {
            dependencies.push("use std::collections::HashMap;".to_string());
        }
        if rust_type.contains("HashSet") {
            dependencies.push("use std::collections::HashSet;".to_string());
        }
        if rust_type.contains("BTreeMap") {
            dependencies.push("use std::collections::BTreeMap;".to_string());
        }
        if rust_type.contains("BTreeSet") {
            dependencies.push("use std::collections::BTreeSet;".to_string());
        }
        
        dependencies
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::schema::generator::field_definition::FieldDefinition;

    #[test]
    fn test_to_singular() {
        assert_eq!(ModelGenerator::to_singular("devices"), "device");
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
    
    #[test]
    fn test_extract_type_dependencies() {
        // Test complex types with multiple dependencies
        let deps = ModelGenerator::extract_type_dependencies("Option<DateTime<Utc>>");
        assert!(deps.contains(&"use chrono::{DateTime, Utc};".to_string()));
        
        let deps = ModelGenerator::extract_type_dependencies("HashMap<String, Value>");
        assert!(deps.contains(&"use std::collections::HashMap;".to_string()));
        assert!(deps.contains(&"use serde_json::Value;".to_string()));
        
        let deps = ModelGenerator::extract_type_dependencies("Vec<Option<BigDecimal>>");
        assert!(deps.contains(&"use bigdecimal::BigDecimal;".to_string()));
        
        // Test that no dependencies are returned for simple types
        let deps = ModelGenerator::extract_type_dependencies("String");
        assert!(deps.is_empty());
        
        let deps = ModelGenerator::extract_type_dependencies("i32");
        assert!(deps.is_empty());
    }
}