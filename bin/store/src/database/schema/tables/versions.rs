use crate::define_table_schema;
use crate::generated::schema::generator::diesel_schema_definition::{
    types::*, DieselTableDefinition,
};
use crate::{system_fields, system_foreign_keys, system_indexes};

/**
 * Auto-converted from TypeScript schema source.
 */
pub struct VersionsTable;

define_table_schema! {
    hypertable: false,
    fields: {
        // System fields - common across all tables ( REQUIRED )
        system_fields!(),

        name: nullable(text()),
        latest_version: nullable(text()),
        minimum_version: nullable(text()),
        update_type: nullable(text()), default "optional",
        release_notes: nullable(text()),
    },
    indexes: {
        // System field indexes ( REQUIRED )
        system_indexes!("versions"),
    },
    foreign_keys: {
        // System field foreign keys ( REQUIRED )
        system_foreign_keys!("versions"),
    }
}
