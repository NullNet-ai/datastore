use crate::builders::generator::diesel_schema_definition::ForeignKeyDefinition;
use crate::builders::generator::field_definition::{FieldDefinition, ForeignKey, TableDefinition};
use crate::builders::generator::migration_generator::MigrationGenerator;
use crate::builders::generator::model_generator::ModelGenerator;
use crate::builders::generator::schema_generator::SchemaGenerator;
use crate::constants::paths::database::{
    HYPERTABLES_FILE, MODELS_DIR, MODELS_MOD_FILE, SCHEMA_FILE, SCHEMA_TABLES_DIR,
    SYSTEM_FIELDS_FILE,
};
use crate::utils::utils::to_singular;
use log::{debug, error, info};
use std::env;
use std::fs;
use std::path::Path;

pub struct GeneratorService;

impl GeneratorService {
    /// Main entry point for schema generation
    pub fn run() -> Result<(), String> {
        // Check if CREATE_SCHEMA flag is enabled
        if !Self::is_create_schema_enabled() {
            return Ok(());
        }

        // Find and process all table definition files
        let all_discovered_tables = Self::discover_table_definitions()?;

        info!(
            "Discovered {} table definition(s)",
            all_discovered_tables.len()
        );
        for table_def in &all_discovered_tables {
            debug!(
                "Found table: {} with {} field(s)",
                table_def.name,
                table_def.fields.len()
            );
        }

        if all_discovered_tables.is_empty() {
            info!("No table definitions found, skipping schema generation");
            return Ok(());
        }

        // Filter out invalid hypertables
        let mut table_definitions = Vec::new();
        for table_def in all_discovered_tables {
            table_definitions.push(table_def);
        }

        let mut all_changes = Vec::new();

        // Process each table definition
        for table_def in &table_definitions {
            // Extract indexes and foreign keys from table definition file
            let (indexes, foreign_keys) = {
                // Find the table definition file to read its content
                let table_files_dir = "src/schema/tables";

                // Try multiple possible file names for the table
                let possible_files = vec![
                    format!("{}/{}.rs", table_files_dir, table_def.name),
                    format!(
                        "{}/{}_catalog.rs",
                        table_files_dir,
                        table_def.name.trim_end_matches('s')
                    ),
                    format!("{}/{}_table.rs", table_files_dir, table_def.name),
                ];

                let mut extracted_indexes = Vec::new();
                let mut extracted_foreign_keys = Vec::new();

                for table_file_path in possible_files {
                    if let Ok(file_content) = fs::read_to_string(&table_file_path) {
                        extracted_indexes = Self::extract_indexes_from_macro(&file_content)
                            .unwrap_or_else(|_| Vec::new());
                        extracted_foreign_keys =
                            Self::extract_foreign_keys_from_macro(&file_content)
                                .unwrap_or_else(|_| Vec::new());
                        break;
                    }
                }

                (extracted_indexes, extracted_foreign_keys)
            };

            // Analyze schema changes with indexes and foreign keys
            let changes = SchemaGenerator::analyze_changes_with_indexes_and_foreign_keys(
                &table_def,
                &indexes,
                &foreign_keys,
            )?;

            if !changes.is_empty() {
                info!(
                    "Table '{}': Found {} schema change(s)",
                    table_def.name,
                    changes.len()
                );
                for change in &changes {
                    debug!(
                        "  - {:?} for table '{}'",
                        change.change_type, change.table_name
                    );
                }
                all_changes.extend(changes);

                // Generate model only when there are schema changes
                let model_content = ModelGenerator::generate_model(&table_def)?;
                Self::write_model_file(&table_def.name, &model_content)?;
                info!(
                    "Regenerated model for table '{}' due to schema changes",
                    table_def.name
                );

                // Update schema.rs
                SchemaGenerator::update_schema_file(table_def)?;
            } else {
                // Check if model file exists, if not, create it (for new tables)
                let singular_name = to_singular(&table_def.name);
                let model_file_path =
                    Path::new(MODELS_DIR).join(format!("{}_model.rs", singular_name));

                if !model_file_path.exists() {
                    let model_content = ModelGenerator::generate_model(&table_def)?;
                    Self::write_model_file(&table_def.name, &model_content)?;
                    info!("Created initial model for new table '{}'", table_def.name);
                } else {
                    // Check if field ordering has changed between schema and model
                    if Self::has_field_ordering_changed(&table_def)? {
                        // Rebuild the entire table in schema with proper field ordering
                        let schema_file_path = SCHEMA_FILE;
                        let existing_content = match fs::read_to_string(schema_file_path) {
                            Ok(content) => content,
                            Err(e) => return Err(format!("Failed to read schema.rs: {}", e)),
                        };

                        SchemaGenerator::rebuild_entire_table_in_schema(
                            &existing_content,
                            &table_def,
                            schema_file_path,
                        )?;

                        let model_content = ModelGenerator::generate_model(&table_def)?;
                        Self::write_model_file(&table_def.name, &model_content)?;
                        info!("Regenerated schema and model for table '{}' due to field ordering mismatch", table_def.name);
                    } else {
                        debug!("Table '{}': Schema and model field ordering match, skipping regeneration", table_def.name);
                    }
                }
            }
        }

        // Generate migration if there are any changes
        if !all_changes.is_empty() {
            info!(
                "Generating migration with {} total change(s) across {} table(s)",
                all_changes.len(),
                all_changes
                    .iter()
                    .map(|c| &c.table_name)
                    .collect::<std::collections::HashSet<_>>()
                    .len()
            );
            // Create a single migration for all changes
            // Pass all table definitions for migration context
            MigrationGenerator::generate_migration(&all_changes, &table_definitions)?
        } else {
            info!("No schema changes detected, skipping migration generation");
        }

        // Update hypertables array based on current table definitions
        Self::update_hypertables_array(&table_definitions)?;

        Ok(())
    }

