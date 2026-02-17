use crate::schema::generator::diesel_schema_definition::{
    DieselTableDefinition, types::*
};
use crate::define_table_schema;

pub struct EpisodeEventsTable;

define_table_schema! {
    hypertable: false,
    fields: {
        system_fields!(),
        user_id: nullable(text()),
        course_id: nullable(text()),
        story_id: nullable(text()),
        episode_id: nullable(text()),
        event_type: nullable(text()),
        game_status: nullable(text()),
    },
    indexes: {
        system_indexes!("episode_events"),
        idx_episode_events_user_id: { columns: ["user_id"], unique: false, type: "btree" },
        idx_episode_events_course_id: { columns: ["course_id"], unique: false, type: "btree" },
        idx_episode_events_story_id: { columns: ["story_id"], unique: false, type: "btree" },
        idx_episode_events_episode_id: { columns: ["episode_id"], unique: false, type: "btree" },
        idx_episode_events_event_type: { columns: ["event_type"], unique: false, type: "btree" },
        idx_episode_events_game_status: { columns: ["game_status"], unique: false, type: "btree" },
    },
    foreign_keys: {
        system_foreign_keys!("episode_events"),
        fk_episode_events_user_id: { columns: ["user_id"], foreign_table: "contacts", foreign_columns: ["id"], on_delete: "no action", on_update: "no action" },
        fk_episode_events_course_id: { columns: ["course_id"], foreign_table: "courses", foreign_columns: ["id"], on_delete: "no action", on_update: "no action" },
        fk_episode_events_story_id: { columns: ["story_id"], foreign_table: "stories", foreign_columns: ["id"], on_delete: "no action", on_update: "no action" },
        fk_episode_events_episode_id: { columns: ["episode_id"], foreign_table: "episodes", foreign_columns: ["id"], on_delete: "no action", on_update: "no action" },
    }
}