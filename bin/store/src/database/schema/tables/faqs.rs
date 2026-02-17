use crate::schema::generator::diesel_schema_definition::{
    DieselTableDefinition, types::*
};
use crate::define_table_schema;

pub struct FaqsTable;

define_table_schema! {
    hypertable: false,
    fields: {
        system_fields!(),
        target_app: nullable(text()),
    },
    indexes: {
        system_indexes!("faqs"),
        idx_faqs_target_app: { columns: ["target_app"], unique: false, type: "btree" },
    },
    foreign_keys: {
        system_foreign_keys!("faqs"),
    }
}