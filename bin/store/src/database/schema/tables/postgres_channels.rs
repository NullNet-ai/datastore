use crate::define_table_schema;
use crate::schema::generator::diesel_schema_definition::{types::*, DieselTableDefinition};

pub struct PostgresChannelsTable;

define_table_schema! {
    hypertable: false,
    fields: {
        system_fields!(),
        channel_name: nullable(text()),
        channel_timestamp: nullable(timestamptz()),
        function: nullable(text()),
    },
    indexes: {
        system_indexes!("postgres_channels"),
        idx_postgres_channels_channel_name: { columns: ["channel_name"], unique: true, type: "btree" },
        idx_postgres_channels_channel_timestamp: { columns: ["channel_timestamp"], unique: false, type: "btree" },
        idx_postgres_channels_function: { columns: ["function"], unique: false, type: "btree" },
    },
    foreign_keys: {
        system_foreign_keys!("postgres_channels"),
    }
}
