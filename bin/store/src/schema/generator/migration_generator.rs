use crate::schema::generator::field_definition::TableDefinition;
use crate::schema::generator::schema_generator::{SchemaChange, SchemaChangeType};
use std::fs;
use std::path::Path;
use std::io::{self, Write};
use chrono::{DateTime, Utc};

pub struct MigrationGenerator;

impl MigrationGenerator {
    /// Generate migration files for the given changes
    pub fn generate_migration(changes: &[SchemaChange], table_def: &TableDefinition) -> Result<(), String> {
        if changes.is_empty() {
            println!("No changes detected, skipping migration generation");
            return Ok(());
        }
        
        // Get migration name from user
        let migration_name = Self::get_migration_name_from_user()?;
        
        // Generate timestamp
        let timestamp = Self::generate_timestamp();
        
        // Create migration directory
        let migration_dir = format!("migrations/{}_{}", timestamp, migration_name);
        
        // Check if migration already exists
        if Path::new(&migration_dir).exists() {
            return Err(format!("Migration directory already exists: {}", migration_dir));
        }
        
        // Create migration directory
        if let Err(e) = fs::create_dir_all(&migration_dir) {
            return Err(format!("Failed to create migration directory: {}", e));
        }
        
        // Generate up.sql and down.sql
        let up_sql = Self::generate_up_sql(changes, table_def)?;
        let down_sql = Self::generate_down_sql(changes, table_def)?;
        
        // Write migration files
        let up_file = format!("{}/up.sql", migration_dir);
        let down_file = format!("{}/down.sql", migration_dir);
        
        if let Err(e) = fs::write(&up_file, up_sql) {
            return Err(format!("Failed to write up.sql: {}", e));
        }
        
        if let Err(e) = fs::write(&down_file, down_sql) {
            return Err(format!("Failed to write down.sql: {}", e));
        }
        
        println!("Generated migration: {}", migration_dir);
        Ok(())
    }
    
    /// Get migration name from user input
    fn get_migration_name_from_user() -> Result<String, String> {
        loop {
            print!("Enter migration name: ");
            io::stdout().flush().map_err(|e| format!("Failed to flush stdout: {}", e))?;
            
            let mut input = String::new();
            io::stdin().read_line(&mut input).map_err(|e| format!("Failed to read input: {}", e))?;
            
            let migration_name = input.trim().to_string();
            
            if migration_name.is_empty() {
                println!("Migration name cannot be empty. Please try again.");
                continue;
            }
            
            // Check if migration with this name already exists
            if Self::migration_exists(&migration_name)? {
                println!("Migration already exists, please enter a different name.");
                continue;
            }
            
            // Validate migration name (only alphanumeric and underscores)
            if !migration_name.chars().all(|c| c.is_alphanumeric() || c == '_') {
                println!("Migration name can only contain alphanumeric characters and underscores. Please try again.");
                continue;
            }
            
            return Ok(migration_name);
        }
    }
    
    /// Check if a migration with the given name already exists
    fn migration_exists(name: &str) -> Result<bool, String> {
        let migrations_dir = "migrations";
        
        if !Path::new(migrations_dir).exists() {
            return Ok(false);
        }
        
        let entries = fs::read_dir(migrations_dir)
            .map_err(|e| format!("Failed to read migrations directory: {}", e))?;
        
        for entry in entries {
            let entry = entry.map_err(|e| format!("Failed to read directory entry: {}", e))?;
            let dir_name = entry.file_name().to_string_lossy().to_string();
            
            // Migration directory format: YYYY-MM-DD-HHMMSS_name
            if let Some(migration_name_part) = dir_name.split('_').nth(1) {
                if migration_name_part == name {
                    return Ok(true);
                }
            }
        }
        
        Ok(false)
    }
    
    /// Generate timestamp for migration
    fn generate_timestamp() -> String {
        let now: DateTime<Utc> = Utc::now();
        now.format("%Y-%m-%d-%H%M%S").to_string()
    }
    
