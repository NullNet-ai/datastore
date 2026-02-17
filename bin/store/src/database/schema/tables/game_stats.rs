use crate::schema::generator::diesel_schema_definition::{
    DieselTableDefinition, types::*
};
use crate::define_table_schema;

pub struct GameStatsTable;

define_table_schema! {
    hypertable: false,
    fields: {
        system_fields!(),
        contact_id: nullable(text()),
        event_id: nullable(text()),
        event_type: nullable(text()),
        course_id: nullable(text()),
        story_id: nullable(text()),
        episode_id: nullable(text()),
        classroom_id: nullable(text()),
        chapter_number: nullable(text()),
        volume_level: nullable(text()),
        question: nullable(text()),
        choices: nullable(jsonb()), default: "'[]'::jsonb",
        selected_choice: nullable(text()),
    },
    indexes: {
        system_indexes!("game_stats"),
        idx_game_stats_contact_id: { columns: ["contact_id"], unique: false, type: "btree" },
        idx_game_stats_event_id: { columns: ["event_id"], unique: false, type: "btree" },
        idx_game_stats_event_type: { columns: ["event_type"], unique: false, type: "btree" },
        idx_game_stats_course_id: { columns: ["course_id"], unique: false, type: "btree" },
        idx_game_stats_story_id: { columns: ["story_id"], unique: false, type: "btree" },
        idx_game_stats_episode_id: { columns: ["episode_id"], unique: false, type: "btree" },
        idx_game_stats_classroom_id: { columns: ["classroom_id"], unique: false, type: "btree" },
        idx_game_stats_chapter_number: { columns: ["chapter_number"], unique: false, type: "btree" },
        idx_game_stats_volume_level: { columns: ["volume_level"], unique: false, type: "btree" },
        idx_game_stats_question: { columns: ["question"], unique: false, type: "btree" },
        idx_game_stats_choices: { columns: ["choices"], unique: false, type: "btree" },
        idx_game_stats_selected_choice: { columns: ["selected_choice"], unique: false, type: "btree" },
    },
    foreign_keys: {
        system_foreign_keys!("game_stats"),
        fk_game_stats_contact_id: { columns: ["contact_id"], foreign_table: "contacts", foreign_columns: ["id"], on_delete: "no action", on_update: "no action" },
        fk_game_stats_course_id: { columns: ["course_id"], foreign_table: "courses", foreign_columns: ["id"], on_delete: "no action", on_update: "no action" },
        fk_game_stats_story_id: { columns: ["story_id"], foreign_table: "stories", foreign_columns: ["id"], on_delete: "no action", on_update: "no action" },
        fk_game_stats_episode_id: { columns: ["episode_id"], foreign_table: "episodes", foreign_columns: ["id"], on_delete: "no action", on_update: "no action" },
        fk_game_stats_classroom_id: { columns: ["classroom_id"], foreign_table: "classrooms", foreign_columns: ["id"], on_delete: "no action", on_update: "no action" },
    }
}