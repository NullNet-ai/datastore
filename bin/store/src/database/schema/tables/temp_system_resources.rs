use crate::define_table_schema;
use crate::generated::schema::generator::diesel_schema_definition::{
    types::*, DieselTableDefinition,
};
use crate::{system_fields, system_foreign_keys, system_indexes};

/**
 * Auto-converted from TypeScript schema source.
 */
pub struct TempSystemResourcesTable;

define_table_schema! {
    hypertable: false,
    fields: {
        // System fields - common across all tables ( REQUIRED )
        system_fields!(),

        num_cpus: nullable(integer()),
        global_cpu_usage: nullable(doubleprecision()),
        cpu_usages: nullable(text()),
        total_memory: nullable(bigint()),
        used_memory: nullable(bigint()),
        total_disk_space: nullable(bigint()),
        available_disk_space: nullable(bigint()),
        read_bytes: nullable(bigint()),
        written_bytes: nullable(bigint()),
        temperatures: nullable(text()),
        device_id: nullable(text()),
    },
    indexes: {
        // System field indexes ( REQUIRED )
        system_indexes!("temp_system_resources"),
    },
    foreign_keys: {
        // System field foreign keys ( REQUIRED )
        system_foreign_keys!("temp_system_resources"),
        fk_temp_system_resources_device_id: { columns: ["device_id"], foreign_table: "devices", foreign_columns: ["id"], on_delete: "no action", on_update: "no action" },
    }
}
