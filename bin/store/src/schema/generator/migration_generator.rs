use crate::schema::generator::field_definition::TableDefinition;
use crate::schema::generator::schema_generator::{SchemaChange, SchemaChangeType};
use std::fs;
use std::path::Path;
use std::io::{self, Write};
use chrono::{DateTime, Utc};
use log::{info, debug};

pub struct MigrationGenerator;

impl MigrationGenerator {
    /// Generate migration files for the given changes
    pub fn generate_migration(changes: &[SchemaChange], table_definitions: &[TableDefinition]) -> Result<(), String> {
        if changes.is_empty() {
            return Ok(());
        }
        
        // Log migration summary
        info!("Creating migration with {} change(s)", changes.len());
        let mut change_summary = std::collections::HashMap::new();
        for change in changes {
            *change_summary.entry(&change.change_type).or_insert(0) += 1;
        }
        for (change_type, count) in &change_summary {
            debug!("  - {:?}: {} change(s)", change_type, count);
        }
        
        // Get migration name from user
        let migration_name = Self::get_migration_name_from_user(changes.len())?;
        
        // Generate timestamp
        let timestamp = Self::generate_timestamp();
        
        // Create migration directory
        let migration_dir = format!("migrations/{}_{}", timestamp, migration_name);
        info!("Creating migration directory: {}", migration_dir);
        
        // Check if migration already exists
        if Path::new(&migration_dir).exists() {
            return Err(format!("Migration directory already exists: {}", migration_dir));
        }
        
        // Create migration directory
        if let Err(e) = fs::create_dir_all(&migration_dir) {
            return Err(format!("Failed to create migration directory: {}", e));
        }
        
        // Generate up.sql and down.sql
        let up_sql = Self::generate_up_sql(changes, table_definitions)?;
        let down_sql = Self::generate_down_sql(changes, table_definitions)?;
        
        // Write migration files
        let up_file = format!("{}/up.sql", migration_dir);
        let down_file = format!("{}/down.sql", migration_dir);
        
        if let Err(e) = fs::write(&up_file, up_sql) {
            return Err(format!("Failed to write up.sql: {}", e));
        }
        
        if let Err(e) = fs::write(&down_file, down_sql) {
            return Err(format!("Failed to write down.sql: {}", e));
        }
        
        info!("Migration '{}' created successfully", migration_name);
        debug!("Migration files written to: {}", migration_dir);
        
        Ok(())
    }
    
