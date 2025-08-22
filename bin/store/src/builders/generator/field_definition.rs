use crate::builders::generator::utils::FieldTypeParser;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldDefinition {
    pub name: String,
    pub diesel_type: String,
    pub rust_type: String,
    pub is_primary_key: bool,
    pub is_indexed: bool,
    pub is_nullable: bool,
    pub is_array: bool,
    pub migration_nullable: bool,
    pub default_value: Option<String>,
    pub migration_type: Option<String>, // Original type for migrations (preserves VARCHAR)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableDefinition {
    pub name: String,
    pub fields: Vec<FieldDefinition>,
    pub indexes: Vec<String>,
    pub foreign_keys: Vec<ForeignKey>,
    pub is_hypertable: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParsedField {
    pub name: String,
    pub field_type: String,
    pub is_primary_key: bool,
    pub is_indexed: bool,
    pub migration_nullable: bool,
    pub default_value: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForeignKey {
    pub field: String,
    pub references_table: String,
    pub references_field: String,
}

#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
pub enum DieselType {
    Text,
    Char(u32),
    Int4,
    Int8,
    BigInt,
    Bool,
    Timestamp,
    Timestamptz,
    Jsonb,
    Inet,
    Float4,
    Float8,
    Nullable(Box<DieselType>),
    Array(Box<DieselType>),
}

#[allow(dead_code)]
impl DieselType {
    pub fn to_rust_type(&self) -> String {
        let diesel_type_str = self.to_diesel_type();
        FieldTypeParser::diesel_to_rust_type(&diesel_type_str)
            .unwrap_or_else(|_| "String".to_string())
    }

    pub fn to_diesel_type(&self) -> String {
        match self {
            DieselType::Text => "Text".to_string(),

            DieselType::Char(len) => format!("Char<{}>", len),
            DieselType::Int4 => "Int4".to_string(),
            DieselType::Int8 => "Int8".to_string(),
            DieselType::BigInt => "BigInt".to_string(),
            DieselType::Bool => "Bool".to_string(),
            DieselType::Timestamp => "Timestamp".to_string(),
            DieselType::Timestamptz => "Timestamptz".to_string(),
            DieselType::Jsonb => "Jsonb".to_string(),
            DieselType::Inet => "Inet".to_string(),
            DieselType::Float4 => "Float4".to_string(),
            DieselType::Float8 => "Float8".to_string(),
            DieselType::Nullable(inner) => format!("Nullable<{}>", inner.to_diesel_type()),
            DieselType::Array(inner) => format!("Array<{}>", inner.to_diesel_type()),
        }
    }
}

impl FieldDefinition {
    /// Create a new FieldDefinition from a field type string
    pub fn new(name: String, field_type: String) -> Result<Self, String> {
        let diesel_type = FieldTypeParser::parse_diesel_type(&field_type)?;
        let rust_type = FieldTypeParser::diesel_to_rust_type(&diesel_type)?;

        let is_nullable = diesel_type.contains("Nullable");
        let is_array = diesel_type.contains("Array");

        Ok(FieldDefinition {
            name,
            diesel_type,
            rust_type,
            is_primary_key: false,
            is_indexed: false,
            is_nullable,
            is_array,
            migration_nullable: true,
            default_value: None,
            migration_type: None,
        })
    }

    /// Create a new FieldDefinition directly from diesel type (bypassing parsing)
    pub fn new_direct(name: String, diesel_type: String) -> Result<Self, String> {
        let rust_type = FieldTypeParser::diesel_to_rust_type(&diesel_type)?;

        let is_nullable = diesel_type.contains("Nullable");
        let is_array = diesel_type.contains("Array");

        Ok(FieldDefinition {
            name,
            diesel_type,
            rust_type,
            is_primary_key: false,
            is_indexed: false,
            is_nullable,
            is_array,
            migration_nullable: true,
            default_value: None,
            migration_type: None,
        })
    }

    /// Parse the diesel field type to extract information
    pub fn parse(&self) -> Result<ParsedField, String> {
        self.parse_for_context(false) // Default to schema generation
    }

    /// Parse the diesel field type to extract information with context
    pub fn parse_for_context(&self, for_migration: bool) -> Result<ParsedField, String> {
        // For migrations, use migration_type if available, otherwise diesel_type
        // For schema, always use diesel_type (which has VARCHAR converted to Text)
        let field_type = if for_migration {
            self.migration_type
                .as_ref()
                .unwrap_or(&self.diesel_type)
                .clone()
        } else {
            self.diesel_type.clone()
        };

        Ok(ParsedField {
            name: self.name.clone(),
            field_type,
            is_primary_key: self.is_primary_key,
            is_indexed: self.is_indexed,
            migration_nullable: self.migration_nullable,
            default_value: self.default_value.clone(),
        })
    }

    /// Set field attributes
    pub fn with_attributes(
        mut self,
        is_primary_key: bool,
        is_indexed: bool,
        migration_nullable: bool,
        default_value: Option<String>,
    ) -> Self {
        self.is_primary_key = is_primary_key;
        self.is_indexed = is_indexed;
        self.migration_nullable = migration_nullable;
        self.default_value = default_value;
        self
    }
}

/// Parse a table definition file
#[allow(dead_code)]
pub fn parse_table_definition_file(content: &str) -> Result<TableDefinition, String> {
    // Try to parse as JSON first
    if let Ok(table_def) = serde_json::from_str::<TableDefinition>(content) {
        return Ok(table_def);
    }

    // If JSON parsing fails, try to parse as a simple format
    // This allows for a more user-friendly format
    let mut fields = Vec::new();
    let mut table_name = String::new();

    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with("//") {
            continue;
        }

        if line.starts_with("table_name:") {
            table_name = line.replace("table_name:", "").trim().to_string();
            continue;
        }

        // Parse field definition
        // Expected format: field_name: field_type [index] [joins_with: table.column] [default: value]
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() < 2 {
            continue;
        }

        let field_name = parts[0].trim_end_matches(':').to_string();
        let field_type = parts[1].to_string();

        let mut is_index = false;
        // joins_with functionality removed
        let mut default_value = None;

        // Parse additional attributes
        let remaining = &parts[2..];
        let mut i = 0;
        while i < remaining.len() {
            match remaining[i] {
                "index" => is_index = true,
                "joins_with:" if i + 1 < remaining.len() => {
                    // joins_with functionality removed - ignoring this attribute
                    i += 1;
                }
                "default:" if i + 1 < remaining.len() => {
                    default_value = Some(remaining[i + 1].to_string());
                    i += 1;
                }
                _ => {}
            }
            i += 1;
        }

        let field_def = FieldDefinition::new(field_name, field_type)
            .map_err(|e| format!("Failed to create field definition: {}", e))?
            .with_attributes(false, is_index, true, default_value);
        fields.push(field_def);
    }

    if table_name.is_empty() {
        return Err("Table name not specified".to_string());
    }

    Ok(TableDefinition {
        name: table_name,
        fields,
        indexes: Vec::new(),
        foreign_keys: Vec::new(),
        is_hypertable: false,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_field_definition() {
        let field =
            FieldDefinition::new("first_name".to_string(), "nullable(text())".to_string()).unwrap();

        let parsed = field.parse().unwrap();
        assert_eq!(parsed.name, "first_name");
        assert_eq!(field.rust_type, "Option<String>");
        assert!(field.is_nullable);
        assert!(!field.is_array);
    }

    #[test]
    fn test_parse_array_field() {
        let field = FieldDefinition::new("tags".to_string(), "nullable(array(text()))".to_string())
            .unwrap();

        // let parsed = field.parse().unwrap();
        assert_eq!(field.rust_type, "Option<Vec<String>>");
        assert!(field.is_nullable);
        assert!(field.is_array);
    }

    #[test]
    fn test_parse_indexed_field() {
        let field = FieldDefinition::new("device_id".to_string(), "nullable(text())".to_string())
            .unwrap()
            .with_attributes(false, true, true, None);

        let parsed = field.parse().unwrap();
        assert_eq!(parsed.name, "device_id");
        assert!(parsed.is_indexed);
        assert!(!parsed.is_primary_key);
    }
}
