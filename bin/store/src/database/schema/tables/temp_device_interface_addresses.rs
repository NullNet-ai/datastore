use crate::define_table_schema;
use crate::generated::schema::generator::diesel_schema_definition::{
    types::*, DieselTableDefinition,
};
use crate::{system_fields, system_foreign_keys, system_indexes};

/**
 * Auto-converted from TypeScript schema source.
 */
pub struct TempDeviceInterfaceAddressesTable;

define_table_schema! {
    hypertable: false,
    fields: {
        // System fields - common across all tables ( REQUIRED )
        system_fields!(),

        device_interface_id: nullable(text()),
        address: nullable(text()),
    },
    indexes: {
        // System field indexes ( REQUIRED )
        system_indexes!("temp_device_interface_addresses"),
    },
    foreign_keys: {
        // System field foreign keys ( REQUIRED )
        system_foreign_keys!("temp_device_interface_addresses"),
        fk_temp_device_interface_addresses_device_interface_id: { columns: ["device_interface_id"], foreign_table: "device_interfaces", foreign_columns: ["id"], on_delete: "no action", on_update: "no action" },
    }
}
