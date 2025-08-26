use crate::define_table_schema;
use crate::schema::generator::diesel_schema_definition::{types::*, DieselTableDefinition};
use crate::{system_fields, system_foreign_keys, system_indexes};

/// User roles table for role-based access control
pub struct UserRolesTable;

define_table_schema! {
    hypertable: false,
    fields: {
        // System fields - common across all tables
        system_fields!(),

        // User roles specific fields
        role: nullable(text()),
        entity: nullable(text()),
    },
    indexes: {
        // System field indexes
        system_indexes!("user_roles"),
    },
    foreign_keys: {
        // System field foreign keys
        system_foreign_keys!("user_roles"),


    }
}
