#[allow(warnings)]
pub const HYPERTABLES: &[&str] = &[
    "test_hypertable",
    // Add more hypertable names as needed
];
#[allow(warnings)]
pub fn is_hypertable(table_name: &str) -> bool {
    HYPERTABLES.contains(&table_name)
}
