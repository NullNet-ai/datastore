// Define an array of system table names
// These tables are considered system tables and may have special handling
// Use this to restrict access to mutate data for developers
pub const SYSTEM_TABLES: &[&str] = &[
    // CRDT related tables
    "crdt_messages",
    "crdt_merkles",
    "sync_endpoints",
    "queues",
    "queue_items",
    "transactions",
    // User Define System Tables
    "counters",
    "entities",
    "fields",
    "entity_fields",
    "permissions",
    "encryption_keys",
    "data_permissions",
    "role_permissions",
    "organization_domains",
    "system_config_fields",
    "record_permissions",
    "role_permission",
    "table_indexes",
    "stream_queue",
    "stream_queue_items",
    "test",
];

// Function to check if a table is a system table
pub fn is_system_table(table_name: &str) -> bool {
    SYSTEM_TABLES.contains(&table_name)
}
