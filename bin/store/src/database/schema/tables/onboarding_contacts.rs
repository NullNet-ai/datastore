use crate::schema::generator::diesel_schema_definition::{
    DieselTableDefinition, types::*
};
use crate::define_table_schema;

pub struct OnboardingContactsTable;

define_table_schema! {
    hypertable: false,
    fields: {
        system_fields!(),
        contact_id: nullable(text()),
        is_done: nullable(boolean()), default: "false",
    },
    indexes: {
        system_indexes!("onboarding_contacts"),
        idx_onboarding_contacts_contact_id: { columns: ["contact_id"], unique: false, type: "btree" },
        idx_onboarding_contacts_is_done: { columns: ["is_done"], unique: false, type: "btree" },
    },
    foreign_keys: {
        system_foreign_keys!("onboarding_contacts"),
        fk_onboarding_contacts_contact_id: { columns: ["contact_id"], foreign_table: "contacts", foreign_columns: ["id"], on_delete: "no action", on_update: "no action" },
    }
}