use crate::define_table_schema;
use crate::generated::schema::generator::diesel_schema_definition::{
    types::*, DieselTableDefinition,
};
use crate::{system_fields, system_foreign_keys, system_indexes};

/**
 * Auto-converted from TypeScript schema source.
 */
pub struct TempAliasesTable;

define_table_schema! {
    hypertable: false,
    fields: {
        // System fields - common across all tables ( REQUIRED )
        system_fields!(),

        device_configuration_id: nullable(text()),
        type_: nullable(text()),
        name: nullable(text()),
        description: nullable(text()),
        alias_status: nullable(text()),
        table_: nullable(text()),
        family: nullable(text()),
    },
    indexes: {
        // System field indexes ( REQUIRED )
        system_indexes!("temp_aliases"),
    },
    foreign_keys: {
        // System field foreign keys ( REQUIRED )
        system_foreign_keys!("temp_aliases"),
        fk_temp_aliases_device_configuration_id: { columns: ["device_configuration_id"], foreign_table: "device_configurations", foreign_columns: ["id"], on_delete: "no action", on_update: "no action" },
    }
}
