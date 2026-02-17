use crate::schema::generator::diesel_schema_definition::{
    DieselTableDefinition, types::*
};
use crate::define_table_schema;

pub struct RelatedContactsTable;

define_table_schema! {
    hypertable: false,
    fields: {
        system_fields!(),
        contact_id: nullable(text()),
        student_id: nullable(text()),
        is_verified: nullable(boolean()), default: "false",
    },
    indexes: {
        system_indexes!("related_contacts"),
        idx_related_contacts_contact_id: { columns: ["contact_id"], unique: false, type: "btree" },
        idx_related_contacts_student_id: { columns: ["student_id"], unique: false, type: "btree" },
        idx_related_contacts_is_verified: { columns: ["is_verified"], unique: false, type: "btree" },
    },
    foreign_keys: {
        system_foreign_keys!("related_contacts"),
        fk_related_contacts_contact_id: { columns: ["contact_id"], foreign_table: "contacts", foreign_columns: ["id"], on_delete: "no action", on_update: "no action" },
        fk_related_contacts_student_id: { columns: ["student_id"], foreign_table: "contacts", foreign_columns: ["id"], on_delete: "no action", on_update: "no action" },
    }
}