use crate::define_table_schema;
use crate::generated::schema::generator::diesel_schema_definition::{
    types::*, DieselTableDefinition,
};
use crate::{system_fields, system_foreign_keys, system_indexes};

/**
 * Auto-converted from TypeScript schema source.
 */
pub struct TempDeviceNatRulesTable;

define_table_schema! {
    hypertable: false,
    fields: {
        // System fields - common across all tables ( REQUIRED )
        system_fields!(),

        device_configuration_id: nullable(text()),
        disabled: nullable(boolean()),
        protocol: nullable(text()),
        ipprotocol: nullable(text()),
        source_inversed: nullable(boolean()),
        source_port_value: nullable(text()),
        source_port_operator: nullable(text()),
        source_ip_value: nullable(text()),
        source_ip_operator: nullable(text()),
        source_ip_version: nullable(integer()),
        source_type: nullable(text()),
        destination_inversed: nullable(boolean()),
        destination_port_value: nullable(text()),
        destination_port_operator: nullable(text()),
        destination_ip_value: nullable(text()),
        destination_ip_operator: nullable(text()),
        destination_ip_version: nullable(integer()),
        destination_type: nullable(text()),
        device_rule_status: nullable(text()), default "Applied",
        description: nullable(text()),
        interface: nullable(text()),
        redirect_ip: nullable(text()),
        redirect_port: nullable(integer()),
        order: nullable(integer()),
        associated_rule_id: nullable(text()), default "",
        table: nullable(text()),
        chain: nullable(text()),
        family: nullable(text()),
    },
    indexes: {
        // System field indexes ( REQUIRED )
        system_indexes!("temp_device_nat_rules"),
    },
    foreign_keys: {
        // System field foreign keys ( REQUIRED )
        system_foreign_keys!("temp_device_nat_rules"),
        fk_temp_device_nat_rules_device_configuration_id: { columns: ["device_configuration_id"], foreign_table: "device_configurations", foreign_columns: ["id"], on_delete: "no action", on_update: "no action" },
    }
}
