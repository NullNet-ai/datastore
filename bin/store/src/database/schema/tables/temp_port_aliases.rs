use crate::define_table_schema;
use crate::generated::schema::generator::diesel_schema_definition::{
    types::*, DieselTableDefinition,
};
use crate::{system_fields, system_foreign_keys, system_indexes};

/**
 * Auto-converted from TypeScript schema source.
 */
pub struct TempPortAliasesTable;

define_table_schema! {
    hypertable: false,
    fields: {
        // System fields - common across all tables ( REQUIRED )
        system_fields!(),

        alias_id: nullable(text()),
        lower_port: nullable(integer()),
        upper_port: nullable(integer()),
    },
    indexes: {
        // System field indexes ( REQUIRED )
        system_indexes!("temp_port_aliases"),
    },
    foreign_keys: {
        // System field foreign keys ( REQUIRED )
        system_foreign_keys!("temp_port_aliases"),
        fk_temp_port_aliases_alias_id: { columns: ["alias_id"], foreign_table: "aliases", foreign_columns: ["id"], on_delete: "no action", on_update: "no action" },
    }
}
