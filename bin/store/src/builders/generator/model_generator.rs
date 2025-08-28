use crate::builders::generator::field_definition::{ParsedField, TableDefinition};
use crate::builders::generator::utils::{FieldTypeParser, StringUtils};
use crate::constants::paths;
use crate::utils::utils::to_singular;
use std::fs;

pub struct ModelGenerator;

impl ModelGenerator {
    /// Generate a Rust model file for the given table definition
    pub fn generate_model(table_def: &TableDefinition) -> Result<String, String> {
        let mut parsed_fields = Vec::new();

        // Parse all fields
        for field in &table_def.fields {
            match field.parse() {
                Ok(parsed) => parsed_fields.push(parsed),
                Err(e) => return Err(format!("Error parsing field {}: {}", field.name, e)),
            }
        }

        // Order fields properly (system fields first, then entity-specific fields)
        let ordered_fields = Self::order_fields_properly(&parsed_fields)?;

        // Generate the model content
        let model_content = Self::generate_model_content(&table_def.name, &ordered_fields)?;
        Ok(model_content)
    }

    /// Generate the actual model file content
    fn generate_model_content(table_name: &str, fields: &[ParsedField]) -> Result<String, String> {
        let singular_name = to_singular(table_name);
        let struct_name = format!("{}Model", StringUtils::to_pascal_case(&singular_name));

        let mut content = String::new();

        // Add basic imports
        content.push_str("use diesel::prelude::*;");
        content.push_str("\nuse serde::{Deserialize, Serialize};");

        // Collect non-chrono imports only
        let mut other_imports = std::collections::HashSet::new();

        for field in fields {
            let rust_type = FieldTypeParser::diesel_to_rust_type(&field.field_type)
                .unwrap_or_else(|_| "String".to_string());
            let field_type = &rust_type;
            let other_deps = Self::extract_non_chrono_dependencies(field_type);
            other_imports.extend(other_deps);
        }

        // Add other imports in sorted order
        let mut other_imports_vec: Vec<_> = other_imports.into_iter().collect();
        other_imports_vec.sort();

        for import in other_imports_vec {
            content.push_str(&format!("\n{}", import));
        }

        content.push_str("\n\n");

        // Generate the struct with all required traits
        content.push_str(&format!("#[derive(\n"));
        content.push_str(&format!("    Queryable, Selectable, Serialize, Default, Deserialize, Clone, AsChangeset, Insertable, Debug,\n"));
        content.push_str(&format!(")]\n"));
        content.push_str(&format!(
            "#[diesel(table_name = crate::generated::schema::{})]\n",
            table_name
        ));
        content.push_str(&format!("#[diesel(check_for_backend(diesel::pg::Pg))]\n"));
        content.push_str(&format!("#[serde(default)]\n"));
        content.push_str(&format!("pub struct {} {{\n", struct_name));

        // Add fields
        for field in fields {
            let rust_type = FieldTypeParser::diesel_to_rust_type(&field.field_type)
                .unwrap_or_else(|_| "String".to_string());
            content.push_str(&format!("    pub {}: {},\n", field.name, rust_type));
        }

        content.push_str("}\n");

        Ok(content)
    }

    /// Order fields properly according to system fields macro and entity-specific fields
    fn order_fields_properly(fields: &[ParsedField]) -> Result<Vec<ParsedField>, String> {
        let system_field_names = Self::get_system_field_names()?;
        let mut ordered_fields = Vec::new();

        // First, add system fields in the order defined by system_fields macro
        for system_field_name in &system_field_names {
            if let Some(field) = fields.iter().find(|f| f.name == *system_field_name) {
                ordered_fields.push(field.clone());
            }
        }

        // Then, add non-system fields (entity-specific fields)
        for field in fields {
            if !system_field_names.contains(&field.name) {
                ordered_fields.push(field.clone());
            }
        }

        Ok(ordered_fields)
    }

    /// Get system field names from the system_fields macro
    fn get_system_field_names() -> Result<Vec<String>, String> {
        let system_fields_path = paths::database::SYSTEM_FIELDS_FILE;
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

    fn extract_non_chrono_dependencies(rust_type: &str) -> Vec<String> {
        let mut dependencies = Vec::new();

        // Handle serde_json types
        if rust_type.contains("Value") || rust_type.contains("serde_json::") {
            dependencies.push("use serde_json::Value;".to_string());
        }

        // Handle std::net types
        if rust_type.contains("IpAddr") || rust_type.contains("std::net::") {
            dependencies.push("use std::net::IpAddr;".to_string());
        }

        // Handle UUID types
        if rust_type.contains("Uuid") || rust_type.contains("uuid::") {
            dependencies.push("use uuid::Uuid;".to_string());
        }

        // Handle BigDecimal types
        if rust_type.contains("BigDecimal") || rust_type.contains("bigdecimal::") {
            dependencies.push("use bigdecimal::BigDecimal;".to_string());
        }

        // Handle collections
        if rust_type.contains("HashMap") {
            dependencies.push("use std::collections::HashMap;".to_string());
        }
        if rust_type.contains("HashSet") {
            dependencies.push("use std::collections::HashSet;".to_string());
        }
        if rust_type.contains("BTreeMap") {
            dependencies.push("use std::collections::BTreeMap;".to_string());
        }
        if rust_type.contains("BTreeSet") {
            dependencies.push("use std::collections::BTreeSet;".to_string());
        }

        dependencies
    }
}
