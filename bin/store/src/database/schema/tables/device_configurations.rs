use crate::define_table_schema;
use crate::generated::schema::generator::diesel_schema_definition::{
    types::*, DieselTableDefinition,
};
use crate::{system_fields, system_foreign_keys, system_indexes};

/**
 * Auto-converted from TypeScript schema source.
 */
pub struct DeviceConfigurationsTable;

define_table_schema! {
    hypertable: false,
    fields: {
        // System fields - common across all tables ( REQUIRED )
        system_fields!(),

        device_id: nullable(text()),
        digest: nullable(text()),
        hostname: nullable(text()),
        raw_content: nullable(text()),
        config_version: nullable(integer()),
        tables: nullable(array(text())),
        chains: nullable(array(text())),
    },
    indexes: {
        // System field indexes ( REQUIRED )
        system_indexes!("device_configurations"),
    },
    foreign_keys: {
        // System field foreign keys ( REQUIRED )
        system_foreign_keys!("device_configurations"),
        fk_device_configurations_device_id: { columns: ["device_id"], foreign_table: "devices", foreign_columns: ["id"], on_delete: "no action", on_update: "no action" },
    }
}
