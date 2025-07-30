use crate::schema::generator::field_definition::{ParsedField, TableDefinition};
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
        
        // Add index changes
         for (index_name, columns, _is_unique, index_type) in indexes {
             // For now, we'll assume all index requests are new
             // In a more sophisticated implementation, we'd check existing indexes
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
         
         // Add foreign key changes
         for foreign_key in foreign_keys {
             let field_def = format!("{}|{}|{}", 
                 foreign_key.column, 
                 foreign_key.references_table, 
                 foreign_key.references_column
             );
             
             // Generate constraint name: fk_tablename_columnname
             let constraint_name = format!("fk_{}_{}", table_def.table_name, foreign_key.column);
             
             changes.push(SchemaChange {
                 table_name: table_def.table_name.clone(),
                 change_type: SchemaChangeType::NewForeignKey,
                 field_name: Some(constraint_name),
                 field_definition: Some(field_def),
             });
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
        let table_definition = Self::generate_table_definition(&table_def.table_name, &parsed_fields)?;
        
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
    
    /// Generate a complete table definition
    fn generate_table_definition(table_name: &str, fields: &[ParsedField]) -> Result<String, String> {
        let mut definition = String::new();
        
        // Determine primary key (assume 'id' if present, otherwise first field)
        let primary_key = fields.iter()
            .find(|f| f.name == "id")
            .map(|f| f.name.as_str())
            .unwrap_or(&fields[0].name);
        
        definition.push_str(&format!("table! {{\n    {}({}) {{\n", table_name, primary_key));
        
        // Add all fields
        for field in fields {
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
        let fields = vec![
            crate::schema::generator::field_definition::ParsedField {
                name: "id".to_string(),
                diesel_type: "Nullable<Text>".to_string(),
                rust_type: "Option<String>".to_string(),
                is_nullable: true,
                is_array: false,
                is_json: false,
                is_index: false,
                foreign_key: None,
                default_value: None,
            },
            crate::schema::generator::field_definition::ParsedField {
                name: "name".to_string(),
                diesel_type: "Nullable<Text>".to_string(),
                rust_type: "Option<String>".to_string(),
                is_nullable: true,
                is_array: false,
                is_json: false,
                is_index: false,
                foreign_key: None,
                default_value: None,
            },
        ];
        
        let result = SchemaGenerator::generate_table_definition("test_table", &fields);
        assert!(result.is_ok());
        let definition = result.unwrap();
        assert!(definition.contains("table! {"));
        assert!(definition.contains("test_table(id)"));
        assert!(definition.contains("id -> Nullable<Text>"));
        assert!(definition.contains("name -> Nullable<Text>"));
    }
}