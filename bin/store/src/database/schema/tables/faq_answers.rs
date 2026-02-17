use crate::schema::generator::diesel_schema_definition::{
    DieselTableDefinition, types::*
};
use crate::define_table_schema;

pub struct FaqAnswersTable;

define_table_schema! {
    hypertable: false,
    fields: {
        system_fields!(),
        faq_id: nullable(text()),
        category_id: nullable(text()),
        question_id: nullable(text()),
        answer: nullable(text()),
        order: nullable(integer()), default: "0",
    },
    indexes: {
        system_indexes!("faq_answers"),
        idx_faq_answers_faq_id: { columns: ["faq_id"], unique: false, type: "btree" },
        idx_faq_answers_category_id: { columns: ["category_id"], unique: false, type: "btree" },
        idx_faq_answers_question_id: { columns: ["question_id"], unique: false, type: "btree" },
        idx_faq_answers_answer: { columns: ["answer"], unique: false, type: "btree" },
        idx_faq_answers_order: { columns: ["order"], unique: false, type: "btree" },
    },
    foreign_keys: {
        system_foreign_keys!("faq_answers"),
        fk_faq_answers_faq_id: { columns: ["faq_id"], foreign_table: "faqs", foreign_columns: ["id"], on_delete: "no action", on_update: "no action" },
        fk_faq_answers_category_id: { columns: ["category_id"], foreign_table: "faq_categories", foreign_columns: ["id"], on_delete: "no action", on_update: "no action" },
        fk_faq_answers_question_id: { columns: ["question_id"], foreign_table: "faq_questions", foreign_columns: ["id"], on_delete: "no action", on_update: "no action" },
    }
}