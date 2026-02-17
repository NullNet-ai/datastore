use crate::define_table_schema;
use crate::schema::generator::diesel_schema_definition::{types::*, DieselTableDefinition};

pub struct SmtpAttachmentHeadersTable;

define_table_schema! {
    hypertable: false,
    fields: {
        system_fields!(),
        header_key: nullable(text()),
        header_value: nullable(text()),
        smtp_attachment_id: nullable(text()),
    },
    indexes: {
        system_indexes!("smtp_attachment_headers"),
        idx_smtp_attachment_headers_header_key: { columns: ["header_key"], unique: false, type: "btree" },
        idx_smtp_attachment_headers_header_value: { columns: ["header_value"], unique: false, type: "btree" },
        idx_smtp_attachment_headers_smtp_attachment_id: { columns: ["smtp_attachment_id"], unique: false, type: "btree" },
    },
    foreign_keys: {
        system_foreign_keys!("smtp_attachment_headers"),
        fk_smtp_attachment_headers_smtp_attachment_id: { columns: ["smtp_attachment_id"], foreign_table: "smtp_attachments", foreign_columns: ["id"], on_delete: "no action", on_update: "no action" },
    }
}
