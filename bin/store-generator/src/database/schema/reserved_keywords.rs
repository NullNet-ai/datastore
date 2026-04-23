//! Reads reserved keywords from the store's reserved_keywords.rs (single source of truth).
//! Parses the store's `RESERVED_KEYWORDS` const array to extract keyword names.

use crate::constants::paths::RESERVED_KEYWORDS_FILE;
use once_cell::sync::Lazy;
use regex::Regex;
use std::collections::HashSet;

static RESERVED_KEYWORDS_SET: Lazy<HashSet<String>> = Lazy::new(|| {
    let path = RESERVED_KEYWORDS_FILE.as_str();
    let content = std::fs::read_to_string(path).unwrap_or_else(|e| {
        log::warn!(
            "Could not read reserved_keywords from {}: {}. Using empty set.",
            path,
            e
        );
        String::new()
    });
    parse_reserved_keywords_from_rust(&content)
});

pub(crate) fn parse_reserved_keywords_from_rust(content: &str) -> HashSet<String> {
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

/// Returns true if the column name is a reserved keyword.
/// Reads from the store's reserved_keywords.rs file.
pub fn is_reserved(name: &str) -> bool {
    RESERVED_KEYWORDS_SET.contains(name)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_reserved_keywords_from_rust() {
        let content =
            r#"pub const RESERVED_KEYWORDS: &[&str] = &["columns", "box", "type", "ref"];"#;
        let parsed = parse_reserved_keywords_from_rust(content);
        assert!(parsed.contains("columns"));
        assert!(parsed.contains("box"));
        assert!(parsed.contains("type"));
        assert!(parsed.contains("ref"));
        assert_eq!(parsed.len(), 4);
    }

    #[test]
    fn test_parse_empty_array() {
        let content = r#"pub const RESERVED_KEYWORDS: &[&str] = &[];"#;
        let parsed = parse_reserved_keywords_from_rust(content);
        assert!(parsed.is_empty());
    }

    #[test]
    fn test_parse_malformed_returns_empty() {
        let parsed = parse_reserved_keywords_from_rust("fn main() {}");
        assert!(parsed.is_empty());
    }

    #[test]
    fn test_reads_from_store_file() {
        // Find store path: STORE_DIR, bin/store, ../store (sibling), cwd
        let cwd = std::env::current_dir().unwrap_or_default();
        let candidates: Vec<std::path::PathBuf> = [
            std::env::var("STORE_DIR")
                .ok()
                .map(std::path::PathBuf::from),
            Some(cwd.join("bin/store")),
            cwd.parent().map(|p| p.join("store")), // when in store-generator, ../store
            Some(cwd.clone()),
        ]
        .into_iter()
        .flatten()
        .collect();

        let path = candidates
            .iter()
            .map(|base| base.join("src/database/schema/reserved_keywords.rs"))
            .find(|p| p.exists());

        let path = path.unwrap_or_else(|| {
            panic!(
                "Could not find store reserved_keywords.rs. Tried {:?}. Cwd: {:?}",
                candidates, cwd
            );
        });

        // Must be the store's file, not store-generator's (which has tests that can confuse the parser)
        assert!(
            !path.to_string_lossy().contains("store-generator"),
            "Should read from store, not store-generator. Path: {:?}",
            path
        );

        let content = std::fs::read_to_string(&path).unwrap();
        let parsed = parse_reserved_keywords_from_rust(&content);
        assert!(
            parsed.contains("columns") && parsed.contains("box"),
            "Store reserved_keywords.rs should contain 'columns' and 'box'. Parsed: {:?}",
            parsed
        );
    }
}
