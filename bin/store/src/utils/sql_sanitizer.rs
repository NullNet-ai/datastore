/// SQL Sanitization Module
/// 
/// Provides comprehensive protection against SQL injection attacks by sanitizing
/// user-supplied values in filter queries.
/// 
/// Protects against:
/// - Quote injection (e.g., ' OR '1'='1)
/// - Comment injection (e.g., ' --)
/// - UNION attacks (e.g., ' UNION SELECT password FROM users)
/// - NULL byte injection
/// - LIKE wildcard injection (%, _)
/// - Backslash escaping
/// - Control character injection

use serde_json::Value;

/// Maximum allowed length for string values to prevent DoS
const MAX_STRING_LENGTH: usize = 10_000;

/// Maximum allowed control characters in a string
const MAX_CONTROL_CHARS: usize = 10;

/// Sanitization error types
#[derive(Debug)]
pub enum SanitizationError {
    ValueTooLong,
    NullByteDetected,
    TooManyControlCharacters,
    UnsupportedType,
}

impl std::fmt::Display for SanitizationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SanitizationError::ValueTooLong => {
                write!(f, "Value exceeds maximum length of {} characters", MAX_STRING_LENGTH)
            }
            SanitizationError::NullByteDetected => {
                write!(f, "NULL bytes are not allowed in values")
            }
            SanitizationError::TooManyControlCharacters => {
                write!(f, "Too many control characters detected")
            }
            SanitizationError::UnsupportedType => {
                write!(f, "Unsupported value type")
            }
        }
    }
}

impl std::error::Error for SanitizationError {}

/// Validates a string value for potential SQL injection attacks
/// 
/// Checks for:
/// - NULL bytes (\0)
/// - Excessive control characters
/// - Length limits
fn validate_string_value(s: &str) -> Result<(), SanitizationError> {
    // Check length
    if s.len() > MAX_STRING_LENGTH {
        log::warn!("String value exceeds maximum length: {} > {}", s.len(), MAX_STRING_LENGTH);
        return Err(SanitizationError::ValueTooLong);
    }

    // Check for NULL bytes (critical security issue)
    if s.contains('\0') {
        log::warn!("NULL byte detected in value");
        return Err(SanitizationError::NullByteDetected);
    }

    // Check for excessive control characters (potential obfuscation attack)
    let control_count = s.chars().filter(|c| c.is_control() && *c != '\n' && *c != '\t' && *c != '\r').count();
    if control_count > MAX_CONTROL_CHARS {
        log::warn!("Too many control characters detected: {}", control_count);
        return Err(SanitizationError::TooManyControlCharacters);
    }

    Ok(())
}

/// Escapes a string value for safe use in SQL queries
/// 
/// This function protects against SQL injection by:
/// 1. Escaping backslashes (\\)
/// 2. Escaping single quotes (')
/// 3. Wrapping the value in single quotes
/// 
/// Example:
/// ```
/// let input = "O'Reilly";
/// let escaped = escape_string_value(input);
/// assert_eq!(escaped, "'O''Reilly'");
/// ```
fn escape_string_value(s: &str) -> String {
    // Escape backslashes first (important: must be done before quotes)
    // This prevents attacks like: test\' OR '1'='1
    let escaped = s
        .replace("\\", "\\\\")  // \ -> \\
        .replace("'", "''");    // ' -> ''
    
    format!("'{}'", escaped)
}

/// Escapes a string value for use in LIKE patterns
/// 
/// In addition to regular SQL escaping, this also escapes LIKE wildcards:
/// - % (matches any sequence of characters)
/// - _ (matches any single character)
/// 
/// Example:
/// ```
/// let input = "50%";
/// let escaped = escape_like_pattern(input);
/// assert_eq!(escaped, "'50\\%'");
/// ```
fn escape_like_pattern(s: &str) -> String {
    // Escape backslashes, LIKE wildcards, and quotes
    let escaped = s
        .replace("\\", "\\\\")  // \ -> \\
        .replace("%", "\\%")    // % -> \%
        .replace("_", "\\_")    // _ -> \_
        .replace("'", "''");    // ' -> ''
    
    format!("'{}'", escaped)
}

