use crate::define_table_schema;
use crate::generated::schema::generator::diesel_schema_definition::{
    types::*, DieselTableDefinition,
};
use crate::{system_fields, system_foreign_keys, system_indexes};

/// Table storing information about Jean
pub struct JeanTable;

define_table_schema! {
    hypertable: false,
    fields: {
        // System fields - common across all tables
        system_fields!(),

        // jean specific fields
        first_name: nullable(text()),
        is_hungry: nullable(boolean()),
        is_sleepy: nullable(boolean()), default: "true",
        eats_candies: nullable(boolean()), default: "true",
        age: nullable(integer())
    },
    indexes: {
        // System field indexes
        system_indexes!("jean"),

        // Custom table-specific indexes
    },
    foreign_keys: {
        // System field foreign keys
        system_foreign_keys!("jean"),

        // Custom foreign keys
    }
}
