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
                write!(
                    f,
                    "Value exceeds maximum length of {} characters",
                    MAX_STRING_LENGTH
                )
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
        log::warn!(
            "String value exceeds maximum length: {} > {}",
            s.len(),
            MAX_STRING_LENGTH
        );
        return Err(SanitizationError::ValueTooLong);
    }

    // Check for NULL bytes (critical security issue)
    if s.contains('\0') {
        log::warn!("NULL byte detected in value");
        return Err(SanitizationError::NullByteDetected);
    }

    // Check for excessive control characters (potential obfuscation attack)
    let control_count = s
        .chars()
        .filter(|c| c.is_control() && *c != '\n' && *c != '\t' && *c != '\r')
        .count();
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
        .replace("\\", "\\\\") // \ -> \\
        .replace("'", "''"); // ' -> ''

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
        .replace("\\", "\\\\") // \ -> \\
        .replace("%", "\\%") // % -> \%
        .replace("_", "\\_") // _ -> \_
        .replace("'", "''"); // ' -> ''

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
pub fn sanitize_values(
    values: &[Value],
    is_like_pattern: bool,
) -> Result<Vec<String>, SanitizationError> {
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
        assert!(matches!(
            result.unwrap_err(),
            SanitizationError::NullByteDetected
        ));
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
        assert!(matches!(
            result.unwrap_err(),
            SanitizationError::ValueTooLong
        ));
    }

    #[test]
    fn test_multiple_values() {
        let values = vec![json!("test1"), json!("test2' OR '1'='1"), json!(123)];
        let results = sanitize_values(&values, false).unwrap();
        assert_eq!(results, vec!["'test1'", "'test2'' OR ''1''=''1'", "123"]);
    }
}

pub fn strip_strings_and_comments(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut chars = s.chars().peekable();
    enum State {
        Normal,
        SingleQuote,
        LineComment,
        BlockComment,
        DollarQuote(String),
    }
    let mut state = State::Normal;
    while let Some(c) = chars.next() {
        match state {
            State::Normal => {
                if c == '\'' {
                    state = State::SingleQuote;
                } else if c == '$' {
                    let mut tag = String::new();
                    let mut consumed = 0;
                    let mut matched = false;
                    let mut tmp = chars.clone();
                    while let Some(nc) = tmp.next() {
                        consumed += 1;
                        if nc == '$' {
                            matched = true;
                            break;
                        } else if nc.is_ascii_alphanumeric() || nc == '_' {
                            tag.push(nc);
                        } else {
                            matched = false;
                            break;
                        }
                    }
                    if matched {
                        for _ in 0..consumed {
                            chars.next();
                        }
                        state = State::DollarQuote(tag);
                    } else {
                        out.push(c);
                    }
                } else if c == '-' {
                    if let Some('-') = chars.peek() {
                        chars.next();
                        state = State::LineComment;
                    } else {
                        out.push(c);
                    }
                } else if c == '/' {
                    if let Some('*') = chars.peek() {
                        chars.next();
                        state = State::BlockComment;
                    } else {
                        out.push(c);
                    }
                } else {
                    out.push(c);
                }
            }
            State::SingleQuote => {
                if c == '\'' {
                    if let Some('\'') = chars.peek() {
                        chars.next();
                    } else {
                        state = State::Normal;
                    }
                }
            }
            State::LineComment => {
                if c == '\n' {
                    state = State::Normal;
                    out.push(c);
                }
            }
            State::BlockComment => {
                if c == '*' {
                    if let Some('/') = chars.peek() {
                        chars.next();
                        state = State::Normal;
                    }
                }
            }
            State::DollarQuote(ref tag) => {
                if c == '$' {
                    let mut tmp = chars.clone();
                    let mut matched_all = true;
                    for ch in tag.chars() {
                        if let Some(nc) = tmp.next() {
                            if nc != ch {
                                matched_all = false;
                                break;
                            }
                        } else {
                            matched_all = false;
                            break;
                        }
                    }
                    if matched_all {
                        if let Some(nc) = tmp.next() {
                            if nc == '$' {
                                for _ in 0..(tag.len() + 1) {
                                    chars.next();
                                }
                                state = State::Normal;
                                continue;
                            }
                        }
                    }
                }
                if c == '\n' {
                    out.push(c);
                }
            }
        }
    }
    out
}

