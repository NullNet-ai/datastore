use crate::define_table_schema;
use crate::generated::schema::generator::diesel_schema_definition::{
    types::*, DieselTableDefinition,
};
use crate::{system_fields, system_foreign_keys, system_indexes};

/**
 * Auto-converted from TypeScript schema source.
 */
pub struct HttpRequestsTable;

define_table_schema! {
    hypertable: false,
    fields: {
        // System fields - common across all tables ( REQUIRED )
        system_fields!(),

        fw_policy: nullable(text()),
        fw_reasons: nullable(text()),
        ip: nullable(text()),
        original_url: nullable(text()),
        user_agent: nullable(text()),
        headers: nullable(text()),
        method: nullable(text()),
        body: nullable(text()),
        query: nullable(text()),
        cookies: nullable(text()),
    },
    indexes: {
        // System field indexes ( REQUIRED )
        system_indexes!("http_requests"),
    },
    foreign_keys: {
        // System field foreign keys ( REQUIRED )
        system_foreign_keys!("http_requests"),
    }
}
