use crate::schema::generator::diesel_schema_definition::{
    DieselTableDefinition, types::*
};
use crate::define_table_schema;

pub struct GameResponsesTable;

define_table_schema! {
    hypertable: false,
    fields: {
        system_fields!(),
        user_id: nullable(text()),
        course_id: nullable(text()),
        story_id: nullable(text()),
        episode_id: nullable(text()),
        chapter_number: nullable(integer()),
        question_text: nullable(text()),
        selected_answer: nullable(text()),
        game_status: nullable(text()),
        options: nullable(jsonb()), default: "'[]'::jsonb",
    },
    indexes: {
        system_indexes!("game_responses"),
        idx_game_responses_user_id: { columns: ["user_id"], unique: false, type: "btree" },
        idx_game_responses_course_id: { columns: ["course_id"], unique: false, type: "btree" },
        idx_game_responses_story_id: { columns: ["story_id"], unique: false, type: "btree" },
        idx_game_responses_episode_id: { columns: ["episode_id"], unique: false, type: "btree" },
        idx_game_responses_chapter_number: { columns: ["chapter_number"], unique: false, type: "btree" },
        idx_game_responses_question_text: { columns: ["question_text"], unique: false, type: "btree" },
        idx_game_responses_selected_answer: { columns: ["selected_answer"], unique: false, type: "btree" },
        idx_game_responses_game_status: { columns: ["game_status"], unique: false, type: "btree" },
        idx_game_responses_options: { columns: ["options"], unique: false, type: "btree" },
    },
    foreign_keys: {
        system_foreign_keys!("game_responses"),
        fk_game_responses_user_id: { columns: ["user_id"], foreign_table: "contacts", foreign_columns: ["id"], on_delete: "no action", on_update: "no action" },
        fk_game_responses_course_id: { columns: ["course_id"], foreign_table: "courses", foreign_columns: ["id"], on_delete: "no action", on_update: "no action" },
        fk_game_responses_story_id: { columns: ["story_id"], foreign_table: "stories", foreign_columns: ["id"], on_delete: "no action", on_update: "no action" },
        fk_game_responses_episode_id: { columns: ["episode_id"], foreign_table: "episodes", foreign_columns: ["id"], on_delete: "no action", on_update: "no action" },
    }
}