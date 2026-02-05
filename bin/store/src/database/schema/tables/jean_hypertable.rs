use crate::define_table_schema;
use crate::generated::schema::generator::diesel_schema_definition::{
    types::*, DieselTableDefinition,
};
use crate::{system_fields, system_foreign_keys, system_indexes};

/// Hypertable for tracking Jean's status and characteristics over time
pub struct JeanHypertableTable;

define_table_schema! {
    hypertable: true,
    fields: {
        // System fields - common across all tables
        system_fields!(),

        // jean hypertable specific fields
        timestamp: nullable(timestamptz()), primary_key: true, indexed: true, migration_nullable: false,
        id: nullable(text()), primary_key: true, indexed: true, migration_nullable: false,
        hypertable_timestamp: nullable(text()),
        first_name: nullable(text()),
        is_hungry: nullable(boolean()),
        is_sleepy: nullable(boolean()), default: "true",
        eats_candies: nullable(boolean()), default: "true",
        age: nullable(integer())
    },
    indexes: {
        // System field indexes
        system_indexes!("jean_hypertable"),

        // Custom table-specific indexes
        idx_jean_hypertable_idx_jean_hypertable_first_name: {
            columns: ["first_name"],
            unique: false,
            type: "btree"
        }
    },
    foreign_keys: {

        // Custom foreign keys
    }
}
