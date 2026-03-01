use crate::builders::generator::field_definition::TableDefinition;
use crate::constants::paths;
use crate::database::schema::reserved_keywords;
use crate::database::schema::verify::{
    field_exists_in_table, field_type_in_table, get_table_fields, FieldTypeInfo,
};
use regex::Regex;
use std::fs;

pub struct SchemaGenerator;

#[derive(Debug, Clone)]
pub struct SchemaChange {
    pub table_name: String,
    pub change_type: SchemaChangeType,
    pub field_name: Option<String>,
    pub field_definition: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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
        // Validate that hypertables don't have foreign key constraints
        if table_def.is_hypertable && !table_def.foreign_keys.is_empty() {
            eprintln!(
                "ERROR: Table '{}' is marked as a hypertable but has foreign key constraints.",
                table_def.name
            );
            eprintln!("TimescaleDB hypertables don't support foreign key constraints.");
            eprintln!("Please remove the foreign key constraints from this table definition.");
            std::process::exit(1);
        }

        let mut changes = Vec::new();

        // Check if table exists
        let existing_fields = get_table_fields(&table_def.name);

        if existing_fields.is_none() {
            // Table doesn't exist, need to create it
            changes.push(SchemaChange {
                table_name: table_def.name.clone(),
                change_type: SchemaChangeType::NewTable,
                field_name: None,
                field_definition: None,
            });
        } else {
            // Table exists, check for new fields
            let existing_fields = existing_fields.unwrap();

            // Check if this table uses system fields and detect new system fields
            let uses_system_fields = Self::table_uses_system_fields(&table_def.name);
            if uses_system_fields && Self::should_force_system_fields_update() {
                // Only add system fields that don't already exist in the table
                for field in &table_def.fields {
                    if Self::is_system_field(&field.name)
                        && !existing_fields.contains(&field.name)
                        && !Self::field_added_in_migration(&table_def.name, &field.name)
                    {
                        changes.push(SchemaChange {
                            table_name: table_def.name.clone(),
                            change_type: SchemaChangeType::NewField,
                            field_name: Some(field.name.clone()),
                            field_definition: Some(
                                field
                                    .migration_type
                                    .as_ref()
                                    .unwrap_or(&field.diesel_type)
                                    .clone(),
                            ),
                        });
                    }
                }
            }

            for field in &table_def.fields {
                if !existing_fields.contains(&field.name) {
                    // Skip system fields if they were already processed above
                    if uses_system_fields
                        && Self::should_force_system_fields_update()
                        && Self::is_system_field(&field.name)
                    {
                        continue;
                    }

                    if Self::field_added_in_migration(&table_def.name, &field.name) {
                        continue;
                    }

                    changes.push(SchemaChange {
                        table_name: table_def.name.clone(),
                        change_type: SchemaChangeType::NewField,
                        field_name: Some(field.name.clone()),
                        field_definition: Some(
                            field
                                .migration_type
                                .as_ref()
                                .unwrap_or(&field.diesel_type)
                                .clone(),
                        ),
                    });
                }
            }

            // Check for removed fields (fields that exist in DB but not in table definition)
            let current_field_names: Vec<String> =
                table_def.fields.iter().map(|f| f.name.clone()).collect();

            for existing_field in &existing_fields {
                if !current_field_names.contains(existing_field) {
                    // Get field type information for the removed field
                    let field_definition = if let Some(field_type_info) =
                        field_type_in_table(&table_def.name, existing_field)
                    {
                        Some(Self::field_type_info_to_migration_definition(
                            &field_type_info,
                        ))
                    } else {
                        None
                    };

                    changes.push(SchemaChange {
                        table_name: table_def.name.clone(),
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
        indexes: &[(
            String,
            Vec<String>,
            bool,
            Option<String>,
            Option<crate::builders::generator::diesel_schema_definition::WhereExpr>,
        )],
        foreign_keys: &[crate::builders::generator::diesel_schema_definition::ForeignKeyDefinition],
    ) -> Result<Vec<SchemaChange>, String> {
        // Validate that hypertables don't have foreign key constraints
        if table_def.is_hypertable
            && (!foreign_keys.is_empty() || !table_def.foreign_keys.is_empty())
        {
            eprintln!(
                "ERROR: Table '{}' is marked as a hypertable but has foreign key constraints.",
                table_def.name
            );
            eprintln!("TimescaleDB hypertables don't support foreign key constraints.");
            eprintln!("Please remove the foreign key constraints from this table definition.");
            std::process::exit(1);
        }

        let mut changes = Self::analyze_changes(table_def)?;

        // Add index changes - only if they don't already exist, or error if definition changed
        for (index_name, columns, is_unique, index_type, where_clause) in indexes {
            if let Some(existing_sql) =
                Self::get_existing_index_sql_from_migrations(&table_def.name, index_name)
            {
                // Index already exists: compare definition; if different, error
                let expected_sql = crate::builders::generator::migration_generator::MigrationGenerator::generate_create_index_sql_full(
                    &table_def.name,
                    index_name,
                    &columns.join(","),
                    index_type.as_deref(),
                    where_clause.as_ref(),
                    *is_unique,
                )?;
                if !Self::normalize_index_sql_for_compare(&existing_sql)
                    .eq(&Self::normalize_index_sql_for_compare(&expected_sql))
                {
                    return Err(format!(
                        "Index '{}' was already created with a different definition. \
                         Already created indexes cannot be modified. \
                         Update your table definition to match the existing index, or use a new index name. \
                         Existing index SQL:\n  {}",
                        index_name, existing_sql
                    ));
                }
                continue;
            }

            if !Self::index_exists_in_schema(&table_def.name, index_name) {
                let type_str = index_type.as_deref().unwrap_or("btree");
                let mut field_def = format!("{}|{}|{}", columns.join(","), type_str, is_unique);
                if let Some(ref w) = where_clause {
                    if let Ok(json) = serde_json::to_string(w) {
                        field_def.push_str("|");
                        field_def.push_str(&json);
                    }
                }

                changes.push(SchemaChange {
                    table_name: table_def.name.clone(),
                    change_type: SchemaChangeType::NewIndex,
                    field_name: Some(index_name.clone()),
                    field_definition: Some(field_def),
                });
            }
        }

        // Add foreign key changes - only if they don't already exist
        for foreign_key in foreign_keys {
            // Constraint name format: fk_{table_name}_{column_name}
            let constraint_name = format!("fk_{}_{}", table_def.name, foreign_key.column)
                .replace(",", "")
                .replace("\"", "");
            // Old format (backward compat): {table}_{column}_{ref_table}_{ref_column}_fk
            let old_constraint_name = format!(
                "{}_{}_{}_{}_fk",
                table_def.name,
                foreign_key.column,
                foreign_key.references_table,
                foreign_key.references_column
            )
            .replace(",", "")
            .replace("\"", "");

            // Check if foreign key already exists (new or old format)
            let exists = Self::foreign_key_exists_in_schema(&table_def.name, &constraint_name)
                || Self::foreign_key_exists_in_schema(&table_def.name, &old_constraint_name);
            if !exists {
                // Clean up any potential quotes or commas in the referenced table and column
                let clean_column = foreign_key.column.replace(",", "").replace("\"", "");
                let clean_references_table = foreign_key
                    .references_table
                    .replace(",", "")
                    .replace("\"", "");
                let clean_references_column = foreign_key
                    .references_column
                    .replace(",", "")
                    .replace("\"", "");

                let field_def = format!(
                    "{}|{}|{}",
                    clean_column, clean_references_table, clean_references_column
                );

                changes.push(SchemaChange {
                    table_name: table_def.name.clone(),
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
        let schema_file_path = paths::database::SCHEMA_FILE.as_str();

        // Read existing schema content
        let existing_content = match fs::read_to_string(schema_file_path) {
            Ok(content) => content,
            Err(e) => return Err(format!("Failed to read schema.rs: {}", e)),
        };

        // Check if table already exists
        if Self::table_exists_in_schema(&existing_content, &table_def.name) {
            // Table exists, we need to handle field changes
            Self::update_existing_table_in_schema(&existing_content, table_def, schema_file_path)
        } else {
            // Table doesn't exist, add new table
            Self::add_new_table_to_schema(&existing_content, table_def, schema_file_path)
        }
    }

    /// Update an existing table in schema.rs by adding new fields and removing deleted fields
    fn update_existing_table_in_schema(
        existing_content: &str,
        table_def: &TableDefinition,
        file_path: &str,
    ) -> Result<(), String> {
        // Get current fields in schema.rs
        let existing_fields = get_table_fields(&table_def.name).unwrap_or_default();

        // Get current field names from table definition
        let current_field_names: Vec<String> =
            table_def.fields.iter().map(|f| f.name.clone()).collect();

        // Find fields to remove (exist in schema but not in table definition)
        let fields_to_remove: Vec<String> = existing_fields
            .iter()
            .filter(|field| !current_field_names.contains(field))
            .cloned()
            .collect();

        // Check if there are new fields to add
        let has_new_fields = table_def
            .fields
            .iter()
            .any(|field| !field_exists_in_table(&table_def.name, &field.name));

        // If there are both removals and additions (like field rename), or any changes at all,
        // use rebuild_entire_table_in_schema to ensure proper field ordering
        if !fields_to_remove.is_empty() || has_new_fields {
            Self::rebuild_entire_table_in_schema(existing_content, table_def, file_path)
        } else {
            // No changes needed
            Ok(())
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
        let migrations_dir = paths::database::MIGRATIONS_DIR.as_str();

        if let Ok(entries) = std::fs::read_dir(migrations_dir) {
            for entry in entries.flatten() {
                if let Ok(up_sql) =
                    std::fs::read_to_string(entry.path().join(paths::database::UP_SQL_FILE))
                {
                    // Check if this index was already created in a previous migration
                    // Pattern matches: CREATE [UNIQUE] INDEX [IF NOT EXISTS] "index_name" ON "table_name"
                    let index_pattern = format!(
                        r#"CREATE\s+(?:UNIQUE\s+)?INDEX\s+(?:IF\s+NOT\s+EXISTS\s+)?["']?{}["']?\s+ON\s+["']?{}["']?"#,
                        regex::escape(index_name),
                        regex::escape(table_name)
                    );
                    if let Ok(regex) = Regex::new(&index_pattern) {
                        if regex.is_match(&up_sql) {
                            return true;
                        }
                    }

                    // Also check for alternative index naming formats
                    // Extract field name from index_name to check for both formats:
                    // Format 1: idx_{table_name}_{field} (new format)
                    // Format 2: {table_name}_{field}_idx (old format)
                    if let Some(field_name) =
                        Self::extract_field_from_index_name(table_name, index_name)
                    {
                        // Check for old format: {table_name}_{field}_idx
                        let old_format_name = format!("{}_{}_{}", table_name, field_name, "idx");
                        let old_pattern = format!(
                            r#"CREATE\s+(?:UNIQUE\s+)?INDEX\s+(?:IF\s+NOT\s+EXISTS\s+)?["']?{}["']?\s+ON\s+["']?{}["']?"#,
                            regex::escape(&old_format_name),
                            regex::escape(table_name)
                        );
                        if let Ok(old_regex) = Regex::new(&old_pattern) {
                            if old_regex.is_match(&up_sql) {
                                return true;
                            }
                        }

                        // Check for new format: idx_{table_name}_{field}
                        let new_format_name = format!("idx_{}_{}", table_name, field_name);
                        let new_pattern = format!(
                            r#"CREATE\s+(?:UNIQUE\s+)?INDEX\s+(?:IF\s+NOT\s+EXISTS\s+)?["']?{}["']?\s+ON\s+["']?{}["']?"#,
                            regex::escape(&new_format_name),
                            regex::escape(table_name)
                        );
                        if let Ok(new_regex) = Regex::new(&new_pattern) {
                            if new_regex.is_match(&up_sql) {
                                return true;
                            }
                        }
                    }
                }
            }
        }
        false
    }

    /// Return the full CREATE INDEX SQL for an existing index from migrations, if found.
    fn get_existing_index_sql_from_migrations(
        table_name: &str,
        index_name: &str,
    ) -> Option<String> {
        let migrations_dir = paths::database::MIGRATIONS_DIR.as_str();
        let entries = std::fs::read_dir(migrations_dir).ok()?;
        for entry in entries.flatten() {
            let up_sql =
                std::fs::read_to_string(entry.path().join(paths::database::UP_SQL_FILE)).ok()?;
            // Match full line: CREATE [UNIQUE] INDEX "index_name" ON "table_name" ... ;
            let on_table = format!("ON \"{}\"", table_name);
            if !up_sql.contains(&on_table) {
                continue;
            }
            for line in up_sql.lines() {
                let line = line.trim();
                if line.starts_with("CREATE ")
                    && (line.contains("UNIQUE") || line.contains("INDEX"))
                    && line.contains(&format!("\"{}\"", index_name))
                    && line.contains(&on_table)
                {
                    // Strip Diesel/statement-breakpoint comment (e.g. ";--> statement-breakpoint") so comparison matches generator output
                    let sql = if let Some(comment_start) = line.find("-->") {
                        line[..comment_start].trim_end().trim_end_matches(';')
                    } else {
                        line.trim_end_matches(';')
                    };
                    return Some(sql.to_string());
                }
            }
        }
        None
    }

    /// Normalize CREATE INDEX SQL for comparison (lowercase, collapse whitespace, no trailing semicolon).
    /// Column names in the index definition are normalized to unquoted form so that
    /// btree(tombstone) and btree("tombstone") from migrations are treated as equal.
    fn normalize_index_sql_for_compare(sql: &str) -> String {
        let s = sql.trim().trim_end_matches(';');
        let s = s.to_lowercase();
        let s = s.split_whitespace().collect::<Vec<_>>().join(" ");
        // Normalize column list: ("col") or (col) -> (col) so existing migrations match generator output
        if let Ok(re) = regex::Regex::new(r"using btree\(([^)]+)\)") {
            let s = re.replace_all(&s, |caps: &regex::Captures<'_>| {
                let inner = &caps[1];
                let unquoted = inner.replace('"', "");
                format!("using btree({})", unquoted)
            });
            s.into_owned()
        } else {
            s
        }
    }

    /// For reserved keywords, use a different Rust identifier and #[sql_name] to avoid clashes.
    fn format_schema_column(field_name: &str, field_type: &str) -> String {
        if reserved_keywords::is_reserved(field_name) {
            let rust_id = reserved_keywords::rust_identifier(field_name);
            format!(
                "        #[sql_name = \"{}\"]\n        {} -> {},",
                field_name, rust_id, field_type
            )
        } else {
            format!("        {} -> {},", field_name, field_type)
        }
    }

    /// Extract field name from index name for both naming formats
    fn extract_field_from_index_name(table_name: &str, index_name: &str) -> Option<String> {
        // Handle new format: idx_{table_name}_{field}
        let new_prefix = format!("idx_{}_", table_name);
        if index_name.starts_with(&new_prefix) {
            return Some(index_name[new_prefix.len()..].to_string());
        }

        // Handle old format: {table_name}_{field}_idx
        let old_suffix = "_idx";
        if index_name.starts_with(table_name) && index_name.ends_with(old_suffix) {
            let start = table_name.len() + 1; // +1 for the underscore
            let end = index_name.len() - old_suffix.len();
            if start < end {
                return Some(index_name[start..end].to_string());
            }
        }

        None
    }

    /// Check if a foreign key already exists for a table
    fn foreign_key_exists_in_schema(table_name: &str, constraint_name: &str) -> bool {
        // Read the migrations directory to check if this foreign key was already created
        let migrations_dir = paths::database::MIGRATIONS_DIR.as_str();

        if let Ok(entries) = std::fs::read_dir(migrations_dir) {
            for entry in entries.flatten() {
                if let Ok(up_sql) =
                    std::fs::read_to_string(entry.path().join(paths::database::UP_SQL_FILE))
                {
                    // Check if this foreign key was already created in a previous migration
                    // Pattern matches: ALTER TABLE "table_name" ADD CONSTRAINT "constraint_name"
                    let fk_pattern = format!(
                        r#"ALTER\s+TABLE\s+["']?{}["']?\s+ADD\s+CONSTRAINT\s+["']?{}["']?"#,
                        regex::escape(table_name),
                        regex::escape(constraint_name)
                    );
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
    fn add_new_table_to_schema(
        existing_content: &str,
        table_def: &TableDefinition,
        file_path: &str,
    ) -> Result<(), String> {
        let mut parsed_fields = Vec::new();

        // Parse all fields
        for field in &table_def.fields {
            match field.parse() {
                Ok(parsed) => parsed_fields.push(parsed),
                Err(e) => return Err(format!("Error parsing field {}: {}", field.name, e)),
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

        Ok(())
    }

    /// Check if a table uses system fields by reading its definition file
    fn table_uses_system_fields(table_name: &str) -> bool {
        let table_file_path = format!(
            "{}/{}.rs",
            paths::database::SCHEMA_TABLES_DIR.as_str(),
            table_name
        );
        if let Ok(content) = std::fs::read_to_string(&table_file_path) {
            content.contains("system_fields!()")
        } else {
            false
        }
    }

    /// Check if we should force update system fields (when CREATE_SCHEMA is enabled)
    fn should_force_system_fields_update() -> bool {
        std::env::var("CREATE_SCHEMA")
            .unwrap_or_default()
            .to_lowercase()
            == "true"
    }

    /// Check if a field is a system field
    fn is_system_field(field_name: &str) -> bool {
        Self::get_system_field_names()
            .unwrap_or_default()
            .contains(&field_name.to_string())
    }

    /// Convert FieldTypeInfo to a field definition string for migration generation (preserves VARCHAR)
    fn field_type_info_to_migration_definition(field_type_info: &FieldTypeInfo) -> String {
        // Convert database types to Diesel types, preserving VARCHAR for migrations
        let diesel_type = match field_type_info.field_type.to_lowercase().as_str() {
            "bool" | "boolean" => "Bool",
            "text" => "Text",
            "char" => "Text",
            "integer" | "int4" => "Int4",
            "bigint" | "int8" => "Int8",
            "float" | "float4" => "Float4",
            "float8" | "double" => "Float8",
            "timestamp" | "timestamptz" => "Timestamp",
            "jsonb" => "Jsonb",
            "json" => "Json",
            "inet" => "Inet",
            "uuid" => "Uuid",
            "bytea" => "Bytea",
            "numeric" | "decimal" => "Numeric",
            "varchar" => "Varchar", // varchar without length
            // Handle VARCHAR with length constraints - preserve for migrations
            t if t.starts_with("varchar(") => {
                // Extract length and format as Varchar<length>
                if let Some(start) = t.find('(') {
                    if let Some(end) = t.find(')') {
                        let length_str = &t[start + 1..end];
                        return format!("Varchar<{}>", length_str);
                    }
                }
                "Varchar" // fallback if parsing fails
            }
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

    /// Order fields properly according to system fields macro and entity-specific fields
    fn order_fields_properly(
        existing_fields: &[crate::builders::generator::field_definition::ParsedField],
        new_fields: &[crate::builders::generator::field_definition::ParsedField],
    ) -> Result<Vec<crate::builders::generator::field_definition::ParsedField>, String> {
        let system_field_names = Self::get_system_field_names()?;
        let mut ordered_fields = Vec::new();

        // Combine all fields and deduplicate by name (prefer new_fields over existing_fields)
        let mut all_fields = Vec::new();
        let mut field_names_seen = std::collections::HashSet::new();

        // First add new_fields (they take precedence)
        for field in new_fields {
            if field_names_seen.insert(field.name.clone()) {
                all_fields.push(field.clone());
            }
        }

        // Then add existing_fields that aren't already present
        for field in existing_fields {
            if field_names_seen.insert(field.name.clone()) {
                all_fields.push(field.clone());
            }
        }

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
        let system_fields_path = paths::database::SYSTEM_FIELDS_FILE.as_str();
        let content = fs::read_to_string(system_fields_path)
            .map_err(|e| format!("Failed to read system_fields.rs: {}", e))?;

        // Find the macro definition
        let macro_start = content
            .find("() => {")
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
    pub fn rebuild_entire_table_in_schema(
        existing_content: &str,
        table_def: &TableDefinition,
        file_path: &str,
    ) -> Result<(), String> {
        // Find the table definition
        let table_pattern = format!(
            r"(?s)(table!\s*\{{\s*{}\s*\([^)]*\)\s*\{{)(.*?)(\}}\s*\}})",
            regex::escape(&table_def.name)
        );

        let table_regex = match Regex::new(&table_pattern) {
            Ok(re) => re,
            Err(e) => return Err(format!("Failed to create table regex: {}", e)),
        };

        if let Some(captures) = table_regex.captures(existing_content) {
            let table_start = captures.get(1).unwrap().as_str();
            let table_end = captures.get(3).unwrap().as_str();

            // Parse all fields from table definition (only use fields from the model, not existing schema)
            let mut parsed_fields = Vec::new();
            for field in &table_def.fields {
                match field.parse() {
                    Ok(parsed) => parsed_fields.push(parsed),
                    Err(e) => return Err(format!("Error parsing field {}: {}", field.name, e)),
                }
            }

            // Order fields properly (system fields first, then entity fields)
            // Pass empty existing_fields to ensure only table_def fields are used
            let ordered_fields = Self::order_fields_properly(&[], &parsed_fields)?;

            // Generate the new table body with properly ordered fields
            let mut new_table_body = String::new();
            for field in &ordered_fields {
                new_table_body
                    .push_str(&Self::format_schema_column(&field.name, &field.field_type));
                new_table_body.push('\n');
            }

            // Ensure proper formatting: add newline after opening brace if not present
            let formatted_table_start = if table_start.trim_end().ends_with("{") {
                format!("{}\n", table_start.trim_end())
            } else {
                table_start.to_string()
            };

            // Reconstruct the table with ordered fields
            let new_table_definition =
                format!("{}{}{}", formatted_table_start, new_table_body, table_end);

            // Replace the old table definition with the new one
            let new_content = table_regex.replace(existing_content, new_table_definition.as_str());

            // Write the updated schema
            if let Err(e) = fs::write(file_path, new_content.as_ref()) {
                return Err(format!("Failed to write schema.rs: {}", e));
            }

            Ok(())
        } else {
            Err(format!("Table '{}' not found in schema", table_def.name))
        }
    }

    /// Generate a complete table definition
    fn generate_table_definition(table_def: &TableDefinition) -> Result<String, String> {
        let mut definition = String::new();

        // Parse all fields
        let mut parsed_fields = Vec::new();
        for field in &table_def.fields {
            match field.parse() {
                Ok(parsed) => parsed_fields.push(parsed),
                Err(e) => return Err(format!("Error parsing field {}: {}", field.name, e)),
            }
        }

        // Order fields properly (system fields first, then entity fields)
        let ordered_fields = Self::order_fields_properly(&[], &parsed_fields)?;

        // Collect primary key fields
        let primary_key_fields: Vec<&str> = table_def
            .fields
            .iter()
            .filter(|field| field.is_primary_key)
            .map(|field| field.name.as_str())
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
            ordered_fields
                .iter()
                .find(|f| f.name == "id")
                .map(|f| f.name.as_str())
                .unwrap_or(&ordered_fields[0].name)
        };

        definition.push_str(&format!(
            "table! {{\n    {}({}) {{\n",
            table_def.name, primary_key
        ));

        // Add all fields in proper order
        for field in &ordered_fields {
            definition.push_str(&Self::format_schema_column(&field.name, &field.field_type));
            definition.push('\n');
        }

        definition.push_str("    }\n}");

        Ok(definition)
    }

    /// Check if a column was already added to a table in any migration file.
    /// Prevents duplicate ALTER TABLE ADD COLUMN when schema.rs is out of sync.
    fn field_added_in_migration(table_name: &str, field_name: &str) -> bool {
        let migrations_dir = paths::database::MIGRATIONS_DIR.as_str();

        if let Ok(entries) = std::fs::read_dir(migrations_dir) {
            for entry in entries.flatten() {
                if let Ok(up_sql) =
                    std::fs::read_to_string(entry.path().join(paths::database::UP_SQL_FILE))
                {
                    // Pattern: ALTER TABLE "table_name" ADD COLUMN "field_name"
                    let add_column_pattern = format!(
                        r#"ALTER\s+TABLE\s+["']?{}["']?\s+ADD\s+COLUMN\s+["']?{}["']?"#,
                        regex::escape(table_name),
                        regex::escape(field_name)
                    );
                    if let Ok(regex) = Regex::new(&add_column_pattern) {
                        if regex.is_match(&up_sql) {
                            return true;
                        }
                    }
                }
            }
        }
        false
    }
}
