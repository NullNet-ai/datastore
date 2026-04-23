//! Reserved keywords that clash with Diesel, Rust, or SQL identifiers.
//! Add column names here when they cause conflicts; the generators will use
//! `{keyword}_data` as the Rust identifier and keep the original for DB/JSON.

/// Column names that need special handling (Diesel `columns` module, etc.).
/// To add a new reserved keyword, simply add it to this array.
pub const RESERVED_KEYWORDS: &[&str] = &["columns", "box"];

/// Returns true if the column name is a reserved keyword.
pub fn is_reserved(name: &str) -> bool {
    RESERVED_KEYWORDS.contains(&name)
}

/// Returns the Rust identifier to use in schema and model.
/// For reserved keywords: `{name}_data`, otherwise the original name.
pub fn rust_identifier(name: &str) -> String {
    if is_reserved(name) {
        format!("{}_data", name)
    } else {
        name.to_string()
    }
}
