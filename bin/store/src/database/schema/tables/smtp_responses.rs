use crate::define_table_schema;
use crate::generated::schema::generator::diesel_schema_definition::{
    types::*, DieselTableDefinition,
};
use crate::{system_fields, system_foreign_keys, system_indexes};

/**
 * Auto-converted from TypeScript schema source.
 */
pub struct SmtpResponsesTable;

define_table_schema! {
    hypertable: false,
    fields: {
        // System fields - common across all tables ( REQUIRED )
        system_fields!(),

        fw_policy: nullable(text()),
        fw_reasons: nullable(text()),
        ip: nullable(text()),
        response_code: nullable(bigint()),
        time: nullable(bigint()),
    },
    indexes: {
        // System field indexes ( REQUIRED )
        system_indexes!("smtp_responses"),
    },
    foreign_keys: {
        // System field foreign keys ( REQUIRED )
        system_foreign_keys!("smtp_responses"),
    }
}
