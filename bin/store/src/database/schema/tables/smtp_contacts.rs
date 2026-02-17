use crate::schema::generator::diesel_schema_definition::{
    DieselTableDefinition, types::*
};
use crate::define_table_schema;

pub struct SmtpContactsTable;

define_table_schema! {
    hypertable: false,
    fields: {
        system_fields!(),
        smtp_payload_id: nullable(text()),
        contact_id: nullable(text()),
    },
    indexes: {
        system_indexes!("smtp_contacts"),
        idx_smtp_contacts_smtp_payload_id: { columns: ["smtp_payload_id"], unique: false, type: "btree" },
        idx_smtp_contacts_contact_id: { columns: ["contact_id"], unique: false, type: "btree" },
    },
    foreign_keys: {
        system_foreign_keys!("smtp_contacts"),
        fk_smtp_contacts_smtp_payload_id: { columns: ["smtp_payload_id"], foreign_table: "smtp_payloads", foreign_columns: ["id"], on_delete: "no action", on_update: "no action" },
        fk_smtp_contacts_contact_id: { columns: ["contact_id"], foreign_table: "contacts", foreign_columns: ["id"], on_delete: "no action", on_update: "no action" },
    }
}