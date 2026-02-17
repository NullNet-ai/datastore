use crate::schema::generator::diesel_schema_definition::{
    DieselTableDefinition, types::*
};
use crate::define_table_schema;

pub struct SmtpAttachmentsTable;

define_table_schema! {
    hypertable: false,
    fields: {
        system_fields!(),
        filename: nullable(text()),
        content_type: nullable(text()),
        content_disposition: nullable(text()),
        checksum: nullable(text()),
        size: nullable(integer()),
        content: nullable(bytea()),
        content_id: nullable(text()),
        cid: nullable(text()),
        related: nullable(boolean()),
        headers: nullable(jsonb()), default: "'{}'::jsonb",
        smtp_payload_id: nullable(text()),
        file_id: nullable(text()),
        type_field: nullable(text()),
        part_id: nullable(text()),
    },
    indexes: {
        system_indexes!("smtp_attachments"),
        idx_smtp_attachments_filename: { columns: ["filename"], unique: false, type: "btree" },
        idx_smtp_attachments_content_type: { columns: ["content_type"], unique: false, type: "btree" },
        idx_smtp_attachments_content_disposition: { columns: ["content_disposition"], unique: false, type: "btree" },
        idx_smtp_attachments_checksum: { columns: ["checksum"], unique: false, type: "btree" },
        idx_smtp_attachments_size: { columns: ["size"], unique: false, type: "btree" },
        idx_smtp_attachments_content_id: { columns: ["content_id"], unique: false, type: "btree" },
        idx_smtp_attachments_cid: { columns: ["cid"], unique: false, type: "btree" },
        idx_smtp_attachments_related: { columns: ["related"], unique: false, type: "btree" },
        idx_smtp_attachments_smtp_payload_id: { columns: ["smtp_payload_id"], unique: false, type: "btree" },
        idx_smtp_attachments_file_id: { columns: ["file_id"], unique: false, type: "btree" },
        idx_smtp_attachments_type_field: { columns: ["type_field"], unique: false, type: "btree" },
        idx_smtp_attachments_part_id: { columns: ["part_id"], unique: false, type: "btree" },
    },
    foreign_keys: {
        system_foreign_keys!("smtp_attachments"),
        fk_smtp_attachments_file_id: { columns: ["file_id"], foreign_table: "files", foreign_columns: ["id"], on_delete: "no action", on_update: "no action" },
    }
}