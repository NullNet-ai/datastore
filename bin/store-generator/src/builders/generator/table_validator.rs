//! Validation rules for table definition files.
//!
//! Enforces:
//! - Table names must be plural (or uncountable)
//! - File name must match table name (same entity, different casing)
//! - Index names: idx_{table_name}_{column_name} (or idx_{table}_{col1}_{col2} for composite)
//! - Foreign key names: fk_{table_name}_{column_name}

use crate::utils::helpers::to_singular;
use regex::Regex;
use std::collections::HashSet;

/// Uncountable nouns: same form for singular and plural. These pass plural validation.
fn uncountable_words() -> HashSet<&'static str> {
    [
        // Common uncountable
        "progress",
        "information",
        "advice",
        "equipment",
        "furniture",
        "luggage",
        "baggage",
        "knowledge",
        "news",
        "research",
        "work",
        "homework",
        "software",
        "hardware",
        "data",
        "traffic",
        "weather",
        "music",
        "art",
        "feedback",
        "metadata",
        "analytics",
        "logistics",
        "content",
        "feedback",
        "series",
        "species",
        "aircraft",
        "headquarters",
        "premises",
        "personnel",
        "staff",
        "equipment",
        "machinery",
        "clothing",
        "jewelry",
        "jewellery",
        "footwear",
        "underwear",
        "mail",
        "correspondence",
        "stationery",
        "stationary",
        "merchandise",
        "inventory",
        "livestock",
        "wildlife",
        "foliage",
        "scenery",
        "evidence",
        "documentation",
        "media",
        "currency",
        "capital",
        "revenue",
        "income",
        "expenditure",
        "debt",
        "wealth",
        "real_estate",
        "legislation",
        "parliament",
        "vocabulary",
        "grammar",
        "jargon",
        "slang",
        "rubbish",
        "garbage",
        "trash",
        "pollution",
        "energy",
        "electricity",
        "power",
        "gas",
        "oil",
        "petrol",
        "water",
        "rice",
        "bread",
        "cheese",
        "meat",
        "fish",
        "salmon",
        "trout",
        "cod",
        "sugar",
        "salt",
        "pepper",
        "flour",
        "sand",
        "dust",
        "dirt",
        "mud",
        "grass",
        "hair",
        "glass",
        "iron",
        "steel",
        "cotton",
        "wool",
        "silk",
        "leather",
        "plastic",
        "rubber",
        "wood",
        "concrete",
        "cement",
        "cash",
        "change",
    ]
    .into_iter()
    .collect()
}

/// Check if a word is uncountable (same form for singular and plural).
fn is_uncountable(word: &str) -> bool {
    uncountable_words().contains(word.to_lowercase().as_str())
}

/// Result of table validation - collects all errors before aborting.
#[derive(Debug, Default)]
pub struct ValidationResult {
    pub errors: Vec<String>,
}

impl ValidationResult {
    pub fn new() -> Self {
        Self { errors: Vec::new() }
    }

    pub fn add_error(&mut self, msg: impl Into<String>) {
        self.errors.push(msg.into());
    }

    pub fn is_valid(&self) -> bool {
        self.errors.is_empty()
    }

    /// Consume and return. If invalid, returns Err with all messages joined.
    pub fn into_result(self) -> Result<(), String> {
        if self.is_valid() {
            Ok(())
        } else {
            Err(self.errors.join("\n"))
        }
    }
}

/// Validate that a table name is plural or uncountable.
/// - Plural: singularize(name) != name
/// - Uncountable: words with same singular/plural form (progress, information, etc.)
pub fn is_plural(table_name: &str) -> bool {
    if is_uncountable(table_name) {
        return true;
    }
    // Check last segment for compound names (e.g. "user_progress" -> "progress")
    if let Some(last) = table_name.split('_').last() {
        if is_uncountable(last) {
            return true;
        }
    }
    let singular = to_singular(table_name);
    singular != table_name
}

/// Normalize file stem to table name format (snake_case).
/// Removes _struct, _table, _catalog suffixes.
pub fn normalize_file_stem_to_table_name(file_stem: &str) -> String {
    file_stem
        .replace("_struct", "")
        .replace("_table", "")
        .replace("_catalog", "")
}