/// Sanitizes and escapes a JSON value for use in SQL WHERE clauses
/// 
/// This is the main entry point for value sanitization. It handles different
/// JSON value types and applies appropriate escaping.
/// 
/// # Arguments
/// * `value` - The JSON value to sanitize
/// * `is_like_pattern` - Whether this value will be used in a LIKE clause
/// 
/// # Returns
/// * `Ok(String)` - The sanitized and escaped SQL value
/// * `Err(SanitizationError)` - If validation fails
/// 
/// # Examples
/// ```
/// // Regular value
/// let value = serde_json::Value::String("test' OR '1'='1".to_string());
/// let result = sanitize_value(&value, false)?;
/// // Result: 'test'' OR ''1''=''1'
/// 
/// // LIKE pattern
/// let value = serde_json::Value::String("50%".to_string());
/// let result = sanitize_value(&value, true)?;
/// // Result: '50\%'
/// ```
pub fn sanitize_value(value: &Value, is_like_pattern: bool) -> Result<String, SanitizationError> {
    match value {
        Value::String(s) => {
            // Validate the string first
            validate_string_value(s)?;
            
            // Escape based on context
            if is_like_pattern {
                Ok(escape_like_pattern(s))
            } else {
                Ok(escape_string_value(s))
            }
        }
        Value::Number(n) => {
            // Numbers are safe as-is (no quotes needed for numeric literals)
            Ok(n.to_string())
        }
        Value::Bool(b) => {
            // Booleans are safe as-is
            Ok(b.to_string())
        }
        Value::Null => {
            // NULL is a SQL keyword, safe as-is
            Ok("NULL".to_string())
        }
        _ => {
            // Arrays and objects should not be used directly in SQL
            log::warn!("Attempt to sanitize unsupported JSON type: {:?}", value);
            Err(SanitizationError::UnsupportedType)
        }
    }
}

/// Sanitizes multiple values (for use with IN clauses)
/// 
/// # Arguments
/// * `values` - Slice of JSON values to sanitize
/// * `is_like_pattern` - Whether these values will be used in LIKE clauses
/// 
/// # Returns
/// * `Ok(Vec<String>)` - Vector of sanitized SQL values
/// * `Err(SanitizationError)` - If any validation fails
#[allow(dead_code)]
pub fn sanitize_values(values: &[Value], is_like_pattern: bool) -> Result<Vec<String>, SanitizationError> {
    values
        .iter()
        .map(|v| sanitize_value(v, is_like_pattern))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_basic_string_escaping() {
        let value = json!("test");
        let result = sanitize_value(&value, false).unwrap();
        assert_eq!(result, "'test'");
    }

    #[test]
    fn test_quote_injection() {
        let value = json!("admin' OR '1'='1");
        let result = sanitize_value(&value, false).unwrap();
        assert_eq!(result, "'admin'' OR ''1''=''1'");
    }

    #[test]
    fn test_comment_injection() {
        let value = json!("admin' --");
        let result = sanitize_value(&value, false).unwrap();
        assert_eq!(result, "'admin'' --'");
    }

    #[test]
    fn test_union_attack() {
        let value = json!("' UNION SELECT password FROM users --");
        let result = sanitize_value(&value, false).unwrap();
        assert_eq!(result, "''' UNION SELECT password FROM users --'");
    }

    #[test]
    fn test_backslash_escaping() {
        let value = json!("test\\' OR '1'='1");
        let result = sanitize_value(&value, false).unwrap();
        assert_eq!(result, "'test\\\\'' OR ''1''=''1'");
    }

    #[test]
    fn test_null_byte_rejection() {
        let value = json!("test\x00malicious");
        let result = sanitize_value(&value, false);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), SanitizationError::NullByteDetected));
    }

    #[test]
    fn test_like_wildcard_escaping() {
        let value = json!("50%");
        let result = sanitize_value(&value, true).unwrap();
        assert_eq!(result, "'50\\%'");
    }

    #[test]
    fn test_like_underscore_escaping() {
        let value = json!("test_value");
        let result = sanitize_value(&value, true).unwrap();
        assert_eq!(result, "'test\\_value'");
    }

    #[test]
    fn test_number_value() {
        let value = json!(123);
        let result = sanitize_value(&value, false).unwrap();
        assert_eq!(result, "123");
    }

    #[test]
    fn test_boolean_value() {
        let value = json!(true);
        let result = sanitize_value(&value, false).unwrap();
        assert_eq!(result, "true");
    }

    #[test]
    fn test_null_value() {
        let value = json!(null);
        let result = sanitize_value(&value, false).unwrap();
        assert_eq!(result, "NULL");
    }

    #[test]
    fn test_value_too_long() {
        let long_string = "a".repeat(MAX_STRING_LENGTH + 1);
        let value = json!(long_string);
        let result = sanitize_value(&value, false);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), SanitizationError::ValueTooLong));
    }

    #[test]
    fn test_multiple_values() {
        let values = vec![
            json!("test1"),
            json!("test2' OR '1'='1"),
            json!(123),
        ];
        let results = sanitize_values(&values, false).unwrap();
        assert_eq!(results, vec!["'test1'", "'test2'' OR ''1''=''1'", "123"]);
    }
}
