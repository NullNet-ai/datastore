use crate::schema::generator::field_definition::{FieldDefinition, TableDefinition};
use crate::schema::generator::model_generator::ModelGenerator;
use crate::schema::generator::schema_generator::SchemaGenerator;
use crate::schema::generator::migration_generator::MigrationGenerator;
use std::env;
use std::fs;
use std::path::Path;

pub struct GeneratorService;

impl GeneratorService {
    /// Main entry point for schema generation
    pub fn run() -> Result<(), String> {
        // Check if CREATE_SCHEMA flag is enabled
        if !Self::is_create_schema_enabled() {
            println!("CREATE_SCHEMA flag is not enabled. Skipping schema generation.");
            return Ok(());
        }
        
        println!("Starting schema generation...");
        
        // Find and process all table definition files
        let table_definitions = Self::discover_table_definitions()?;
        
        if table_definitions.is_empty() {
            println!("No table definition files found. Skipping schema generation.");
            return Ok(());
        }
        
        let mut all_changes = Vec::new();
        
        // Process each table definition
        for table_def in &table_definitions {
            println!("Processing table: {}", table_def.table_name);
            
            // Generate model
            ModelGenerator::generate_model(table_def)?;
            
            // Analyze schema changes
            let changes = SchemaGenerator::analyze_changes(table_def)?;
            
            if !changes.is_empty() {
                println!("Found {} changes for table {}", changes.len(), table_def.table_name);
                all_changes.extend(changes);
                
                // Update schema.rs
                SchemaGenerator::update_schema_file(table_def)?;
            } else {
                println!("No changes detected for table {}", table_def.table_name);
            }
        }
        
        // Generate migration if there are any changes
        if !all_changes.is_empty() {
            println!("Generating migration for {} total changes", all_changes.len());
            
            // For simplicity, we'll create one migration per table that has changes
            // Group changes by table
            let mut changes_by_table = std::collections::HashMap::new();
            for change in all_changes {
                changes_by_table.entry(change.table_name.clone())
                    .or_insert_with(Vec::new)
                    .push(change);
            }
            
            // Generate migration for each table with changes
            for (table_name, table_changes) in changes_by_table {
                if let Some(table_def) = table_definitions.iter().find(|t| t.table_name == table_name) {
                    MigrationGenerator::generate_migration(&table_changes, table_def)?;
                }
            }
        } else {
            println!("No changes detected across all tables. No migration needed.");
        }
        
        println!("Schema generation completed successfully!");
        Ok(())
    }
    
    /// Check if CREATE_SCHEMA environment variable is enabled
    fn is_create_schema_enabled() -> bool {
        match env::var("CREATE_SCHEMA") {
            Ok(value) => {
                let normalized = value.to_lowercase();
                normalized == "true" || normalized == "1" || normalized == "yes"
            },
            Err(_) => false,
        }
    }
    
    /// Discover all table definition files in the schema tables directory
    fn discover_table_definitions() -> Result<Vec<TableDefinition>, String> {
        let tables_dir = "src/schema/tables";
        
        if !Path::new(tables_dir).exists() {
            return Ok(Vec::new());
        }
        
        let entries = fs::read_dir(tables_dir)
            .map_err(|e| format!("Failed to read tables directory: {}", e))?;
        
        let mut table_definitions = Vec::new();
        
        for entry in entries {
            let entry = entry.map_err(|e| format!("Failed to read directory entry: {}", e))?;
            let path = entry.path();
            
            // Skip if not a file
            if !path.is_file() {
                continue;
            }
            
            // Skip if not a .rs file
            if let Some(extension) = path.extension() {
                if extension != "rs" {
                    continue;
                }
            } else {
                continue;
            }
            
            // Skip module files
            if let Some(file_name) = path.file_name() {
                let file_name_str = file_name.to_string_lossy();
                if file_name_str == "mod.rs" || 
                   file_name_str.ends_with("_generator.rs") ||
                   file_name_str == "field_definition.rs" ||
                   file_name_str == "generator_service.rs" {
                    continue;
                }
            }
            
            // Try to parse as table definition
            match Self::parse_table_definition_file(&path) {
                Ok(Some(table_def)) => {
                    table_definitions.push(table_def);
                },
                Ok(None) => {
                    // File doesn't contain table definition, skip
                    continue;
                },
                Err(e) => {
                    println!("Warning: Failed to parse {}: {}", path.display(), e);
                    continue;
                }
            }
        }
        
        Ok(table_definitions)
    }
    
