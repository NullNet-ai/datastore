use crate::define_table_schema;
use crate::generated::schema::generator::diesel_schema_definition::{
    types::*, DieselTableDefinition,
};
use crate::{system_fields, system_foreign_keys, system_indexes};

/**
 * Test table for validating store-generator.
 * Table name from file: demo_items
 */
/// Demo items table - used to verify store-generator works
pub struct DemoItemsTable;

define_table_schema! {
    hypertable: false,
    fields: {
        // System fields - common across all tables ( REQUIRED )
        system_fields!(),

        // Demo-specific fields
        title: nullable(text()),
        description: nullable(text()),
        quantity: nullable(integer()),
        name: nullable(text()),
    },
    indexes: {
        // System field indexes ( REQUIRED )
        system_indexes!("demo_items"),

        // Custom table-specific indexes
        idx_demo_items_title: {
            columns: ["title"],
            unique: false,
            type: "btree"
        },
        idx_demo_items_name: {
            columns: ["name"],
            unique: false,
            type: "btree"
        },
    },
    foreign_keys: {
        // System field foreign keys ( REQUIRED )
        system_foreign_keys!("demo_items"),
    }
}
