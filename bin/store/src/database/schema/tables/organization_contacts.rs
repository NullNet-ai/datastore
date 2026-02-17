use crate::define_table_schema;
use crate::schema::generator::diesel_schema_definition::{types::*, DieselTableDefinition};

pub struct OrganizationContactsTable;

define_table_schema! {
    hypertable: false,
    fields: {
        system_fields!(),
        contact_organization_id: nullable(text()),
        contact_id: nullable(text()),
        is_primary: nullable(boolean()), default: "false",
    },
    indexes: {
        system_indexes!("organization_contacts"),
        idx_organization_contacts_contact_organization_id: { columns: ["contact_organization_id"], unique: false, type: "btree" },
        idx_organization_contacts_contact_id: { columns: ["contact_id"], unique: false, type: "btree" },
        idx_organization_contacts_is_primary: { columns: ["is_primary"], unique: false, type: "btree" },
    },
    foreign_keys: {
        system_foreign_keys!("organization_contacts"),
        fk_organization_contacts_contact_organization_id: { columns: ["contact_organization_id"], foreign_table: "organizations", foreign_columns: ["id"], on_delete: "no action", on_update: "no action" },
        fk_organization_contacts_contact_id: { columns: ["contact_id"], foreign_table: "contacts", foreign_columns: ["id"], on_delete: "no action", on_update: "no action" },
    }
}
