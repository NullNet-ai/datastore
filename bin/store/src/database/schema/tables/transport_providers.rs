use crate::define_table_schema;
use crate::schema::generator::diesel_schema_definition::{types::*, DieselTableDefinition};

pub struct TransportProvidersTable;

define_table_schema! {
    hypertable: false,
    fields: {
        system_fields!(),
        name: nullable(text()),
    },
    indexes: {
        system_indexes!("transport_providers"),
        idx_transport_providers_name: { columns: ["name"], unique: false, type: "btree" },
    },
    foreign_keys: {
        system_foreign_keys!("transport_providers"),
    }
}
