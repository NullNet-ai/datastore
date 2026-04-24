use crate::define_table_schema;
use crate::generated::schema::generator::diesel_schema_definition::{
    types::*, DieselTableDefinition,
};
use crate::{system_fields, system_foreign_keys, system_indexes};

/**
 * Auto-converted from TypeScript schema source.
 */
pub struct DevicesTable;

define_table_schema! {
    hypertable: false,
    fields: {
        // System fields - common across all tables ( REQUIRED )
        system_fields!(),

        is_traffic_monitoring_enabled: nullable(boolean()), default "false",
        is_config_monitoring_enabled: nullable(boolean()), default "false",
        is_telemetry_monitoring_enabled: nullable(boolean()), default "false",
        is_device_authorized: nullable(boolean()), default "false",
        device_uuid: nullable(text()), default "",
        device_name: nullable(text()), default "",
        device_category: nullable(text()), default "",
        device_type: nullable(text()), default "",
        device_os: nullable(text()), default "",
        device_version: nullable(text()), default "",
        is_device_online: nullable(boolean()), default "false",
        address_id: nullable(text()),
    },
    indexes: {
        // System field indexes ( REQUIRED )
        system_indexes!("devices"),
    },
    foreign_keys: {
        // System field foreign keys ( REQUIRED )
        system_foreign_keys!("devices"),
        fk_devices_address_id: { columns: ["address_id"], foreign_table: "addresses", foreign_columns: ["id"], on_delete: "no action", on_update: "no action" },
    }
}
