use crate::define_table_schema;
use crate::schema::generator::diesel_schema_definition::{types::*, DieselTableDefinition};

pub struct FaqCategoriesTable;

define_table_schema! {
    hypertable: false,
    fields: {
        system_fields!(),
        faq_id: nullable(text()),
        category: nullable(text()),
        order: nullable(integer()), default: "0",
    },
    indexes: {
        system_indexes!("faq_categories"),
        idx_faq_categories_faq_id: { columns: ["faq_id"], unique: false, type: "btree" },
        idx_faq_categories_category: { columns: ["category"], unique: false, type: "btree" },
        idx_faq_categories_order: { columns: ["order"], unique: false, type: "btree" },
    },
    foreign_keys: {
        system_foreign_keys!("faq_categories"),
        fk_faq_categories_faq_id: { columns: ["faq_id"], foreign_table: "faqs", foreign_columns: ["id"], on_delete: "no action", on_update: "no action" },
    }
}
