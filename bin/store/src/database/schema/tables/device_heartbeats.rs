use crate::define_table_schema;
use crate::generated::schema::generator::diesel_schema_definition::{
    types::*, DieselTableDefinition,
};
use crate::{system_fields, system_foreign_keys, system_indexes};

/**
 * Auto-converted from TypeScript schema source.
 */
pub struct DeviceHeartbeatsTable;

define_table_schema! {
    hypertable: false,
    fields: {
        // System fields - common across all tables ( REQUIRED )
        system_fields!(),

        device_id: nullable(text()),
        hypertable_timestamp: nullable(text()),
    },
    indexes: {
        // System field indexes ( REQUIRED )
        system_indexes!("device_heartbeats"),
    },
    foreign_keys: {
        // System field foreign keys ( REQUIRED )
        system_foreign_keys!("device_heartbeats"),
        fk_device_heartbeats_device_id: { columns: ["device_id"], foreign_table: "devices", foreign_columns: ["id"], on_delete: "no action", on_update: "no action" },
    }
}
