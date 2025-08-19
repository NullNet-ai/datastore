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
        tombstone: nullable(integer()), default: "0",
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
        code: nullable(text()),
        id: nullable(text()), primary_key: true,
        sensitivity_level: nullable(integer()), default: 1000
        sync_status: nullable(text()), default: "in_process",
        is_batch: nullable(boolean()), default: "false"
    };
}

/// Macro to include common system field indexes in table definitions
/// Usage: system_indexes!(table_name) - e.g., system_indexes!("users") creates idx_users_tombstone, etc.
#[macro_export]
macro_rules! system_indexes {
    ($table_name:expr) => {
        paste::paste! {
            [<idx_ $table_name _tombstone>]: {
                columns: ["tombstone"],
                unique: false,
                type: "btree"
            },
            [<idx_ $table_name _status>]: {
                columns: ["status"],
                unique: false,
                type: "btree"
            },
            [<idx_ $table_name _previous_status>]: {
                columns: ["previous_status"],
                unique: false,
                type: "btree"
            },
            [<idx_ $table_name _version>]: {
                columns: ["version"],
                unique: false,
                type: "btree"
            },
            [<idx_ $table_name _created_date>]: {
                columns: ["created_date"],
                unique: false,
                type: "btree"
            },
            [<idx_ $table_name _updated_date>]: {
                columns: ["updated_date"],
                unique: false,
                type: "btree"
            },
            [<idx_ $table_name _organization_id>]: {
                columns: ["organization_id"],
                unique: false,
                type: "btree"
            },
            [<idx_ $table_name _created_by>]: {
                columns: ["created_by"],
                unique: false,
                type: "btree"
            },
            [<idx_ $table_name _updated_by>]: {
                columns: ["updated_by"],
                unique: false,
                type: "btree"
            },
            [<idx_ $table_name _deleted_by>]: {
                columns: ["deleted_by"],
                unique: false,
                type: "btree"
            },
            [<idx_ $table_name _requested_by>]: {
                columns: ["requested_by"],
                unique: false,
                type: "btree"
            },
            [<idx_ $table_name _tags>]: {
                columns: ["tags"],
                unique: false,
                type: "btree"
            },
            [<idx_ $table_name _categories>]: {
                columns: ["categories"],
                unique: false,
                type: "btree"
            },
            [<idx_ $table_name _code>]: {
                columns: ["code"],
                unique: false,
                type: "btree"
            },
            [<idx_ $table_name _sensitivity_level>]: {
                columns: ["sensitivity_level"],
                unique: false,
                type: "btree"
            }
        }
    };
}

/// Macro to include common system field foreign keys in table definitions
/// Usage: system_foreign_keys!(table_name) - e.g., system_foreign_keys!("users") creates foreign key constraints
#[macro_export]
macro_rules! system_foreign_keys {
    ($table_name:expr) => {
        paste::paste! {
            [<$table_name _organization_id_organizations_id_fk>]: {
                columns: ["organization_id"],
                foreign_table: "organizations",
                foreign_columns: ["id"],
                on_delete: "no action",
                on_update: "no action"
            },
            [<$table_name _created_by_account_organizations_id_fk>]: {
                columns: ["created_by"],
                foreign_table: "account_organizations",
                foreign_columns: ["id"],
                on_delete: "no action",
                on_update: "no action"
            },
            [<$table_name _updated_by_account_organizations_id_fk>]: {
                columns: ["updated_by"],
                foreign_table: "account_organizations",
                foreign_columns: ["id"],
                on_delete: "no action",
                on_update: "no action"
            },
            [<$table_name _deleted_by_account_organizations_id_fk>]: {
                columns: ["deleted_by"],
                foreign_table: "account_organizations",
                foreign_columns: ["id"],
                on_delete: "no action",
                on_update: "no action"
            },
            [<$table_name _requested_by_account_organizations_id_fk>]: {
                columns: ["requested_by"],
                foreign_table: "account_organizations",
                foreign_columns: ["id"],
                on_delete: "no action",
                on_update: "no action"
            }
        }
    };
}
