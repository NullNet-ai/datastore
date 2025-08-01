use crate::schema::generator::field_definition::TableDefinition;
use crate::schema::verify::{field_exists_in_table, get_table_fields};
use std::fs;
use regex::Regex;

pub struct SchemaGenerator;

#[derive(Debug, Clone)]
pub struct SchemaChange {
    pub table_name: String,
    pub change_type: SchemaChangeType,
    pub field_name: Option<String>,
    pub field_definition: Option<String>,
}

#[derive(Debug, Clone)]
pub enum SchemaChangeType {
    NewTable,
    NewField,
    RemovedField,
    NewIndex,
    NewForeignKey,
}

impl SchemaGenerator {
    /// Analyze what changes need to be made to the schema
    pub fn analyze_changes(table_def: &TableDefinition) -> Result<Vec<SchemaChange>, String> {
        let mut changes = Vec::new();
        
        // Check if table exists
        let existing_fields = get_table_fields(&table_def.table_name);
        
        if existing_fields.is_none() {
            // Table doesn't exist, need to create it
            changes.push(SchemaChange {
                table_name: table_def.table_name.clone(),
                change_type: SchemaChangeType::NewTable,
                field_name: None,
                field_definition: None,
            });
        } else {
            // Table exists, check for new fields
            let existing_fields = existing_fields.unwrap();
            
            // Check if this table uses system fields and detect new system fields
            let uses_system_fields = Self::table_uses_system_fields(&table_def.table_name);
            if uses_system_fields && Self::should_force_system_fields_update() {
                // Only add system fields that don't already exist in the table
                for field in &table_def.fields {
                    if Self::is_system_field(&field.field_name) && !existing_fields.contains(&field.field_name) {
                        changes.push(SchemaChange {
                            table_name: table_def.table_name.clone(),
                            change_type: SchemaChangeType::NewField,
                            field_name: Some(field.field_name.clone()),
                            field_definition: Some(field.field_type.clone()),
                        });
                    }
                }
            }
            
            for field in &table_def.fields {
                if !existing_fields.contains(&field.field_name) {
                    changes.push(SchemaChange {
                        table_name: table_def.table_name.clone(),
                        change_type: SchemaChangeType::NewField,
                        field_name: Some(field.field_name.clone()),
                        field_definition: Some(field.field_type.clone()),
                    });
                }
            }
            
            // Check for removed fields (fields that exist in DB but not in table definition)
            let current_field_names: Vec<String> = table_def.fields.iter()
                .map(|f| f.field_name.clone())
                .collect();
            
            for existing_field in &existing_fields {
                if !current_field_names.contains(existing_field) {
                    changes.push(SchemaChange {
                        table_name: table_def.table_name.clone(),
                        change_type: SchemaChangeType::RemovedField,
                        field_name: Some(existing_field.clone()),
                        field_definition: None,
                    });
                }
            }
        }
        
        Ok(changes)
    }
    
    /// Analyze what changes need to be made to the schema with indexes and foreign keys
    pub fn analyze_changes_with_indexes_and_foreign_keys(
        table_def: &TableDefinition, 
        indexes: &[(String, Vec<String>, bool, Option<String>)],
        foreign_keys: &[crate::schema::generator::diesel_schema_definition::ForeignKeyDefinition]
    ) -> Result<Vec<SchemaChange>, String> {
        let mut changes = Self::analyze_changes(table_def)?;
        
        // Add index changes - only if they don't already exist
         for (index_name, columns, _is_unique, index_type) in indexes {
             // Check if index already exists by looking for it in existing schema
             if !Self::index_exists_in_schema(&table_def.table_name, index_name) {
                 let field_def = if let Some(idx_type) = index_type {
                     format!("{}|{}", columns.join(","), idx_type)
                 } else {
                     columns.join(",")
                 };
                 
                 changes.push(SchemaChange {
                     table_name: table_def.table_name.clone(),
                     change_type: SchemaChangeType::NewIndex,
                     field_name: Some(index_name.clone()),
                     field_definition: Some(field_def),
                 });
             }
         }
         
         // Add foreign key changes - only if they don't already exist
         for foreign_key in foreign_keys {
             // Generate constraint name: fk_tablename_columnname
             let constraint_name = format!("fk_{}_{}", table_def.table_name, foreign_key.column);
             
             // Check if foreign key already exists
             if !Self::foreign_key_exists_in_schema(&table_def.table_name, &constraint_name) {
                 let field_def = format!("{}|{}|{}", 
                     foreign_key.column, 
                     foreign_key.references_table, 
                     foreign_key.references_column
                 );
                 
                 changes.push(SchemaChange {
                     table_name: table_def.table_name.clone(),
                     change_type: SchemaChangeType::NewForeignKey,
                     field_name: Some(constraint_name),
                     field_definition: Some(field_def),
                 });
             }
         }
         
         Ok(changes)
      }
    
