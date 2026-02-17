use crate::schema::generator::diesel_schema_definition::{
    DieselTableDefinition, types::*
};
use crate::define_table_schema;

pub struct FaqQuestionsTable;

define_table_schema! {
    hypertable: false,
    fields: {
        system_fields!(),
        faq_id: nullable(text()),
        category_id: nullable(text()),
        question: nullable(text()),
        order: nullable(integer()), default: "0",
    },
    indexes: {
        system_indexes!("faq_questions"),
        idx_faq_questions_faq_id: { columns: ["faq_id"], unique: false, type: "btree" },
        idx_faq_questions_category_id: { columns: ["category_id"], unique: false, type: "btree" },
        idx_faq_questions_question: { columns: ["question"], unique: false, type: "btree" },
        idx_faq_questions_order: { columns: ["order"], unique: false, type: "btree" },
    },
    foreign_keys: {
        system_foreign_keys!("faq_questions"),
        fk_faq_questions_faq_id: { columns: ["faq_id"], foreign_table: "faqs", foreign_columns: ["id"], on_delete: "no action", on_update: "no action" },
        fk_faq_questions_category_id: { columns: ["category_id"], foreign_table: "faq_categories", foreign_columns: ["id"], on_delete: "no action", on_update: "no action" },
    }
}