    /// Check if CREATE_SCHEMA environment variable is enabled
    fn is_create_schema_enabled() -> bool {
        match env::var("CREATE_SCHEMA") {
            Ok(value) => {
                let normalized = value.to_lowercase();
                normalized == "true" || normalized == "1" || normalized == "yes"
            }
            Err(_) => false,
        }
    }

    /// Check if field ordering has changed by comparing current model with expected ordering
    fn has_field_ordering_changed(table_def: &TableDefinition) -> Result<bool, String> {
        let singular_name = to_singular(&table_def.name);
        let model_file_path = Path::new(MODELS_DIR).join(format!("{}_model.rs", singular_name));

        if !model_file_path.exists() {
            return Ok(true); // Model doesn't exist, needs to be created
        }

        // Read existing model file
        let existing_content = fs::read_to_string(&model_file_path)
            .map_err(|e| format!("Failed to read existing model file: {}", e))?;

        // Get expected field ordering using the same logic as generators
        let expected_fields = Self::get_expected_field_order(table_def)?;
        let model_fields = Self::extract_field_order_from_model(&existing_content);

        // Compare expected field order with current model field order
        Ok(expected_fields != model_fields)
    }

    /// Get expected field ordering using the same logic as generators (system fields first, then entity fields)
    fn get_expected_field_order(table_def: &TableDefinition) -> Result<Vec<String>, String> {
        let system_field_names = Self::get_system_field_names()?;
        let mut ordered_fields = Vec::new();

        // Parse all fields from table definition
        let mut parsed_fields = Vec::new();
        for field in &table_def.fields {
            match field.parse() {
                Ok(parsed) => parsed_fields.push(parsed),
                Err(e) => return Err(format!("Error parsing field {}: {}", field.name, e)),
            }
        }

        // First, add system fields in the order defined by system_fields macro
        for system_field_name in &system_field_names {
            if let Some(field) = parsed_fields.iter().find(|f| f.name == *system_field_name) {
                ordered_fields.push(field.name.clone());
            }
        }

        // Then, add non-system fields (entity-specific fields)
        for field in &parsed_fields {
            if !system_field_names.contains(&field.name) {
                ordered_fields.push(field.name.clone());
            }
        }

        Ok(ordered_fields)
    }

    /// Get system field names from the system_fields macro
    fn get_system_field_names() -> Result<Vec<String>, String> {
        let system_fields_path = SYSTEM_FIELDS_FILE;
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

        let macro_content = &content[macro_content_start..macro_end];
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

    /// Extract field order from model content for comparison
    fn extract_field_order_from_model(content: &str) -> Vec<String> {
        let mut fields = Vec::new();

        for line in content.lines() {
            let line = line.trim();
            if line.starts_with("pub ") && line.contains(":") && !line.contains("struct") {
                if let Some(field_name) = line.split_whitespace().nth(1) {
                    if let Some(name_only) = field_name.split(':').next() {
                        fields.push(name_only.to_string());
                    }
                }
            }
        }

        fields
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
        let singular_name = to_singular(table_name);
        let model_file_path = models_dir.join(format!("{}_model.rs", singular_name));

        // Write model content to file
        fs::write(&model_file_path, model_content).map_err(|e| {
            format!(
                "Failed to write model file {}: {}",
                model_file_path.display(),
                e
            )
        })?;

        // Add module declaration to mod.rs
        Self::add_module_to_mod_rs(&singular_name)?;

        Ok(())
    }

    /// Add module declaration to models/mod.rs
    fn add_module_to_mod_rs(singular_name: &str) -> Result<(), String> {
        let mod_file_path = Path::new(MODELS_MOD_FILE);

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

        Ok(())
    }

    /// Discover all table definition files in the schema directory
    fn discover_table_definitions() -> Result<Vec<TableDefinition>, String> {
        let tables_dir = SCHEMA_TABLES_DIR;

        debug!("Scanning directory: {}", tables_dir);

        if !Path::new(tables_dir).exists() {
            debug!("Tables directory does not exist: {}", tables_dir);
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
                if file_name_str == "mod.rs"
                    || file_name_str.ends_with("_generator.rs")
                    || file_name_str == "field_definition.rs"
                    || file_name_str == "generator_service.rs"
                {
                    continue;
                }
            }

            // Try to parse as table definition
            debug!("Processing file: {:?}", path);
            match Self::parse_table_definition_file(&path) {
                Ok(Some(table_def)) => {
                    debug!("Successfully parsed table definition: {}", table_def.name);
                    table_definitions.push(table_def);
                }
                Ok(None) => {
                    // File doesn't contain table definition, skip
                    debug!(
                        "File {:?} does not contain table definition, skipping",
                        path
                    );
                    continue;
                }
                Err(e) if e.starts_with("Skipping table") => {
                    // Table was skipped due to validation error, continue with other tables

                    continue;
                }
                Err(e) => {
                    error!("Failed to parse table definition: {}", e);
                    continue;
                }
            }
        }

        Ok(table_definitions)
    }

