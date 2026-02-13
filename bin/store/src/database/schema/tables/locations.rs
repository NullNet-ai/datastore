use crate::define_table_schema;
use crate::schema::generator::diesel_schema_definition::{types::*, DieselTableDefinition};
use crate::{system_fields, system_foreign_keys, system_indexes};

/**
 * For defining proper diesel types check it here: bin/store/src/builders/generator/README.md
 */

/// Locations table for managing location information
pub struct LocationsTable;

define_table_schema! {
    hypertable: false,
    fields: {
        // System fields - common across all tables ( REQUIRED )
        system_fields!(),

        // Locations fields
        location_name: nullable(text()),
        address_id: nullable(text()),
    },
    indexes: {
        // System field indexes ( REQUIRED )
        system_indexes!("locations"),

        // Custom table-specific indexes - all non-primary key fields
        idx_location_name: {
            columns: ["location_name"],
            unique: false,
            type: "btree"
        },
        idx_address_id: {
            columns: ["address_id"],
            unique: false,
            type: "btree"
        },
    },
    foreign_keys: {
        // System field foreign keys ( REQUIRED )
        system_foreign_keys!("locations"),

        // Custom foreign keys - all fields ending with "_id"
        fk_address_id: {
            columns: ["address_id"],
            foreign_table: "addresses",
            foreign_columns: ["id"],
            on_delete: "no action",
            on_update: "no action"
        },
    }
}