#[allow(warnings)]
pub const HYPERTABLES: &[&str] = &[
    "test_hypertables",
    // Add more hypertable names as needed
];
#[allow(warnings)]
// Helper function to check if a table is a hypertable
pub fn is_hypertable(table_name: &str) -> bool {
    HYPERTABLES.contains(&table_name)
}
