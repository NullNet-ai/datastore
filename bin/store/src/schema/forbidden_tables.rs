/// List of tables that are exceptions to organization_id filtering
pub const FORBIDDEN_TABLES: &[&str] = &[
    // Add table names that should not have organization_id automatically added
    // Example: "system_config", "global_settings"
];

/// Check if a table is in the forbidden tables list
pub fn is_forbidden_table(table: &str) -> bool {
    FORBIDDEN_TABLES.contains(&table)
}