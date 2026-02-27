//! Utility functions and constants for the schema generator
//! This module centralizes common functionality to reduce code duplication

use once_cell::sync::Lazy;
use std::collections::HashMap;

/// Centralized type mappings for field types
pub static DIESEL_TYPE_MAPPINGS: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    let mut map = HashMap::new();

    // Basic types
    map.insert("text()", "Text");
    map.insert("integer()", "Int4");
    map.insert("boolean()", "Bool");
    map.insert("timestamp()", "Timestamp");
    map.insert("timestamptz()", "Timestamptz");
    map.insert("jsonb()", "Jsonb");
    map.insert("inet()", "Inet");
    map.insert("bigint()", "BigInt");
    map.insert("float()", "Float4");
    map.insert("double()", "Float8");

    // Nullable variants
    map.insert("nullable(text())", "Nullable<Text>");
    map.insert("nullable(integer())", "Nullable<Int4>");
    map.insert("nullable(boolean())", "Nullable<Bool>");
    map.insert("nullable(timestamp())", "Nullable<Timestamp>");
    map.insert("nullable(timestamptz())", "Nullable<Timestamptz>");
    map.insert("nullable(jsonb())", "Nullable<Jsonb>");
    map.insert("nullable(inet())", "Nullable<Inet>");
    map.insert("nullable(bigint())", "Nullable<BigInt>");
    map.insert("nullable(float())", "Nullable<Float4>");
    map.insert("nullable(double())", "Nullable<Float8>");

    // Array types
    map.insert("array(text())", "Array<Text>");
    map.insert("nullable(array(text()))", "Nullable<Array<Text>>");

    // VarChar types - basic patterns
    map.insert("varchar()", "Text");
    map.insert("nullable(varchar())", "Nullable<Text>");

    map
});

/// Rust type mappings for model generation
pub static RUST_TYPE_MAPPINGS: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    let mut map = HashMap::new();

    map.insert("Text", "String");
    map.insert("Int4", "i32");
    map.insert("Int8", "i64");
    map.insert("BigInt", "i64");
    map.insert("Bool", "bool");
    map.insert("Timestamp", "chrono::NaiveDateTime");
    map.insert("Timestamptz", "chrono::DateTime<chrono::Utc>");
    map.insert("Jsonb", "serde_json::Value");
    map.insert("Inet", "std::net::IpAddr");
    map.insert("Float4", "f32");
    map.insert("Float8", "f64");

    // VarChar types (now converted to Text)
    // map.insert("Varchar", "String"); // Removed since we convert VarChar to Text

    map
});

/// String manipulation utilities
pub struct StringUtils;

impl StringUtils {
    /// Convert snake_case to PascalCase
    pub fn to_pascal_case(snake_str: &str) -> String {
        snake_str
            .split('_')
            .map(|word| {
                let mut chars = word.chars();
                match chars.next() {
                    None => String::new(),
                    Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
                }
            })
            .collect()
    }
}

/// Field type parsing utilities
pub struct FieldTypeParser;

impl FieldTypeParser {
    /// Parse a diesel field type string and return the corresponding diesel type
    pub fn parse_diesel_type(type_str: &str) -> Result<String, String> {
        let cleaned = type_str.trim();

        // Check direct mappings first
        if let Some(&mapped_type) = DIESEL_TYPE_MAPPINGS.get(cleaned) {
            return Ok(mapped_type.to_string());
        }

        // Handle VarChar with length constraints - convert all to Text
        if cleaned.starts_with("varchar(") {
            return Ok("Text".to_string());
        }

        // Handle complex nested types
        if cleaned.starts_with("nullable(") {
            let inner = Self::extract_inner_type(cleaned, "nullable")?;
            let inner_type = Self::parse_diesel_type(&inner)?;
            return Ok(format!("Nullable<{}>", inner_type));
        }

        if cleaned.starts_with("array(") {
            let inner = Self::extract_inner_type(cleaned, "array")?;
            let inner_type = Self::parse_diesel_type(&inner)?;
            return Ok(format!("Array<{}>", inner_type));
        }

        // Default fallback
        Ok("Text".to_string())
    }