/// Validate that file stem and table name from content refer to the same entity.
/// Both should be snake_case and match (e.g. "test_products" == "test_products").
pub fn validate_file_name_matches_table(
    file_stem: &str,
    table_name_from_content: &str,
) -> Result<(), String> {
    let normalized_file = normalize_file_stem_to_table_name(file_stem);
    let table_snake = table_name_from_content.to_lowercase().replace("-", "_");

    if normalized_file != table_snake {
        return Err(format!(
            "File name '{}' does not match table name '{}'. File stem (without .rs) must match table name in snake_case. Expected file: {}.rs",
            file_stem,
            table_name_from_content,
            table_snake
        ));
    }
    Ok(())
}

/// Validate table name is plural.
pub fn validate_table_name_plural(table_name: &str) -> Result<(), String> {
    if !is_plural(table_name) {
        return Err(format!(
            "Table name '{}' must be plural (e.g. 'test_products' not 'test_product', 'demo_items' not 'demo_item')",
            table_name
        ));
    }
    Ok(())
}

/// Expected index name: idx_{table_name}_{column_names_joined}.
/// For single column: idx_{table}_{column}
/// For composite: idx_{table}_{col1}_{col2}
pub fn expected_index_name(table_name: &str, columns: &[String]) -> String {
    let cols = columns.join("_");
    format!("idx_{}_{}", table_name, cols)
}

/// Validate index name follows idx_{table_name}_{column_name(s)}.
pub fn validate_index_name(
    index_name: &str,
    table_name: &str,
    columns: &[String],
) -> Result<(), String> {
    let expected = expected_index_name(table_name, columns);
    if index_name != expected {
        return Err(format!(
            "Index '{}' must follow format idx_{{table_name}}_{{column_name}}. Expected: '{}'",
            index_name, expected
        ));
    }
    Ok(())
}

/// Expected foreign key name: fk_{table_name}_{column_name}.
pub fn expected_foreign_key_name(table_name: &str, column_name: &str) -> String {
    format!("fk_{}_{}", table_name, column_name)
}

/// Validate foreign key constraint name follows fk_{table_name}_{column_name}.
pub fn validate_foreign_key_name(
    constraint_name: &str,
    table_name: &str,
    column_name: &str,
) -> Result<(), String> {
    let expected = expected_foreign_key_name(table_name, column_name);
    if constraint_name != expected {
        return Err(format!(
            "Foreign key '{}' must follow format fk_{{table_name}}_{{column_name}}. Expected: '{}'",
            constraint_name, expected
        ));
    }
    Ok(())
}

