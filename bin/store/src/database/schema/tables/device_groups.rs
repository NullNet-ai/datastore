use crate::define_table_schema;
use crate::generated::schema::generator::diesel_schema_definition::{
    types::*, DieselTableDefinition,
};
use crate::{system_fields, system_foreign_keys, system_indexes};

/**
 * Auto-converted from TypeScript schema source.
 */
pub struct DeviceGroupsTable;

define_table_schema! {
    hypertable: false,
    fields: {
        // System fields - common across all tables ( REQUIRED )
        system_fields!(),

        device_id: nullable(text()),
        device_group_setting_id: nullable(text()),
    },
    indexes: {
        // System field indexes ( REQUIRED )
        system_indexes!("device_groups"),
    },
    foreign_keys: {
        // System field foreign keys ( REQUIRED )
        system_foreign_keys!("device_groups"),
        fk_device_groups_device_id: { columns: ["device_id"], foreign_table: "devices", foreign_columns: ["id"], on_delete: "no action", on_update: "no action" },
        fk_device_groups_device_group_setting_id: { columns: ["device_group_setting_id"], foreign_table: "device_group_settings", foreign_columns: ["id"], on_delete: "no action", on_update: "no action" },
    }
}
