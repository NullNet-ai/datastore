use crate::schema::generator::diesel_schema_definition::{
    DieselTableDefinition, types::*
};
use crate::define_table_schema;

pub struct StudentStatsTable;

define_table_schema! {
    hypertable: false,
    fields: {
        system_fields!(),
        student_id: nullable(text()),
        classroom_id: nullable(text()),
        course_id: nullable(text()),
        story_id: nullable(text()),
        total_episodes_started: nullable(integer()), default: "0",
        total_episodes_completed: nullable(integer()), default: "0",
        total_chapters_started: nullable(integer()), default: "0",
        total_chapters_completed: nullable(integer()), default: "0",
        total_questions_answered: nullable(integer()), default: "0",
        total_episodes_completed_in_string: nullable(text()), default: "''",
        total_episodes: nullable(integer()), default: "0",
        last_activity: nullable(timestamptz()),
        last_activity_date: nullable(timestamptz()),
        last_activity_time: nullable(text()), default: "'00:00'",
    },
    indexes: {
        system_indexes!("student_stats"),
        idx_student_stats_student_id: { columns: ["student_id"], unique: false, type: "btree" },
        idx_student_stats_classroom_id: { columns: ["classroom_id"], unique: false, type: "btree" },
        idx_student_stats_course_id: { columns: ["course_id"], unique: false, type: "btree" },
        idx_student_stats_story_id: { columns: ["story_id"], unique: false, type: "btree" },
        idx_student_stats_total_episodes_started: { columns: ["total_episodes_started"], unique: false, type: "btree" },
        idx_student_stats_total_episodes_completed: { columns: ["total_episodes_completed"], unique: false, type: "btree" },
        idx_student_stats_total_chapters_started: { columns: ["total_chapters_started"], unique: false, type: "btree" },
        idx_student_stats_total_chapters_completed: { columns: ["total_chapters_completed"], unique: false, type: "btree" },
        idx_student_stats_total_questions_answered: { columns: ["total_questions_answered"], unique: false, type: "btree" },
        idx_student_stats_total_episodes_completed_in_string: { columns: ["total_episodes_completed_in_string"], unique: false, type: "btree" },
        idx_student_stats_total_episodes: { columns: ["total_episodes"], unique: false, type: "btree" },
        idx_student_stats_last_activity: { columns: ["last_activity"], unique: false, type: "btree" },
        idx_student_stats_last_activity_date: { columns: ["last_activity_date"], unique: false, type: "btree" },
    },
    foreign_keys: {
        system_foreign_keys!("student_stats"),
        fk_student_stats_student_id: { columns: ["student_id"], foreign_table: "contacts", foreign_columns: ["id"], on_delete: "no action", on_update: "no action" },
        fk_student_stats_classroom_id: { columns: ["classroom_id"], foreign_table: "classrooms", foreign_columns: ["id"], on_delete: "no action", on_update: "no action" },
        fk_student_stats_course_id: { columns: ["course_id"], foreign_table: "courses", foreign_columns: ["id"], on_delete: "no action", on_update: "no action" },
        fk_student_stats_story_id: { columns: ["story_id"], foreign_table: "stories", foreign_columns: ["id"], on_delete: "no action", on_update: "no action" },
    }
}