    /// Generate the up.sql content
    fn generate_up_sql(changes: &[SchemaChange], table_def: &TableDefinition) -> Result<String, String> {
        let mut sql = String::new();
        sql.push_str("-- Your SQL goes here\n\n");
        
        // Group changes by type
        let mut new_tables = Vec::new();
        let mut new_fields = Vec::new();
        let mut new_indexes = Vec::new();
        
        for change in changes {
            match change.change_type {
                SchemaChangeType::NewTable => new_tables.push(change),
                SchemaChangeType::NewField => new_fields.push(change),
                SchemaChangeType::NewIndex => new_indexes.push(change),
            }
        }
        
        // Generate CREATE TABLE statements
        for change in new_tables {
            let create_table_sql = Self::generate_create_table_sql(&change.table_name, table_def)?;
            sql.push_str(&create_table_sql);
            sql.push_str("\n\n");
        }
        
        // Generate ALTER TABLE ADD COLUMN statements
        for change in new_fields {
            if let (Some(field_name), Some(field_definition)) = (&change.field_name, &change.field_definition) {
                let alter_sql = Self::generate_add_column_sql(&change.table_name, field_name, field_definition)?;
                sql.push_str(&alter_sql);
                sql.push_str("\n\n");
            }
        }
        
        // Generate CREATE INDEX statements
        for change in new_indexes {
            if let Some(field_name) = &change.field_name {
                let index_sql = Self::generate_create_index_sql(&change.table_name, field_name)?;
                sql.push_str(&index_sql);
                sql.push_str("\n\n");
            }
        }
        
        Ok(sql)
    }
    
    /// Generate the down.sql content
    fn generate_down_sql(changes: &[SchemaChange], _table_def: &TableDefinition) -> Result<String, String> {
        let mut sql = String::new();
        sql.push_str("-- This file should undo anything in `up.sql`\n\n");
        
        // Generate reverse operations (in reverse order)
        let mut reverse_changes = changes.to_vec();
        reverse_changes.reverse();
        
        for change in reverse_changes {
            match change.change_type {
                SchemaChangeType::NewTable => {
                    sql.push_str(&format!("DROP TABLE IF EXISTS \"{}\";\n", change.table_name));
                },
                SchemaChangeType::NewField => {
                    if let Some(field_name) = &change.field_name {
                        sql.push_str(&format!(
                            "ALTER TABLE \"{}\" DROP COLUMN IF EXISTS \"{}\";\n",
                            change.table_name, field_name
                        ));
                    }
                },
                SchemaChangeType::NewIndex => {
                    if let Some(field_name) = &change.field_name {
                        let index_name = format!("idx_{}_{}", change.table_name, field_name);
                        sql.push_str(&format!("DROP INDEX IF EXISTS \"{}\";\n", index_name));
                    }
                },
            }
        }
        
        Ok(sql)
    }
    
    /// Generate CREATE TABLE SQL
    fn generate_create_table_sql(table_name: &str, table_def: &TableDefinition) -> Result<String, String> {
        let mut sql = String::new();
        
        sql.push_str(&format!("CREATE TABLE \"{}\" (\n", table_name));
        
        let mut parsed_fields = Vec::new();
        for field in &table_def.fields {
            match field.parse() {
                Ok(parsed) => parsed_fields.push(parsed),
                Err(e) => return Err(format!("Error parsing field {}: {}", field.field_name, e)),
            }
        }
        
        // Add fields
        for (i, field) in parsed_fields.iter().enumerate() {
            let postgres_type = Self::diesel_to_postgres_type(&field.diesel_type)?;
            let default_clause = if let Some(default) = &field.default_value {
                format!(" DEFAULT {}", default)
            } else {
                String::new()
            };
            
            sql.push_str(&format!(
                "    \"{}\" {}{}",
                field.name, postgres_type, default_clause
            ));
            
            if i < parsed_fields.len() - 1 {
                sql.push(',');
            }
            sql.push('\n');
        }
        
        sql.push_str(");");
        
        Ok(sql)
    }
    
