use crate::schema::generator::field_definition::TableDefinition;
use crate::schema::verify::{field_exists_in_table, get_table_fields, field_type_in_table, FieldTypeInfo};
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
                    // Skip system fields if they were already processed above
                    if uses_system_fields && Self::should_force_system_fields_update() && Self::is_system_field(&field.field_name) {
                        continue;
                    }
                    
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
                    // Get field type information for the removed field
                    let field_definition = if let Some(field_type_info) = field_type_in_table(&table_def.table_name, existing_field) {
                        Some(Self::field_type_info_to_definition(&field_type_info))
                    } else {
                        None
                    };
                    
                    changes.push(SchemaChange {
                        table_name: table_def.table_name.clone(),
                        change_type: SchemaChangeType::RemovedField,
                        field_name: Some(existing_field.clone()),
                        field_definition,
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
            // Table exists, we need to handle field changes
            Self::update_existing_table_in_schema(&existing_content, table_def, schema_file_path)
        } else {
            // Table doesn't exist, add new table
            Self::add_new_table_to_schema(&existing_content, table_def, schema_file_path)
        }
    }
    
    /// Update an existing table in schema.rs by adding new fields and removing deleted fields
    fn update_existing_table_in_schema(existing_content: &str, table_def: &TableDefinition, file_path: &str) -> Result<(), String> {
        // Get current fields in schema.rs
        let existing_fields = get_table_fields(&table_def.table_name).unwrap_or_default();
        
        // Get current field names from table definition
        let current_field_names: Vec<String> = table_def.fields.iter()
            .map(|f| f.field_name.clone())
            .collect();
        
        // Find fields to remove (exist in schema but not in table definition)
        let fields_to_remove: Vec<String> = existing_fields.iter()
            .filter(|field| !current_field_names.contains(field))
            .cloned()
            .collect();
        
        // Start with existing content
        let mut updated_content = existing_content.to_string();
        
        // Remove fields that are no longer in the table definition
        if !fields_to_remove.is_empty() {
            updated_content = Self::remove_fields_from_table(&updated_content, &table_def.table_name, &fields_to_remove)?;
            println!("Removed {} fields from table '{}' in schema.rs", fields_to_remove.len(), table_def.table_name);
        }
        
        // Check if there are new fields to add
        let has_new_fields = table_def.fields.iter()
            .any(|field| !field_exists_in_table(&table_def.table_name, &field.field_name));
        
        if has_new_fields {
            // Add new fields
            Self::add_fields_to_existing_table(&updated_content, table_def, file_path)
        } else if !fields_to_remove.is_empty() {
            // Only removed fields, write the updated content
            if let Err(e) = fs::write(file_path, updated_content) {
                return Err(format!("Failed to write schema.rs: {}", e));
            }
            Ok(())
        } else {
            // No changes needed
            Ok(())
        }
    }
    
    /// Remove specified fields from a table in schema.rs
    fn remove_fields_from_table(existing_content: &str, table_name: &str, fields_to_remove: &[String]) -> Result<String, String> {
        // Find the table definition
        let table_pattern = format!(
            r"(?s)(table!\s*\{{\s*{}\s*\([^)]*\)\s*\{{)(.*?)(\}}\s*\}})",
            regex::escape(table_name)
        );
        
        let table_regex = match Regex::new(&table_pattern) {
            Ok(re) => re,
            Err(e) => return Err(format!("Failed to create table regex: {}", e)),
        };
        
        if let Some(captures) = table_regex.captures(existing_content) {
            let table_start = captures.get(1).unwrap().as_str();
            let table_body = captures.get(2).unwrap().as_str();
            let table_end = captures.get(3).unwrap().as_str();
            
            // Remove specified fields from table body
            let mut new_table_body = String::new();
            for line in table_body.lines() {
                let trimmed_line = line.trim();
                
                // Skip empty lines and comments
                if trimmed_line.is_empty() || trimmed_line.starts_with("//") {
                    new_table_body.push_str(line);
                    new_table_body.push('\n');
                    continue;
                }
                
                // Check if this line defines a field to remove
                let mut should_remove = false;
                for field_to_remove in fields_to_remove {
                    if trimmed_line.starts_with(&format!("{} ->", field_to_remove)) {
                        should_remove = true;
                        break;
                    }
                }
                
                if !should_remove {
                    new_table_body.push_str(line);
                    new_table_body.push('\n');
                }
            }
            
            // Reconstruct the table definition
            let new_table_definition = format!("{}{}{}", table_start, new_table_body, table_end);
            
            // Replace the old table definition with the new one
            let new_content = table_regex.replace(existing_content, new_table_definition.as_str());
            
            Ok(new_content.to_string())
        } else {
            Err(format!("Could not find table '{}' in schema.rs", table_name))
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
    
    /// Add fields to an existing table in the schema with proper field ordering
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
            
            // Get existing fields from the table body
            let existing_fields = Self::parse_existing_fields_from_table_body(table_body)?;
            
            // Combine existing and new fields with proper ordering
            let ordered_fields = Self::order_fields_properly(&existing_fields, &new_fields)?;
            
            // Generate the new table body with properly ordered fields
            let mut new_table_body = String::new();
            for field in &ordered_fields {
                new_table_body.push_str(&format!("        {} -> {},\n", field.name, field.diesel_type));
            }
            
            // Reconstruct the table with ordered fields
            let new_table_definition = format!("{}{}{}", table_start, new_table_body, table_end);
            
            // Replace the old table definition with the new one
            let new_content = table_regex.replace(existing_content, new_table_definition.as_str());
            
            // Write the updated schema
            if let Err(e) = fs::write(file_path, new_content.as_ref()) {
                return Err(format!("Failed to write schema.rs: {}", e));
            }
            
            println!("Added {} new fields to table '{}' in schema.rs with proper ordering", new_fields.len(), table_def.table_name);
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
    /// Convert FieldTypeInfo to a field definition string for migrations
    fn field_type_info_to_definition(field_type_info: &FieldTypeInfo) -> String {
        // Convert database types to Diesel types
        let diesel_type = match field_type_info.field_type.to_lowercase().as_str() {
            "bool" | "boolean" => "Bool",
            "text" | "varchar" | "char" => "Text",
            "integer" | "int4" => "Int4",
            "float" | "float4" => "Float4",
            "float8" | "double" => "Float8",
            "timestamp" | "timestamptz" => "Timestamp",
            "jsonb" => "Jsonb",
            "json" => "Json",
            "inet" => "Inet",
            "uuid" => "Uuid",
            "bytea" => "Bytea",
            "numeric" | "decimal" => "Numeric",
            _ => &field_type_info.field_type, // fallback to original
        };
        
        let mut definition = diesel_type.to_string();
        
        // Handle nullable wrapper
        if field_type_info.nullable {
            definition = format!("Nullable<{}>", definition);
        }
        
        // Handle array wrapper
        if field_type_info.is_array {
            definition = format!("Array<{}>", definition);
        }
        
        definition
    }
    
    /// Parse existing fields from table body in schema.rs
    fn parse_existing_fields_from_table_body(table_body: &str) -> Result<Vec<crate::schema::generator::field_definition::ParsedField>, String> {
        let mut fields = Vec::new();
        
        for line in table_body.lines() {
            let line = line.trim();
            if line.is_empty() || !line.contains(" -> ") {
                continue;
            }
            
            // Parse field definition: "field_name -> Type,"
            if let Some(arrow_pos) = line.find(" -> ") {
                let field_name = line[..arrow_pos].trim();
                let rest = &line[arrow_pos + 4..];
                let diesel_type = if let Some(comma_pos) = rest.find(',') {
                    rest[..comma_pos].trim()
                } else {
                    rest.trim()
                };
                
                // Parse the diesel type to determine other properties
                let is_nullable = diesel_type.starts_with("Nullable<");
                let is_array = diesel_type.contains("Array<");
                let is_json = diesel_type.contains("Jsonb");
                
                // Extract core type for rust type mapping
                let mut core_type = diesel_type;
                if is_nullable {
                    core_type = &diesel_type[9..diesel_type.len()-1]; // Remove "Nullable<" and ">"
                }
                if is_array {
                    if let Some(start) = core_type.find("Array<") {
                        let end = core_type.rfind(">").unwrap_or(core_type.len());
                        core_type = &core_type[start+6..end];
                    }
                }
                
                // Map to rust type
                let base_rust_type = match core_type {
                    "Text" => "String",
                    "Int4" => "i32",
                    "Int8" | "BigInt" => "i64",
                    "Bool" => "bool",
                    "Timestamp" | "Timestamptz" => "chrono::NaiveDateTime",
                    "Jsonb" => "Value",
                    "Inet" => "std::net::IpAddr",
                    _ => "String", // Default fallback
                };
                
                let rust_type = if is_array {
                    if is_nullable {
                        format!("Option<Vec<{}>>", base_rust_type)
                    } else {
                        format!("Vec<{}>", base_rust_type)
                    }
                } else if is_nullable {
                    format!("Option<{}>", base_rust_type)
                } else {
                    base_rust_type.to_string()
                };
                
                fields.push(crate::schema::generator::field_definition::ParsedField {
                    name: field_name.to_string(),
                    diesel_type: diesel_type.to_string(),
                    rust_type,
                    is_nullable,
                    migration_nullable: is_nullable, // Assume same as nullable
                    is_array,
                    is_json,
                    is_index: false, // Can't determine from schema.rs
                    is_primary_key: false, // Can't determine from schema.rs
                    foreign_key: None, // Can't determine from schema.rs
                    default_value: None, // Can't determine from schema.rs
                });
            }
        }
        
        Ok(fields)
    }
    
    /// Order fields properly according to system fields macro and entity-specific fields
    fn order_fields_properly(
        existing_fields: &[crate::schema::generator::field_definition::ParsedField],
        new_fields: &[crate::schema::generator::field_definition::ParsedField]
    ) -> Result<Vec<crate::schema::generator::field_definition::ParsedField>, String> {
        let system_field_names = Self::get_system_field_names()?;
        let mut ordered_fields = Vec::new();
        
        // Combine all fields
        let mut all_fields = existing_fields.to_vec();
        all_fields.extend_from_slice(new_fields);
        
        // First, add system fields in the order defined by system_fields macro
        for system_field_name in &system_field_names {
            if let Some(field) = all_fields.iter().find(|f| f.name == *system_field_name) {
                ordered_fields.push(field.clone());
            }
        }
        
        // Then, add non-system fields (entity-specific fields)
        for field in &all_fields {
            if !system_field_names.contains(&field.name) {
                ordered_fields.push(field.clone());
            }
        }
        
        Ok(ordered_fields)
    }
    
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
            hypertable: false,
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