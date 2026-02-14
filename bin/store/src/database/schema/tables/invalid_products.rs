use crate::define_table_schema;
use crate::generated::schema::generator::diesel_schema_definition::{
    types::*, DieselTableDefinition,
};
use crate::{system_fields, system_foreign_keys, system_indexes};

/**
 * Intentionally invalid table for testing store-generator validation.
 * Violations:
 * - Table name "invalid_product" is singular (should be plural like "invalid_products")
 * - Index "bad_index_name" should be idx_invalid_product_sku
 * - Foreign key "wrong_fk_name" should be fk_invalid_product_reference_id
 */
pub struct InvalidProductsTable;

define_table_schema! {
    hypertable: false,
    fields: {
        system_fields!(),

        sku: nullable(text()),
        reference_id: nullable(text()),
    },
    indexes: {
        system_indexes!("invalid_products"),

        // INVALID: should be idx_invalid_product_sku
        idx_invalid_products_sku: { columns: ["sku"], unique: false, type: "btree" },
    },
    foreign_keys: {
        system_foreign_keys!("invalid_products"),

        // INVALID: should be fk_invalid_product_reference_id
        wrong_fk_name: {
            columns: ["reference_id"],
            foreign_table: "organizations",
            foreign_columns: ["id"],
            on_delete: "no action",
            on_update: "no action"
        },
    }
}
