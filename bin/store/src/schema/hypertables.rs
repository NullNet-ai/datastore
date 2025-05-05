pub const HYPERTABLES: &[&str] = &[
    "packets",
    "connections",
    // Add more hypertable names as needed
];

// Helper function to check if a table is a hypertable
pub fn is_hypertable(table_name: &str) -> bool {
    HYPERTABLES.contains(&table_name)
}