    /// Update the schema.rs file with new table definition
    pub fn update_schema_file(table_def: &TableDefinition) -> Result<(), String> {
        let schema_file_path = "src/schema/schema.rs";
        
        // Read existing schema content
        let existing_content = match fs::read_to_string(schema_file_path) {
            Ok(content) => content,
            Err(e) => return Err(format!("Failed to read schema.rs: {}", e)),
        };
        
        // Check if table already exists
        if Self::table_exists_in_schema(&existing_content, &table_def.table_name) {
            // Table exists, we need to add new fields
            Self::add_fields_to_existing_table(&existing_content, table_def, schema_file_path)
        } else {
            // Table doesn't exist, add new table
            Self::add_new_table_to_schema(&existing_content, table_def, schema_file_path)
        }
    }
    
    /// Check if a table exists in the schema content
    fn table_exists_in_schema(content: &str, table_name: &str) -> bool {
        let pattern = format!(r"table!\s*\{{\s*{}\s*\(", regex::escape(table_name));
        if let Ok(regex) = Regex::new(&pattern) {
            regex.is_match(content)
        } else {
            false
        }
    }
    
    /// Check if an index already exists for a table
    fn index_exists_in_schema(table_name: &str, index_name: &str) -> bool {
        // Read the migrations directory to check if this index was already created
        let migrations_dir = "migrations";
        
        if let Ok(entries) = std::fs::read_dir(migrations_dir) {
            for entry in entries.flatten() {
                if let Ok(up_sql) = std::fs::read_to_string(entry.path().join("up.sql")) {
                    // Check if this index was already created in a previous migration
                    // Pattern matches: CREATE [UNIQUE] INDEX [IF NOT EXISTS] "index_name" ON "table_name"
                    let index_pattern = format!(r#"CREATE\s+(?:UNIQUE\s+)?INDEX\s+(?:IF\s+NOT\s+EXISTS\s+)?["']?{}["']?\s+ON\s+["']?{}["']?"#, 
                        regex::escape(index_name), regex::escape(table_name));
                    if let Ok(regex) = Regex::new(&index_pattern) {
                        if regex.is_match(&up_sql) {
                            return true;
                        }
                    }
                }
            }
        }
        false
     }
     
     /// Check if a foreign key already exists for a table
     fn foreign_key_exists_in_schema(table_name: &str, constraint_name: &str) -> bool {
         // Read the migrations directory to check if this foreign key was already created
         let migrations_dir = "migrations";
         
         if let Ok(entries) = std::fs::read_dir(migrations_dir) {
             for entry in entries.flatten() {
                 if let Ok(up_sql) = std::fs::read_to_string(entry.path().join("up.sql")) {
                     // Check if this foreign key was already created in a previous migration
                     // Pattern matches: ALTER TABLE "table_name" ADD CONSTRAINT "constraint_name"
                     let fk_pattern = format!(r#"ALTER\s+TABLE\s+["']?{}["']?\s+ADD\s+CONSTRAINT\s+["']?{}["']?"#, 
                         regex::escape(table_name), regex::escape(constraint_name));
                     if let Ok(regex) = Regex::new(&fk_pattern) {
                         if regex.is_match(&up_sql) {
                             return true;
                         }
                     }
                 }
             }
         }
         false
     }
     
     /// Add a new table to the schema
    fn add_new_table_to_schema(existing_content: &str, table_def: &TableDefinition, file_path: &str) -> Result<(), String> {
        let mut parsed_fields = Vec::new();
        
        // Parse all fields
        for field in &table_def.fields {
            match field.parse() {
                Ok(parsed) => parsed_fields.push(parsed),
                Err(e) => return Err(format!("Error parsing field {}: {}", field.field_name, e)),
            }
        }
        
        // Generate table definition
        let table_definition = Self::generate_table_definition(table_def)?;
        
        // Add the new table at the end of the file
        let mut new_content = existing_content.to_string();
        if !new_content.ends_with('\n') {
            new_content.push('\n');
        }
        new_content.push('\n');
        new_content.push_str(&table_definition);
        new_content.push('\n');
        
        // Write the updated schema
        if let Err(e) = fs::write(file_path, new_content) {
            return Err(format!("Failed to write schema.rs: {}", e));
        }
        
        println!("Added new table '{}' to schema.rs", table_def.table_name);
        Ok(())
    }
    
