use serde::{Deserialize, Serialize};

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldDefinition {
    pub field_name: String,
    pub field_type: String, // e.g., "Nullable<Text>", "Nullable<Jsonb>"
    pub is_index: bool,
    pub joins_with: Option<String>, // e.g., "devices.id"
    pub default_value: Option<String>,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableDefinition {
    pub table_name: String,
    pub fields: Vec<FieldDefinition>,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ParsedField {
    pub name: String,
    pub diesel_type: String,
    pub rust_type: String,
    pub is_nullable: bool,
    pub is_array: bool,
    pub is_json: bool,
    pub is_index: bool,
    pub foreign_key: Option<ForeignKey>,
    pub default_value: Option<String>,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ForeignKey {
    pub table: String,
    pub column: String,
}

impl FieldDefinition {
    /// Parse the diesel field type to extract information
    pub fn parse(&self) -> Result<ParsedField, String> {
        let diesel_type = self.field_type.trim();
        let mut is_nullable = false;
        let mut is_array = false;
        let mut is_json = false;
        let mut core_type = diesel_type;

        // Check if nullable
        if diesel_type.starts_with("Nullable<") && diesel_type.ends_with(">") {
            is_nullable = true;
            core_type = &diesel_type[9..diesel_type.len()-1]; // Remove "Nullable<" and ">"
        }

        // Check if array
        if core_type.starts_with("Array<") && core_type.ends_with(">") {
            is_array = true;
            core_type = &core_type[6..core_type.len()-1]; // Remove "Array<" and ">"
        }

        // Determine rust type and check for JSON
        let rust_type = match core_type {
            "Text" => "String".to_string(),
            "Int4" => "i32".to_string(),
            "Int8" => "i64".to_string(),
            "BigInt" => "i64".to_string(),
            "Bool" => "bool".to_string(),
            "Timestamp" => "chrono::NaiveDateTime".to_string(),
            "Timestamptz" => "DateTime<Utc>".to_string(),
            "Jsonb" => {
                is_json = true;
                "Value".to_string()
            },
            "Inet" => "std::net::IpAddr".to_string(),
            _ => return Err(format!("Unsupported field type: {}", core_type)),
        };

        // Parse foreign key if present
        let foreign_key = if let Some(ref joins_with) = self.joins_with {
            let parts: Vec<&str> = joins_with.split('.').collect();
            if parts.len() == 2 {
                Some(ForeignKey {
                    table: parts[0].to_string(),
                    column: parts[1].to_string(),
                })
            } else {
                return Err(format!("Invalid foreign key format: {}", joins_with));
            }
        } else {
            None
        };

        // Adjust rust type for arrays and nullability
        let final_rust_type = if is_array {
            if is_nullable {
                format!("Option<Vec<{}>>", rust_type)
            } else {
                format!("Vec<{}>", rust_type)
            }
        } else if is_nullable {
            format!("Option<{}>", rust_type)
        } else {
            rust_type
        };

        Ok(ParsedField {
            name: self.field_name.clone(),
            diesel_type: self.field_type.clone(),
            rust_type: final_rust_type,
            is_nullable,
            is_array,
            is_json,
            is_index: self.is_index,
            foreign_key,
            default_value: self.default_value.clone(),
        })
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
        let mut joins_with = None;
        let mut default_value = None;
        
        // Parse additional attributes
        let remaining = &parts[2..];
        let mut i = 0;
        while i < remaining.len() {
            match remaining[i] {
                "index" => is_index = true,
                "joins_with:" if i + 1 < remaining.len() => {
                    joins_with = Some(remaining[i + 1].to_string());
                    i += 1;
                },
                "default:" if i + 1 < remaining.len() => {
                    default_value = Some(remaining[i + 1].to_string());
                    i += 1;
                },
                _ => {},
            }
            i += 1;
        }
        
        fields.push(FieldDefinition {
            field_name,
            field_type,
            is_index,
            joins_with,
            default_value,
        });
    }
    
    if table_name.is_empty() {
        return Err("Table name not specified".to_string());
    }
    
    Ok(TableDefinition {
        table_name,
        fields,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_field_definition() {
        let field = FieldDefinition {
            field_name: "first_name".to_string(),
            field_type: "Nullable<Text>".to_string(),
            is_index: false,
            joins_with: None,
            default_value: None,
        };
        
        let parsed = field.parse().unwrap();
        assert_eq!(parsed.name, "first_name");
        assert_eq!(parsed.rust_type, "Option<String>");
        assert!(parsed.is_nullable);
        assert!(!parsed.is_array);
    }

    #[test]
    fn test_parse_array_field() {
        let field = FieldDefinition {
            field_name: "tags".to_string(),
            field_type: "Nullable<Array<Text>>".to_string(),
            is_index: false,
            joins_with: None,
            default_value: None,
        };
        
        let parsed = field.parse().unwrap();
        assert_eq!(parsed.rust_type, "Option<Vec<String>>");
        assert!(parsed.is_nullable);
        assert!(parsed.is_array);
    }

    #[test]
    fn test_parse_foreign_key() {
        let field = FieldDefinition {
            field_name: "device_id".to_string(),
            field_type: "Nullable<Text>".to_string(),
            is_index: true,
            joins_with: Some("devices.id".to_string()),
            default_value: None,
        };
        
        let parsed = field.parse().unwrap();
        assert!(parsed.foreign_key.is_some());
        let fk = parsed.foreign_key.unwrap();
        assert_eq!(fk.table, "devices");
        assert_eq!(fk.column, "id");
    }
}