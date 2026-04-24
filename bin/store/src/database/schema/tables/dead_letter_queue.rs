use crate::define_table_schema;
use crate::generated::schema::generator::diesel_schema_definition::{
    types::*, DieselTableDefinition,
};
use crate::{system_fields, system_foreign_keys, system_indexes};

/**
 * Auto-converted from TypeScript schema source.
 */
pub struct DeadLetterQueueTable;

define_table_schema! {
    hypertable: false,
    fields: {
        // System fields - common across all tables ( REQUIRED )
        system_fields!(),

        record_id: nullable(text()),
        table: nullable(text()),
        prefix: nullable(text()),
        error: nullable(text()),
    },
    indexes: {
        // System field indexes ( REQUIRED )
        system_indexes!("dead_letter_queue"),
    },
    foreign_keys: {
        // System field foreign keys ( REQUIRED )
        system_foreign_keys!("dead_letter_queue"),
    }
}
