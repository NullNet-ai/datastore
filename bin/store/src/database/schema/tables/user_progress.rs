use crate::schema::generator::diesel_schema_definition::{
    DieselTableDefinition, types::*
};
use crate::define_table_schema;

pub struct UserProgressTable;

define_table_schema! {
    hypertable: false,
    fields: {
        system_fields!(),
        user_id: nullable(text()),
        total_episodes_started: nullable(integer()), default: "0",
        total_episodes_completed: nullable(integer()), default: "0",
        total_chapters_started: nullable(integer()), default: "0",
        total_chapters_completed: nullable(integer()), default: "0",
        total_questions_answered: nullable(integer()), default: "0",
        last_activity: nullable(timestamptz()),
    },
    indexes: {
        system_indexes!("user_progress"),
        idx_user_progress_user_id: { columns: ["user_id"], unique: true, type: "btree" },
        idx_user_progress_total_episodes_started: { columns: ["total_episodes_started"], unique: false, type: "btree" },
        idx_user_progress_total_episodes_completed: { columns: ["total_episodes_completed"], unique: false, type: "btree" },
        idx_user_progress_total_chapters_started: { columns: ["total_chapters_started"], unique: false, type: "btree" },
        idx_user_progress_total_chapters_completed: { columns: ["total_chapters_completed"], unique: false, type: "btree" },
        idx_user_progress_total_questions_answered: { columns: ["total_questions_answered"], unique: false, type: "btree" },
        idx_user_progress_last_activity: { columns: ["last_activity"], unique: false, type: "btree" },
    },
    foreign_keys: {
        system_foreign_keys!("user_progress"),
        fk_user_progress_user_id: { columns: ["user_id"], foreign_table: "contacts", foreign_columns: ["id"], on_delete: "no action", on_update: "no action" },
    }
}