    /// Parse a table definition file
    fn parse_table_definition_file(file_path: &Path) -> Result<Option<TableDefinition>, String> {
        let content = fs::read_to_string(file_path)
            .map_err(|e| format!("Failed to read file: {}", e))?;
        
        // Try to extract table name from file name
        let file_stem = file_path.file_stem()
            .and_then(|s| s.to_str())
            .ok_or("Invalid file name")?;
        
        // Look for table definition patterns in the file
        if Self::contains_table_definition(&content) {
            // Parse the table definition
            Self::parse_table_definition_content(&content, file_stem)
        } else {
            Ok(None)
        }
    }
    
    /// Check if content contains table definition patterns
    fn contains_table_definition(content: &str) -> bool {
        // Look for comment-based field definition patterns
        let has_comment_based = content.contains("field_name:") && content.contains("field_type:");
        
        // Look for struct-based patterns
        let has_struct_based = content.contains("DieselTableDefinition") ||
                              content.contains("define_table_schema!") ||
                              content.contains("impl DieselTableDefinition");
        
        // Look for legacy patterns
        let has_legacy = content.contains("FieldDefinition") || content.contains("TableDefinition");
        
        has_comment_based || has_struct_based || has_legacy
    }
    
    /// Parse table definition content
    fn parse_table_definition_content(content: &str, table_name: &str) -> Result<Option<TableDefinition>, String> {
        // Try to parse as new struct-based Diesel definition first
        if let Ok(table_def) = Self::parse_diesel_table_definition(content, table_name) {
            return Ok(Some(table_def));
        }
        
        // Try to parse as structured Rust code
        if let Ok(table_def) = Self::parse_rust_table_definition(content, table_name) {
            return Ok(Some(table_def));
        }
        
        // Try to parse as simple comment-based format
        if let Ok(table_def) = Self::parse_simple_table_definition(content, table_name) {
            return Ok(Some(table_def));
        }
        
        Ok(None)
    }
    
    /// Parse new struct-based Diesel table definition
    fn parse_diesel_table_definition(content: &str, file_name: &str) -> Result<TableDefinition, String> {
        // Look for struct definitions that implement DieselTableDefinition
        if content.contains("DieselTableDefinition") || content.contains("define_table_schema!") {
            // Extract table name from macro or struct
            let table_name = Self::extract_table_name_from_diesel_def(content, file_name)?;
            
            // For now, create a basic TableDefinition
            // In a real implementation, we would need to compile and execute the Rust code
            // or use a more sophisticated parser to extract the actual field definitions
            
            // This is a simplified approach - in practice, you'd want to:
            // 1. Use syn crate to parse the Rust AST
            // 2. Extract the actual DieselTableDefinition implementation
            // 3. Convert DieselFieldDefinition to FieldDefinition
            
            let mut fields = Vec::new();
            
            // Try to extract field information from macro usage
            if let Ok(extracted_fields) = Self::extract_fields_from_macro(content) {
                fields = extracted_fields;
            } else {
                // Fallback: create a placeholder field to indicate this is a valid table
                fields.push(FieldDefinition {
                    field_name: "id".to_string(),
                    field_type: "Int4".to_string(),
                    is_index: true,
                    joins_with: None,
                    default_value: None,
                });
            }
            
            return Ok(TableDefinition {
                table_name,
                fields,
            });
        }
        
        Err("No Diesel table definition found".to_string())
    }
    
