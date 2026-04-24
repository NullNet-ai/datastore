use crate::define_table_schema;
use crate::generated::schema::generator::diesel_schema_definition::{
    types::*, DieselTableDefinition,
};
use crate::{system_fields, system_foreign_keys, system_indexes};

/**
 * Auto-converted from TypeScript schema source.
 */
pub struct DeviceRemoteAccessSessionsTable;

define_table_schema! {
    hypertable: false,
    fields: {
        // System fields - common across all tables ( REQUIRED )
        system_fields!(),

        device_id: nullable(text()),
        remote_access_type: nullable(text()),
        remote_access_session: nullable(text()),
        remote_access_status: nullable(text()),
        instance_id: nullable(text()),
        remote_access_category: nullable(text()),
        remote_access_local_addr: nullable(text()),
        remote_access_local_port: nullable(integer()),
        remote_access_local_protocol: nullable(text()),
    },
    indexes: {
        // System field indexes ( REQUIRED )
        system_indexes!("device_remote_access_sessions"),
    },
    foreign_keys: {
        // System field foreign keys ( REQUIRED )
        system_foreign_keys!("device_remote_access_sessions"),
        fk_device_remote_access_sessions_device_id: { columns: ["device_id"], foreign_table: "devices", foreign_columns: ["id"], on_delete: "no action", on_update: "no action" },
    }
}
