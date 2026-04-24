use crate::define_table_schema;
use crate::generated::schema::generator::diesel_schema_definition::{
    types::*, DieselTableDefinition,
};
use crate::{system_fields, system_foreign_keys, system_indexes};

/**
 * Auto-converted from TypeScript schema source.
 */
pub struct UserRolesTable;

define_table_schema! {
    hypertable: false,
    fields: {
        // System fields - common across all tables ( REQUIRED )
        system_fields!(),

        role: nullable(text()),
        entity: nullable(text()),
    },
    indexes: {
        // System field indexes ( REQUIRED )
        system_indexes!("user_roles"),
        // Custom table-specific indexes
        idx_user_roles_entity: {
            columns: ["entity"],
            unique: false,
            type: "btree"
        },
        idx_user_roles_role: {
            columns: ["role"],
            unique: false,
            type: "btree"
        },
    },
    foreign_keys: {
        // System field foreign keys ( REQUIRED )
        system_foreign_keys!("user_roles"),
    }
}