pub fn contains_dangerous_removal_statements(sql: &str) -> bool {
    let cleaned = strip_strings_and_comments(sql);
    let upper = cleaned.to_uppercase();
    let tokens: Vec<&str> = upper
        .split(|c: char| !c.is_ascii_alphabetic())
        .filter(|t| !t.is_empty())
        .collect();
    tokens
        .iter()
        .any(|t| *t == "DELETE" || *t == "DROP" || *t == "TRUNCATE")
}

pub fn validate_select_limits(sql: &str) -> Result<(), String> {
    let cleaned = strip_strings_and_comments(sql);
    for stmt in cleaned.split(';') {
        let s = stmt.trim();
        if s.is_empty() {
            continue;
        }
        let s_upper = s.to_uppercase();
        if s_upper.contains("SELECT") {
            if s_upper.contains("LIMIT") {
                if let Some(idx) = s_upper.find("LIMIT") {
                    let mut digits = String::new();
                    for c in s_upper[idx + "LIMIT".len()..].chars() {
                        if c.is_ascii_whitespace() {
                            if digits.is_empty() {
                                continue;
                            } else {
                                break;
                            }
                        } else if c.is_ascii_digit() {
                            digits.push(c);
                        } else {
                            break;
                        }
                    }
                    if digits.is_empty() {
                        return Err("SELECT statement LIMIT must be numeric".to_string());
                    }
                    let n: usize = digits
                        .parse()
                        .map_err(|_| "SELECT statement LIMIT must be numeric".to_string())?;
                    if n > 10_000 {
                        return Err("SELECT statement LIMIT exceeds maximum of 10000".to_string());
                    }
                }
            } else if !s_upper.contains("WHERE") {
                return Err("SELECT statement missing LIMIT".to_string());
            }
        }
    }
    Ok(())
}

pub fn validate_update_has_where(sql: &str) -> Result<(), String> {
    let cleaned = strip_strings_and_comments(sql);
    for stmt in cleaned.split(';') {
        let s = stmt.trim();
        if s.is_empty() {
            continue;
        }
        let s_upper = s.to_uppercase();
        if s_upper.contains("UPDATE") && !s_upper.contains("WHERE") {
            return Err("UPDATE statement missing WHERE clause".to_string());
        }
    }
    Ok(())
}

