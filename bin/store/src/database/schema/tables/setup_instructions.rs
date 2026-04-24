use crate::define_table_schema;
use crate::generated::schema::generator::diesel_schema_definition::{
    types::*, DieselTableDefinition,
};
use crate::{system_fields, system_foreign_keys, system_indexes};

/**
 * Auto-converted from TypeScript schema source.
 */
pub struct SetupInstructionsTable;

define_table_schema! {
    hypertable: false,
    fields: {
        // System fields - common across all tables ( REQUIRED )
        system_fields!(),

        device_category: nullable(text()),
        device_type: nullable(text()),
        markdown: nullable(text()),
    },
    indexes: {
        // System field indexes ( REQUIRED )
        system_indexes!("setup_instructions"),
    },
    foreign_keys: {
        // System field foreign keys ( REQUIRED )
        system_foreign_keys!("setup_instructions"),
    }
}