    /// Get migration name from user input
    fn get_migration_name_from_user(change_count: usize) -> Result<String, String> {
        loop {
            info!("Enter migration name for {} change(s): ", change_count);
            io::stdout().flush().map_err(|e| format!("Failed to flush stdout: {}", e))?;
            
            let mut input = String::new();
            io::stdin().read_line(&mut input).map_err(|e| format!("Failed to read input: {}", e))?;
            
            let migration_name = input.trim().to_string();
            
            if migration_name.is_empty() {
                continue;
            }
            
            // Check if migration with this name already exists
            if Self::migration_exists(&migration_name)? {
                continue;
            }
            
            // Validate migration name (only alphanumeric and underscores)
            if !migration_name.chars().all(|c| c.is_alphanumeric() || c == '_') {
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
    pub fn generate_up_sql(changes: &[SchemaChange], table_definitions: &[TableDefinition]) -> Result<String, String> {
        let mut sql = String::new();
        sql.push_str("-- Your SQL goes here\n\n");
        
        // Group changes by table name and type
        use std::collections::HashMap;
        let _tables_by_name: HashMap<String, &TableDefinition> = HashMap::new();
        let mut new_tables = Vec::new();
        let mut new_fields = Vec::new();
        let mut removed_fields = Vec::new();
        let mut new_indexes = Vec::new();
        let mut new_foreign_keys = Vec::new();
        
        for change in changes {
            match change.change_type {
                SchemaChangeType::NewTable => new_tables.push(change),
                SchemaChangeType::NewField => new_fields.push(change),
                SchemaChangeType::RemovedField => removed_fields.push(change),
                SchemaChangeType::NewIndex => new_indexes.push(change),
                SchemaChangeType::NewForeignKey => new_foreign_keys.push(change),
            }
        }
        
        let mut first_statement = true;
        
        // Generate CREATE TABLE statements
        for change in new_tables {
            if !first_statement {
                sql.push_str("--> statement-breakpoint\n");
            }
            
            // Find the correct table definition for this table
            let table_def = table_definitions.iter()
                .find(|def| def.name == change.table_name)
                .ok_or_else(|| format!("Table definition not found for table: {}", change.table_name))?;
            
            let create_table_sql = Self::generate_create_table_sql(&change.table_name, table_def)?;
            sql.push_str(&create_table_sql);
            sql.push_str("\n");
            
            // Add hypertable creation if this is a hypertable
            if table_def.is_hypertable {
                sql.push_str("--> statement-breakpoint\n");
                let hypertable_sql = Self::generate_hypertable_sql(&change.table_name)?;
                sql.push_str(&hypertable_sql);
                sql.push_str("\n");
            }
            
            first_statement = false;
        }
        
        // Generate ALTER TABLE ADD COLUMN statements
        for change in new_fields {
            if let (Some(field_name), Some(field_definition)) = (&change.field_name, &change.field_definition) {
                if !first_statement {
                    sql.push_str("--> statement-breakpoint\n");
                }
                let alter_sql = Self::generate_add_column_sql(&change.table_name, field_name, field_definition, true)?;
                sql.push_str(&alter_sql);
                sql.push_str("\n");
                first_statement = false;
            }
        }
        
        // Generate ALTER TABLE DROP COLUMN statements
        for change in removed_fields {
            if let Some(field_name) = &change.field_name {
                if !first_statement {
                    sql.push_str("--> statement-breakpoint\n");
                }
                let drop_sql = Self::generate_drop_column_sql(&change.table_name, field_name)?;
                sql.push_str(&drop_sql);
                sql.push_str("\n");
                first_statement = false;
            }
        }
        
        // Generate CREATE INDEX statements
        for change in new_indexes {
            if let (Some(index_name), Some(column_names)) = (&change.field_name, &change.field_definition) {
                if !first_statement {
                    sql.push_str("--> statement-breakpoint\n");
                }
                // Parse index type from column_names if it contains type info
                let (columns, index_type) = if column_names.contains("|") {
                    let parts: Vec<&str> = column_names.split("|").collect();
                    (parts[0], Some(parts[1]))
                } else {
                    (column_names.as_str(), None)
                };
                let index_sql = Self::generate_create_index_sql(&change.table_name, index_name, columns, index_type)?;
                sql.push_str(&index_sql);
                sql.push_str("\n");
                first_statement = false;
            }
        }
        
        // Generate ALTER TABLE ADD CONSTRAINT statements for foreign keys
        for change in new_foreign_keys {
            if let (Some(constraint_name), Some(field_definition)) = (&change.field_name, &change.field_definition) {
                if !first_statement {
                    sql.push_str("--> statement-breakpoint\n");
                }
                let foreign_key_sql = Self::generate_add_foreign_key_sql(&change.table_name, constraint_name, field_definition)?;
                sql.push_str(&foreign_key_sql);
                sql.push_str("\n");
                first_statement = false;
            }
        }
        
        Ok(sql)
    }
    
    /// Generate the down.sql content
    fn generate_down_sql(changes: &[SchemaChange], _table_definitions: &[TableDefinition]) -> Result<String, String> {
        let mut sql = String::new();
        sql.push_str("-- This file should undo anything in `up.sql`\n\n");
        
        // Generate reverse operations (in reverse order)
        let mut reverse_changes = changes.to_vec();
        reverse_changes.reverse();
        
        let first_statement = true;
        
        for change in reverse_changes {
            if !first_statement {
                sql.push_str("--> statement-breakpoint\n");
            }
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
                SchemaChangeType::RemovedField => {
                    if let (Some(field_name), Some(field_definition)) = (&change.field_name, &change.field_definition) {
                        // For removed fields, the down migration should add the field back
                        let postgres_type = Self::diesel_to_postgres_type(field_definition, true).unwrap_or_else(|_| "TEXT".to_string());
                        sql.push_str(&format!(
                            "ALTER TABLE \"{}\" ADD COLUMN \"{}\" {};\n",
                            change.table_name, field_name, postgres_type
                        ));
                    }
                },
                SchemaChangeType::NewIndex => {
                    if let Some(field_name) = &change.field_name {
                        let index_name = format!("idx_{}_{}", change.table_name, field_name);
                        sql.push_str(&format!("DROP INDEX IF EXISTS \"{}\";
", index_name));
                    }
                },
                SchemaChangeType::NewForeignKey => {
                    if let Some(constraint_name) = &change.field_name {
                        sql.push_str(&format!(
                            "ALTER TABLE \"{}\" DROP CONSTRAINT IF EXISTS \"{}\";
",
                            change.table_name, constraint_name
                        ));
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
            match field.parse_for_context(true) { // Use migration context to preserve VARCHAR
                Ok(parsed) => parsed_fields.push(parsed),
                Err(e) => return Err(format!("Error parsing field {}: {}", field.name, e)),
            }
        }
        
        // Collect primary key fields
        let primary_key_fields: Vec<&str> = table_def.fields
            .iter()
            .filter(|field| field.is_primary_key)
            .map(|field| field.name.as_str())
            .collect();
        
        // Add fields
        for (i, field) in parsed_fields.iter().enumerate() {
            let postgres_type = Self::diesel_to_postgres_type(&field.field_type, field.migration_nullable)?;
            let default_clause = if let Some(default) = &field.default_value {
                // Check if this is a TEXT field that needs quotes around string defaults
                if postgres_type.contains("TEXT") && !default.starts_with("'") && !default.chars().all(|c| c.is_ascii_digit() || c == '.' || c == '-') {
                    format!(" DEFAULT '{}'", default)
                } else {
                    format!(" DEFAULT {}", default)
                }
            } else {
                String::new()
            };
            
            sql.push_str(&format!(
                "    \"{}\" {}{}",
                field.name, postgres_type, default_clause
            ));
            
            // Add comma if not the last field or if we have primary keys to add
            if i < parsed_fields.len() - 1 || !primary_key_fields.is_empty() {
                sql.push(',');
            }
            sql.push('\n');
        }
        
        // Add primary key constraint if any primary key fields exist
        if !primary_key_fields.is_empty() {
            if primary_key_fields.len() == 1 {
                sql.push_str(&format!("    PRIMARY KEY (\"{}\")", primary_key_fields[0]));
            } else {
                let pk_columns = primary_key_fields
                    .iter()
                    .map(|field| format!("\"{}\"", field))
                    .collect::<Vec<_>>()
                    .join(", ");
                sql.push_str(&format!("    PRIMARY KEY ({})", pk_columns));
            }
            sql.push('\n');
        }
        
        sql.push_str(");");
        
        Ok(sql)
    }
    
    /// Generate ALTER TABLE ADD COLUMN SQL
    fn generate_add_column_sql(table_name: &str, field_name: &str, field_type: &str, migration_nullable: bool) -> Result<String, String> {
        let postgres_type = Self::diesel_to_postgres_type(field_type, migration_nullable)?;
        Ok(format!(
            "ALTER TABLE \"{}\" ADD COLUMN \"{}\" {};",
            table_name, field_name, postgres_type
        ))
    }
    
    /// Generate ALTER TABLE DROP COLUMN SQL
    fn generate_drop_column_sql(table_name: &str, field_name: &str) -> Result<String, String> {
        Ok(format!(
            "ALTER TABLE \"{}\" DROP COLUMN \"{}\";",
            table_name, field_name
        ))
    }
    
    /// Generate CREATE INDEX SQL
    fn generate_create_index_sql(table_name: &str, index_name: &str, column_names: &str, index_type: Option<&str>) -> Result<String, String> {
        // The index_name already contains the full name from the macro
        let using_clause = if let Some(idx_type) = index_type {
             format!(" USING {}", idx_type)
         } else {
             String::new()
         };
         
         Ok(format!(
             "CREATE INDEX \"{}\" ON \"{}\"{}({});",
             index_name, table_name, using_clause, column_names
         ))
    }
    
    /// Generate ADD CONSTRAINT FOREIGN KEY SQL
    pub fn generate_add_foreign_key_sql(table_name: &str, constraint_name: &str, field_definition: &str) -> Result<String, String> {
        // Parse field_definition: "column_name|referenced_table|referenced_column"
        let parts: Vec<&str> = field_definition.split('|').collect();
        if parts.len() != 3 {
            return Err(format!("Invalid foreign key definition: {}", field_definition));
        }
        
        let column_name = parts[0];
        let referenced_table = parts[1];
        let referenced_column = parts[2];
        
        // Clean up any potential quotes or commas in all fields
        let clean_column_name = column_name.replace(",", "").replace("\"", "");
        let clean_referenced_table = referenced_table.replace(",", "").replace("\"", "");
        let clean_referenced_column = referenced_column.replace(",", "").replace("\"", "");
        
        Ok(format!(
            "ALTER TABLE \"{}\" ADD CONSTRAINT \"{}\" FOREIGN KEY (\"{}\") REFERENCES \"public\".\"{}\"(\"{}\") ON DELETE no action ON UPDATE no action;",
            table_name, constraint_name, clean_column_name, clean_referenced_table, clean_referenced_column
        ))
    }
    
    /// Convert Diesel type to PostgreSQL type
    fn diesel_to_postgres_type(diesel_type: &str, migration_nullable: bool) -> Result<String, String> {
        let mut postgres_type = diesel_type;
        
        // Handle Nullable wrapper - extract the inner type
        if diesel_type.starts_with("Nullable<") && diesel_type.ends_with(">") {
            postgres_type = &diesel_type[9..diesel_type.len()-1];
        }
        
        // Handle Array wrapper
        let mut is_array = false;
        if postgres_type.starts_with("Array<") && postgres_type.ends_with(">") {
            is_array = true;
            postgres_type = &postgres_type[6..postgres_type.len()-1];
        }
        
        // Convert core type
        let core_type = if postgres_type.contains('<') && postgres_type.ends_with('>') {
            // Handle generic types with parameters like "Varchar<300>", "Decimal<10,2>", etc.
            if let Some(angle_pos) = postgres_type.find('<') {
                let base_type = &postgres_type[..angle_pos];
                let params = &postgres_type[angle_pos+1..postgres_type.len()-1];
                
                match base_type {
                    "Varchar" => format!("VARCHAR({})", params),
                    "Char" => format!("CHAR({})", params),
                    "Decimal" | "Numeric" => format!("DECIMAL({})", params),
                    _ => return Err(format!("Unsupported generic Diesel type: {}", postgres_type)),
                }
            } else {
                return Err(format!("Invalid generic type format: {}", postgres_type));
            }
        } else {
            match postgres_type {
                "Text" => "TEXT".to_string(),
                "Varchar" => "VARCHAR".to_string(),
                "Char" => "CHAR".to_string(),
                "Int4" => "INTEGER".to_string(),
                "Int8" => "BIGINT".to_string(),
                "BigInt" => "BIGINT".to_string(),
                "Bool" => "BOOLEAN".to_string(),
                "Timestamp" => "TIMESTAMP".to_string(),
                "Timestamptz" => "TIMESTAMPTZ".to_string(),
                "Jsonb" => "JSONB".to_string(),
                "Inet" => "INET".to_string(),
                "Decimal" | "Numeric" => "DECIMAL".to_string(),
                _ => return Err(format!("Unsupported Diesel type: {}", postgres_type)),
            }
        };
        
        // Build final type
        let mut final_type = core_type;
        
        if is_array {
            final_type = format!("{}[]", final_type);
        }
        
        if !migration_nullable {
            final_type.push_str(" NOT NULL");
        }
        
        Ok(final_type)
    }
    
    /// Generate hypertable creation SQL
    fn generate_hypertable_sql(table_name: &str) -> Result<String, String> {
        Ok(format!(
            "SELECT create_hypertable('{}', 'timestamp', chunk_time_interval => INTERVAL '1 day', if_not_exists => TRUE);",
            table_name
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_diesel_to_postgres_type() {
        // Test with migration_nullable = false (NOT NULL)
        assert_eq!(MigrationGenerator::diesel_to_postgres_type("Text", false).unwrap(), "TEXT NOT NULL");
        assert_eq!(MigrationGenerator::diesel_to_postgres_type("Nullable<Text>", false).unwrap(), "TEXT NOT NULL");
        assert_eq!(MigrationGenerator::diesel_to_postgres_type("Int4", false).unwrap(), "INTEGER NOT NULL");
        
        // Test with migration_nullable = true (nullable)
        assert_eq!(MigrationGenerator::diesel_to_postgres_type("Text", true).unwrap(), "TEXT");
        assert_eq!(MigrationGenerator::diesel_to_postgres_type("Nullable<Text>", true).unwrap(), "TEXT");
        assert_eq!(MigrationGenerator::diesel_to_postgres_type("Nullable<Array<Text>>", true).unwrap(), "TEXT[]");
        assert_eq!(MigrationGenerator::diesel_to_postgres_type("Nullable<Jsonb>", true).unwrap(), "JSONB");
    }

    #[test]
    fn test_generate_create_index_sql() {
        let result = MigrationGenerator::generate_create_index_sql("users", "idx_users_email", "\"email\"", None);
        assert!(result.is_ok());
        let sql = result.unwrap();
        assert_eq!(sql, "CREATE INDEX \"idx_users_email\" ON \"users\" (\"email\");");
        
        let result_with_type = MigrationGenerator::generate_create_index_sql("users", "idx_users_data", "\"data\"", Some("gin"));
         assert!(result_with_type.is_ok());
         let sql_with_type = result_with_type.unwrap();
         assert_eq!(sql_with_type, "CREATE INDEX \"idx_users_data\" ON \"users\" USING gin (\"data\");");
    }

    #[test]
    fn test_generate_add_column_sql() {
        let result = MigrationGenerator::generate_add_column_sql("users", "email", "Nullable<Text>", true);
        assert!(result.is_ok());
        let sql = result.unwrap();
        assert_eq!(sql, "ALTER TABLE \"users\" ADD COLUMN \"email\" TEXT;");
    }
}