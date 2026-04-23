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
use std::collections::HashSet;

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
            validate_string_value(s)?;

            if is_like_pattern {
                Ok(escape_like_pattern(s))
            } else {
                Ok(escape_string_value(s))
            }
        }
        Value::Number(n) => Ok(n.to_string()),
        Value::Bool(b) => Ok(b.to_string()),
        Value::Null => Ok("NULL".to_string()),
        _ => {
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ThreatCategory {
    ClassicInjection,
    UnionBased,
    StackedQueries,
    CommentInjection,
    BlindBooleanBased,
    BlindTimeBased,
    ErrorBased,
    OutOfBand,
    EncodingEvasion,
    TautologyAttack,
    PiggybackQuery,
    SystemCommandExec,
    InformationDisclosure,
    PrivilegeEscalation,
    DataExfiltration,
    DdlManipulation,
    StoredProcAbuse,
    XmlInjection,
    JsonInjection,
    NoSqlPatterns,
    LdapInjection,
    BatchExecution,
    WildcardAbuse,
    OverflowAttempt,
    NullByteInjection,
    BackslashEvasion,
    AlternateQuoting,
    WhitespaceObfuscation,
    CaseObfuscation,
    KeywordFragmentation,
    HexEncodedPayload,
    CharEncodedPayload,
    DoubleEncoding,
    UnicodeEvasion,
    BufferOverflow,
    DenialOfService,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Severity {
    Low,
    Medium,
    High,
    Critical,
}

impl std::fmt::Display for Severity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Severity::Low => write!(f, "LOW"),
            Severity::Medium => write!(f, "MEDIUM"),
            Severity::High => write!(f, "HIGH"),
            Severity::Critical => write!(f, "CRITICAL"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Threat {
    pub category: ThreatCategory,
    pub severity: Severity,
    pub description: String,
    pub matched_pattern: String,
    pub position: usize,
}

#[derive(Debug, Clone)]
pub struct ScanResult {
    pub is_safe: bool,
    pub threats: Vec<Threat>,
    pub risk_score: u32,
}

impl std::fmt::Display for ScanResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Safe: {}", if self.is_safe { "YES" } else { "NO" })?;
        writeln!(f, "Risk: {}/100", self.risk_score)?;
        writeln!(f, "Threats: {}", self.threats.len())?;
        for (i, t) in self.threats.iter().enumerate() {
            writeln!(
                f,
                "[{}] {} ({}) at {}: {} [{}]",
                i + 1,
                match t.category {
                    ThreatCategory::ClassicInjection => "ClassicInjection",
                    ThreatCategory::UnionBased => "UnionBased",
                    ThreatCategory::StackedQueries => "StackedQueries",
                    ThreatCategory::CommentInjection => "CommentInjection",
                    ThreatCategory::BlindBooleanBased => "BlindBooleanBased",
                    ThreatCategory::BlindTimeBased => "BlindTimeBased",
                    ThreatCategory::ErrorBased => "ErrorBased",
                    ThreatCategory::OutOfBand => "OutOfBand",
                    ThreatCategory::EncodingEvasion => "EncodingEvasion",
                    ThreatCategory::TautologyAttack => "TautologyAttack",
                    ThreatCategory::PiggybackQuery => "PiggybackQuery",
                    ThreatCategory::SystemCommandExec => "SystemCommandExec",
                    ThreatCategory::InformationDisclosure => "InformationDisclosure",
                    ThreatCategory::PrivilegeEscalation => "PrivilegeEscalation",
                    ThreatCategory::DataExfiltration => "DataExfiltration",
                    ThreatCategory::DdlManipulation => "DdlManipulation",
                    ThreatCategory::StoredProcAbuse => "StoredProcAbuse",
                    ThreatCategory::XmlInjection => "XmlInjection",
                    ThreatCategory::JsonInjection => "JsonInjection",
                    ThreatCategory::NoSqlPatterns => "NoSqlPatterns",
                    ThreatCategory::LdapInjection => "LdapInjection",
                    ThreatCategory::BatchExecution => "BatchExecution",
                    ThreatCategory::WildcardAbuse => "WildcardAbuse",
                    ThreatCategory::OverflowAttempt => "OverflowAttempt",
                    ThreatCategory::NullByteInjection => "NullByteInjection",
                    ThreatCategory::BackslashEvasion => "BackslashEvasion",
                    ThreatCategory::AlternateQuoting => "AlternateQuoting",
                    ThreatCategory::WhitespaceObfuscation => "WhitespaceObfuscation",
                    ThreatCategory::CaseObfuscation => "CaseObfuscation",
                    ThreatCategory::KeywordFragmentation => "KeywordFragmentation",
                    ThreatCategory::HexEncodedPayload => "HexEncodedPayload",
                    ThreatCategory::CharEncodedPayload => "CharEncodedPayload",
                    ThreatCategory::DoubleEncoding => "DoubleEncoding",
                    ThreatCategory::UnicodeEvasion => "UnicodeEvasion",
                    ThreatCategory::BufferOverflow => "BufferOverflow",
                    ThreatCategory::DenialOfService => "DenialOfService",
                },
                t.severity,
                t.position,
                t.description,
                t.matched_pattern
            )?;
        }
        Ok(())
    }
}

pub struct SqlInjectionScanner {
    max_input_length: usize,
    dangerous_keywords: HashSet<&'static str>,
    dangerous_functions: HashSet<&'static str>,
}

impl Default for SqlInjectionScanner {
    fn default() -> Self {
        Self::new()
    }
}

impl SqlInjectionScanner {
    pub fn new() -> Self {
        Self {
            max_input_length: 1000,
            dangerous_keywords: Self::build_dangerous_keywords(),
            dangerous_functions: Self::build_dangerous_functions(),
        }
    }

    pub fn with_max_length(mut self, len: usize) -> Self {
        self.max_input_length = len;
        self
    }

    fn build_dangerous_keywords() -> HashSet<&'static str> {
        [
            "select",
            "insert",
            "update",
            "delete",
            "drop",
            "alter",
            "create",
            "truncate",
            "replace",
            "merge",
            "union",
            "except",
            "intersect",
            "grant",
            "revoke",
            "exec",
            "execute",
            "xp_cmdshell",
            "sp_executesql",
            "shutdown",
            "backup",
            "restore",
            "dbcc",
            "bulk",
            "openrowset",
            "opendatasource",
            "openquery",
            "into outfile",
            "into dumpfile",
            "load_file",
            "load data",
            "rename",
            "handler",
            "call",
        ]
        .into_iter()
        .collect()
    }

    fn build_dangerous_functions() -> HashSet<&'static str> {
        [
            "sleep",
            "benchmark",
            "waitfor",
            "delay",
            "pg_sleep",
            "extractvalue",
            "updatexml",
            "xmltype",
            "dbms_pipe",
            "utl_http",
            "utl_inaddr",
            "utl_smtp",
            "utl_file",
            "char",
            "chr",
            "concat",
            "concat_ws",
            "group_concat",
            "substring",
            "substr",
            "mid",
            "ascii",
            "ord",
            "hex",
            "unhex",
            "conv",
            "convert",
            "cast",
            "coalesce",
            "nullif",
            "ifnull",
            "if",
            "elt",
            "field",
            "make_set",
            "export_set",
            "load_file",
            "compress",
            "uncompress",
            "aes_encrypt",
            "aes_decrypt",
            "des_encrypt",
            "des_decrypt",
            "encode",
            "decode",
            "encrypt",
            "md5",
            "sha1",
            "sha2",
            "password",
            "old_password",
            "version",
            "database",
            "schema",
            "user",
            "current_user",
            "session_user",
            "system_user",
            "connection_id",
            "last_insert_id",
            "row_count",
            "found_rows",
            "json_extract",
            "json_set",
            "json_replace",
            "json_remove",
            "regexp",
            "rlike",
            "sounds like",
            "match",
            "against",
            "geometrycollection",
            "multipoint",
            "polygon",
            "multipolygon",
            "linestring",
            "multilinestring",
        ]
        .into_iter()
        .collect()
    }

    pub fn scan(&self, input: &str) -> ScanResult {
        let mut threats: Vec<Threat> = Vec::new();
        let decoded = self.full_decode(input);
        let lower = decoded.to_lowercase();
        let normalized = self.normalize_whitespace(&lower);
        self.check_buffer_overflow(input, &mut threats);
        self.check_null_bytes(input, &mut threats);
        self.check_encoding_evasion(input, &mut threats);
        self.check_unicode_evasion(input, &mut threats);
        self.check_double_encoding(input, &mut threats);
        self.check_comment_injection(&normalized, &mut threats);
        self.check_stacked_queries(&normalized, &mut threats);
        self.check_quote_manipulation(input, &normalized, &mut threats);
        self.check_classic_injection(&normalized, &mut threats);
        self.check_tautologies(&normalized, &mut threats);
        self.check_union_based(&normalized, &mut threats);
        self.check_blind_boolean(&normalized, &mut threats);
        self.check_blind_time_based(&normalized, &mut threats);
        self.check_error_based(&normalized, &mut threats);
        self.check_out_of_band(&normalized, &mut threats);
        self.check_system_command_exec(&normalized, &mut threats);
        self.check_information_disclosure(&normalized, &mut threats);
        self.check_privilege_escalation(&normalized, &mut threats);
        self.check_data_exfiltration(&normalized, &mut threats);
        self.check_ddl_manipulation(&normalized, &mut threats);
        self.check_stored_proc_abuse(&normalized, &mut threats);
        self.check_xml_injection(&normalized, &mut threats);
        self.check_json_injection(&normalized, &mut threats);
        self.check_nosql_patterns(&normalized, &mut threats);
        self.check_ldap_injection(&normalized, &mut threats);
        self.check_batch_execution(&normalized, &mut threats);
        self.check_wildcard_abuse(&normalized, &mut threats);
        self.check_integer_overflow(&normalized, &mut threats);
        self.check_hex_payloads(&normalized, &mut threats);
        self.check_char_encoded_payloads(&normalized, &mut threats);
        self.check_keyword_fragmentation(&normalized, &mut threats);
        self.check_case_obfuscation(input, &mut threats);
        self.check_whitespace_obfuscation(input, &mut threats);
        self.check_alternate_quoting(&normalized, &mut threats);
        self.check_backslash_evasion(input, &mut threats);
        self.check_dangerous_keywords(&normalized, &mut threats);
        self.check_dangerous_functions(&normalized, &mut threats);
        self.check_dos_patterns(&normalized, &mut threats);
        self.check_conditional_injection(&normalized, &mut threats);
        let risk_score = self.calculate_risk_score(&threats);
        ScanResult {
            is_safe: threats.is_empty(),
            threats,
            risk_score,
        }
    }

    fn check_buffer_overflow(&self, input: &str, threats: &mut Vec<Threat>) {
        if input.len() > self.max_input_length {
            threats.push(Threat {
                category: ThreatCategory::BufferOverflow,
                severity: Severity::High,
                description: format!(
                    "Input length {} exceeds max {}",
                    input.len(),
                    self.max_input_length
                ),
                matched_pattern: format!("len={}", input.len()),
                position: 0,
            });
        }
    }

    fn check_null_bytes(&self, input: &str, threats: &mut Vec<Threat>) {
        let patterns = ["\0", "%00", "\\0", "\\x00", "\u{0000}"];
        for p in &patterns {
            if let Some(pos) = input.find(p) {
                threats.push(Threat {
                    category: ThreatCategory::NullByteInjection,
                    severity: Severity::Critical,
                    description: "Null byte injection detected".into(),
                    matched_pattern: p.to_string(),
                    position: pos,
                });
            }
        }
    }

    fn check_encoding_evasion(&self, input: &str, threats: &mut Vec<Threat>) {
        let url_patterns = [
            "%27", "%22", "%3b", "%2d%2d", "%23", "%2f%2a", "%2a%2f", "%3d", "%3c", "%3e", "%28",
            "%29", "%7c", "%60",
        ];
        for p in &url_patterns {
            if input.to_lowercase().contains(p) {
                threats.push(Threat {
                    category: ThreatCategory::EncodingEvasion,
                    severity: Severity::High,
                    description: format!("URL-encoded SQL metacharacter: {}", p),
                    matched_pattern: p.to_string(),
                    position: input.to_lowercase().find(p).unwrap_or(0),
                });
            }
        }
    }

    fn check_unicode_evasion(&self, input: &str, threats: &mut Vec<Threat>) {
        let fullwidth: &[(char, &str)] = &[
            ('\u{FF07}', "fullwidth apostrophe"),
            ('\u{FF02}', "fullwidth quotation"),
            ('\u{FF1B}', "fullwidth semicolon"),
            ('\u{FF0D}', "fullwidth hyphen"),
            ('\u{FF08}', "fullwidth left paren"),
            ('\u{FF09}', "fullwidth right paren"),
            ('\u{FF1D}', "fullwidth equals"),
            ('\u{2018}', "left single curly quote"),
            ('\u{2019}', "right single curly quote"),
            ('\u{201C}', "left double curly quote"),
            ('\u{201D}', "right double curly quote"),
            ('\u{02B9}', "modifier letter prime"),
            ('\u{02BC}', "modifier letter apostrophe"),
        ];
        for (ch, desc) in fullwidth {
            if let Some(pos) = input.find(*ch) {
                threats.push(Threat {
                    category: ThreatCategory::UnicodeEvasion,
                    severity: Severity::High,
                    description: format!("Unicode evasion via {}", desc),
                    matched_pattern: ch.to_string(),
                    position: pos,
                });
            }
        }
    }

    fn check_double_encoding(&self, input: &str, threats: &mut Vec<Threat>) {
        let patterns = ["%2527", "%252d%252d", "%253b", "%2522", "%2523"];
        for p in &patterns {
            if input.to_lowercase().contains(p) {
                threats.push(Threat {
                    category: ThreatCategory::DoubleEncoding,
                    severity: Severity::Critical,
                    description: "Double URL encoding detected (evasion attempt)".into(),
                    matched_pattern: p.to_string(),
                    position: input.to_lowercase().find(p).unwrap_or(0),
                });
            }
        }
    }

    fn check_comment_injection(&self, input: &str, threats: &mut Vec<Threat>) {
        let patterns: &[(&str, &str)] = &[
            ("--", "SQL line comment"),
            ("#", "MySQL line comment"),
            ("/*", "Block comment open"),
            ("*/", "Block comment close"),
            ("/*!", "MySQL conditional comment"),
            ("--%20", "Comment with encoded space"),
            ("--+", "Comment with plus"),
            ("--\t", "Comment with tab"),
            (";--", "Semicolon + comment"),
            ("//", "Double slash comment"),
        ];
        for (p, desc) in patterns {
            if let Some(pos) = input.find(p) {
                threats.push(Threat {
                    category: ThreatCategory::CommentInjection,
                    severity: Severity::High,
                    description: desc.to_string(),
                    matched_pattern: p.to_string(),
                    position: pos,
                });
            }
        }
    }

    fn check_stacked_queries(&self, input: &str, threats: &mut Vec<Threat>) {
        if let Some(pos) = input.find(';') {
            threats.push(Threat {
                category: ThreatCategory::StackedQueries,
                severity: Severity::High,
                description: "Stacked queries via semicolon".into(),
                matched_pattern: ";".into(),
                position: pos,
            });
        }
    }

    fn check_quote_manipulation(&self, raw: &str, normalized: &str, threats: &mut Vec<Threat>) {
        if raw.contains('\'') && normalized.contains(" or ") {
            threats.push(Threat {
                category: ThreatCategory::ClassicInjection,
                severity: Severity::High,
                description: "Quote manipulation with OR".into(),
                matched_pattern: "OR".into(),
                position: raw.find('\'').unwrap_or(0),
            });
        }
    }

    fn check_classic_injection(&self, input: &str, threats: &mut Vec<Threat>) {
        let patterns = [
            " or 1=1",
            "' or '1'='1",
            "\" or \"1\"=\"1",
            " or true",
            " and 1=1",
            " and 'a'='a",
        ];
        for p in &patterns {
            if let Some(pos) = input.find(p) {
                threats.push(Threat {
                    category: ThreatCategory::ClassicInjection,
                    severity: Severity::High,
                    description: "Classic tautology injection".into(),
                    matched_pattern: p.to_string(),
                    position: pos,
                });
            }
        }
    }

    fn check_tautologies(&self, input: &str, threats: &mut Vec<Threat>) {
        let patterns = ["= true", "is true", "like '%'"];
        for p in &patterns {
            if let Some(pos) = input.find(p) {
                threats.push(Threat {
                    category: ThreatCategory::TautologyAttack,
                    severity: Severity::Medium,
                    description: "Always-true condition".into(),
                    matched_pattern: p.to_string(),
                    position: pos,
                });
            }
        }
    }

    fn check_union_based(&self, input: &str, threats: &mut Vec<Threat>) {
        if let Some(pos) = input.find("union") {
            threats.push(Threat {
                category: ThreatCategory::UnionBased,
                severity: Severity::High,
                description: "UNION-based injection".into(),
                matched_pattern: "union".into(),
                position: pos,
            });
        }
    }

    fn check_blind_boolean(&self, input: &str, threats: &mut Vec<Threat>) {
        let patterns = [" and ", " or "];
        for p in &patterns {
            if input.contains(p) && (input.contains("=1") || input.contains("='a'")) {
                threats.push(Threat {
                    category: ThreatCategory::BlindBooleanBased,
                    severity: Severity::Medium,
                    description: "Boolean-based blind injection".into(),
                    matched_pattern: p.to_string(),
                    position: input.find(p).unwrap_or(0),
                });
            }
        }
    }

    fn check_blind_time_based(&self, input: &str, threats: &mut Vec<Threat>) {
        let patterns = ["sleep(", "benchmark(", "waitfor", "pg_sleep("];
        for p in &patterns {
            if let Some(pos) = input.find(p) {
                threats.push(Threat {
                    category: ThreatCategory::BlindTimeBased,
                    severity: Severity::High,
                    description: "Time-based blind injection".into(),
                    matched_pattern: p.to_string(),
                    position: pos,
                });
            }
        }
    }

    fn check_error_based(&self, input: &str, threats: &mut Vec<Threat>) {
        let patterns = ["extractvalue(", "updatexml(", "convert("];
        for p in &patterns {
            if let Some(pos) = input.find(p) {
                threats.push(Threat {
                    category: ThreatCategory::ErrorBased,
                    severity: Severity::High,
                    description: "Error-based injection".into(),
                    matched_pattern: p.to_string(),
                    position: pos,
                });
            }
        }
    }

    fn check_out_of_band(&self, input: &str, threats: &mut Vec<Threat>) {
        let patterns: &[(&str, &str)] = &[
            ("load_file(", "LOAD_FILE OOB"),
            ("into outfile", "INTO OUTFILE"),
            ("into dumpfile", "INTO DUMPFILE"),
            ("utl_http.request", "UTL_HTTP OOB"),
            ("utl_inaddr", "UTL_INADDR OOB"),
            ("dbms_ldap", "DBMS_LDAP OOB"),
            ("httpuritype", "HTTPURITYPE OOB"),
            ("utl_smtp", "UTL_SMTP OOB"),
            ("utl_file", "UTL_FILE OOB"),
            ("xp_dirtree", "MSSQL xp_dirtree OOB"),
            ("master..xp_dirtree", "MSSQL xp_dirtree full path"),
            ("fn_xe_file_target_read", "MSSQL file read"),
            ("copy to program", "PostgreSQL COPY OOB"),
        ];
        for (p, desc) in patterns {
            if let Some(pos) = input.find(p) {
                threats.push(Threat {
                    category: ThreatCategory::OutOfBand,
                    severity: Severity::Critical,
                    description: desc.to_string(),
                    matched_pattern: p.to_string(),
                    position: pos,
                });
            }
        }
    }

    fn check_system_command_exec(&self, input: &str, threats: &mut Vec<Threat>) {
        let patterns = ["xp_cmdshell", "xp_", "sp_executesql", "exec "];
        for p in &patterns {
            if let Some(pos) = input.find(p) {
                threats.push(Threat {
                    category: ThreatCategory::SystemCommandExec,
                    severity: Severity::Critical,
                    description: "System command execution".into(),
                    matched_pattern: p.to_string(),
                    position: pos,
                });
            }
        }
    }

    fn check_information_disclosure(&self, input: &str, threats: &mut Vec<Threat>) {
        let patterns = [
            "information_schema",
            "pg_catalog",
            "mysql.user",
            "sys.tables",
        ];
        for p in &patterns {
            if let Some(pos) = input.find(p) {
                threats.push(Threat {
                    category: ThreatCategory::InformationDisclosure,
                    severity: Severity::Medium,
                    description: "System table access".into(),
                    matched_pattern: p.to_string(),
                    position: pos,
                });
            }
        }
    }

    fn check_privilege_escalation(&self, input: &str, threats: &mut Vec<Threat>) {
        let patterns = ["grant ", "revoke ", "alter user"];
        for p in &patterns {
            if let Some(pos) = input.find(p) {
                threats.push(Threat {
                    category: ThreatCategory::PrivilegeEscalation,
                    severity: Severity::High,
                    description: "Privilege manipulation".into(),
                    matched_pattern: p.to_string(),
                    position: pos,
                });
            }
        }
    }

    fn check_data_exfiltration(&self, input: &str, threats: &mut Vec<Threat>) {
        let patterns = ["into outfile", "into dumpfile"];
        for p in &patterns {
            if let Some(pos) = input.find(p) {
                threats.push(Threat {
                    category: ThreatCategory::DataExfiltration,
                    severity: Severity::Critical,
                    description: "Data exfiltration".into(),
                    matched_pattern: p.to_string(),
                    position: pos,
                });
            }
        }
    }

    fn check_ddl_manipulation(&self, input: &str, threats: &mut Vec<Threat>) {
        let patterns = ["drop ", "alter ", "create ", "truncate "];
        for p in &patterns {
            if let Some(pos) = input.find(p) {
                threats.push(Threat {
                    category: ThreatCategory::DdlManipulation,
                    severity: Severity::High,
                    description: "DDL manipulation".into(),
                    matched_pattern: p.to_string(),
                    position: pos,
                });
            }
        }
    }

    fn check_stored_proc_abuse(&self, input: &str, threats: &mut Vec<Threat>) {
        let patterns: &[(&str, &str)] = &[
            ("exec ", "EXEC stored procedure call"),
            ("execute ", "EXECUTE stored procedure call"),
            ("call ", "CALL stored procedure"),
            ("exec(", "Dynamic EXEC"),
            ("execute(", "Dynamic EXECUTE"),
            ("exec sp_", "System stored procedure"),
            ("exec xp_", "Extended stored procedure"),
        ];
        for (p, desc) in patterns {
            if let Some(pos) = input.find(p) {
                threats.push(Threat {
                    category: ThreatCategory::StoredProcAbuse,
                    severity: Severity::Critical,
                    description: desc.to_string(),
                    matched_pattern: p.to_string(),
                    position: pos,
                });
            }
        }
    }

    fn check_xml_injection(&self, input: &str, threats: &mut Vec<Threat>) {
        let patterns = ["extractvalue(", "xmltype(", "updatexml("];
        for p in &patterns {
            if let Some(pos) = input.find(p) {
                threats.push(Threat {
                    category: ThreatCategory::XmlInjection,
                    severity: Severity::High,
                    description: "XML function abuse".into(),
                    matched_pattern: p.to_string(),
                    position: pos,
                });
            }
        }
    }

    fn check_json_injection(&self, input: &str, threats: &mut Vec<Threat>) {
        let patterns = ["json_extract(", "json_set(", "json_remove("];
        for p in &patterns {
            if let Some(pos) = input.find(p) {
                threats.push(Threat {
                    category: ThreatCategory::JsonInjection,
                    severity: Severity::Medium,
                    description: "JSON function abuse".into(),
                    matched_pattern: p.to_string(),
                    position: pos,
                });
            }
        }
    }

    fn check_nosql_patterns(&self, input: &str, threats: &mut Vec<Threat>) {
        let patterns: &[(&str, &str)] = &[
            ("$gt", "NoSQL $gt operator"),
            ("$lt", "NoSQL $lt operator"),
            ("$ne", "NoSQL $ne operator"),
            ("$eq", "NoSQL $eq operator"),
            ("$in", "NoSQL $in operator"),
            ("$nin", "NoSQL $nin operator"),
            ("$regex", "NoSQL $regex operator"),
            ("$where", "NoSQL $where operator"),
            ("$exists", "NoSQL $exists operator"),
            ("$or", "NoSQL $or operator"),
            ("$and", "NoSQL $and operator"),
            ("$not", "NoSQL $not operator"),
        ];
        for (p, desc) in patterns {
            if let Some(pos) = input.find(p) {
                threats.push(Threat {
                    category: ThreatCategory::NoSqlPatterns,
                    severity: Severity::High,
                    description: desc.to_string(),
                    matched_pattern: p.to_string(),
                    position: pos,
                });
            }
        }
    }

    fn check_ldap_injection(&self, input: &str, threats: &mut Vec<Threat>) {
        let patterns = ["(|", "*)", "(*)", "(&", "!)"];
        for p in &patterns {
            if let Some(pos) = input.find(p) {
                threats.push(Threat {
                    category: ThreatCategory::LdapInjection,
                    severity: Severity::Medium,
                    description: "LDAP special character usage".into(),
                    matched_pattern: p.to_string(),
                    position: pos,
                });
            }
        }
    }

    fn check_batch_execution(&self, input: &str, threats: &mut Vec<Threat>) {
        if let Some(pos) = input.find("go") {
            threats.push(Threat {
                category: ThreatCategory::BatchExecution,
                severity: Severity::Medium,
                description: "Batch execution".into(),
                matched_pattern: "go".into(),
                position: pos,
            });
        }
    }

    fn check_wildcard_abuse(&self, input: &str, threats: &mut Vec<Threat>) {
        if input.contains(" like '%") {
            if let Some(pos) = input.find(" like '%") {
                threats.push(Threat {
                    category: ThreatCategory::WildcardAbuse,
                    severity: Severity::Medium,
                    description: "LIKE '%' abuse".into(),
                    matched_pattern: "like '%'".into(),
                    position: pos,
                });
            }
        }
    }

    fn check_integer_overflow(&self, input: &str, threats: &mut Vec<Threat>) {
        for tok in input.split(|c: char| !c.is_ascii_alphanumeric()) {
            if let Ok(n) = tok.parse::<i128>() {
                if n.abs() > i64::MAX as i128 {
                    threats.push(Threat {
                        category: ThreatCategory::OverflowAttempt,
                        severity: Severity::Medium,
                        description: "Integer overflow attempt".into(),
                        matched_pattern: tok.to_string(),
                        position: input.find(tok).unwrap_or(0),
                    });
                }
            }
        }
    }

    fn check_hex_payloads(&self, input: &str, threats: &mut Vec<Threat>) {
        let patterns = ["0x", "unhex(", "hex("];
        for p in &patterns {
            if let Some(pos) = input.find(p) {
                threats.push(Threat {
                    category: ThreatCategory::HexEncodedPayload,
                    severity: Severity::Medium,
                    description: "Hex encoded payload".into(),
                    matched_pattern: p.to_string(),
                    position: pos,
                });
            }
        }
    }

    fn check_char_encoded_payloads(&self, input: &str, threats: &mut Vec<Threat>) {
        if input.contains("char(") || input.contains("chr(") {
            let pos = input
                .find("char(")
                .or_else(|| input.find("chr("))
                .unwrap_or(0);
            threats.push(Threat {
                category: ThreatCategory::CharEncodedPayload,
                severity: Severity::Medium,
                description: "CHAR()/CHR() encoded payload".into(),
                matched_pattern: "char/chr".into(),
                position: pos,
            });
        }
    }

    fn check_keyword_fragmentation(&self, input: &str, threats: &mut Vec<Threat>) {
        if input.contains("cOncat(") || input.contains("||") {
            threats.push(Threat {
                category: ThreatCategory::KeywordFragmentation,
                severity: Severity::Medium,
                description: "Keyword fragmentation via concat".into(),
                matched_pattern: "concat".into(),
                position: input.find("concat").unwrap_or(0),
            });
        }
    }

    fn check_case_obfuscation(&self, raw: &str, threats: &mut Vec<Threat>) {
        if raw.contains("SeLeCt") || raw.contains("UnIoN") {
            threats.push(Threat {
                category: ThreatCategory::CaseObfuscation,
                severity: Severity::Low,
                description: "Case obfuscation".into(),
                matched_pattern: "mixed-case".into(),
                position: 0,
            });
        }
    }

    fn check_whitespace_obfuscation(&self, raw: &str, threats: &mut Vec<Threat>) {
        if raw.contains("/**/") {
            threats.push(Threat {
                category: ThreatCategory::WhitespaceObfuscation,
                severity: Severity::Low,
                description: "Whitespace obfuscation".into(),
                matched_pattern: "/**/".into(),
                position: raw.find("/**/").unwrap_or(0),
            });
        }
    }

    fn check_alternate_quoting(&self, input: &str, threats: &mut Vec<Threat>) {
        if input.contains("q'") || input.contains("$$") || input.contains('`') {
            threats.push(Threat {
                category: ThreatCategory::AlternateQuoting,
                severity: Severity::Low,
                description: "Alternate quoting".into(),
                matched_pattern: "q'[]'/$$/`".into(),
                position: 0,
            });
        }
    }

    fn check_backslash_evasion(&self, raw: &str, threats: &mut Vec<Threat>) {
        if raw.contains("\\'") {
            threats.push(Threat {
                category: ThreatCategory::BackslashEvasion,
                severity: Severity::Low,
                description: "Backslash quote evasion".into(),
                matched_pattern: "\\'".into(),
                position: raw.find("\\'").unwrap_or(0),
            });
        }
    }

    fn check_dangerous_keywords(&self, input: &str, threats: &mut Vec<Threat>) {
        for kw in &self.dangerous_keywords {
            if let Some(pos) = input.find(kw) {
                threats.push(Threat {
                    category: ThreatCategory::PiggybackQuery,
                    severity: Severity::Medium,
                    description: "Potential query keyword".into(),
                    matched_pattern: kw.to_string(),
                    position: pos,
                });
            }
        }
    }

    fn check_dangerous_functions(&self, input: &str, threats: &mut Vec<Threat>) {
        for f in &self.dangerous_functions {
            if let Some(pos) = input.find(f) {
                threats.push(Threat {
                    category: ThreatCategory::ErrorBased,
                    severity: Severity::Medium,
                    description: "Dangerous function".into(),
                    matched_pattern: f.to_string(),
                    position: pos,
                });
            }
        }
    }

    fn check_dos_patterns(&self, input: &str, threats: &mut Vec<Threat>) {
        let patterns = [
            "benchmark(",
            "sleep(",
            "pg_sleep(",
            "waitfor",
            "generate_series(",
            "repeat(",
            "cross join",
            "natural join",
        ];
        for p in &patterns {
            if let Some(pos) = input.find(p) {
                threats.push(Threat {
                    category: ThreatCategory::DenialOfService,
                    severity: Severity::High,
                    description: format!("Resource-heavy operation: {}", p),
                    matched_pattern: p.to_string(),
                    position: pos,
                });
            }
        }
    }

    fn check_conditional_injection(&self, input: &str, threats: &mut Vec<Threat>) {
        let patterns = [" and ", " or "];
        for p in &patterns {
            if input.contains(p) && (input.contains("select ") || input.contains("union ")) {
                threats.push(Threat {
                    category: ThreatCategory::BlindBooleanBased,
                    severity: Severity::Medium,
                    description: "Conditional injection context".into(),
                    matched_pattern: p.to_string(),
                    position: input.find(p).unwrap_or(0),
                });
            }
        }
    }

    fn full_decode(&self, input: &str) -> String {
        let mut s = input.to_string();
        for _ in 0..3 {
            let decoded = Self::url_decode(&s);
            if decoded == s {
                break;
            }
            s = decoded;
        }
        s
    }

    fn url_decode(input: &str) -> String {
        let mut result = String::with_capacity(input.len());
        let bytes = input.as_bytes();
        let mut i = 0;
        while i < bytes.len() {
            if bytes[i] == b'%' && i + 2 < bytes.len() {
                if let Ok(hex_str) = std::str::from_utf8(&bytes[i + 1..i + 3]) {
                    if let Ok(byte_val) = u8::from_str_radix(hex_str, 16) {
                        result.push(byte_val as char);
                        i += 3;
                        continue;
                    }
                }
            }
            result.push(bytes[i] as char);
            i += 1;
        }
        result
    }

    fn normalize_whitespace(&self, input: &str) -> String {
        let mut result = String::with_capacity(input.len());
        let mut prev_space = false;
        for ch in input.chars() {
            if ch.is_whitespace() {
                if !prev_space {
                    result.push(' ');
                    prev_space = true;
                }
            } else {
                result.push(ch);
                prev_space = false;
            }
        }
        result
    }

    fn calculate_risk_score(&self, threats: &[Threat]) -> u32 {
        let mut score: u32 = 0;
        for t in threats {
            score += match t.severity {
                Severity::Low => 5,
                Severity::Medium => 15,
                Severity::High => 30,
                Severity::Critical => 50,
            };
        }
        score.min(100)
    }
}