    /// Parse a table definition file
    fn parse_table_definition_file(file_path: &Path) -> Result<Option<TableDefinition>, String> {
        let content =
            fs::read_to_string(file_path).map_err(|e| format!("Failed to read file: {}", e))?;

        // Try to extract table name from file name
        let file_stem = file_path
            .file_stem()
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
        let has_struct_based = content.contains("DieselTableDefinition")
            || content.contains("define_table_schema!")
            || content.contains("impl DieselTableDefinition");

        // Look for legacy patterns
        let has_legacy = content.contains("FieldDefinition") || content.contains("TableDefinition");

        has_comment_based || has_struct_based || has_legacy
    }

    /// Parse table definition content
    fn parse_table_definition_content(
        content: &str,
        table_name: &str,
    ) -> Result<Option<TableDefinition>, String> {
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
    fn parse_diesel_table_definition(
        content: &str,
        file_name: &str,
    ) -> Result<TableDefinition, String> {
        // Look for struct definitions that implement DieselTableDefinition
        if content.contains("DieselTableDefinition") || content.contains("define_table_schema!") {
            // Extract table name from macro or struct
            let table_name = Self::extract_table_name_from_diesel_def(content, file_name)?;

            // Extract hypertable parameter
            let hypertable = Self::extract_hypertable_from_macro(content)?;

            let mut fields = Vec::new();

            // Try to extract field information from macro usage
            if let Ok(extracted_fields) = Self::extract_fields_from_macro(content) {
                fields = extracted_fields;
            } else {
                // Fallback: create a placeholder field to indicate this is a valid table
                let id_field = FieldDefinition::new("id".to_string(), "integer()".to_string())
                    .map_err(|e| format!("Failed to create fallback field: {}", e))?
                    .with_attributes(true, false, true, None);
                fields.push(id_field);
            }

            // Extract foreign keys from macro
            let foreign_key_definitions =
                Self::extract_foreign_keys_from_macro(content).unwrap_or_else(|_| Vec::new());

            // Convert ForeignKeyDefinition to ForeignKey
            let foreign_keys: Vec<ForeignKey> = foreign_key_definitions
                .into_iter()
                .map(|fk_def| ForeignKey {
                    field: fk_def.column,
                    references_table: fk_def.references_table,
                    references_field: fk_def.references_column,
                })
                .collect();

            // Validate hypertable constraints if hypertable is enabled
            if hypertable {
                if let Err(validation_error) =
                    Self::validate_hypertable_constraints(&table_name, &fields)
                {
                    return Err(format!(
                        "Skipping table '{}': {}",
                        table_name, validation_error
                    ));
                }
            }

            return Ok(TableDefinition {
                name: table_name,
                fields,
                indexes: Vec::new(),
                foreign_keys,
                is_hypertable: hypertable,
            });
        }

        Err("No Diesel table definition found".to_string())
    }

    /// Extract table name from Diesel definition
    fn extract_table_name_from_diesel_def(
        content: &str,
        file_name: &str,
    ) -> Result<String, String> {
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

    /// Convert field type for schema generation (VarChar -> Text)
    fn convert_field_type_for_schema(field_type: &str) -> String {
        if field_type.starts_with("Nullable<Varchar<") {
            "Nullable<Text>".to_string()
        } else if field_type.starts_with("Varchar<") {
            "Text".to_string()
        } else if field_type == "Nullable<Varchar>" {
            "Nullable<Text>".to_string()
        } else if field_type == "Varchar" {
            "Text".to_string()
        } else {
            field_type.to_string()
        }
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
                    }
                    _ => {}
                }
            }

