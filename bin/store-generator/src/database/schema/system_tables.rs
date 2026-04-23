//! Reads system tables from the store's system_tables.rs (single source of truth).
//! Parses the store's `SYSTEM_TABLES` const array to extract table names.

use crate::constants::paths::SYSTEM_TABLES_FILE;
use once_cell::sync::Lazy;
use regex::Regex;
use std::collections::HashSet;

static SYSTEM_TABLES_SET: Lazy<HashSet<String>> = Lazy::new(|| {
    let path = SYSTEM_TABLES_FILE.as_str();
    let content = std::fs::read_to_string(path).unwrap_or_else(|e| {
        log::warn!(
            "Could not read system_tables from {}: {}. Using empty set.",
            path,
            e
        );
        String::new()
    });
    parse_system_tables_from_rust(&content)
});

fn parse_system_tables_from_rust(content: &str) -> HashSet<String> {
    // Find the array body: between "= &[" and "];"
    let start = match content.find("= &[") {
        Some(i) => i + 4,
        None => return HashSet::new(),
    };
    let rest = &content[start..];
    let end = rest.find("];").unwrap_or(rest.len());
    let body = &rest[..end];

    let re = Regex::new(r#""([^"]+)""#).unwrap();
    re.captures_iter(body)
        .filter_map(|c| c.get(1))
        .map(|m| m.as_str().to_string())
        .collect()
}

/// Returns true if the table is a system table.
/// Reads from the store's system_tables.rs file.
pub fn is_system_table(table_name: &str) -> bool {
    SYSTEM_TABLES_SET.contains(table_name)
}