pub fn extract_execute_payloads(sql: &str) -> Vec<String> {
    let mut payloads = Vec::new();
    let mut chars = sql.chars().peekable();
    enum State {
        Normal,
        SingleQuote,
        DollarQuote(String),
        LineComment,
        BlockComment,
    }
    let mut state = State::Normal;
    while let Some(c) = chars.next() {
        match state {
            State::Normal => {
                if c == '\'' {
                    state = State::SingleQuote;
                } else if c == '$' {
                    let mut tag = String::new();
                    let mut consumed = 0;
                    let mut matched = false;
                    let mut tmp = chars.clone();
                    while let Some(nc) = tmp.next() {
                        consumed += 1;
                        if nc == '$' {
                            matched = true;
                            break;
                        } else if nc.is_ascii_alphanumeric() || nc == '_' {
                            tag.push(nc);
                        } else {
                            matched = false;
                            break;
                        }
                    }
                    if matched {
                        for _ in 0..consumed {
                            chars.next();
                        }
                        state = State::DollarQuote(tag);
                    }
                } else if c == '-' {
                    if let Some('-') = chars.peek() {
                        chars.next();
                        state = State::LineComment;
                    }
                } else if c == '/' {
                    if let Some('*') = chars.peek() {
                        chars.next();
                        state = State::BlockComment;
                    }
                } else if c.to_ascii_uppercase() == 'E' {
                    let mut tmp = chars.clone();
                    let mut word = String::new();
                    word.push(c);
                    for _ in 0..6 {
                        if let Some(nc) = tmp.next() {
                            word.push(nc);
                        } else {
                            break;
                        }
                    }
                    if word.to_ascii_uppercase() == "EXECUTE" {
                        for _ in 0..6 {
                            chars.next();
                        }
                        while let Some(ws) = chars.peek() {
                            if ws.is_ascii_whitespace() {
                                chars.next();
                            } else {
                                break;
                            }
                        }
                        if let Some(nextc) = chars.peek().cloned() {
                            if nextc == '\'' {
                                let mut content = String::new();
                                chars.next();
                                while let Some(sc) = chars.next() {
                                    if sc == '\'' {
                                        if let Some('\'') = chars.peek() {
                                            chars.next();
                                            content.push('\'');
                                        } else {
                                            break;
                                        }
                                    } else {
                                        content.push(sc);
                                    }
                                }
                                payloads.push(content);
                            } else if nextc == '$' {
                                chars.next();
                                let mut tag = String::new();
                                while let Some(tc) = chars.peek() {
                                    if *tc == '$' {
                                        chars.next();
                                        break;
                                    } else if tc.is_ascii_alphanumeric() || *tc == '_' {
                                        tag.push(*tc);
                                        chars.next();
                                    } else {
                                        break;
                                    }
                                }
                                let mut content = String::new();
                                loop {
                                    if let Some(dc) = chars.next() {
                                        if dc == '$' {
                                            let mut tmp2 = chars.clone();
                                            let mut matched_all = true;
                                            for ch in tag.chars() {
                                                if let Some(nc) = tmp2.next() {
                                                    if nc != ch {
                                                        matched_all = false;
                                                        break;
                                                    }
                                                } else {
                                                    matched_all = false;
                                                    break;
                                                }
                                            }
                                            if matched_all {
                                                if let Some(nc) = tmp2.next() {
                                                    if nc == '$' {
                                                        for _ in 0..(tag.len() + 1) {
                                                            chars.next();
                                                        }
                                                        break;
                                                    }
                                                }
                                            }
                                            content.push(dc);
                                        } else {
                                            content.push(dc);
                                        }
                                    } else {
                                        break;
                                    }
                                }
                                payloads.push(content);
                            }
                        }
                    }
                }
            }
            State::SingleQuote => {
                if c == '\'' {
                    if let Some('\'') = chars.peek() {
                        chars.next();
                    } else {
                        state = State::Normal;
                    }
                }
            }
            State::DollarQuote(ref tag) => {
                if c == '$' {
                    let mut tmp = chars.clone();
                    let mut matched_all = true;
                    for ch in tag.chars() {
                        if let Some(nc) = tmp.next() {
                            if nc != ch {
                                matched_all = false;
                                break;
                            }
                        } else {
                            matched_all = false;
                            break;
                        }
                    }
                    if matched_all {
                        if let Some(nc) = tmp.next() {
                            if nc == '$' {
                                for _ in 0..(tag.len() + 1) {
                                    chars.next();
                                }
                                state = State::Normal;
                                continue;
                            }
                        }
                    }
                }
            }
            State::LineComment => {
                if c == '\n' {
                    state = State::Normal;
                }
            }
            State::BlockComment => {
                if c == '*' {
                    if let Some('/') = chars.peek() {
                        chars.next();
                        state = State::Normal;
                    }
                }
            }
        }
    }
    payloads
}

pub fn validate_execute_payloads(sql: &str) -> Result<(), String> {
    let payloads = extract_execute_payloads(sql);
    for p in payloads {
        if contains_dangerous_removal_statements(&p) {
            return Err("EXECUTE payload contains potentially destructive statements".to_string());
        }
        if let Err(e) = validate_select_limits(&p) {
            return Err(format!("EXECUTE payload invalid: {}", e));
        }
        if let Err(e) = validate_update_has_where(&p) {
            return Err(format!("EXECUTE payload invalid: {}", e));
        }
    }
    Ok(())
}
