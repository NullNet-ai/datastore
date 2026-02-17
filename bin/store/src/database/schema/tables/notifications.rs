use crate::define_table_schema;
use crate::schema::generator::diesel_schema_definition::{types::*, DieselTableDefinition};
use crate::{system_fields, system_foreign_keys, system_indexes};

/**
 * For defining proper diesel types check it here: bin/store/src/builders/generator/README.md
 */

/// Notifications table for storing user notifications
pub struct NotificationsTable;

define_table_schema! {
    hypertable: false,
    fields: {
        // System fields - common across all tables ( REQUIRED )
        system_fields!(),

        // Notifications specific fields
        title: nullable(text()),
        description: nullable(text()),
        event_timestamp: nullable(text()),
        link: nullable(text()), default: "''",
        icon: nullable(text()), default: "''",
        source: nullable(text()),
        is_pinned: nullable(boolean()), default: "false",
        recipient_id: nullable(text()),
        actions: nullable(jsonb()), default: "[]",
        notification_status: nullable(text()), default: "'unread'",
        priority_label: nullable(text()), default: "'low'",
        priority_level: nullable(integer()), default: "0",
        expiry_date: nullable(text()), default: "''",
        expiry_time: nullable(text()), default: "'00:00'",
        metadata: nullable(text()),
    },
    indexes: {
        // System field indexes ( REQUIRED )
        system_indexes!("notifications"),

        // Custom table-specific indexes - searchable fields
        idx_notifications_title: {
            columns: ["title"],
            unique: false,
            type: "btree"
        },
        idx_notifications_description: {
            columns: ["description"],
            unique: false,
            type: "btree"
        },
        idx_notifications_event_timestamp: {
            columns: ["event_timestamp"],
            unique: false,
            type: "btree"
        },
        idx_notifications_link: {
            columns: ["link"],
            unique: false,
            type: "btree"
        },
        idx_notifications_icon: {
            columns: ["icon"],
            unique: false,
            type: "btree"
        },
        idx_notifications_source: {
            columns: ["source"],
            unique: false,
            type: "btree"
        },
        idx_notifications_is_pinned: {
            columns: ["is_pinned"],
            unique: false,
            type: "btree"
        },
        idx_notifications_recipient_id: {
            columns: ["recipient_id"],
            unique: false,
            type: "btree"
        },
        idx_notifications_notification_status: {
            columns: ["notification_status"],
            unique: false,
            type: "btree"
        },
        idx_notifications_priority_label: {
            columns: ["priority_label"],
            unique: false,
            type: "btree"
        },
        idx_notifications_priority_level: {
            columns: ["priority_level"],
            unique: false,
            type: "btree"
        },
        idx_notifications_expiry_date: {
            columns: ["expiry_date"],
            unique: false,
            type: "btree"
        },
    },
    foreign_keys: {
        // System field foreign keys ( REQUIRED )
        system_foreign_keys!("notifications"),

        // Custom foreign keys
        fk_notifications_recipient_id: {
            columns: ["recipient_id"],
            foreign_table: "contacts",
            foreign_columns: ["id"],
            on_delete: "no action",
            on_update: "no action"
        },
    }
}