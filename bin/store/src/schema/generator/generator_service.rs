use crate::schema::generator::field_definition::{FieldDefinition, TableDefinition};
use crate::schema::generator::model_generator::ModelGenerator;
use crate::schema::generator::schema_generator::SchemaGenerator;
use crate::schema::generator::migration_generator::MigrationGenerator;
use crate::schema::generator::diesel_schema_definition::ForeignKeyDefinition;
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
            let model_content = ModelGenerator::generate_model(&table_def)?;
            Self::write_model_file(&table_def.table_name, &model_content)?;
            
            // Extract indexes and foreign keys from table definition file
            let (indexes, foreign_keys) = {
                // Find the table definition file to read its content
                let table_files_dir = "src/schema/tables";
                
                // Try multiple possible file names for the table
                let possible_files = vec![
                    format!("{}/{}.rs", table_files_dir, table_def.table_name),
                    format!("{}/{}_catalog.rs", table_files_dir, table_def.table_name.trim_end_matches('s')),
                    format!("{}/{}_table.rs", table_files_dir, table_def.table_name),
                ];
                
                let mut extracted_indexes = Vec::new();
                let mut extracted_foreign_keys = Vec::new();
                 
                 for table_file_path in possible_files {
                     if let Ok(file_content) = fs::read_to_string(&table_file_path) {
                         extracted_indexes = Self::extract_indexes_from_macro(&file_content).unwrap_or_else(|_| Vec::new());
                         extracted_foreign_keys = Self::extract_foreign_keys_from_macro(&file_content).unwrap_or_else(|_| Vec::new());
                         break;
                     }
                 }
                
                (extracted_indexes, extracted_foreign_keys)
            };
            
            // Analyze schema changes with indexes and foreign keys
            let changes = SchemaGenerator::analyze_changes_with_indexes_and_foreign_keys(&table_def, &indexes, &foreign_keys)?;
            
            if !changes.is_empty() {
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
    
    /// Write model file to the models directory
    fn write_model_file(table_name: &str, model_content: &str) -> Result<(), String> {
        let models_dir = Path::new("src/models");
        
        // Create models directory if it doesn't exist
        if !models_dir.exists() {
            fs::create_dir_all(models_dir)
                .map_err(|e| format!("Failed to create models directory: {}", e))?;
        }
        
        // Convert table name to singular for model file name
        let singular_name = Self::to_singular(table_name);
        let model_file_path = models_dir.join(format!("{}_model.rs", singular_name));
        
        // Write model content to file
        fs::write(&model_file_path, model_content)
            .map_err(|e| format!("Failed to write model file {}: {}", model_file_path.display(), e))?;
        
        // Add module declaration to mod.rs
        Self::add_module_to_mod_rs(&singular_name)?;
        
        println!("Generated model file: {}", model_file_path.display());
        Ok(())
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
    
    /// Add module declaration to models/mod.rs
    fn add_module_to_mod_rs(singular_name: &str) -> Result<(), String> {
        let mod_file_path = Path::new("src/models/mod.rs");
        
        // Read existing mod.rs content
        let content = fs::read_to_string(mod_file_path)
            .map_err(|e| format!("Failed to read mod.rs: {}", e))?;
        
        let module_declaration = format!("pub mod {}_model;", singular_name);
        
        // Check if module is already declared
        if content.contains(&module_declaration) {
            return Ok(()); // Already exists
        }
        
        // Find the right place to insert (alphabetically)
        let mut lines: Vec<&str> = content.lines().collect();
        let mut insert_index = lines.len();
        
        for (i, line) in lines.iter().enumerate() {
            if line.starts_with("pub mod ") && *line > module_declaration.as_str() {
                insert_index = i;
                break;
            }
        }
        
        // Insert the new module declaration
        lines.insert(insert_index, &module_declaration);
        
        // Write back to file
        let new_content = lines.join("\n");
        fs::write(mod_file_path, new_content)
            .map_err(|e| format!("Failed to update mod.rs: {}", e))?;
        
        println!("Added module declaration: {}", module_declaration);
        Ok(())
    }
    
    /// Discover all table definition files in the schema directory
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
            
            let mut fields = Vec::new();
            
            // Try to extract field information from macro usage
            if let Ok(extracted_fields) = Self::extract_fields_from_macro(content) {
                fields = extracted_fields;
            } else {
                // Fallback: create a placeholder field to indicate this is a valid table
                fields.push(FieldDefinition {
                    field_name: "id".to_string(),
                    field_type: "Int4".to_string(),
                    is_index: false, // Remove indexed: true from field level
                    joins_with: None,
                    default_value: None,
                });
            }
            
            // Extract foreign keys from macro and add to fields
            if let Ok(foreign_keys) = Self::extract_foreign_keys_from_macro(content) {
                for fk in foreign_keys {
                    // Find the field and add foreign key info
                    if let Some(field) = fields.iter_mut().find(|f| f.field_name == fk.column) {
                        field.joins_with = Some(format!("{}.{}", fk.references_table, fk.references_column));
                    }
                }
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
            
            // Find the end of the fields section by counting braces
            let mut brace_count = 1;
            let mut fields_end = 0;
            let chars: Vec<char> = after_fields.chars().collect();
            
            for (i, &ch) in chars.iter().enumerate() {
                match ch {
                    '{' => brace_count += 1,
                    '}' => {
                        brace_count -= 1;
                        if brace_count == 0 {
                            fields_end = i;
                            break;
                        }
                    },
                    _ => {}
                }
            }
            
            if fields_end > 0 {
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
                        
                        let is_index = rest.contains("primary_key: true"); // Remove indexed: true check
                        
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
    
    /// Extract indexes from macro definition
    fn extract_indexes_from_macro(content: &str) -> Result<Vec<(String, Vec<String>, bool, Option<String>)>, String> {
        let mut indexes = Vec::new();
        
        // Look for indexes section in macro
        if let Some(indexes_start) = content.find("indexes: {") {
            let after_indexes = &content[indexes_start + 10..];
            
            // Find the end of the indexes section by counting braces
            let mut brace_count = 1;
            let mut indexes_end = 0;
            let chars: Vec<char> = after_indexes.chars().collect();
            
            for (i, &ch) in chars.iter().enumerate() {
                match ch {
                    '{' => brace_count += 1,
                    '}' => {
                        brace_count -= 1;
                        if brace_count == 0 {
                            indexes_end = i;
                            break;
                        }
                    },
                    _ => {}
                }
            }
            
            if indexes_end > 0 {
                let indexes_content = &after_indexes[..indexes_end];
                
                // Parse each index definition
                let mut current_index: Option<(String, Vec<String>, bool, Option<String>)> = None;
                let mut in_index_def = false;
                
                for line in indexes_content.lines() {
                    let line = line.trim();
                    if line.is_empty() || line.starts_with("//") {
                        continue;
                    }
                    
                    // Check for index name
                    if line.contains(": {") {
                        // Save previous index if exists
                        if let Some(index) = current_index.take() {
                            indexes.push(index);
                        }
                        
                        let index_name = line.split(':').next().unwrap().trim().to_string();
                        current_index = Some((index_name, Vec::new(), false, None));
                        in_index_def = true;
                    } else if in_index_def {
                        if line.contains("columns:") {
                            // Extract columns
                            if let Some(bracket_start) = line.find('[') {
                                if let Some(bracket_end) = line.find(']') {
                                    let columns_str = &line[bracket_start + 1..bracket_end];
                                    let columns: Vec<String> = columns_str
                                        .split(',')
                                        .map(|s| s.trim().trim_matches('"').to_string())
                                        .collect();
                                    if let Some(ref mut index) = current_index {
                                        index.1 = columns;
                                    }
                                }
                            }
                        } else if line.contains("unique:") {
                            let unique_val = line.split(':').nth(1).unwrap().trim();
                            let is_unique = unique_val == "true";
                            if let Some(ref mut index) = current_index {
                                index.2 = is_unique;
                            }
                        } else if line.contains("type:") {
                            let type_val = line.split(':').nth(1).unwrap().trim().trim_matches('"').to_string();
                            if let Some(ref mut index) = current_index {
                                index.3 = Some(type_val);
                            }
                        } else if line == "}" {
                            in_index_def = false;
                        }
                    }
                }
                
                // Save last index if exists
                if let Some(index) = current_index {
                    indexes.push(index);
                }
            }
        }
        
        Ok(indexes)
    }
    
    /// Extract foreign keys from macro definition
    fn extract_foreign_keys_from_macro(content: &str) -> Result<Vec<ForeignKeyDefinition>, String> {
        use crate::schema::generator::diesel_schema_definition::ForeignKeyDefinition;
        
        let mut foreign_keys = Vec::new();
        
        // Look for foreign_keys section in macro
        if let Some(fk_start) = content.find("foreign_keys: {") {
            let after_fk = &content[fk_start + 15..];
            
            // Find the end of the foreign_keys section by counting braces
            let mut brace_count = 1;
            let mut fk_end = 0;
            let chars: Vec<char> = after_fk.chars().collect();
            
            for (i, &ch) in chars.iter().enumerate() {
                match ch {
                    '{' => brace_count += 1,
                    '}' => {
                        brace_count -= 1;
                        if brace_count == 0 {
                            fk_end = i;
                            break;
                        }
                    },
                    _ => {}
                }
            }
            
            if fk_end > 0 {
                let fk_content = &after_fk[..fk_end];
                
                // Parse each foreign key line
                for line in fk_content.lines() {
                    let line = line.trim();
                    if line.is_empty() || line.starts_with("//") {
                        continue;
                    }
                    
                    // Parse foreign key like: "category_id -> "categories"."id""
                    if line.contains(" -> ") {
                        let parts: Vec<&str> = line.split(" -> ").collect();
                        if parts.len() == 2 {
                            let column = parts[0].trim().to_string();
                            let reference = parts[1].trim().trim_matches(',');
                            
                            // Parse "table"."column" format
                            if let Some(dot_pos) = reference.find('.') {
                                let table_part = &reference[..dot_pos].trim_matches('"');
                                let column_part = &reference[dot_pos + 1..].trim_matches('"');
                                
                                foreign_keys.push(ForeignKeyDefinition {
                                    column,
                                    references_table: table_part.to_string(),
                                    references_column: column_part.to_string(),
                                    on_delete: None,
                                    on_update: None,
                                });
                            }
                        }
                    }
                }
            }
        }
        
        Ok(foreign_keys)
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