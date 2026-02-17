use crate::define_table_schema;
use crate::generated::schema::generator::diesel_schema_definition::{
    types::*, DieselTableDefinition,
};
use crate::{system_fields, system_foreign_keys, system_indexes};

/**
 * For defining proper diesel types check it here: bin/store/src/builders/generator/README.md
 */

/// Test hypertable for time-series data
pub struct TestHypertablesTable;

define_table_schema! {
    hypertable: true,
    fields: {
        // System fields - common across all tables ( REQUIRED )
        system_fields!(),

        timestamp: nullable(timestamptz()), primary_key: true,

        hypertable_timestamp: nullable(text()), primary_key: false,

        // Additional fields for time-series data
        sensor_id: nullable(text()),
        temperature: nullable(integer()),
        humidity: nullable(integer()),
        location: nullable(text()),
    },
    indexes: {
        // System field indexes ( REQUIRED )
        system_indexes!("test_hypertables"),

        // Custom table-specific indexes
        idx_test_hypertables_location: {
            columns: ["location"],
            unique: false,
            type: "btree"
        }
    },
    foreign_keys: {}
}

impl DieselTableDefinition for TestHypertablesTable {
    fn is_hypertable() -> bool {
        true
    }
}
