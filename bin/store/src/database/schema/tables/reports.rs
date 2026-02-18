use crate::define_table_schema;
use crate::schema::generator::diesel_schema_definition::{types::*, DieselTableDefinition};

pub struct ReportsTable;

define_table_schema! {
    hypertable: false,
    fields: {
        system_fields!(),
        report_type: nullable(array(text())),
        job_status: nullable(text()),
        start_date: nullable(text()),
        start_time: nullable(text()), default: "'00:00'",
        end_date: nullable(text()),
        end_time: nullable(text()), default: "'00:00'",
        progress: nullable(text()), default: "'0%'",
        reason: nullable(text()), default: "''",
        department_id: nullable(text()),
        district_id: nullable(text()),
        school_id: nullable(text()),
        course_id: nullable(text()),
        story_id: nullable(text()),
        record_count: nullable(text()), default: "'-'",
    },
    indexes: {
        system_indexes!("reports"),
        idx_reports_report_type: { columns: ["report_type"], unique: false, type: "btree" },
        idx_reports_job_status: { columns: ["job_status"], unique: false, type: "btree" },
        idx_reports_start_date: { columns: ["start_date"], unique: false, type: "btree" },
        idx_reports_start_time: { columns: ["start_time"], unique: false, type: "btree" },
        idx_reports_end_date: { columns: ["end_date"], unique: false, type: "btree" },
        idx_reports_end_time: { columns: ["end_time"], unique: false, type: "btree" },
        idx_reports_progress: { columns: ["progress"], unique: false, type: "btree" },
        idx_reports_reason: { columns: ["reason"], unique: false, type: "btree" },
        idx_reports_department_id: { columns: ["department_id"], unique: false, type: "btree" },
        idx_reports_district_id: { columns: ["district_id"], unique: false, type: "btree" },
        idx_reports_school_id: { columns: ["school_id"], unique: false, type: "btree" },
        idx_reports_course_id: { columns: ["course_id"], unique: false, type: "btree" },
        idx_reports_story_id: { columns: ["story_id"], unique: false, type: "btree" },
        idx_reports_record_count: { columns: ["record_count"], unique: false, type: "btree" },
    },
    foreign_keys: {
        system_foreign_keys!("reports"),
        fk_reports_department_id: { columns: ["department_id"], foreign_table: "organizations", foreign_columns: ["id"], on_delete: "no action", on_update: "no action" },
        fk_reports_district_id: { columns: ["district_id"], foreign_table: "organizations", foreign_columns: ["id"], on_delete: "no action", on_update: "no action" },
        fk_reports_school_id: { columns: ["school_id"], foreign_table: "organizations", foreign_columns: ["id"], on_delete: "no action", on_update: "no action" },
        fk_reports_course_id: { columns: ["course_id"], foreign_table: "courses", foreign_columns: ["id"], on_delete: "no action", on_update: "no action" },
        fk_reports_story_id: { columns: ["story_id"], foreign_table: "stories", foreign_columns: ["id"], on_delete: "no action", on_update: "no action" },
    }
}