    /// Extract table name from Diesel definition
    fn extract_table_name_from_diesel_def(content: &str, file_name: &str) -> Result<String, String> {
        // Look for table_name in macro
        if let Some(start) = content.find("table_name:") {
            let after_colon = &content[start + 11..]; // "table_name:".len() = 11
            if let Some(quote_start) = after_colon.find('"') {
                let after_quote = &after_colon[quote_start + 1..];
                if let Some(quote_end) = after_quote.find('"') {
                    return Ok(after_quote[..quote_end].to_string());
                }
            }
        }
        
        // Fallback to file name
        Ok(file_name.replace("_struct", "").replace("_table", ""))
    }
    
    /// Extract fields from macro definition
    fn extract_fields_from_macro(content: &str) -> Result<Vec<FieldDefinition>, String> {
        let mut fields = Vec::new();
        
        // Look for fields section in macro
        if let Some(fields_start) = content.find("fields: {") {
            let after_fields = &content[fields_start + 9..];
            if let Some(fields_end) = after_fields.find("}\n") {
                let fields_content = &after_fields[..fields_end];
                
                // Parse each field line
                for line in fields_content.lines() {
                    let line = line.trim();
                    if line.is_empty() || line.starts_with("//") {
                        continue;
                    }
                    
                    // Parse field definition like: "id: integer(), primary_key: true,"
                    if let Some(colon_pos) = line.find(':') {
                        let field_name = line[..colon_pos].trim().to_string();
                        let rest = &line[colon_pos + 1..];
                        
                        // Extract type (simplified)
                        let field_type = if rest.contains("integer()") {
                            "Int4".to_string()
                        } else if rest.contains("text()") {
                            "Text".to_string()
                        } else if rest.contains("boolean()") {
                            "Bool".to_string()
                        } else if rest.contains("timestamptz()") {
                            "Timestamptz".to_string()
                        } else if rest.contains("nullable(") {
                            // Extract inner type
                            if rest.contains("nullable(text())") {
                                "Nullable<Text>".to_string()
                            } else if rest.contains("nullable(boolean())") {
                                "Nullable<Bool>".to_string()
                            } else if rest.contains("nullable(jsonb())") {
                                "Nullable<Jsonb>".to_string()
                            } else if rest.contains("nullable(array(text()))") {
                                "Nullable<Array<Text>>".to_string()
                            } else {
                                "Nullable<Text>".to_string() // default
                            }
                        } else {
                            "Text".to_string() // default
                        };
                        
                        let is_index = rest.contains("primary_key: true") || rest.contains("indexed: true");
                        
                        fields.push(FieldDefinition {
                            field_name,
                            field_type,
                            is_index,
                            joins_with: None,
                            default_value: None,
                        });
                    }
                }
            }
        }
        
        if fields.is_empty() {
            return Err("No fields found in macro definition".to_string());
        }
        
        Ok(fields)
    }
    
    /// Parse Rust-style table definition
    fn parse_rust_table_definition(content: &str, _table_name: &str) -> Result<TableDefinition, String> {
        // Look for TableDefinition struct instantiation
        if content.contains("TableDefinition") {
            // This would be a more complex parser for Rust syntax
            // For now, we'll implement a simple pattern matcher
            return Err("Rust table definition parsing not yet implemented".to_string());
        }
        
        Err("No Rust table definition found".to_string())
    }
    