            if fields_end > 0 {
                let mut fields_content = after_fields[..fields_end].to_string();

                // First, collect all explicitly defined fields to track overrides
                let mut explicit_fields = std::collections::HashSet::new();
                for line in fields_content.lines() {
                    let line = line.trim();
                    if line.is_empty()
                        || line.starts_with("//")
                        || line.contains("system_fields!()")
                    {
                        continue;
                    }
                    if let Some(colon_pos) = line.find(':') {
                        let field_name = line[..colon_pos].trim().to_string();
                        explicit_fields.insert(field_name.clone());
                    }
                }

                // Expand system_fields!() macro if present, but filter out overridden fields
                if fields_content.contains("system_fields!()") {
                    let system_fields_expansion = Self::get_system_fields_expansion()?;

                    let filtered_system_fields =
                        Self::filter_system_fields(&system_fields_expansion, &explicit_fields)?;

                    fields_content =
                        fields_content.replace("system_fields!()", &filtered_system_fields);
                }

                // Parse each field line - process explicit fields first, then system fields
                let mut explicit_field_lines = Vec::new();
                let mut system_field_lines = Vec::new();

                for line in fields_content.lines() {
                    let line = line.trim();
                    if line.is_empty() || line.starts_with("//") {
                        continue;
                    }

                    // Check if this line defines an explicit field (has a colon and field name)
                    if let Some(colon_pos) = line.find(':') {
                        let field_name = line[..colon_pos].trim();
                        if explicit_fields.contains(field_name) {
                            explicit_field_lines.push(line);
                        } else {
                            system_field_lines.push(line);
                        }
                    }
                }

                // Process explicit fields first, then system fields
                let all_lines: Vec<&str> = explicit_field_lines
                    .into_iter()
                    .chain(system_field_lines.into_iter())
                    .collect();

                for line in all_lines {
                    let line = line.trim();
                    if line.is_empty() || line.starts_with("//") {
                        continue;
                    }

                    // Parse field definition like: "id: integer(), primary_key: true,"
                    if let Some(colon_pos) = line.find(':') {
                        let field_name = line[..colon_pos].trim().to_string();
                        let rest = &line[colon_pos + 1..];

                        // Extract type (check nullable first to avoid false matches)
                        let field_type = if rest.contains("nullable(") {
                            // Extract inner type from nullable wrapper
                            if rest.contains("nullable(text())") {
                                "Nullable<Text>".to_string()
                            } else if rest.contains("nullable(integer())") {
                                "Nullable<Int4>".to_string()
                            } else if rest.contains("nullable(bigint())") {
                                "Nullable<Int8>".to_string()
                            } else if rest.contains("nullable(boolean())") {
                                "Nullable<Bool>".to_string()
                            } else if rest.contains("nullable(jsonb())") {
                                "Nullable<Jsonb>".to_string()
                            } else if rest.contains("nullable(timestamp())") {
                                "Nullable<Timestamp>".to_string()
                            } else if rest.contains("nullable(timestamptz())") {
                                "Nullable<Timestamptz>".to_string()
                            } else if rest.contains("nullable(array(text()))") {
                                "Nullable<Array<Text>>".to_string()
                            } else if rest.contains("nullable(varchar(Some(") {
                                // Parse varchar(Some(300)) format
                                if let Some(start) = rest.find("varchar(Some(") {
                                    if let Some(end) = rest[start..].find("))") {
                                        let length_part = &rest[start + 13..start + end]; // Skip "varchar(Some("
                                        format!("Nullable<Varchar<{}>>", length_part)
                                    } else {
                                        "Nullable<Varchar>".to_string()
                                    }
                                } else {
                                    "Nullable<Varchar>".to_string()
                                }
                            } else {
                                "Nullable<Text>".to_string() // default
                            }
                        } else if rest.contains("varchar(Some(") {
                            // Parse varchar(Some(300)) format for non-nullable
                            if let Some(start) = rest.find("varchar(Some(") {
                                if let Some(end) = rest[start..].find("))") {
                                    let length_part = &rest[start + 13..start + end]; // Skip "varchar(Some("
                                    format!("Varchar<{}>", length_part)
                                } else {
                                    "Varchar".to_string()
                                }
                            } else {
                                "Varchar".to_string()
                            }
                        } else if rest.contains("integer()") {
                            "Int4".to_string()
                        } else if rest.contains("bigint()") {
                            "Int8".to_string()
                        } else if rest.contains("text()") {
                            "Text".to_string()
                        } else if rest.contains("boolean()") {
                            "Bool".to_string()
                        } else if rest.contains("timestamp()") {
                            "Timestamp".to_string()
                        } else if rest.contains("timestamptz()") {
                            "Timestamptz".to_string()
                        } else {
                            "Text".to_string() // default
                        };

                        let is_primary_key = rest.contains("primary_key: true");
                        let is_index = rest.contains("indexed: true");

                        // Extract migration_nullable value, default to true if not specified
                        let migration_nullable = if rest.contains("migration_nullable: false") {
                            false
                        } else {
                            true // Default to nullable
                        };

                        // Extract default value if specified
                        let default_value = if let Some(default_start) = rest.find("default: ") {
                            let default_part = &rest[default_start + 9..]; // Skip "default: "
                            if let Some(comma_pos) = default_part.find(',') {
                                Some(
                                    default_part[..comma_pos]
                                        .trim()
                                        .trim_matches('"')
                                        .to_string(),
                                )
                            } else {
                                Some(default_part.trim().trim_matches('"').to_string())
                            }
                        } else {
                            None
                        };

                        // Check for duplicate fields and replace if found
                        if let Some(existing_index) = fields
                            .iter()
                            .position(|f: &FieldDefinition| f.name == field_name)
                        {
                            if fields[existing_index].diesel_type != field_type {
                                fields.remove(existing_index);
                            } else {
                                continue;
                            }
                        }

                        // Create field definition with original type for migrations
                        let mut field_def =
                            FieldDefinition::new_direct(field_name.clone(), field_type.clone())
                                .map_err(|e| format!("Failed to create field definition: {}", e))?
                                .with_attributes(
                                    is_primary_key,
                                    is_index,
                                    migration_nullable,
                                    default_value.clone(),
                                );

                        // Store the original type for migrations and converted type for schema
                        field_def.migration_type = Some(field_type.clone());
                        field_def.diesel_type = Self::convert_field_type_for_schema(&field_type);

                        fields.push(field_def);
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
    fn extract_indexes_from_macro(
        content: &str,
    ) -> Result<Vec<(String, Vec<String>, bool, Option<String>)>, String> {
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
                    }
                    _ => {}
                }
            }

            if indexes_end > 0 {
                let mut indexes_content = after_indexes[..indexes_end].to_string();

                // Expand system_indexes!(table_name) macro if present
                if let Some(table_name) =
                    Self::extract_table_name_from_system_indexes(&indexes_content)
                {
                    let system_indexes_expansion = Self::get_system_indexes_expansion(&table_name)?;
                    let pattern = format!("system_indexes!(\"{}\")", table_name);
                    indexes_content = indexes_content.replace(&pattern, &system_indexes_expansion);
                }

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
                        if line.contains("columns:") && !line.contains("foreign_columns:") {
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
                            let type_val = line
                                .split(':')
                                .nth(1)
                                .unwrap()
                                .trim()
                                .trim_matches('"')
                                .to_string();
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
        use crate::builders::generator::diesel_schema_definition::ForeignKeyDefinition;

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
                    }
                    _ => {}
                }
            }

