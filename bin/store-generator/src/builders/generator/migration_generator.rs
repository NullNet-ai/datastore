use crate::builders::generator::field_definition::TableDefinition;
use crate::builders::generator::schema_generator::{SchemaChange, SchemaChangeType};
use crate::constants::paths::database::MIGRATIONS_DIR;
use chrono::{DateTime, Utc};
use log::{debug, info};
use std::fs;
use std::io::{self, Write};
use std::path::Path;

pub struct MigrationGenerator;

impl MigrationGenerator {
    /// Generate migration files for the given changes
    pub fn generate_migration(
        changes: &[SchemaChange],
        table_definitions: &[TableDefinition],
    ) -> Result<(), String> {
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
        let migration_dir = format!(
            "{}/{}_{}",
            MIGRATIONS_DIR.as_str(),
            timestamp,
            migration_name
        );
        info!("Creating migration directory: {}", migration_dir);

        // Check if migration already exists
        if Path::new(&migration_dir).exists() {
            return Err(format!(
                "Migration directory already exists: {}",
                migration_dir
            ));
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
            io::stdout()
                .flush()
                .map_err(|e| format!("Failed to flush stdout: {}", e))?;

            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .map_err(|e| format!("Failed to read input: {}", e))?;

            let migration_name = input.trim().to_string();

            if migration_name.is_empty() {
                continue;
            }

            // Check if migration with this name already exists
            if Self::migration_exists(&migration_name)? {
                continue;
            }

            // Validate migration name (only alphanumeric and underscores)
            if !migration_name
                .chars()
                .all(|c| c.is_alphanumeric() || c == '_')
            {
                continue;
            }

            return Ok(migration_name);
        }
    }

    /// Check if a migration with the given name already exists
    fn migration_exists(name: &str) -> Result<bool, String> {
        let migrations_dir = MIGRATIONS_DIR.as_str();

        if !Path::new(migrations_dir).exists() {
            return Ok(false);
        }

        let entries = fs::read_dir(migrations_dir)
            .map_err(|e| format!("Failed to read migrations directory: {}", e))?;

        for entry in entries {
            let entry = entry.map_err(|e| format!("Failed to read directory entry: {}", e))?;
            let dir_name = entry.file_name().to_string_lossy().to_string();

            // Migration directory format: YYYYMMDDHHMMSS_name (Diesel format)
            if let Some(migration_name_part) = dir_name.splitn(2, '_').nth(1) {
                if migration_name_part == name {
                    return Ok(true);
                }
            }
        }

        Ok(false)
    }

    /// Generate timestamp for migration (Diesel format: YYYYMMDDHHMMSS, 14 digits, no separators)
    fn generate_timestamp() -> String {
        let now: DateTime<Utc> = Utc::now();
        now.format("%Y%m%d%H%M%S").to_string()
    }

