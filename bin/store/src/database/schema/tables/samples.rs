use crate::define_table_schema;
use crate::generated::schema::generator::diesel_schema_definition::{
    types::*, DieselTableDefinition,
};
use crate::{system_fields, system_foreign_keys, system_indexes};

/**
 * Auto-converted from TypeScript schema source.
 */
pub struct SamplesTable;

define_table_schema! {
    hypertable: false,
    fields: {
        // System fields - common across all tables ( REQUIRED )
        system_fields!(),

        name: nullable(text()),
        sample_text: nullable(text()),
        test_obj: nullable(jsonb()), default "sql`'{}'::jsonb`",
    },
    indexes: {
        // System field indexes ( REQUIRED )
        system_indexes!("samples"),
    },
    foreign_keys: {
        // System field foreign keys ( REQUIRED )
        system_foreign_keys!("samples"),
    }
}
