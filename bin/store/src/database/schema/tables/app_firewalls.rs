use crate::define_table_schema;
use crate::generated::schema::generator::diesel_schema_definition::{
    types::*, DieselTableDefinition,
};
use crate::{system_fields, system_foreign_keys, system_indexes};

/**
 * Auto-converted from TypeScript schema source.
 */
pub struct AppFirewallsTable;

define_table_schema! {
    hypertable: false,
    fields: {
        // System fields - common across all tables ( REQUIRED )
        system_fields!(),

        active: nullable(boolean()), default "true",
        app_id: nullable(text()),
        firewall: nullable(text()),
    },
    indexes: {
        // System field indexes ( REQUIRED )
        system_indexes!("app_firewalls"),
    },
    foreign_keys: {
        // System field foreign keys ( REQUIRED )
        system_foreign_keys!("app_firewalls"),
    }
}
