use crate::schema::generator::diesel_schema_definition::{
    DieselTableDefinition, types::*
};
use crate::define_table_schema;

pub struct SmtpResponseLinksTable;

define_table_schema! {
    hypertable: false,
    fields: {
        system_fields!(),
        smtp_payload_id: nullable(text()),
        callback_url: nullable(text()),
        expiry: nullable(text()),
    },
    indexes: {
        system_indexes!("smtp_response_links"),
        idx_smtp_response_links_smtp_payload_id: { columns: ["smtp_payload_id"], unique: false, type: "btree" },
        idx_smtp_response_links_callback_url: { columns: ["callback_url"], unique: false, type: "btree" },
        idx_smtp_response_links_expiry: { columns: ["expiry"], unique: false, type: "btree" },
    },
    foreign_keys: {
        system_foreign_keys!("smtp_response_links"),
        fk_smtp_response_links_smtp_payload_id: { columns: ["smtp_payload_id"], foreign_table: "smtp_payloads", foreign_columns: ["id"], on_delete: "no action", on_update: "no action" },
    }
}