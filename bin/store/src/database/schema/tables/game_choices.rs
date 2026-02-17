use crate::schema::generator::diesel_schema_definition::{
    DieselTableDefinition, types::*
};
use crate::define_table_schema;

pub struct GameChoicesTable;

define_table_schema! {
    hypertable: false,
    fields: {
        system_fields!(),
        question_id: nullable(text()),
        choice_text: nullable(text()), default: "''",
        is_correct: nullable(boolean()), default: "false",
    },
    indexes: {
        system_indexes!("game_choices"),
        idx_game_choices_question_id: { columns: ["question_id"], unique: false, type: "btree" },
        idx_game_choices_choice_text: { columns: ["choice_text"], unique: false, type: "btree" },
        idx_game_choices_is_correct: { columns: ["is_correct"], unique: false, type: "btree" },
    },
    foreign_keys: {
        system_foreign_keys!("game_choices"),
        fk_game_choices_question_id: { columns: ["question_id"], foreign_table: "game_questions", foreign_columns: ["id"], on_delete: "no action", on_update: "no action" },
    }
}