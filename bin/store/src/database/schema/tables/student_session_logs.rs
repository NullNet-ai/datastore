use crate::schema::generator::diesel_schema_definition::{
    DieselTableDefinition, types::*
};
use crate::define_table_schema;

pub struct StudentSessionLogsTable;

define_table_schema! {
    hypertable: false,
    fields: {
        system_fields!(),
        user_id: nullable(text()),
        action: nullable(text()),
        session_token: nullable(text()),
        timestamp: nullable(timestamptz()),
        device_details: nullable(text()),
        ip_address: nullable(text()),
        route: nullable(text()),
        reason: nullable(text()),
    },
    indexes: {
        system_indexes!("student_session_logs"),
        idx_student_session_logs_user_id: { columns: ["user_id"], unique: false, type: "btree" },
        idx_student_session_logs_action: { columns: ["action"], unique: false, type: "btree" },
        idx_student_session_logs_session_token: { columns: ["session_token"], unique: false, type: "btree" },
        idx_student_session_logs_timestamp: { columns: ["timestamp"], unique: false, type: "btree" },
        idx_student_session_logs_device_details: { columns: ["device_details"], unique: false, type: "btree" },
        idx_student_session_logs_ip_address: { columns: ["ip_address"], unique: false, type: "btree" },
        idx_student_session_logs_route: { columns: ["route"], unique: false, type: "btree" },
        idx_student_session_logs_reason: { columns: ["reason"], unique: false, type: "btree" },
    },
    foreign_keys: {
        system_foreign_keys!("student_session_logs"),
        fk_student_session_logs_user_id: { columns: ["user_id"], foreign_table: "contacts", foreign_columns: ["id"], on_delete: "no action", on_update: "no action" },
    }
}