    /// Add fields to an existing table in the schema
    fn add_fields_to_existing_table(existing_content: &str, table_def: &TableDefinition, file_path: &str) -> Result<(), String> {
        
        // Find the table definition
        let table_pattern = format!(
            r"(?s)(table!\s*\{{\s*{}\s*\([^)]*\)\s*\{{)(.*?)(\}}\s*\}})",
            regex::escape(&table_def.table_name)
        );
        
        let table_regex = match Regex::new(&table_pattern) {
            Ok(re) => re,
            Err(e) => return Err(format!("Failed to create table regex: {}", e)),
        };
        
        if let Some(captures) = table_regex.captures(existing_content) {
            let table_start = captures.get(1).unwrap().as_str();
            let table_body = captures.get(2).unwrap().as_str();
            let table_end = captures.get(3).unwrap().as_str();
            
            // Parse new fields that don't exist
            let mut new_fields = Vec::new();
            for field in &table_def.fields {
                if !field_exists_in_table(&table_def.table_name, &field.field_name) {
                    match field.parse() {
                        Ok(parsed) => new_fields.push(parsed),
                        Err(e) => return Err(format!("Error parsing field {}: {}", field.field_name, e)),
                    }
                }
            }
            
            if new_fields.is_empty() {
                println!("No new fields to add to table '{}'", table_def.table_name);
                return Ok(());
            }
            
            // Generate field definitions for new fields
            let mut new_field_definitions = String::new();
            for field in &new_fields {
                new_field_definitions.push_str(&format!("        {} -> {},\n", field.name, field.diesel_type));
            }
            
            // Reconstruct the table with new fields
            let new_table_body = format!("{}{}", table_body, new_field_definitions);
            let new_table_definition = format!("{}{}{}", table_start, new_table_body, table_end);
            
            // Replace the old table definition with the new one
            let new_content = table_regex.replace(existing_content, new_table_definition.as_str());
            
            // Write the updated schema
            if let Err(e) = fs::write(file_path, new_content.as_ref()) {
                return Err(format!("Failed to write schema.rs: {}", e));
            }
            
            println!("Added {} new fields to table '{}' in schema.rs", new_fields.len(), table_def.table_name);
            Ok(())
        } else {
            Err(format!("Could not find table '{}' in schema.rs", table_def.table_name))
        }
    }
    
    /// Check if a table uses system fields by reading its definition file
    fn table_uses_system_fields(table_name: &str) -> bool {
        let table_file_path = format!("src/schema/tables/{}.rs", table_name);
        if let Ok(content) = std::fs::read_to_string(&table_file_path) {
            content.contains("system_fields!()")
        } else {
            false
        }
    }
    
    /// Check if we should force update system fields (when CREATE_SCHEMA is enabled)
    fn should_force_system_fields_update() -> bool {
        std::env::var("CREATE_SCHEMA").unwrap_or_default().to_lowercase() == "true"
    }
    
    /// Check if a field is a system field
    fn is_system_field(field_name: &str) -> bool {
        Self::get_system_field_names()
            .unwrap_or_default()
            .contains(&field_name.to_string())
    }

    /// Dynamically extracts system field names from the system_fields macro
    fn get_system_field_names() -> Result<Vec<String>, String> {
        let system_fields_path = "src/schema/generator/system_fields.rs";
        let content = fs::read_to_string(system_fields_path)
            .map_err(|e| format!("Failed to read system_fields.rs: {}", e))?;
        
        // Find the macro definition
        let macro_start = content.find("() => {")
            .ok_or("Could not find system_fields macro definition")?;
        let macro_content_start = macro_start + "() => {".len();
        
        // Find the closing brace of the macro
        let mut brace_count = 1;
        let mut macro_end = macro_content_start;
        let chars: Vec<char> = content.chars().collect();
        
        for i in macro_content_start..chars.len() {
            match chars[i] {
                '{' => brace_count += 1,
                '}' => {
                    brace_count -= 1;
                    if brace_count == 0 {
                        macro_end = i;
                        break;
                    }
                }
                _ => {}
            }
        }
        
        if brace_count != 0 {
            return Err("Could not find closing brace for system_fields macro".to_string());
        }
        
        let macro_content = &content[macro_content_start..macro_end];
        
        // Extract field names from the macro content
        let mut field_names = Vec::new();
        for line in macro_content.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with("//") {
                continue;
            }
            
            // Look for field definitions (field_name: type)
            if let Some(colon_pos) = line.find(':') {
                let field_name = line[..colon_pos].trim();
                if !field_name.is_empty() {
                    field_names.push(field_name.to_string());
                }
            }
        }
        
