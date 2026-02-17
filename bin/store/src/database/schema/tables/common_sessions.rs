use crate::define_table_schema;
use crate::schema::generator::diesel_schema_definition::{types::*, DieselTableDefinition};

pub struct CommonSessionsTable;

define_table_schema! {
    hypertable: false,
    fields: {
        system_fields!(),
        user_id: nullable(text()),
        session_token: nullable(text()),
        route: nullable(text()),
        session_status: nullable(text()),
        activity_status: nullable(text()),
        last_activity_at: nullable(timestamp_with_time_zone()),
        expires_at: nullable(timestamp_with_time_zone()),
        device_details: nullable(text()),
        ip_address: nullable(text()),
        portal_application_type: nullable(text()),
    },
    indexes: {
        system_indexes!("common_sessions"),
        idx_common_sessions_user_id: { columns: ["user_id"], unique: false, type: "btree" },
        idx_common_sessions_session_token: { columns: ["session_token"], unique: false, type: "btree" },
        idx_common_sessions_route: { columns: ["route"], unique: false, type: "btree" },
        idx_common_sessions_session_status: { columns: ["session_status"], unique: false, type: "btree" },
        idx_common_sessions_activity_status: { columns: ["activity_status"], unique: false, type: "btree" },
        idx_common_sessions_last_activity_at: { columns: ["last_activity_at"], unique: false, type: "btree" },
        idx_common_sessions_expires_at: { columns: ["expires_at"], unique: false, type: "btree" },
        idx_common_sessions_device_details: { columns: ["device_details"], unique: false, type: "btree" },
        idx_common_sessions_ip_address: { columns: ["ip_address"], unique: false, type: "btree" },
        idx_common_sessions_portal_application_type: { columns: ["portal_application_type"], unique: false, type: "btree" },
    },
    foreign_keys: {
        system_foreign_keys!("common_sessions"),
        fk_common_sessions_user_id: { columns: ["user_id"], foreign_table: "contacts", foreign_columns: ["id"], on_delete: "no action", on_update: "no action" },
    }
}
