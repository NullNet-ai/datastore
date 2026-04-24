use crate::define_table_schema;
use crate::generated::schema::generator::diesel_schema_definition::{
    types::*, DieselTableDefinition,
};
use crate::{system_fields, system_foreign_keys, system_indexes};

/**
 * Auto-converted from TypeScript schema source.
 */
pub struct DummyPacketsTable;

define_table_schema! {
    hypertable: false,
    fields: {
        // System fields - common across all tables ( REQUIRED )
        system_fields!(),

        interface_name: nullable(text()),
        total_length: nullable(integer()),
        device_id: nullable(text()),
        ether_type: nullable(text()),
        protocol: nullable(text()),
        source_ip: nullable(text()),
        destination_ip: nullable(text()),
        remote_ip: nullable(text()),
        source_port: nullable(integer()),
        destination_port: nullable(integer()),
        hypertable_timestamp: nullable(text()),
        source_mac: nullable(text()),
        destination_mac: nullable(text()),
        tcp_header_length: nullable(integer()),
        tcp_sequence_number: nullable(bigint()),
        tcp_acknowledgment_number: nullable(bigint()),
        tcp_data_offset: nullable(integer()),
        tcp_flags: nullable(integer()),
        tcp_window_size: nullable(integer()),
        tcp_urgent_pointer: nullable(integer()),
        icmp_type: nullable(integer()),
        icmp_code: nullable(integer()),
    },
    indexes: {
        // System field indexes ( REQUIRED )
        system_indexes!("dummy_packets"),
        // Custom table-specific indexes
        idx_dummy_packets_total_length: {
            columns: ["total_length"],
            unique: false,
            type: "btree"
        },
    },
    foreign_keys: {
        // System field foreign keys ( REQUIRED )
        system_foreign_keys!("dummy_packets"),
        fk_dummy_packets_device_id: { columns: ["device_id"], foreign_table: "devices", foreign_columns: ["id"], on_delete: "no action", on_update: "no action" },
    }
}
