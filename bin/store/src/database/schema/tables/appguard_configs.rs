use crate::define_table_schema;
use crate::generated::schema::generator::diesel_schema_definition::{
    types::*, DieselTableDefinition,
};
use crate::{system_fields, system_foreign_keys, system_indexes};

/**
 * Auto-converted from TypeScript schema source.
 */
pub struct AppguardConfigsTable;

define_table_schema! {
    hypertable: false,
    fields: {
        // System fields - common across all tables ( REQUIRED )
        system_fields!(),

        active: nullable(boolean()), default "true",
        log_request: nullable(boolean()),
        log_response: nullable(boolean()),
        retention_sec: nullable(integer()),
        ip_info_cache_size: nullable(integer()),
    },
    indexes: {
        // System field indexes ( REQUIRED )
        system_indexes!("appguard_configs"),
    },
    foreign_keys: {
        // System field foreign keys ( REQUIRED )
        system_foreign_keys!("appguard_configs"),
    }
}
