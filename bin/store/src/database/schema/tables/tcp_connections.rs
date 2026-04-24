use crate::define_table_schema;
use crate::generated::schema::generator::diesel_schema_definition::{
    types::*, DieselTableDefinition,
};
use crate::{system_fields, system_foreign_keys, system_indexes};

/**
 * Auto-converted from TypeScript schema source.
 */
pub struct TcpConnectionsTable;

define_table_schema! {
    hypertable: false,
    fields: {
        // System fields - common across all tables ( REQUIRED )
        system_fields!(),

        source: nullable(text()),
        sport: nullable(integer()),
        dest: nullable(text()),
        dport: nullable(integer()),
        proto: nullable(text()),
    },
    indexes: {
        // System field indexes ( REQUIRED )
        system_indexes!("tcp_connections"),
    },
    foreign_keys: {
        // System field foreign keys ( REQUIRED )
        system_foreign_keys!("tcp_connections"),
    }
}
