use crate::define_table_schema;
use crate::schema::generator::diesel_schema_definition::{types::*, DieselTableDefinition};

pub struct CourseCompletionReportsTable;

define_table_schema! {
    hypertable: false,
    fields: {
        system_fields!(),
        contact_code: nullable(text()),
        role: nullable(text()),
        student_course_status: nullable(text()),
        status_date: nullable(text()),
        status_time: nullable(text()), default: "'00:00'",
        report_id: nullable(text()),
    },
    indexes: {
        system_indexes!("course_completion_reports"),
        idx_course_completion_reports_contact_code: { columns: ["contact_code"], unique: false, type: "btree" },
        idx_course_completion_reports_role: { columns: ["role"], unique: false, type: "btree" },
        idx_course_completion_reports_student_course_status: { columns: ["student_course_status"], unique: false, type: "btree" },
        idx_course_completion_reports_status_date: { columns: ["status_date"], unique: false, type: "btree" },
        idx_course_completion_reports_status_time: { columns: ["status_time"], unique: false, type: "btree" },
        idx_course_completion_reports_report_id: { columns: ["report_id"], unique: false, type: "btree" },
    },
    foreign_keys: {
        system_foreign_keys!("course_completion_reports"),
        fk_course_completion_reports_report_id: { columns: ["report_id"], foreign_table: "reports", foreign_columns: ["id"], on_delete: "no action", on_update: "no action" },
    }
}
