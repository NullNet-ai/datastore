use crate::schema::generator::diesel_schema_definition::{
    DieselTableDefinition, types::*
};
use crate::define_table_schema;

pub struct GameQuestionsTable;

define_table_schema! {
    hypertable: false,
    fields: {
        system_fields!(),
        course_id: nullable(text()),
        course_title: nullable(text()), default: "''",
        story_id: nullable(text()),
        story_title: nullable(text()), default: "''",
        episode_number: nullable(integer()), default: "0",
        question: nullable(text()), default: "''",
        topic: nullable(text()), default: "''",
    },
    indexes: {
        system_indexes!("game_questions"),
        idx_game_questions_course_id: { columns: ["course_id"], unique: false, type: "btree" },
        idx_game_questions_course_title: { columns: ["course_title"], unique: false, type: "btree" },
        idx_game_questions_story_id: { columns: ["story_id"], unique: false, type: "btree" },
        idx_game_questions_story_title: { columns: ["story_title"], unique: false, type: "btree" },
        idx_game_questions_episode_number: { columns: ["episode_number"], unique: false, type: "btree" },
        idx_game_questions_question: { columns: ["question"], unique: false, type: "btree" },
        idx_game_questions_topic: { columns: ["topic"], unique: false, type: "btree" },
    },
    foreign_keys: {
        system_foreign_keys!("game_questions"),
        fk_game_questions_course_id: { columns: ["course_id"], foreign_table: "courses", foreign_columns: ["id"], on_delete: "no action", on_update: "no action" },
        fk_game_questions_story_id: { columns: ["story_id"], foreign_table: "stories", foreign_columns: ["id"], on_delete: "no action", on_update: "no action" },
    }
}