use crate::define_table_schema;
use crate::schema::generator::diesel_schema_definition::{types::*, DieselTableDefinition};

pub struct SmtpPayloadsTable;

define_table_schema! {
    hypertable: false,
    fields: {
        system_fields!(),
        sender: nullable(text()),
        recipients: nullable(jsonb()), default: "'[]'::jsonb",
        cc_recipients: nullable(jsonb()), default: "'[]'::jsonb",
        bcc_recipients: nullable(jsonb()), default: "'[]'::jsonb",
        subject: nullable(text()),
        html: nullable(text()),
        parent_id: nullable(text()),
        attachment_ids: nullable(jsonb()), default: "'[]'::jsonb",
        thread_id: nullable(text()),
        send_strategy: nullable(text()),
        priority: nullable(text()),
        transport_provider_id: nullable(text()),
        source: nullable(text()),
        method: nullable(text()),
    },
    indexes: {
        system_indexes!("smtp_payloads"),
        idx_smtp_payloads_sender: { columns: ["sender"], unique: false, type: "btree" },
        idx_smtp_payloads_recipients: { columns: ["recipients"], unique: false, type: "btree" },
        idx_smtp_payloads_cc_recipients: { columns: ["cc_recipients"], unique: false, type: "btree" },
        idx_smtp_payloads_bcc_recipients: { columns: ["bcc_recipients"], unique: false, type: "btree" },
        idx_smtp_payloads_subject: { columns: ["subject"], unique: false, type: "btree" },
        idx_smtp_payloads_html: { columns: ["html"], unique: false, type: "btree" },
        idx_smtp_payloads_parent_id: { columns: ["parent_id"], unique: false, type: "btree" },
        idx_smtp_payloads_attachment_ids: { columns: ["attachment_ids"], unique: false, type: "btree" },
        idx_smtp_payloads_thread_id: { columns: ["thread_id"], unique: false, type: "btree" },
        idx_smtp_payloads_send_strategy: { columns: ["send_strategy"], unique: false, type: "btree" },
        idx_smtp_payloads_priority: { columns: ["priority"], unique: false, type: "btree" },
        idx_smtp_payloads_transport_provider_id: { columns: ["transport_provider_id"], unique: false, type: "btree" },
        idx_smtp_payloads_source: { columns: ["source"], unique: false, type: "btree" },
        idx_smtp_payloads_method: { columns: ["method"], unique: false, type: "btree" },
    },
    foreign_keys: {
        system_foreign_keys!("smtp_payloads"),
        fk_smtp_payloads_transport_provider_id: { columns: ["transport_provider_id"], foreign_table: "transport_providers", foreign_columns: ["id"], on_delete: "no action", on_update: "no action" },
    }
}
