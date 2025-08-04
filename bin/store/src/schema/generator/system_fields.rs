/// System fields that are common across most tables
/// These can be destructured into table definitions to avoid repetition
/// 
/// Default values:
/// - tombstone: 0 (not deleted)
/// - status: "Active"
/// - is_batch: false
/// - sync_status: "in_process"

/// Macro to include common system fields in table definitions
#[macro_export]
macro_rules! system_fields {
    () => {
        tombstone: nullable(integer()), migration_nullable: false, default: "0",
        status: nullable(text()), default: "Active",
        previous_status: nullable(text()),
        version: nullable(integer()),
        created_date: nullable(text()),
        created_time: nullable(text()),
        updated_date: nullable(text()),
        updated_time: nullable(text()),
        organization_id: nullable(text()),
        created_by: nullable(text()),
        updated_by: nullable(text()),
        deleted_by: nullable(text()),
        requested_by: nullable(text()),
        timestamp: nullable(timestamp()),
        tags: nullable(array(text())),
        categories: nullable(array(text())),
        code: nullable(text()), migration_nullable: false,
        id: nullable(text()), primary_key: true, migration_nullable: false,
        sensitivity_level: nullable(integer()),
        sync_status: nullable(text()), default: "in_process",
        is_batch: nullable(boolean()), default: "false"


    };
}