    /// Generate the up.sql content
    pub fn generate_up_sql(
        changes: &[SchemaChange],
        table_definitions: &[TableDefinition],
    ) -> Result<String, String> {
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

        // Group indexes by table so we can emit them right after each table (and its hypertable)
        let mut indexes_by_table: std::collections::HashMap<String, Vec<&SchemaChange>> =
            std::collections::HashMap::new();
        for change in &new_indexes {
            indexes_by_table
                .entry(change.table_name.clone())
                .or_default()
                .push(change);
        }
        let tables_created_this_migration: std::collections::HashSet<String> =
            new_tables.iter().map(|c| c.table_name.clone()).collect();

        let mut first_statement = true;

        // For each table: CREATE TABLE → create_hypertable (if hypertable) → CREATE INDEX for this table
        for change in new_tables {
            if !first_statement {
                sql.push_str("--> statement-breakpoint\n");
            }

            let table_def = table_definitions
                .iter()
                .find(|def| def.name == change.table_name)
                .ok_or_else(|| {
                    format!(
                        "Table definition not found for table: {}",
                        change.table_name
                    )
                })?;

            let create_table_sql = Self::generate_create_table_sql(&change.table_name, table_def)?;
            sql.push_str(&create_table_sql);
            sql.push_str("\n");

            // Convert to hypertable before creating indexes (required for TimescaleDB)
            if table_def.is_hypertable {
                sql.push_str("--> statement-breakpoint\n");
                sql.push_str("-- Convert to hypertable before creating indexes\n");
                let hypertable_sql = Self::generate_hypertable_sql(&change.table_name)?;
                sql.push_str(&hypertable_sql);
                sql.push_str("\n");
            }

            // Create indexes for this table (after table and hypertable, so indexes are on the hypertable)
            if let Some(index_changes) = indexes_by_table.get(&change.table_name) {
                for index_change in index_changes {
                    if let (Some(index_name), Some(column_names)) =
                        (&index_change.field_name, &index_change.field_definition)
                    {
                        sql.push_str("--> statement-breakpoint\n");
                        let (columns, index_type) = if column_names.contains("|") {
                            let parts: Vec<&str> = column_names.split("|").collect();
                            (parts[0], Some(parts[1]))
                        } else {
                            (column_names.as_str(), None)
                        };
                        let index_sql = Self::generate_create_index_sql(
                            &change.table_name,
                            index_name,
                            columns,
                            index_type,
                        )?;
                        sql.push_str(&index_sql);
                        sql.push_str("\n");
                    }
                }
            }

            first_statement = false;
        }

        // Generate ALTER TABLE ADD COLUMN statements
        for change in new_fields {
            if let (Some(field_name), Some(field_definition)) =
                (&change.field_name, &change.field_definition)
            {
                if !first_statement {
                    sql.push_str("--> statement-breakpoint\n");
                }
                let alter_sql = Self::generate_add_column_sql(
                    &change.table_name,
                    field_name,
                    field_definition,
                    true,
                )?;
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

        // Generate CREATE INDEX for tables that weren't in new_tables (e.g. indexes added in same migration for existing tables)
        for change in &new_indexes {
            if tables_created_this_migration.contains(&change.table_name) {
                continue; // already emitted above with the table
            }
            if let (Some(index_name), Some(column_names)) =
                (&change.field_name, &change.field_definition)
            {
                if !first_statement {
                    sql.push_str("--> statement-breakpoint\n");
                }
                let (columns, index_type) = if column_names.contains("|") {
                    let parts: Vec<&str> = column_names.split("|").collect();
                    (parts[0], Some(parts[1]))
                } else {
                    (column_names.as_str(), None)
                };
                let index_sql = Self::generate_create_index_sql(
                    &change.table_name,
                    index_name,
                    columns,
                    index_type,
                )?;
                sql.push_str(&index_sql);
                sql.push_str("\n");
                first_statement = false;
            }
        }

        // Generate ALTER TABLE ADD CONSTRAINT statements for foreign keys
        for change in new_foreign_keys {
            if let (Some(constraint_name), Some(field_definition)) =
                (&change.field_name, &change.field_definition)
            {
                if !first_statement {
                    sql.push_str("--> statement-breakpoint\n");
                }
                let foreign_key_sql = Self::generate_add_foreign_key_sql(
                    &change.table_name,
                    constraint_name,
                    field_definition,
                )?;
                sql.push_str(&foreign_key_sql);
                sql.push_str("\n");
                first_statement = false;
            }
        }

        Ok(sql)
    }

    /// Generate the down.sql content. When the table is a hypertable, uses CASCADE so TimescaleDB
    /// cleans up chunks and catalog (avoids "cache lookup failed for relation _hyper_*_chunk").
    pub(crate) fn generate_down_sql(
        changes: &[SchemaChange],
        table_definitions: &[TableDefinition],
    ) -> Result<String, String> {
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
                    let is_hypertable = table_definitions
                        .iter()
                        .find(|def| def.name == change.table_name)
                        .map(|def| def.is_hypertable)
                        .unwrap_or(false);
                    if is_hypertable {
                        sql.push_str(
                            "-- Drop hypertable in one go so TimescaleDB cleans up chunks and catalog.\n",
                        );
                        sql.push_str(&format!(
                            "DROP TABLE IF EXISTS \"{}\" CASCADE;\n",
                            change.table_name
                        ));
                    } else {
                        sql.push_str(&format!(
                            "DROP TABLE IF EXISTS \"{}\";\n",
                            change.table_name
                        ));
                    }
                }
                SchemaChangeType::NewField => {
                    if let Some(field_name) = &change.field_name {
                        sql.push_str(&format!(
                            "ALTER TABLE \"{}\" DROP COLUMN IF EXISTS \"{}\";\n",
                            change.table_name, field_name
                        ));
                    }
                }
                SchemaChangeType::RemovedField => {
                    if let (Some(field_name), Some(field_definition)) =
                        (&change.field_name, &change.field_definition)
                    {
                        // For removed fields, the down migration should add the field back
                        let postgres_type = Self::diesel_to_postgres_type(field_definition, true)
                            .unwrap_or_else(|_| "TEXT".to_string());
                        sql.push_str(&format!(
                            "ALTER TABLE \"{}\" ADD COLUMN \"{}\" {};\n",
                            change.table_name, field_name, postgres_type
                        ));
                    }
                }
                SchemaChangeType::NewIndex => {
                    if let Some(field_name) = &change.field_name {
                        let index_name = format!("idx_{}_{}", change.table_name, field_name);
                        sql.push_str(&format!(
                            "DROP INDEX IF EXISTS \"{}\";
",
                            index_name
                        ));
                    }
                }
                SchemaChangeType::NewForeignKey => {
                    if let Some(constraint_name) = &change.field_name {
                        sql.push_str(&format!(
                            "ALTER TABLE \"{}\" DROP CONSTRAINT IF EXISTS \"{}\";
",
                            change.table_name, constraint_name
                        ));
                    }
                }
            }
        }