    /// Convert diesel type to rust type for model generation
    pub fn diesel_to_rust_type(diesel_type: &str) -> Result<String, String> {
        // Handle nullable types
        if diesel_type.starts_with("Nullable<") && diesel_type.ends_with(">") {
            let inner = &diesel_type[9..diesel_type.len() - 1];
            let inner_rust = Self::diesel_to_rust_type(inner)?;
            return Ok(format!("Option<{}>", inner_rust));
        }

        // Handle array types
        if diesel_type.starts_with("Array<") && diesel_type.ends_with(">") {
            let inner = &diesel_type[6..diesel_type.len() - 1];
            let inner_rust = Self::diesel_to_rust_type(inner)?;
            return Ok(format!("Vec<{}>", inner_rust));
        }

        // Handle Varchar with length constraints - convert to String
        if diesel_type.starts_with("Varchar<") && diesel_type.ends_with(">") {
            return Ok("String".to_string());
        }

        // Handle standalone Varchar
        if diesel_type == "Varchar" {
            return Ok("String".to_string());
        }

        // Direct mapping
        if let Some(&rust_type) = RUST_TYPE_MAPPINGS.get(diesel_type) {
            Ok(rust_type.to_string())
        } else {
            Err(format!("Unknown diesel type: {}", diesel_type))
        }
    }

    /// Extract inner type from wrapper functions like nullable() or array()
    fn extract_inner_type(type_str: &str, wrapper: &str) -> Result<String, String> {
        let prefix = format!("{}(", wrapper);
        if !type_str.starts_with(&prefix) || !type_str.ends_with(")") {
            return Err(format!("Invalid {} type format: {}", wrapper, type_str));
        }

        let inner = &type_str[prefix.len()..type_str.len() - 1];
        Ok(inner.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::FieldTypeParser;

    #[test]
    fn test_parse_diesel_type_timestamp() {
        assert_eq!(
            FieldTypeParser::parse_diesel_type("timestamp()").unwrap(),
            "Timestamp"
        );
    }

    #[test]
    fn test_parse_diesel_type_timestamptz() {
        assert_eq!(
            FieldTypeParser::parse_diesel_type("timestamptz()").unwrap(),
            "Timestamptz"
        );
    }

    #[test]
    fn test_parse_diesel_type_nullable_timestamp() {
        assert_eq!(
            FieldTypeParser::parse_diesel_type("nullable(timestamp())").unwrap(),
            "Nullable<Timestamp>"
        );
    }

    #[test]
    fn test_parse_diesel_type_nullable_timestamptz() {
        assert_eq!(
            FieldTypeParser::parse_diesel_type("nullable(timestamptz())").unwrap(),
            "Nullable<Timestamptz>"
        );
    }

    #[test]
    fn test_diesel_to_rust_type_timestamp() {
        assert_eq!(
            FieldTypeParser::diesel_to_rust_type("Timestamp").unwrap(),
            "chrono::NaiveDateTime"
        );
    }

    #[test]
    fn test_diesel_to_rust_type_timestamptz() {
        assert_eq!(
            FieldTypeParser::diesel_to_rust_type("Timestamptz").unwrap(),
            "chrono::DateTime<chrono::Utc>"
        );
    }

    #[test]
    fn test_diesel_to_rust_type_nullable_timestamp() {
        assert_eq!(
            FieldTypeParser::diesel_to_rust_type("Nullable<Timestamp>").unwrap(),
            "Option<chrono::NaiveDateTime>"
        );
    }

    #[test]
    fn test_diesel_to_rust_type_nullable_timestamptz() {
        assert_eq!(
            FieldTypeParser::diesel_to_rust_type("Nullable<Timestamptz>").unwrap(),
            "Option<chrono::DateTime<chrono::Utc>>"
        );
    }

    #[test]
    fn test_diesel_to_rust_type_from_raw_timestamp() {
        let diesel = FieldTypeParser::parse_diesel_type("timestamp()").unwrap();
        assert_eq!(
            FieldTypeParser::diesel_to_rust_type(&diesel).unwrap(),
            "chrono::NaiveDateTime"
        );
    }

    #[test]
    fn test_diesel_to_rust_type_from_raw_timestamptz() {
        let diesel = FieldTypeParser::parse_diesel_type("timestamptz()").unwrap();
        assert_eq!(
            FieldTypeParser::diesel_to_rust_type(&diesel).unwrap(),
            "chrono::DateTime<chrono::Utc>"
        );
    }

    #[test]
    fn test_diesel_to_rust_type_from_raw_nullable_timestamptz() {
        let diesel =
            FieldTypeParser::parse_diesel_type("nullable(timestamptz())").unwrap();
        assert_eq!(
            FieldTypeParser::diesel_to_rust_type(&diesel).unwrap(),
            "Option<chrono::DateTime<chrono::Utc>>"
        );
    }
}