        Ok(field_names)
    }
    
    /// Replace an entire table definition in the schema
    fn replace_table_definition(existing_content: &str, table_def: &TableDefinition, file_path: &str) -> Result<(), String> {
        // Find the table definition
        let table_pattern = format!(
            r"(?s)table!\s*\{{\s*{}\s*\([^)]*\)\s*\{{.*?\}}\s*\}}",
            regex::escape(&table_def.table_name)
        );
        
        let table_regex = match Regex::new(&table_pattern) {
            Ok(re) => re,
            Err(e) => return Err(format!("Failed to create table regex: {}", e)),
        };
        
        // Generate new table definition
        let new_table_definition = Self::generate_table_definition(table_def)?;
        
        // Replace the old table definition with the new one
        let new_content = table_regex.replace(existing_content, new_table_definition.as_str());
        
        // Write the updated schema
        if let Err(e) = fs::write(file_path, new_content.as_ref()) {
            return Err(format!("Failed to write schema.rs: {}", e));
        }
        
        println!("Replaced table '{}' definition in schema.rs with updated system fields", table_def.table_name);
        Ok(())
    }
    
    /// Generate a complete table definition
    fn generate_table_definition(table_def: &TableDefinition) -> Result<String, String> {
        let mut definition = String::new();
        
        // Parse all fields
        let mut parsed_fields = Vec::new();
        for field in &table_def.fields {
            match field.parse() {
                Ok(parsed) => parsed_fields.push(parsed),
                Err(e) => return Err(format!("Error parsing field {}: {}", field.field_name, e)),
            }
        }
        
        // Collect primary key fields
        let primary_key_fields: Vec<&str> = table_def.fields
            .iter()
            .filter(|field| field.is_primary_key)
            .map(|field| field.field_name.as_str())
            .collect();
        
        // Determine primary key for Diesel schema
        let primary_key = if !primary_key_fields.is_empty() {
            if primary_key_fields.len() == 1 {
                primary_key_fields[0]
            } else {
                // For composite primary keys, use the first one as Diesel's primary key
                // Diesel doesn't directly support composite primary keys in the table! macro
                primary_key_fields[0]
            }
        } else {
            // Fallback: assume 'id' if present, otherwise first field
            parsed_fields.iter()
                .find(|f| f.name == "id")
                .map(|f| f.name.as_str())
                .unwrap_or(&parsed_fields[0].name)
        };
        
        definition.push_str(&format!("table! {{\n    {}({}) {{\n", table_def.table_name, primary_key));
        
        // Add all fields
        for field in &parsed_fields {
            definition.push_str(&format!("        {} -> {},\n", field.name, field.diesel_type));
        }
        
        definition.push_str("    }\n}");
        
        Ok(definition)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::schema::generator::field_definition::FieldDefinition;

    #[test]
    fn test_table_exists_in_schema() {
        let schema_content = r#"
        table! {
            users(id) {
                id -> Nullable<Text>,
                name -> Nullable<Text>,
            }
        }
        "#;
        
        assert!(SchemaGenerator::table_exists_in_schema(schema_content, "users"));
        assert!(!SchemaGenerator::table_exists_in_schema(schema_content, "posts"));
    }

    #[test]
    fn test_generate_table_definition() {
        let table_def = TableDefinition {
            table_name: "test_table".to_string(),
            fields: vec![
                FieldDefinition {
                    field_name: "id".to_string(),
                    field_type: "Text".to_string(),
                    is_index: false,
                    is_primary_key: true,
                    joins_with: None,
                    default_value: None,
                    migration_nullable: true,
                },
                FieldDefinition {
                    field_name: "name".to_string(),
                    field_type: "Text".to_string(),
                    is_index: false,
                    is_primary_key: false,
                    joins_with: None,
                    default_value: None,
                    migration_nullable: true,
                },
            ],
        };
        
        let result = SchemaGenerator::generate_table_definition(&table_def);
        assert!(result.is_ok());
        let definition = result.unwrap();
        assert!(definition.contains("table! {"));
        assert!(definition.contains("test_table(id) {"));
        assert!(definition.contains("id -> Nullable<Text>"));
        assert!(definition.contains("name -> Nullable<Text>"));
    }
}