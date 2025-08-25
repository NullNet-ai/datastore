use crate::generated::schema::generator::diesel_schema_definition::{
    types::*, DieselTableDefinition,
};
use crate::define_table_schema;
use crate::{system_fields, system_foreign_keys, system_indexes};

/// Test hypertable for time-series data
pub struct TestHypertableTable;

define_table_schema! {
    hypertable: true,
    fields: {
        // System fields - common across all tables
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
        // System field indexes
        system_indexes!("test_hypertable"),

        // Custom table-specific indexes
        idx_test_hypertable_sensor: {
            columns: ["sensor_id"],
            unique: false,
            type: "btree"
        },
        idx_test_hypertable_location: {
            columns: ["location"],
            unique: false,
            type: "btree"
        }
    },
    foreign_keys: {
        // System field foreign keys
        // system_foreign_keys!("test_hypertable")
    }
}

impl DieselTableDefinition for TestHypertableTable {
    fn is_hypertable() -> bool {
        true
    }
}