/// Extract table name from system_indexes!("table_name") or system_foreign_keys!("table_name").
pub fn extract_table_name_from_content(content: &str) -> Option<String> {
    // system_indexes!("demo_items")
    let re_indexes = Regex::new(r#"system_indexes!\("([^"]+)"\)"#).ok()?;
    if let Some(cap) = re_indexes.captures(content) {
        return Some(cap.get(1)?.as_str().to_string());
    }
    // system_foreign_keys!("demo_items")
    let re_fk = Regex::new(r#"system_foreign_keys!\("([^"]+)"\)"#).ok()?;
    if let Some(cap) = re_fk.captures(content) {
        return Some(cap.get(1)?.as_str().to_string());
    }
    None
}

/// System FK columns (from system_foreign_keys! macro).
/// User must not redefine these - validation errors if duplicate FK for these columns.
const SYSTEM_FK_COLUMNS: &[&str] = &[
    "organization_id",
    "created_by",
    "updated_by",
    "deleted_by",
    "requested_by",
];

/// System field names (from system_fields! macro).
/// User must not redefine these - validation errors on duplicate field names.
const SYSTEM_FIELD_NAMES: &[&str] = &[
    "tombstone",
    "status",
    "previous_status",
    "version",
    "created_date",
    "created_time",
    "updated_date",
    "updated_time",
    "organization_id",
    "created_by",
    "updated_by",
    "deleted_by",
    "requested_by",
    "timestamp",
    "tags",
    "categories",
    "code",
    "id",
    "sensitivity_level",
    "sync_status",
    "is_batch",
    "image_url",
];

/// Validate no duplicate field names. Errors if user redefines a system field or has duplicate fields.
pub fn validate_no_duplicate_fields(field_names: &[String]) -> Result<(), String> {
    let mut seen = std::collections::HashSet::new();
    for name in field_names {
        if !seen.insert(name) {
            return Err(format!(
                "Duplicate field name '{}'. Remove the duplicate or do not redefine system fields ({}).",
                name,
                SYSTEM_FIELD_NAMES.join(", ")
            ));
        }
    }
    Ok(())
}

/// Index type from generator (name, columns, unique, type, where).
pub type TableFileIndex = (
    String,
    Vec<String>,
    bool,
    Option<String>,
    Option<crate::builders::generator::diesel_schema_definition::WhereExpr>,
);

/// Validate no duplicate index names.
pub fn validate_no_duplicate_index_names(indexes: &[TableFileIndex]) -> Result<(), String> {
    let mut seen = std::collections::HashSet::new();
    for (name, _, _, _, _) in indexes {
        if !seen.insert(name) {
            return Err(format!(
                "Duplicate index name '{}'. Remove the duplicate - system_indexes! already defines indexes for system columns.",
                name
            ));
        }
    }
    Ok(())
}

/// Validate no duplicate foreign key names or columns.
/// Errors if user redefines a system FK (organization_id, created_by, etc.) or has duplicate FKs.
pub fn validate_no_duplicate_foreign_keys(fk_names: &[(String, String)]) -> Result<(), String> {
    let mut seen_names = std::collections::HashSet::new();
    let mut seen_system_columns = std::collections::HashSet::new();

    for (name, column) in fk_names {
        if !seen_names.insert(name) {
            return Err(format!(
                "Duplicate foreign key constraint '{}'. Remove the duplicate - system_foreign_keys! may already define it.",
                name
            ));
        }
        if SYSTEM_FK_COLUMNS.contains(&column.as_str()) {
            if !seen_system_columns.insert(column) {
                return Err(format!(
                    "Duplicate foreign key for column '{}'. This column is already defined in system_foreign_keys!. Remove the duplicate.",
                    column
                ));
            }
        }
    }
    Ok(())
}

/// Run all table validations. Aborts process on first error (returns Err).
pub fn validate_table_file(
    file_stem: &str,
    content: &str,
    table_name: &str,
    field_names: &[String],
    indexes: &[TableFileIndex],
    foreign_keys: &[(String, String)], // (constraint_name, column)
) -> Result<(), String> {
    let mut result = ValidationResult::new();

    // 1. Table name must be plural
    if let Err(e) = validate_table_name_plural(table_name) {
        result.add_error(e);
    }

    // 2. File name must match table name
    let table_from_content =
        extract_table_name_from_content(content).unwrap_or_else(|| table_name.to_string());
    if let Err(e) = validate_file_name_matches_table(file_stem, &table_from_content) {
        result.add_error(e);
    }

    // 3. No duplicate field names (user must not redefine system fields)
    if let Err(e) = validate_no_duplicate_fields(field_names) {
        result.add_error(e);
    }

    // 4. No duplicate index names (user must not redefine system indexes)
    if let Err(e) = validate_no_duplicate_index_names(indexes) {
        result.add_error(e);
    }

    // 5. No duplicate foreign key names or columns (user must not redefine system FKs)
    if let Err(e) = validate_no_duplicate_foreign_keys(foreign_keys) {
        result.add_error(e);
    }

    // 6. Validate index names and that index/where columns exist in the table
    let field_set: std::collections::HashSet<&str> =
        field_names.iter().map(String::as_str).collect();
    for (index_name, columns, _, _, where_clause) in indexes {
        if columns.is_empty() {
            result.add_error(format!("Index '{}' has no columns", index_name));
        } else {
            if let Err(e) = validate_index_name(index_name, table_name, columns) {
                result.add_error(e);
            }
            for col in columns {
                if !field_set.contains(col.as_str()) {
                    result.add_error(format!(
                        "Index '{}' references column '{}' which is not found in table '{}'. Table columns: {}",
                        index_name,
                        col,
                        table_name,
                        field_names.join(", ")
                    ));
                }
            }
            if let Some(ref w) = where_clause {
                for col in w.column_names() {
                    if !field_set.contains(col.as_str()) {
                        result.add_error(format!(
                            "Index '{}' where clause references column '{}' which is not found in table '{}'. Table columns: {}",
                            index_name,
                            col,
                            table_name,
                            field_names.join(", ")
                        ));
                    }
                }
            }
        }
    }

    // 7. Validate foreign key names format (skip system FKs - they use different format from macro)
    for (constraint_name, column) in foreign_keys {
        if SYSTEM_FK_COLUMNS.contains(&column.as_str()) {
            continue;
        }
        if let Err(e) = validate_foreign_key_name(constraint_name, table_name, column) {
            result.add_error(e);
        }
    }

    result.into_result()
}

/// Validates JSONB column default value format.
/// - If a default is set, it must include `::jsonb` (e.g. `default: "'[]'::jsonb"` or `default: "'{\"k\":1}'::jsonb"`).
/// - Empty array default is not allowed: omit default (array is empty by default when no value).
pub fn validate_jsonb_default(field_name: &str, default_value: Option<&str>) -> Result<(), String> {
    let d = match default_value {
        None => return Ok(()),
        Some(d) => d.trim(),
    };
    if d.is_empty() {
        return Ok(());
    }
    // JSONB default must include ::jsonb
    if !d.contains("::jsonb") {
        return Err(format!(
            "JSONB column '{}' default must use format with '::jsonb' (e.g. default: \"'[]'::jsonb\"). Got: {}",
            field_name, d
        ));
    }
    // Do not allow default empty array — omit default (array is empty by default when no value)
    let before_cast = d
        .split("::jsonb")
        .next()
        .unwrap_or("")
        .trim()
        .trim_matches('\'')
        .trim();
    if before_cast == "[]" {
        return Err(format!(
            "JSONB column '{}' must not have default empty array ('[]'::jsonb); omit default (array is empty by default when no value).",
            field_name
        ));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_plural() {
        assert!(is_plural("test_products"));
        assert!(is_plural("demo_items"));
        assert!(is_plural("classroom_courses"));
        assert!(is_plural("organizations"));
        assert!(is_plural("files"));
        assert!(!is_plural("test_product"));
        assert!(!is_plural("demo_item"));
        assert!(!is_plural("organization"));
    }

    #[test]
    fn test_uncountable_words_pass_plural_validation() {
        assert!(is_plural("progress"));
        assert!(is_plural("information"));
        assert!(is_plural("data"));
        assert!(is_plural("equipment"));
        assert!(is_plural("software"));
        assert!(is_plural("feedback"));
        assert!(is_plural("metadata"));
        assert!(is_plural("analytics"));
        assert!(is_plural("research"));
        assert!(is_plural("user_progress"));
        assert!(is_plural("project_information"));
        assert!(is_plural("system_data"));
        assert!(validate_table_name_plural("progress").is_ok());
        assert!(validate_table_name_plural("user_progress").is_ok());
        assert!(validate_table_name_plural("feedback").is_ok());
    }

    #[test]
    fn test_normalize_file_stem() {
        assert_eq!(
            normalize_file_stem_to_table_name("test_products"),
            "test_products"
        );
        assert_eq!(
            normalize_file_stem_to_table_name("demo_items_table"),
            "demo_items"
        );
        assert_eq!(
            normalize_file_stem_to_table_name("samples_struct"),
            "samples"
        );
    }

    #[test]
    fn test_validate_file_name_matches_table_ok() {
        assert!(validate_file_name_matches_table("demo_items", "demo_items").is_ok());
        assert!(validate_file_name_matches_table("test_products", "test_products").is_ok());
    }

    #[test]
    fn test_validate_file_name_matches_table_err() {
        assert!(validate_file_name_matches_table("demo_item", "demo_items").is_err());
        assert!(validate_file_name_matches_table("test_products", "test_product").is_err());
    }

    #[test]
    fn test_validate_table_name_plural_ok() {
        assert!(validate_table_name_plural("test_products").is_ok());
        assert!(validate_table_name_plural("demo_items").is_ok());
    }

    #[test]
    fn test_validate_table_name_plural_err() {
        assert!(validate_table_name_plural("test_product").is_err());
        assert!(validate_table_name_plural("demo_item").is_err());
    }

    #[test]
    fn test_expected_index_name() {
        assert_eq!(
            expected_index_name("demo_items", &["title".to_string()]),
            "idx_demo_items_title"
        );
        assert_eq!(
            expected_index_name("classroom_courses", &["classroom_id".to_string()]),
            "idx_classroom_courses_classroom_id"
        );
        assert_eq!(
            expected_index_name("samples", &["col1".to_string(), "col2".to_string()]),
            "idx_samples_col1_col2"
        );
    }

    #[test]
    fn test_validate_index_name_ok() {
        assert!(
            validate_index_name("idx_demo_items_title", "demo_items", &["title".to_string()])
                .is_ok()
        );
        assert!(validate_index_name(
            "idx_classroom_courses_classroom_id",
            "classroom_courses",
            &["classroom_id".to_string()]
        )
        .is_ok());
    }

    #[test]
    fn test_validate_index_name_err() {
        assert!(
            validate_index_name("idx_demo_items_title", "demo_items", &["name".to_string()])
                .is_err()
        );
        assert!(validate_index_name("wrong_format", "demo_items", &["title".to_string()]).is_err());
        assert!(
            validate_index_name("idx_demo_item_title", "demo_items", &["title".to_string()])
                .is_err()
        );
    }

    #[test]
    fn test_expected_foreign_key_name() {
        assert_eq!(
            expected_foreign_key_name("classroom_courses", "classroom_id"),
            "fk_classroom_courses_classroom_id"
        );
        assert_eq!(
            expected_foreign_key_name("samples", "sample_with_reference_id"),
            "fk_samples_sample_with_reference_id"
        );
    }

    #[test]
    fn test_validate_foreign_key_name_ok() {
        assert!(validate_foreign_key_name(
            "fk_classroom_courses_classroom_id",
            "classroom_courses",
            "classroom_id"
        )
        .is_ok());
        assert!(validate_foreign_key_name(
            "fk_samples_sample_with_reference_id",
            "samples",
            "sample_with_reference_id"
        )
        .is_ok());
    }

    #[test]
    fn test_validate_foreign_key_name_err() {
        assert!(validate_foreign_key_name(
            "fk_classroom_courses_classroom_id",
            "classroom_courses",
            "other_column"
        )
        .is_err());
        assert!(
            validate_foreign_key_name("wrong_format", "samples", "sample_with_reference_id")
                .is_err()
        );
    }

    #[test]
    fn test_extract_table_name_from_content() {
        let content = r#"
        system_indexes!("demo_items"),
        something else
        "#;
        assert_eq!(
            extract_table_name_from_content(content),
            Some("demo_items".to_string())
        );

        let content2 = r#"system_foreign_keys!("test_products")"#;
        assert_eq!(
            extract_table_name_from_content(content2),
            Some("test_products".to_string())
        );

        let content3 = "no macro here";
        assert_eq!(extract_table_name_from_content(content3), None);
    }

    #[test]
    fn test_validate_table_file_success() {
        let content = r#"
        define_table_schema! {
            fields: { system_fields!() },
            indexes: { system_indexes!("demo_items"), idx_demo_items_title: { columns: ["title"], unique: false, type: "btree" } },
            foreign_keys: { system_foreign_keys!("demo_items") }
        }
        "#;
        let field_names = vec!["id".to_string(), "title".to_string()];
        let indexes = vec![(
            "idx_demo_items_title".to_string(),
            vec!["title".to_string()],
            false,
            Some("btree".to_string()),
            None,
        )];
        let foreign_keys: Vec<(String, String)> = vec![];
        assert!(validate_table_file(
            "demo_items",
            content,
            "demo_items",
            &field_names,
            &indexes,
            &foreign_keys
        )
        .is_ok());
    }

    #[test]
    fn test_validate_table_file_plural_fail() {
        let content = r#"system_indexes!("demo_item")"#;
        let field_names: Vec<String> = vec![];
        let indexes: Vec<TableFileIndex> = vec![];
        let foreign_keys: Vec<(String, String)> = vec![];
        let result = validate_table_file(
            "demo_item",
            content,
            "demo_item",
            &field_names,
            &indexes,
            &foreign_keys,
        );
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("must be plural"));
    }

    #[test]
    fn test_validate_table_file_filename_mismatch() {
        let content = r#"system_indexes!("demo_items")"#;
        let field_names: Vec<String> = vec![];
        let indexes: Vec<TableFileIndex> = vec![];
        let foreign_keys: Vec<(String, String)> = vec![];
        let result = validate_table_file(
            "wrong_name",
            content,
            "demo_items",
            &field_names,
            &indexes,
            &foreign_keys,
        );
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("does not match"));
    }

    #[test]
    fn test_validate_table_file_index_name_fail() {
        let content = r#"system_indexes!("demo_items")"#;
        let field_names = vec!["id".to_string(), "title".to_string()];
        let indexes = vec![(
            "wrong_index_name".to_string(),
            vec!["title".to_string()],
            false,
            None,
            None,
        )];
        let foreign_keys: Vec<(String, String)> = vec![];
        let result = validate_table_file(
            "demo_items",
            content,
            "demo_items",
            &field_names,
            &indexes,
            &foreign_keys,
        );
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Index"));
    }

    #[test]
    fn test_validate_table_file_fk_name_fail() {
        let content = r#"system_foreign_keys!("samples")"#;
        let field_names = vec!["id".to_string(), "sample_with_reference_id".to_string()];
        let indexes: Vec<TableFileIndex> = vec![];
        let foreign_keys = vec![(
            "wrong_fk_name".to_string(),
            "sample_with_reference_id".to_string(),
        )];
        let result = validate_table_file(
            "samples",
            content,
            "samples",
            &field_names,
            &indexes,
            &foreign_keys,
        );
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Foreign key"));
    }

    #[test]
    fn test_validate_table_file_skips_system_fk_columns() {
        let content = r#"system_foreign_keys!("demo_items")"#;
        let field_names = vec!["id".to_string(), "organization_id".to_string()];
        let indexes: Vec<TableFileIndex> = vec![];
        // System FK columns use different format - we skip format validation for them
        let foreign_keys = vec![(
            "demo_items_organization_id_organizations_id_fk".to_string(),
            "organization_id".to_string(),
        )];
        // Should pass - we skip format validation for system columns (organization_id)
        let result = validate_table_file(
            "demo_items",
            content,
            "demo_items",
            &field_names,
            &indexes,
            &foreign_keys,
        );
        assert!(result.is_ok());
    }

    /// Partial index with where clause: when table has the where-clause column, validation passes.
    #[test]
    fn test_validate_index_with_where_clause_columns_found() {
        use crate::builders::generator::diesel_schema_definition::WhereExpr;

        let content = r#"system_indexes!("school_admins")"#;
        let field_names = vec![
            "id".to_string(),
            "school_id".to_string(),
            "school_admin_id".to_string(),
            "status".to_string(),
        ];
        let where_clause = Some(WhereExpr::Pred {
            op: "=".to_string(),
            column: "status".to_string(),
            value: Some(serde_json::Value::String("Active".to_string())),
        });
        let indexes: Vec<TableFileIndex> = vec![(
            "idx_school_admins_school_id_school_admin_id".to_string(),
            vec!["school_id".to_string(), "school_admin_id".to_string()],
            true,
            Some("btree".to_string()),
            where_clause,
        )];
        let foreign_keys: Vec<(String, String)> = vec![];

        let result = validate_table_file(
            "school_admins",
            content,
            "school_admins",
            &field_names,
            &indexes,
            &foreign_keys,
        );
        assert!(result.is_ok(), "expected ok when table has status column, got: {:?}", result);
    }

    /// Partial index with where clause: when table does not have the where-clause column, validation fails.
    #[test]
    fn test_validate_index_with_where_clause_column_not_found() {
        use crate::builders::generator::diesel_schema_definition::WhereExpr;

        let content = r#"system_indexes!("school_admins")"#;
        // Table has school_id, school_admin_id but NOT "status"
        let field_names = vec![
            "id".to_string(),
            "school_id".to_string(),
            "school_admin_id".to_string(),
        ];
        let where_clause = Some(WhereExpr::Pred {
            op: "=".to_string(),
            column: "status".to_string(),
            value: Some(serde_json::Value::String("Active".to_string())),
        });
        let indexes: Vec<TableFileIndex> = vec![(
            "idx_school_admins_school_id_school_admin_id".to_string(),
            vec!["school_id".to_string(), "school_admin_id".to_string()],
            true,
            Some("btree".to_string()),
            where_clause,
        )];
        let foreign_keys: Vec<(String, String)> = vec![];

        let result = validate_table_file(
            "school_admins",
            content,
            "school_admins",
            &field_names,
            &indexes,
            &foreign_keys,
        );
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(
            err.contains("not found") && err.contains("status"),
            "error should mention column 'status' not found, got: {}",
            err
        );
    }

    /// Index with where clause: index column not in table fails.
    #[test]
    fn test_validate_index_column_not_found() {
        let content = r#"system_indexes!("school_admins")"#;
        let field_names = vec!["id".to_string(), "school_id".to_string()]; // no school_admin_id
        let indexes: Vec<TableFileIndex> = vec![(
            "idx_school_admins_school_id_school_admin_id".to_string(),
            vec!["school_id".to_string(), "school_admin_id".to_string()],
            true,
            Some("btree".to_string()),
            None,
        )];
        let foreign_keys: Vec<(String, String)> = vec![];

        let result = validate_table_file(
            "school_admins",
            content,
            "school_admins",
            &field_names,
            &indexes,
            &foreign_keys,
        );
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(
            err.contains("not found") && err.contains("school_admin_id"),
            "error should mention column 'school_admin_id' not found, got: {}",
            err
        );
    }

    #[test]
    fn test_validate_no_duplicate_fields() {
        let unique = vec!["a".to_string(), "b".to_string()];
        assert!(validate_no_duplicate_fields(&unique).is_ok());
        let dup = vec!["a".to_string(), "b".to_string(), "a".to_string()];
        let err = validate_no_duplicate_fields(&dup).unwrap_err();
        assert!(err.contains("Duplicate field"));
        assert!(err.contains("a"));
    }

    #[test]
    fn test_validate_no_duplicate_index_names() {
        let indexes = vec![
            (
                "idx_t_c1".to_string(),
                vec!["c1".to_string()],
                false,
                None,
                None,
            ),
            (
                "idx_t_c2".to_string(),
                vec!["c2".to_string()],
                false,
                None,
                None,
            ),
        ];
        assert!(validate_no_duplicate_index_names(&indexes).is_ok());
        let dup_indexes = vec![
            (
                "idx_t_c1".to_string(),
                vec!["c1".to_string()],
                false,
                None,
                None,
            ),
            (
                "idx_t_c1".to_string(),
                vec!["c1".to_string()],
                false,
                None,
                None,
            ),
        ];
        let err = validate_no_duplicate_index_names(&dup_indexes).unwrap_err();
        assert!(err.contains("Duplicate index"));
    }

    #[test]
    fn test_validate_no_duplicate_foreign_keys() {
        let fks = vec![
            ("fk_t_col1".to_string(), "col1".to_string()),
            ("fk_t_col2".to_string(), "col2".to_string()),
        ];
        assert!(validate_no_duplicate_foreign_keys(&fks).is_ok());
        let dup_fks = vec![
            ("fk_t_org".to_string(), "organization_id".to_string()),
            ("fk_t_org2".to_string(), "organization_id".to_string()),
        ];
        let err = validate_no_duplicate_foreign_keys(&dup_fks).unwrap_err();
        assert!(err.contains("Duplicate foreign key"));
    }

    #[test]
    fn test_validate_jsonb_default_ok_no_default() {
        assert!(validate_jsonb_default("tags", None).is_ok());
        assert!(validate_jsonb_default("meta", Some("")).is_ok());
    }

    #[test]
    fn test_validate_jsonb_default_ok_with_cast() {
        assert!(validate_jsonb_default("tags", Some("'[1,2]'::jsonb")).is_ok());
        assert!(validate_jsonb_default("data", Some("'{\"a\": 1}'::jsonb")).is_ok());
        assert!(validate_jsonb_default("prefs", Some("'{}'::jsonb")).is_ok());
    }

    #[test]
    fn test_validate_jsonb_default_err_missing_cast() {
        let err = validate_jsonb_default("tags", Some("'[]'")).unwrap_err();
        assert!(
            err.contains("::jsonb"),
            "error should require ::jsonb: {}",
            err
        );
        assert!(
            err.contains("tags"),
            "error should mention field name: {}",
            err
        );
    }

    #[test]
    fn test_validate_jsonb_default_err_empty_array_default() {
        let err = validate_jsonb_default("tags", Some("'[]'::jsonb")).unwrap_err();
        assert!(
            err.contains("must not have default empty array"),
            "error should reject empty array default: {}",
            err
        );
        assert!(
            err.contains("tags"),
            "error should mention field name: {}",
            err
        );
    }

    #[test]
    fn test_validate_jsonb_default_err_empty_array_no_quotes() {
        let err = validate_jsonb_default("meta", Some("[]::jsonb")).unwrap_err();
        assert!(
            err.contains("must not have default empty array"),
            "error should reject empty array default: {}",
            err
        );
    }
}
