use crate::define_table_schema;
use crate::generated::schema::generator::diesel_schema_definition::{
    types::*, DieselTableDefinition,
};
use crate::{system_fields, system_foreign_keys, system_indexes};

/**
 * Auto-converted from TypeScript schema source.
 */
pub struct TempDeviceInterfacesTable;

define_table_schema! {
    hypertable: false,
    fields: {
        // System fields - common across all tables ( REQUIRED )
        system_fields!(),

        device_configuration_id: nullable(text()),
        name: nullable(text()),
        device: nullable(text()),
        description: nullable(text()),
    },
    indexes: {
        // System field indexes ( REQUIRED )
        system_indexes!("temp_device_interfaces"),
    },
    foreign_keys: {
        // System field foreign keys ( REQUIRED )
        system_foreign_keys!("temp_device_interfaces"),
        fk_temp_device_interfaces_device_configuration_id: { columns: ["device_configuration_id"], foreign_table: "device_configurations", foreign_columns: ["id"], on_delete: "no action", on_update: "no action" },
    }
}
