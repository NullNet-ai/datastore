// Define an array of system table names
// These tables are considered system tables and may have special handling

pub const SYSTEM_TABLES: &[&str] = &[
    "crdt_messages",
    "crdt_merkles",
    "sync_endpoints",
    "queues",
    "queue_items",
    "transactions",
    "counters",
    "entities",
    "fields",
    "entity_fields",
    "permissions",
    "encryption_keys",
    "sessions",
    "data_permissions",
    "user_roles",
    "role_permissions",
    "organization_domains",
    "system_config_fields",
    "record_permissions",
    "role_permission",
    "table_indexes",
    "stream_queue",
    "stream_queue_items",
    "test",
 
    "app_firewalls",
    "appguard_logs",
    "temp_appguard_logs",
    "device_aliases",
    "temp_device_aliases",
    "device_configurations",
    "device_interface_addresses",
    "temp_device_interface_addresses",
    "device_interfaces",
    "temp_device_interfaces",
    "device_remote_access_sessions",
    "temp_device_remote_access_sessions",
    "device_rules",
    "temp_device_rules",
    "packets",
    "temp_packets",
    "connections",
    "temp_connections",
    "device_ssh_keys",
    "device_groups",
    "temp_device_groups",
    "device_group_devices",
    "temp_device_group_devices",
    "device_group_rules",
    "temp_device_group_rules",
    "device_group_settings",
    "products",
    "resolutions",
    "wallguard_logs",
    "temp_wallguard_logs",
    "blacklist",
    "temp_blacklist"

];

// Function to check if a table is a system table
pub fn is_system_table(table_name: &str) -> bool {
    SYSTEM_TABLES.contains(&table_name)
}
