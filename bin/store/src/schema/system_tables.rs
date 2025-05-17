// Define an array of system table names
// These tables are considered system tables and may have special handling

pub const SYSTEM_TABLES: [&str; 7] = [
    "crdt_messages",
    "crdt_merkles",
    "sync_endpoints",
    "queues",
    "queue_items",
    "transactions",
    "counters"
];

// Function to check if a table is a system table
pub fn is_system_table(table_name: &str) -> bool {
    SYSTEM_TABLES.contains(&table_name)
}