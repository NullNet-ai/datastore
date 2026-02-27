use crate::define_table_schema;
use crate::generated::schema::generator::diesel_schema_definition::{
    types::*, DieselTableDefinition,
};
use crate::{system_fields, system_foreign_keys, system_indexes};

/**
 * Sample table for testing timestamp/timestamptz type-based handling in find, where, order_by, search.
 * - some_test_field: type timestamp (without time zone)
 * - timestamp2: type timestamptz (with time zone)
 */
/// Sample table with timestamp and timestamptz columns (for type-detection tests)
pub struct SampleChecksTable;

define_table_schema! {
    hypertable: false,
    fields: {
        // System fields - common across all tables ( REQUIRED )
        system_fields!(),

        // some_test_field = "some test field", type timestamp
        some_test_field: nullable(timestamp()),

        // timestamp2, type timestamptz
        timestamp2: nullable(timestamptz()),
    },
    indexes: {
        system_indexes!("sample_checks"),
    },
    foreign_keys: {
        system_foreign_keys!("sample_checks"),
    }
}
