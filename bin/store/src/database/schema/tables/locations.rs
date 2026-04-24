use crate::define_table_schema;
use crate::generated::schema::generator::diesel_schema_definition::{
    types::*, DieselTableDefinition,
};
use crate::{system_fields, system_foreign_keys, system_indexes};

/**
 * Auto-converted from TypeScript schema source.
 */
pub struct LocationsTable;

define_table_schema! {
    hypertable: false,
    fields: {
        // System fields - common across all tables ( REQUIRED )
        system_fields!(),

        location_name: nullable(text()),
        address_id: nullable(text()),
    },
    indexes: {
        // System field indexes ( REQUIRED )
        system_indexes!("locations"),
        // Custom table-specific indexes
        idx_locations_address_id: {
            columns: ["address_id"],
            unique: false,
            type: "btree"
        },
        idx_locations_location_name: {
            columns: ["location_name"],
            unique: false,
            type: "btree"
        },
    },
    foreign_keys: {
        // System field foreign keys ( REQUIRED )
        system_foreign_keys!("locations"),
        fk_locations_address_id: { columns: ["address_id"], foreign_table: "addresses", foreign_columns: ["id"], on_delete: "no action", on_update: "no action" },
    }
}