            if fk_end > 0 {
                let mut fk_content = after_fk[..fk_end].to_string();

                // Expand system_foreign_keys!(table_name) macro if present
                if let Some(table_name) =
                    Self::extract_table_name_from_system_foreign_keys(&fk_content)
                {
                    let system_foreign_keys_expansion =
                        Self::get_system_foreign_keys_expansion(&table_name)?;
                    let pattern = format!("system_foreign_keys!(\"{}\")", table_name);
                    fk_content = fk_content.replace(&pattern, &system_foreign_keys_expansion);
                }

                // Parse each foreign key definition
                let mut current_fk: Option<ForeignKeyDefinition> = None;
                let mut in_fk_def = false;

                for line in fk_content.lines() {
                    let line = line.trim();
                    if line.is_empty() || line.starts_with("//") {
                        continue;
                    }

                    // Check for foreign key constraint name
                    if line.contains(": {") {
                        // Save previous foreign key if exists
                        if let Some(fk) = current_fk.take() {
                            foreign_keys.push(fk);
                        }

                        current_fk = Some(ForeignKeyDefinition {
                            column: String::new(),
                            references_table: String::new(),
                            references_column: String::new(),
                            on_delete: None,
                            on_update: None,
                        });
                        in_fk_def = true;
                    } else if in_fk_def {
                        if line.contains("columns:") && !line.contains("foreign_columns:") {
                            // Extract columns - this is the local table column
                            if let Some(bracket_start) = line.find('[') {
                                if let Some(bracket_end) = line.find(']') {
                                    let columns_str = &line[bracket_start + 1..bracket_end];
                                    let column = columns_str
                                        .trim()
                                        .trim_matches('"')
                                        .replace(",", "")
                                        .replace("\"", "")
                                        .to_string();

                                    if let Some(ref mut fk) = current_fk {
                                        fk.column = column;
                                    }
                                }
                            }
                        } else if line.contains("foreign_table:") {
                            let table_val = line
                                .split(':')
                                .nth(1)
                                .unwrap()
                                .trim()
                                .trim_matches('"')
                                .replace(",", "")
                                .replace("\"", "")
                                .to_string();
                            if let Some(ref mut fk) = current_fk {
                                fk.references_table = table_val.replace(",", "").replace("\"", "");
                            }
                        } else if line.contains("foreign_columns:") {
                            // Extract foreign_columns - this is the referenced table column
                            if let Some(bracket_start) = line.find('[') {
                                if let Some(bracket_end) = line.find(']') {
                                    let columns_str = &line[bracket_start + 1..bracket_end];
                                    let column = columns_str
                                        .trim()
                                        .trim_matches('"')
                                        .replace(",", "")
                                        .replace("\"", "")
                                        .to_string();

                                    if let Some(ref mut fk) = current_fk {
                                        fk.references_column = column;
                                    }
                                }
                            }
                        } else if line.contains("on_delete:") {
                            let delete_val = line
                                .split(':')
                                .nth(1)
                                .unwrap()
                                .trim()
                                .trim_matches('"')
                                .to_string();
                            if let Some(ref mut fk) = current_fk {
                                fk.on_delete = Some(delete_val);
                            }
                        } else if line.contains("on_update:") {
                            let update_val = line
                                .split(':')
                                .nth(1)
                                .unwrap()
                                .trim()
                                .trim_matches('"')
                                .to_string();
                            if let Some(ref mut fk) = current_fk {
                                fk.on_update = Some(update_val);
                            }
                        } else if line == "}" {
                            in_fk_def = false;
                        }
                    }

                    // Also handle old format: "category_id -> "categories"."id""
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

                // Save last foreign key if exists
                if let Some(fk) = current_fk {
                    foreign_keys.push(fk);
                }
            }
        }

