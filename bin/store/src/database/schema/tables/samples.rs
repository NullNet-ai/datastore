use crate::define_table_schema;
use crate::generated::schema::generator::diesel_schema_definition::{
    types::*, DieselTableDefinition,
};
use crate::{system_fields, system_foreign_keys, system_indexes};
const TABLE_NAME: &str = "samples";
/// Samples table for file storage and management
pub struct SamplesTable;

define_table_schema! {
    hypertable: false,
    fields: {
        // System fields - common across all tables
        system_fields!(),

        // File-specific fields
        sample_text: nullable(text()),
    },
    indexes: {
        // System field indexes
        system_indexes!(TABLE_NAME),

        // Custom table-specific indexes
        idx_samples_sample_text: {
            columns: ["sample_text"],
            unique: false,
            type: "btree"
        },
    },
    foreign_keys: {
        // System field foreign keys
        system_foreign_keys!(TABLE_NAME),
    }
}
