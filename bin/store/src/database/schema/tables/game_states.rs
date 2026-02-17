use crate::schema::generator::diesel_schema_definition::{
    DieselTableDefinition, types::*
};
use crate::define_table_schema;

pub struct GameStatesTable;

define_table_schema! {
    hypertable: false,
    fields: {
        system_fields!(),
        user_id: nullable(text()), default: "''",
        course_id: nullable(text()), default: "''",
        story_id: nullable(text()), default: "''",
        episode_id: nullable(text()), default: "''",
        chapter_number: nullable(text()), default: "''",
        game_token: nullable(jsonb()), default: "'{}'::jsonb",
    },
    indexes: {
        system_indexes!("game_states"),
        idx_game_states_user_id: { columns: ["user_id"], unique: false, type: "btree" },
        idx_game_states_course_id: { columns: ["course_id"], unique: false, type: "btree" },
        idx_game_states_story_id: { columns: ["story_id"], unique: false, type: "btree" },
        idx_game_states_episode_id: { columns: ["episode_id"], unique: false, type: "btree" },
        idx_game_states_chapter_number: { columns: ["chapter_number"], unique: false, type: "btree" },
    },
    foreign_keys: {
        system_foreign_keys!("game_states"),
    }
}