use crate::define_table_schema;
use crate::generated::schema::generator::diesel_schema_definition::{
    types::*, DieselTableDefinition,
};
use crate::{system_fields, system_foreign_keys, system_indexes};

/**
 * Test table for validating store-generator flow.
 * Table name from file: test_products
 */
/// Test products table - used to verify store-generator flow
pub struct TestProductsTable;

define_table_schema! {
    hypertable: false,
    fields: {
        // System fields - common across all tables ( REQUIRED )
        system_fields!(),

        // Product-specific fields
        sku: nullable(text()),
        price: nullable(integer()),
        in_stock: nullable(bool()),
    },
    indexes: {
        // System field indexes ( REQUIRED )
        system_indexes!("test_products"),

        // Custom table-specific indexes
        idx_test_products_sku: {
            columns: ["sku"],
            unique: true,
            type: "btree"
        },
        idx_test_products_price: {
            columns: ["price"],
            unique: false,
            type: "btree"
        },
    },
    foreign_keys: {
        // System field foreign keys ( REQUIRED )
        system_foreign_keys!("test_products"),
    }
}