    /// Generate ALTER TABLE ADD COLUMN SQL
    fn generate_add_column_sql(table_name: &str, field_name: &str, field_type: &str) -> Result<String, String> {
        let postgres_type = Self::diesel_to_postgres_type(field_type)?;
        Ok(format!(
            "ALTER TABLE \"{}\" ADD COLUMN \"{}\" {};",
            table_name, field_name, postgres_type
        ))
    }
    
    /// Generate CREATE INDEX SQL
    fn generate_create_index_sql(table_name: &str, field_name: &str) -> Result<String, String> {
        let index_name = format!("idx_{}_{}", table_name, field_name);
        Ok(format!(
            "CREATE INDEX \"{}\" ON \"{}\" (\"{}\");",
            index_name, table_name, field_name
        ))
    }
    
    /// Convert Diesel type to PostgreSQL type
    fn diesel_to_postgres_type(diesel_type: &str) -> Result<String, String> {
        let mut postgres_type = diesel_type;
        let mut is_nullable = false;
        
        // Handle Nullable wrapper
        if diesel_type.starts_with("Nullable<") && diesel_type.ends_with(">") {
            is_nullable = true;
            postgres_type = &diesel_type[9..diesel_type.len()-1];
        }
        
        // Handle Array wrapper
        let mut is_array = false;
        if postgres_type.starts_with("Array<") && postgres_type.ends_with(">") {
            is_array = true;
            postgres_type = &postgres_type[6..postgres_type.len()-1];
        }
        
        // Convert core type
        let core_type = match postgres_type {
            "Text" => "TEXT",
            "Int4" => "INTEGER",
            "Int8" => "BIGINT",
            "BigInt" => "BIGINT",
            "Bool" => "BOOLEAN",
            "Timestamp" => "TIMESTAMP",
            "Timestamptz" => "TIMESTAMPTZ",
            "Jsonb" => "JSONB",
            "Inet" => "INET",
            _ => return Err(format!("Unsupported Diesel type: {}", postgres_type)),
        };
        
        // Build final type
        let mut final_type = core_type.to_string();
        
        if is_array {
            final_type = format!("{}[]", final_type);
        }
        
        if !is_nullable {
            final_type.push_str(" NOT NULL");
        }
        
        Ok(final_type)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_diesel_to_postgres_type() {
        assert_eq!(MigrationGenerator::diesel_to_postgres_type("Text").unwrap(), "TEXT NOT NULL");
        assert_eq!(MigrationGenerator::diesel_to_postgres_type("Nullable<Text>").unwrap(), "TEXT");
        assert_eq!(MigrationGenerator::diesel_to_postgres_type("Nullable<Array<Text>>").unwrap(), "TEXT[]");
        assert_eq!(MigrationGenerator::diesel_to_postgres_type("Int4").unwrap(), "INTEGER NOT NULL");
        assert_eq!(MigrationGenerator::diesel_to_postgres_type("Nullable<Jsonb>").unwrap(), "JSONB");
    }

    #[test]
    fn test_generate_create_index_sql() {
        let result = MigrationGenerator::generate_create_index_sql("users", "email");
        assert!(result.is_ok());
        let sql = result.unwrap();
        assert_eq!(sql, "CREATE INDEX \"idx_users_email\" ON \"users\" (\"email\");");
    }

    #[test]
    fn test_generate_add_column_sql() {
        let result = MigrationGenerator::generate_add_column_sql("users", "email", "Nullable<Text>");
        assert!(result.is_ok());
        let sql = result.unwrap();
        assert_eq!(sql, "ALTER TABLE \"users\" ADD COLUMN \"email\" TEXT;");
    }
}