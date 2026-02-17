use crate::define_table_schema;
use crate::schema::generator::diesel_schema_definition::{types::*, DieselTableDefinition};

pub struct ClassroomStatsTable;

define_table_schema! {
    hypertable: false,
    fields: {
        system_fields!(),
        classroom_id: nullable(text()),
        story_id: nullable(text()),
        student_id: nullable(text()),
        course_id: nullable(text()),
        number_of_students: nullable(text()), default: "'0'",
        student_completed: nullable(text()), default: "'0/0'",
        progress_status: nullable(text()), default: "'Not Started'",
        progress_start_date: nullable(timestamptz()),
        progress_start_time: nullable(text()), default: "'00:00'",
        progress_start_date_string: nullable(text()), default: "''",
        completed_date: nullable(timestamptz()),
        completed_time: nullable(text()), default: "'00:00'",
    },
    indexes: {
        system_indexes!("classroom_stats"),
        idx_classroom_stats_classroom_id: { columns: ["classroom_id"], unique: false, type: "btree" },
        idx_classroom_stats_story_id: { columns: ["story_id"], unique: false, type: "btree" },
        idx_classroom_stats_student_id: { columns: ["student_id"], unique: false, type: "btree" },
        idx_classroom_stats_course_id: { columns: ["course_id"], unique: false, type: "btree" },
        idx_classroom_stats_number_of_students: { columns: ["number_of_students"], unique: false, type: "btree" },
        idx_classroom_stats_student_completed: { columns: ["student_completed"], unique: false, type: "btree" },
        idx_classroom_stats_progress_status: { columns: ["progress_status"], unique: false, type: "btree" },
        idx_classroom_stats_progress_start_date: { columns: ["progress_start_date"], unique: false, type: "btree" },
        idx_classroom_stats_progress_start_time: { columns: ["progress_start_time"], unique: false, type: "btree" },
        idx_classroom_stats_progress_start_date_string: { columns: ["progress_start_date_string"], unique: false, type: "btree" },
        idx_classroom_stats_completed_date: { columns: ["completed_date"], unique: false, type: "btree" },
        idx_classroom_stats_completed_time: { columns: ["completed_time"], unique: false, type: "btree" },
    },
    foreign_keys: {
        system_foreign_keys!("classroom_stats"),
        fk_classroom_stats_classroom_id: { columns: ["classroom_id"], foreign_table: "classrooms", foreign_columns: ["id"], on_delete: "no action", on_update: "no action" },
        fk_classroom_stats_story_id: { columns: ["story_id"], foreign_table: "stories", foreign_columns: ["id"], on_delete: "no action", on_update: "no action" },
        fk_classroom_stats_student_id: { columns: ["student_id"], foreign_table: "contacts", foreign_columns: ["id"], on_delete: "no action", on_update: "no action" },
        fk_classroom_stats_course_id: { columns: ["course_id"], foreign_table: "courses", foreign_columns: ["id"], on_delete: "no action", on_update: "no action" },
    }
}