        Ok(sql)
    }

    /// Generate CREATE TABLE SQL
    fn generate_create_table_sql(
        table_name: &str,
        table_def: &TableDefinition,
    ) -> Result<String, String> {
        let mut sql = String::new();

        sql.push_str(&format!("CREATE TABLE \"{}\" (\n", table_name));

        let mut parsed_fields = Vec::new();
        for field in &table_def.fields {
            match field.parse_for_context(true) {
                // Use migration context to preserve VARCHAR
                Ok(parsed) => parsed_fields.push(parsed),
                Err(e) => return Err(format!("Error parsing field {}: {}", field.name, e)),
            }
        }

        // Collect primary key fields
        let primary_key_fields: Vec<&str> = table_def
            .fields
            .iter()
            .filter(|field| field.is_primary_key)
            .map(|field| field.name.as_str())
            .collect();

        // Add fields
        for (i, field) in parsed_fields.iter().enumerate() {
            let postgres_type =
                Self::diesel_to_postgres_type(&field.field_type, field.migration_nullable)?;
            let default_clause = if let Some(default) = &field.default_value {
                // Check if this is a TEXT field that needs quotes around string defaults
                if postgres_type.contains("TEXT")
                    && !default.starts_with("'")
                    && !default
                        .chars()
                        .all(|c| c.is_ascii_digit() || c == '.' || c == '-')
                {
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
    fn generate_add_column_sql(
        table_name: &str,
        field_name: &str,
        field_type: &str,
        migration_nullable: bool,
    ) -> Result<String, String> {
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
    fn generate_create_index_sql(
        table_name: &str,
        index_name: &str,
        column_names: &str,
        index_type: Option<&str>,
    ) -> Result<String, String> {
        // Sanitize index_type: strip any stray " } or "}, from malformed extraction
        let clean_index_type = index_type.map(|s| {
            s.trim()
                .trim_end_matches(|c: char| c == '"' || c == ' ' || c == '}' || c == ',')
                .to_string()
        });
        // The index_name already contains the full name from the macro
        let using_clause = if let Some(ref idx_type) = clean_index_type {
            if idx_type.is_empty() {
                String::new()
            } else {
                format!(" USING {}", idx_type)
            }
        } else {
            String::new()
        };

        let quoted_columns = column_names
            .split(',')
            .map(|c| {
                let t = c.trim();
                let s = t.trim_matches('"');
                format!("\"{}\"", s)
            })
            .collect::<Vec<_>>()
            .join(", ");

        Ok(format!(
            "CREATE INDEX \"{}\" ON \"{}\"{}({});",
            index_name, table_name, using_clause, quoted_columns
        ))
    }

    /// Generate ADD CONSTRAINT FOREIGN KEY SQL
    pub fn generate_add_foreign_key_sql(
        table_name: &str,
        constraint_name: &str,
        field_definition: &str,
    ) -> Result<String, String> {
        // Parse field_definition: "column_name|referenced_table|referenced_column"
        let parts: Vec<&str> = field_definition.split('|').collect();
        if parts.len() != 3 {
            return Err(format!(
                "Invalid foreign key definition: {}",
                field_definition
            ));
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
    fn diesel_to_postgres_type(
        diesel_type: &str,
        migration_nullable: bool,
    ) -> Result<String, String> {
        let mut postgres_type = diesel_type;

        // Handle Nullable wrapper - extract the inner type
        if diesel_type.starts_with("Nullable<") && diesel_type.ends_with(">") {
            postgres_type = &diesel_type[9..diesel_type.len() - 1];
        }

        // Handle Array wrapper
        let mut is_array = false;
        if postgres_type.starts_with("Array<") && postgres_type.ends_with(">") {
            is_array = true;
            postgres_type = &postgres_type[6..postgres_type.len() - 1];
        }

        // Convert core type
        let core_type = if postgres_type.contains('<') && postgres_type.ends_with('>') {
            // Handle generic types with parameters like "Varchar<300>", "Decimal<10,2>", etc.
            if let Some(angle_pos) = postgres_type.find('<') {
                let base_type = &postgres_type[..angle_pos];
                let params = &postgres_type[angle_pos + 1..postgres_type.len() - 1];

                match base_type {
                    "Varchar" => format!("VARCHAR({})", params),
                    "Char" => format!("CHAR({})", params),
                    "Decimal" | "Numeric" => format!("DECIMAL({})", params),
                    _ => {
                        return Err(format!(
                            "Unsupported generic Diesel type: {}",
                            postgres_type
                        ))
                    }
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
    use super::MigrationGenerator;
    use crate::builders::generator::field_definition::{FieldDefinition, TableDefinition};
    use crate::builders::generator::schema_generator::{SchemaChange, SchemaChangeType};

    #[test]
    fn test_down_sql_hypertable_uses_cascade() {
        let table_name = "signed_in_activities";
        let changes = vec![SchemaChange {
            table_name: table_name.to_string(),
            change_type: SchemaChangeType::NewTable,
            field_name: None,
            field_definition: None,
        }];
        let table_definitions = vec![TableDefinition {
            name: table_name.to_string(),
            fields: vec![FieldDefinition {
                name: "id".to_string(),
                diesel_type: "Text".to_string(),
                rust_type: "String".to_string(),
                is_primary_key: true,
                is_indexed: false,
                is_nullable: false,
                is_array: false,
                migration_nullable: false,
                default_value: None,
                migration_type: None,
            }],
            indexes: vec![],
            foreign_keys: vec![],
            is_hypertable: true,
        }];
        let down_sql = MigrationGenerator::generate_down_sql(&changes, &table_definitions).unwrap();
        assert!(
            down_sql.contains("CASCADE"),
            "down SQL for hypertable should use CASCADE; got: {}",
            down_sql
        );
        assert!(
            down_sql.contains("TimescaleDB"),
            "down SQL for hypertable should mention TimescaleDB in comment; got: {}",
            down_sql
        );
        assert!(
            down_sql.contains(&format!("DROP TABLE IF EXISTS \"{}\" CASCADE", table_name)),
            "down SQL should drop the hypertable with CASCADE; got: {}",
            down_sql
        );
    }

    #[test]
    fn test_down_sql_regular_table_no_cascade() {
        let table_name = "contacts";
        let changes = vec![SchemaChange {
            table_name: table_name.to_string(),
            change_type: SchemaChangeType::NewTable,
            field_name: None,
            field_definition: None,
        }];
        let table_definitions = vec![TableDefinition {
            name: table_name.to_string(),
            fields: vec![],
            indexes: vec![],
            foreign_keys: vec![],
            is_hypertable: false,
        }];
        let down_sql = MigrationGenerator::generate_down_sql(&changes, &table_definitions).unwrap();
        assert!(
            !down_sql.contains("CASCADE"),
            "down SQL for non-hypertable should not use CASCADE; got: {}",
            down_sql
        );
        assert!(
            down_sql.contains(&format!("DROP TABLE IF EXISTS \"{}\";", table_name)),
            "down SQL should drop the table without CASCADE; got: {}",
            down_sql
        );
    }

    #[test]
    fn test_index_sql_quotes_single_column_btree() {
        let sql = MigrationGenerator::generate_create_index_sql(
            "episodes",
            "idx_order",
            "order",
            Some("btree"),
        )
        .unwrap();
        assert_eq!(
            sql,
            "CREATE INDEX \"idx_order\" ON \"episodes\" USING btree(\"order\");"
        );
    }

    #[test]
    fn test_index_sql_quotes_multiple_columns() {
        let sql = MigrationGenerator::generate_create_index_sql(
            "episodes",
            "idx_order_created",
            "order, created_at",
            None,
        )
        .unwrap();
        assert_eq!(
            sql,
            "CREATE INDEX \"idx_order_created\" ON \"episodes\"(\"order\", \"created_at\");"
        );
    }

    #[test]
    fn test_postgres_channels_index_sql_no_invalid_chars() {
        // Regression: single-line index format used to produce btree" }("column") due to
        // type extraction capturing closing brace. All indexes must be valid SQL.
        for (col, index_name) in [
            ("channel_name", "idx_postgres_channels_channel_name"),
            (
                "channel_timestamp",
                "idx_postgres_channels_channel_timestamp",
            ),
            ("function", "idx_postgres_channels_function"),
        ] {
            let sql = MigrationGenerator::generate_create_index_sql(
                "postgres_channels",
                index_name,
                col,
                Some("btree"),
            )
            .unwrap();
            assert!(
                !sql.contains("\" }"),
                "Generated SQL must not contain invalid '\" }}' - got: {}",
                sql
            );
            assert_eq!(
                sql,
                format!(
                    "CREATE INDEX \"{}\" ON \"postgres_channels\" USING btree(\"{}\");",
                    index_name, col
                )
            );
        }
    }

    #[test]
    fn test_organizations_style_index_sql_no_invalid_chars() {
        // Regression: organizations table single-line indexes (type: "btree" },) produced
        // USING btree" }("column") - index_type sanitization must strip stray " }
        let sql = MigrationGenerator::generate_create_index_sql(
            "organizations",
            "idx_organizations_name",
            "name",
            Some(r#"btree" }"#),
        )
        .unwrap();
        assert!(
            !sql.contains("\" }"),
            "Generated SQL must not contain invalid '\" }}' - got: {}",
            sql
        );
        assert_eq!(
            sql,
            "CREATE INDEX \"idx_organizations_name\" ON \"organizations\" USING btree(\"name\");"
        );
    }

    /// Ensures up.sql for a new hypertable with indexes has order: CREATE TABLE → create_hypertable → CREATE INDEX.
    #[test]
    fn test_up_sql_hypertable_indexes_after_create_hypertable() {
        let table_name = "events";
        let changes = vec![
            SchemaChange {
                table_name: table_name.to_string(),
                change_type: SchemaChangeType::NewTable,
                field_name: None,
                field_definition: None,
            },
            SchemaChange {
                table_name: table_name.to_string(),
                change_type: SchemaChangeType::NewIndex,
                field_name: Some("idx_events_timestamp".to_string()),
                field_definition: Some("timestamp".to_string()),
            },
        ];
        let table_definitions = vec![TableDefinition {
            name: table_name.to_string(),
            fields: vec![
                FieldDefinition {
                    name: "id".to_string(),
                    diesel_type: "Text".to_string(),
                    rust_type: "String".to_string(),
                    is_primary_key: true,
                    is_indexed: false,
                    is_nullable: false,
                    is_array: false,
                    migration_nullable: false,
                    default_value: None,
                    migration_type: None,
                },
                FieldDefinition {
                    name: "timestamp".to_string(),
                    diesel_type: "Timestamptz".to_string(),
                    rust_type: "String".to_string(),
                    is_primary_key: false,
                    is_indexed: false,
                    is_nullable: false,
                    is_array: false,
                    migration_nullable: false,
                    default_value: None,
                    migration_type: None,
                },
            ],
            indexes: vec![],
            foreign_keys: vec![],
            is_hypertable: true,
        }];
        let up_sql = MigrationGenerator::generate_up_sql(&changes, &table_definitions).unwrap();

        assert!(
            up_sql.contains("CREATE TABLE \"events\""),
            "up SQL should create the table; got: {}",
            up_sql
        );
        assert!(
            up_sql.contains("create_hypertable"),
            "up SQL should create hypertable; got: {}",
            up_sql
        );
        assert!(
            up_sql.contains("CREATE INDEX \"idx_events_timestamp\""),
            "up SQL should create index on events; got: {}",
            up_sql
        );
        assert!(
            up_sql.contains("Convert to hypertable before creating indexes"),
            "up SQL should include comment before create_hypertable; got: {}",
            up_sql
        );

        let create_table_pos = up_sql.find("CREATE TABLE \"events\"").unwrap();
        let hypertable_pos = up_sql.find("create_hypertable").unwrap();
        let index_pos = up_sql
            .find("CREATE INDEX \"idx_events_timestamp\"")
            .unwrap();
        assert!(
            create_table_pos < hypertable_pos,
            "CREATE TABLE must appear before create_hypertable"
        );
        assert!(
            hypertable_pos < index_pos,
            "create_hypertable must appear before CREATE INDEX for the hypertable"
        );
    }
}
