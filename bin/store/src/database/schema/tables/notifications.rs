use crate::define_table_schema;
use crate::generated::schema::generator::diesel_schema_definition::{
    types::*, DieselTableDefinition,
};
use crate::{system_fields, system_foreign_keys, system_indexes};

/**
 * Auto-converted from TypeScript schema source.
 */
pub struct NotificationsTable;

define_table_schema! {
    hypertable: false,
    fields: {
        // System fields - common across all tables ( REQUIRED )
        system_fields!(),

        title: nullable(text()),
        description: nullable(text()),
        event_timestamp: nullable(text()),
        link: nullable(text()), default "",
        icon: nullable(text()), default "",
        source: nullable(text()),
        is_pinned: nullable(boolean()), default "false",
        recipient_id: nullable(text()),
        actions: nullable(jsonb()), default "'[]'::jsonb",
        unread: nullable(text()), default "unread",
        low: nullable(text()), default "low",
        priority_level: nullable(integer()), default "0",
        expiry_date: nullable(text()), default "",
        metadata: nullable(text()),
    },
    indexes: {
        // System field indexes ( REQUIRED )
        system_indexes!("notifications"),
        // Custom table-specific indexes
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
        fk_notifications_recipient_id: { columns: ["recipient_id"], foreign_table: "contacts", foreign_columns: ["id"], on_delete: "no action", on_update: "no action" },
    }
}