        Ok(foreign_keys)
    }

    /// Extract hypertable parameter from macro definition
    fn extract_hypertable_from_macro(content: &str) -> Result<bool, String> {
        // Look for hypertable parameter in macro
        if let Some(hypertable_start) = content.find("hypertable:") {
            let after_hypertable = &content[hypertable_start + 11..]; // "hypertable:".len() = 11
            let line = after_hypertable.lines().next().unwrap_or("").trim();

            if line.starts_with("true") {
                return Ok(true);
            } else if line.starts_with("false") {
                return Ok(false);
            }
        }

        // Default to false if not specified
        Ok(false)
    }

    /// Update the hypertables array in hypertables.rs based on current table definitions
    fn update_hypertables_array(table_definitions: &[TableDefinition]) -> Result<(), String> {
        let hypertables_file_path = HYPERTABLES_FILE;

        // Collect all hypertable names
        let hypertable_names: Vec<&str> = table_definitions
            .iter()
            .filter(|def| def.is_hypertable)
            .map(|def| def.name.as_str())
            .collect();

        // Generate the new hypertables.rs content
        let mut content = String::new();
        content.push_str("#[allow(warnings)]\n");
        content.push_str("pub const HYPERTABLES: &[&str] = &[\n");

        for table_name in &hypertable_names {
            content.push_str(&format!("    \"{}\",\n", table_name));
        }

        content.push_str("    // Add more hypertable names as needed\n");
        content.push_str("];\n");
        content.push_str("#[allow(warnings)]\n");
        content.push_str("// Helper function to check if a table is a hypertable\n");
        content.push_str("pub fn is_hypertable(table_name: &str) -> bool {\n");
        content.push_str("    HYPERTABLES.contains(&table_name)\n");
        content.push_str("}\n");

        // Write the updated content to the file
        fs::write(hypertables_file_path, content)
            .map_err(|e| format!("Failed to update hypertables.rs: {}", e))?;

        if !hypertable_names.is_empty() {
        } else {
        }

        Ok(())
    }

    /// Validate hypertable constraints
    fn validate_hypertable_constraints(
        table_name: &str,
        fields: &[FieldDefinition],
    ) -> Result<(), String> {
        // Check for hypertable_timestamp field with text type
        let has_hypertable_timestamp = fields.iter().any(|f| {
            f.name == "hypertable_timestamp"
                && (f.diesel_type == "Text" || f.diesel_type == "Nullable<Text>")
        });

        if !has_hypertable_timestamp {
            return Err(format!(
                "Hypertable '{}' must have a 'hypertable_timestamp' field of type 'Text'",
                table_name
            ));
        }

        // Check for composite primary key (id, timestamp)
        let primary_key_fields: Vec<&str> = fields
            .iter()
            .filter(|f| f.is_primary_key)
            .map(|f| f.name.as_str())
            .collect();

        let has_id_pk = primary_key_fields.contains(&"id");
        let has_timestamp_pk = primary_key_fields.contains(&"timestamp")
            || primary_key_fields.contains(&"hypertable_timestamp");

        if !has_id_pk {
            return Err(format!(
                "Hypertable '{}' must have 'id' as part of the primary key",
                table_name
            ));
        }

        if !has_timestamp_pk {
            return Err(format!(
                "Hypertable '{}' must have 'timestamp' or 'hypertable_timestamp' as part of the primary key", 
                table_name
            ));
        }

        // Ensure timestamp field has timestamptz type
        let timestamp_field = fields.iter().find(|f| {
            (f.name == "timestamp" || f.name == "hypertable_timestamp") && f.is_primary_key
        });
        if let Some(ts_field) = timestamp_field {
            if ts_field.name == "timestamp"
                && !(ts_field.diesel_type == "Timestamptz"
                    || ts_field.diesel_type == "Nullable<Timestamptz>")
            {
                return Err(format!(
                    "Hypertable '{}' timestamp field must have type 'Timestamptz'",
                    table_name
                ));
            }
        }

        Ok(())
    }

    /// Parse Rust-style table definition
    fn parse_rust_table_definition(
        content: &str,
        _table_name: &str,
    ) -> Result<TableDefinition, String> {
        // Look for TableDefinition struct instantiation
        if content.contains("TableDefinition") {
            // This would be a more complex parser for Rust syntax
            // For now, we'll implement a simple pattern matcher
            return Err("Rust table definition parsing not yet implemented".to_string());
        }

        Err("No Rust table definition found".to_string())
    }

    /// Parse simple table definition format
    fn parse_simple_table_definition(
        content: &str,
        table_name: &str,
    ) -> Result<TableDefinition, String> {
        let mut fields = Vec::new();
        let lines: Vec<&str> = content.lines().collect();

        let mut current_field_name: Option<String> = None;
        let mut current_field_type: Option<String> = None;
        let mut current_is_index = false;
        // joins_with functionality removed
        let mut current_default_value: Option<String> = None;

        for line in lines {
            let line = line.trim();

            // Skip empty lines and comments
            if line.is_empty() || line.starts_with("//") || line.starts_with("/*") {
                continue;
            }

            // Parse field properties
            if line.starts_with("field_name:") {
                // Save previous field if exists
                if let (Some(name), Some(field_type)) =
                    (current_field_name.take(), current_field_type.take())
                {
                    let field = FieldDefinition::new(name, field_type)
                        .map_err(|e| format!("Failed to create field: {}", e))?
                        .with_attributes(
                            false,
                            current_is_index,
                            true,
                            current_default_value.take(),
                        );
                    fields.push(field);
                    current_is_index = false;
                }

                current_field_name = Some(
                    line.split(':')
                        .nth(1)
                        .ok_or("Invalid field_name format")?
                        .trim()
                        .to_string(),
                );
            } else if line.starts_with("field_type:") {
                current_field_type = Some(
                    line.split(':')
                        .nth(1)
                        .ok_or("Invalid field_type format")?
                        .trim()
                        .to_string(),
                );
            } else if line.starts_with("is_index:") {
                let value = line
                    .split(':')
                    .nth(1)
                    .ok_or("Invalid is_index format")?
                    .trim();
                current_is_index = value == "true";
            } else if line.starts_with("joins_with:") {
                // joins_with functionality removed - ignoring this attribute
            } else if line.starts_with("default_value:") {
                let value = line
                    .split(':')
                    .nth(1)
                    .ok_or("Invalid default_value format")?
                    .trim();
                if !value.is_empty() && value != "null" && value != "None" {
                    current_default_value = Some(value.to_string());
                }
            }
        }

        // Save last field
        if let (Some(name), Some(field_type)) = (current_field_name, current_field_type) {
            let field = FieldDefinition::new(name, field_type)
                .map_err(|e| format!("Failed to create field: {}", e))?
                .with_attributes(false, current_is_index, true, current_default_value);
            fields.push(field);
        }

        if fields.is_empty() {
            return Err("No fields found in table definition".to_string());
        }

        Ok(TableDefinition {
            name: table_name.to_string(),
            fields,
            indexes: Vec::new(),
            foreign_keys: Vec::new(),
            is_hypertable: false,
        })
    }

    /// Dynamically reads the system_fields macro from system_fields.rs
    fn get_system_fields_expansion() -> Result<String, String> {
        let system_fields_path = SYSTEM_FIELDS_FILE;
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

        // Clean up the content - remove extra whitespace and format for diesel schema
        let cleaned_content = macro_content
            .trim()
            .lines()
            .map(|line| line.trim())
            .filter(|line| !line.is_empty())
            .collect::<Vec<&str>>()
            .join("\n        ");

        Ok(cleaned_content)
    }

    /// Dynamically reads the system_indexes macro from system_fields.rs
    fn extract_table_name_from_system_indexes(content: &str) -> Option<String> {
        use regex::Regex;
        let re = Regex::new(r#"system_indexes!\("([^"]+)"\)"#).ok()?;
        if let Some(captures) = re.captures(content) {
            captures.get(1).map(|m| m.as_str().to_string())
        } else {
            None
        }
    }

    fn get_system_indexes_expansion(table_name: &str) -> Result<String, String> {
        // Generate the expanded system indexes with table name prefixes
        let indexes = vec![
            ("tombstone", "tombstone"),
            ("status", "status"),
            ("previous_status", "previous_status"),
            ("version", "version"),
            ("created_date", "created_date"),
            ("updated_date", "updated_date"),
            ("organization_id", "organization_id"),
            ("created_by", "created_by"),
            ("updated_by", "updated_by"),
            ("deleted_by", "deleted_by"),
            ("requested_by", "requested_by"),
            ("tags", "tags"),
            ("categories", "categories"),
            ("code", "code"),
            ("sensitivity_level", "sensitivity_level"),
        ];

        let mut result = String::new();
        for (i, (field_suffix, column_name)) in indexes.iter().enumerate() {
            if i > 0 {
                result.push_str(",\n        ");
            }
            result.push_str(&format!(
                "idx_{}_{}: {{\n            columns: [\"{}\"],\n            unique: false,\n            type: \"btree\"\n        }}",
                table_name, field_suffix, column_name
            ));
        }

        Ok(result)
    }

    /// Extract table name from system_foreign_keys macro call
    fn extract_table_name_from_system_foreign_keys(content: &str) -> Option<String> {
        use regex::Regex;
        let re = Regex::new(r#"system_foreign_keys!\("([^"]+)"\)"#).ok()?;
        if let Some(captures) = re.captures(content) {
            captures.get(1).map(|m| m.as_str().to_string())
        } else {
            None
        }
    }

    /// Get system foreign keys expansion for a given table name
    fn get_system_foreign_keys_expansion(table_name: &str) -> Result<String, String> {
        // Use the system_foreign_keys macro to generate the foreign key definitions
        // This ensures consistency with the macro definition in system_fields.rs
        // Clean the table name to avoid issues with quotes or commas
        let clean_table_name = table_name.replace(",", "").replace("\"", "");

        let result = format!(
            "{}_organization_id_organizations_id_fk: {{\n            columns: [\"organization_id\"],\n            foreign_table: \"organizations\",\n            foreign_columns: [\"id\"],\n            on_delete: \"no action\",\n            on_update: \"no action\"\n        }},\n        {}_created_by_account_organizations_id_fk: {{\n            columns: [\"created_by\"],\n            foreign_table: \"account_organizations\",\n            foreign_columns: [\"id\"],\n            on_delete: \"no action\",\n            on_update: \"no action\"\n        }},\n        {}_updated_by_account_organizations_id_fk: {{\n            columns: [\"updated_by\"],\n            foreign_table: \"account_organizations\",\n            foreign_columns: [\"id\"],\n            on_delete: \"no action\",\n            on_update: \"no action\"\n        }},\n        {}_deleted_by_account_organizations_id_fk: {{\n            columns: [\"deleted_by\"],\n            foreign_table: \"account_organizations\",\n            foreign_columns: [\"id\"],\n            on_delete: \"no action\",\n            on_update: \"no action\"\n        }},\n        {}_requested_by_account_organizations_id_fk: {{\n            columns: [\"requested_by\"],\n            foreign_table: \"account_organizations\",\n            foreign_columns: [\"id\"],\n            on_delete: \"no action\",\n            on_update: \"no action\"\n        }}",
            clean_table_name,
            clean_table_name,
            clean_table_name,
            clean_table_name,
            clean_table_name
        );

        Ok(result)
    }

    /// Filter system fields to exclude those that are explicitly overridden
    fn filter_system_fields(
        system_fields_expansion: &str,
        explicit_fields: &std::collections::HashSet<String>,
    ) -> Result<String, String> {
        let mut filtered_lines = Vec::new();

        for line in system_fields_expansion.lines() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }

            // Extract field name from line
            if let Some(colon_pos) = line.find(':') {
                let field_name = line[..colon_pos].trim();
                // Only include system field if it's not explicitly overridden
                if !explicit_fields.contains(field_name) {
                    filtered_lines.push(line);
                }
            }
        }

        Ok(filtered_lines.join("\n        "))
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
        assert!(GeneratorService::contains_table_definition(
            content_with_def
        ));

        let content_without_def = "some random content";
        assert!(!GeneratorService::contains_table_definition(
            content_without_def
        ));
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
        assert_eq!(table_def.name, "test_table");
        assert_eq!(table_def.fields.len(), 2);
        assert_eq!(table_def.fields[0].name, "id");
        assert_eq!(table_def.fields[0].diesel_type, "Int4");
        assert!(table_def.fields[0].is_indexed);
        assert_eq!(table_def.fields[1].name, "name");
        assert_eq!(table_def.fields[1].diesel_type, "Nullable<Text>");
        assert!(!table_def.fields[1].is_indexed);
    }
}