    /// Parse simple table definition format
    fn parse_simple_table_definition(content: &str, table_name: &str) -> Result<TableDefinition, String> {
        let mut fields = Vec::new();
        let lines: Vec<&str> = content.lines().collect();
        
        let mut current_field: Option<FieldDefinition> = None;
        
        for line in lines {
            let line = line.trim();
            
            // Skip empty lines and comments
            if line.is_empty() || line.starts_with("//") || line.starts_with("/*") {
                continue;
            }
            
            // Parse field properties
            if line.starts_with("field_name:") {
                // Save previous field if exists
                if let Some(field) = current_field.take() {
                    fields.push(field);
                }
                
                let field_name = line.split(':').nth(1)
                    .ok_or("Invalid field_name format")?
                    .trim()
                    .to_string();
                
                current_field = Some(FieldDefinition {
                    field_name,
                    field_type: String::new(),
                    is_index: false,
                    joins_with: None,
                    default_value: None,
                });
            } else if line.starts_with("field_type:") {
                if let Some(ref mut field) = current_field {
                    field.field_type = line.split(':').nth(1)
                        .ok_or("Invalid field_type format")?
                        .trim()
                        .to_string();
                }
            } else if line.starts_with("is_index:") {
                if let Some(ref mut field) = current_field {
                    let value = line.split(':').nth(1)
                        .ok_or("Invalid is_index format")?
                        .trim();
                    field.is_index = value == "true";
                }
            } else if line.starts_with("joins_with:") {
                if let Some(ref mut field) = current_field {
                    let value = line.split(':').nth(1)
                        .ok_or("Invalid joins_with format")?
                        .trim();
                    if !value.is_empty() && value != "null" && value != "None" {
                        field.joins_with = Some(value.to_string());
                    }
                }
            } else if line.starts_with("default_value:") {
                if let Some(ref mut field) = current_field {
                    let value = line.split(':').nth(1)
                        .ok_or("Invalid default_value format")?
                        .trim();
                    if !value.is_empty() && value != "null" && value != "None" {
                        field.default_value = Some(value.to_string());
                    }
                }
            }
        }
        
        // Save last field
        if let Some(field) = current_field {
            fields.push(field);
        }
        
        if fields.is_empty() {
            return Err("No fields found in table definition".to_string());
        }
        
        Ok(TableDefinition {
            table_name: table_name.to_string(),
            fields,
        })
    }
    

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_create_schema_enabled() {
        // Test with different environment values
        env::set_var("CREATE_SCHEMA", "true");
        assert!(GeneratorService::is_create_schema_enabled());
        
        env::set_var("CREATE_SCHEMA", "1");
        assert!(GeneratorService::is_create_schema_enabled());
        
        env::set_var("CREATE_SCHEMA", "yes");
        assert!(GeneratorService::is_create_schema_enabled());
        
        env::set_var("CREATE_SCHEMA", "false");
        assert!(!GeneratorService::is_create_schema_enabled());
        
        env::remove_var("CREATE_SCHEMA");
        assert!(!GeneratorService::is_create_schema_enabled());
    }

    #[test]
    fn test_contains_table_definition() {
        let content_with_def = "field_name: test\nfield_type: Text";
        assert!(GeneratorService::contains_table_definition(content_with_def));
        
        let content_without_def = "some random content";
        assert!(!GeneratorService::contains_table_definition(content_without_def));
    }

    #[test]
    fn test_parse_simple_table_definition() {
        let content = r#"
field_name: id
field_type: Int4
is_index: true
joins_with: 
default_value: 

field_name: name
field_type: Nullable<Text>
is_index: false
joins_with: 
default_value: 
"#;
        
        let result = GeneratorService::parse_simple_table_definition(content, "test_table");
        assert!(result.is_ok());
        
        let table_def = result.unwrap();
        assert_eq!(table_def.table_name, "test_table");
        assert_eq!(table_def.fields.len(), 2);
        assert_eq!(table_def.fields[0].field_name, "id");
        assert_eq!(table_def.fields[0].field_type, "Int4");
        assert!(table_def.fields[0].is_index);
        assert_eq!(table_def.fields[1].field_name, "name");
        assert_eq!(table_def.fields[1].field_type, "Nullable<Text>");
        assert!(!table_def.fields[1].is_index);
    }
}