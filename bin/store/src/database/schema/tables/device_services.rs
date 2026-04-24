use crate::define_table_schema;
use crate::generated::schema::generator::diesel_schema_definition::{
    types::*, DieselTableDefinition,
};
use crate::{system_fields, system_foreign_keys, system_indexes};

/**
 * Auto-converted from TypeScript schema source.
 */
pub struct DeviceServicesTable;

define_table_schema! {
    hypertable: false,
    fields: {
        // System fields - common across all tables ( REQUIRED )
        system_fields!(),

        device_id: nullable(text()),
        address: nullable(text()),
        port: nullable(integer()),
        protocol: nullable(text()),
        program: nullable(text()),
    },
    indexes: {
        // System field indexes ( REQUIRED )
        system_indexes!("device_services"),
    },
    foreign_keys: {
        // System field foreign keys ( REQUIRED )
        system_foreign_keys!("device_services"),
        fk_device_services_device_id: { columns: ["device_id"], foreign_table: "devices", foreign_columns: ["id"], on_delete: "no action", on_update: "no action" },
    }
}
