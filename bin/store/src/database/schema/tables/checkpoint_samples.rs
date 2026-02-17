use crate::define_table_schema;
use crate::generated::schema::generator::diesel_schema_definition::{
    types::*, DieselTableDefinition,
};
use crate::{system_fields, system_foreign_keys, system_indexes};

/**
 * Sample table for testing checkpoint flow.
 * For defining proper diesel types check: bin/store/src/builders/generator/README.md
 */
/// Checkpoint samples – minimal table for checkpoint testing
pub struct CheckpointSamplesTable;

define_table_schema! {
    hypertable: false,
    fields: {
        // System fields - common across all tables ( REQUIRED )
        system_fields!(),

        // Sample fields for checkpoint testing
        label: nullable(text()),
        value: nullable(integer()),
    },
    indexes: {
        // System field indexes ( REQUIRED )
        system_indexes!("checkpoint_samples"),

        // Custom table-specific indexes
        idx_checkpoint_samples_label: {
            columns: ["label"],
            unique: false,
            type: "btree"
        },
    },
    foreign_keys: {
        // System field foreign keys ( REQUIRED )
        system_foreign_keys!("checkpoint_samples"),
    }
}
