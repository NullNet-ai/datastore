use crate::define_table_schema;
use crate::generated::schema::generator::diesel_schema_definition::{
    types::*, DieselTableDefinition,
};
use crate::{system_fields, system_foreign_keys, system_indexes};

/**
 * Auto-converted from TypeScript schema source.
 */
pub struct TempConnectionsTable;

define_table_schema! {
    hypertable: false,
    fields: {
        // System fields - common across all tables ( REQUIRED )
        system_fields!(),

        hypertable_timestamp: nullable(text()),
        interface_name: nullable(text()),
        total_packet: nullable(integer()),
        total_byte: nullable(integer()),
        device_id: nullable(text()),
        protocol: nullable(text()),
        source_ip: nullable(text()),
        destination_ip: nullable(text()),
        source_port: nullable(integer()),
        destination_port: nullable(integer()),
        remote_ip: nullable(text()),
    },
    indexes: {
        // System field indexes ( REQUIRED )
        system_indexes!("temp_connections"),
    },
    foreign_keys: {
        // System field foreign keys ( REQUIRED )
        system_foreign_keys!("temp_connections"),
        fk_temp_connections_device_id: { columns: ["device_id"], foreign_table: "devices", foreign_columns: ["id"], on_delete: "no action", on_update: "no action" },
    }
}