pub fn validate_query_safety(sql: &str) -> Result<(), String> {
    let scanner = SqlInjectionScanner::new().with_max_length(MAX_STRING_LENGTH);
    let result = scanner.scan(sql);
    let relevant = result.threats.iter().filter(|t| {
        matches!(
            t.category,
            ThreatCategory::OutOfBand
                | ThreatCategory::SystemCommandExec
                | ThreatCategory::InformationDisclosure
                | ThreatCategory::PrivilegeEscalation
                | ThreatCategory::DataExfiltration
                | ThreatCategory::DdlManipulation
                | ThreatCategory::StoredProcAbuse
                | ThreatCategory::BlindTimeBased
                | ThreatCategory::ErrorBased
                | ThreatCategory::HexEncodedPayload
                | ThreatCategory::CharEncodedPayload
                | ThreatCategory::DoubleEncoding
                | ThreatCategory::UnicodeEvasion
                | ThreatCategory::NullByteInjection
                | ThreatCategory::DenialOfService
        )
    });
    if relevant.count() == 0 {
        Ok(())
    } else {
        Err(format!(
            "Unsafe SQL features detected (risk={}): {}",
            result.risk_score, result
        ))
    }
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

    #[test]
    fn test_validate_query_safety_allows_simple_select() {
        let sql = "SELECT 1 LIMIT 1";
        assert!(validate_query_safety(sql).is_ok());
    }

    #[test]
    fn test_validate_query_safety_rejects_union_select() {
        let sql = "SELECT * FROM users UNION SELECT username, password FROM accounts";
        assert!(validate_query_safety(sql).is_err());
    }

    #[test]
    fn test_validate_query_safety_rejects_time_based() {
        let sql = "SELECT 1 FROM pg_sleep(5)";
        assert!(validate_query_safety(sql).is_err());
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

pub fn normalize_whitespace_outside_strings(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut chars = s.chars().peekable();
    enum State {
        Normal,
        SingleQuote,
        DollarQuote(String),
    }
    let mut state = State::Normal;
    let mut last_space = false;
    while let Some(c) = chars.next() {
        match state {
            State::Normal => {
                if c == '\'' {
                    state = State::SingleQuote;
                    out.push(c);
                    last_space = false;
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
                        out.push('$');
                        out.push_str(&tag);
                        out.push('$');
                        for _ in 0..consumed {
                            chars.next();
                        }
                        state = State::DollarQuote(tag);
                        last_space = false;
                    } else {
                        out.push(c);
                        last_space = false;
                    }
                } else if c.is_whitespace() {
                    if !last_space {
                        out.push(' ');
                        last_space = true;
                    }
                } else {
                    out.push(c);
                    last_space = false;
                }
            }
            State::SingleQuote => {
                out.push(c);
                if c == '\'' {
                    if let Some('\'') = chars.peek() {
                        out.push('\'');
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
                                out.push('$');
                                out.push_str(tag);
                                out.push('$');
                                for _ in 0..(tag.len() + 1) {
                                    chars.next();
                                }
                                state = State::Normal;
                                last_space = false;
                                continue;
                            }
                        }
                    }
                }
                out.push(c);
            }
        }
    }
    out.trim().to_string()
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
