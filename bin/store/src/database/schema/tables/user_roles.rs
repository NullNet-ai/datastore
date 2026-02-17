use crate::define_table_schema;
use crate::schema::generator::diesel_schema_definition::{types::*, DieselTableDefinition};
use crate::{system_fields, system_foreign_keys, system_indexes};

/**
 * For defining proper diesel types check it here: bin/store/src/builders/generator/README.md
 */

/// User roles table for role-based access control
pub struct UserRolesTable;

define_table_schema! {
    hypertable: false,
    fields: {
        // System fields - common across all tables ( REQUIRED )
        system_fields!(),

        // User roles specific fields
        role: nullable(text()),
        entity: nullable(text()),
    },
    indexes: {
        // System field indexes ( REQUIRED )
        system_indexes!("user_roles"),
    },
    foreign_keys: {
        // System field foreign keys ( REQUIRED )
        system_foreign_keys!("user_roles"),


    }
}
