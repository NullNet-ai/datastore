use crate::define_table_schema;
use crate::schema::generator::diesel_schema_definition::{types::*, DieselTableDefinition};

pub struct OrganizationContactUserRolesTable;

define_table_schema! {
    hypertable: false,
    fields: {
        system_fields!(),
        organization_contact_id: nullable(text()),
        user_role_id: nullable(text()),
    },
    indexes: {
        system_indexes!("organization_contact_user_roles"),
        idx_organization_contact_user_roles_organization_contact_id: { columns: ["organization_contact_id"], unique: false, type: "btree" },
        idx_organization_contact_user_roles_user_role_id: { columns: ["user_role_id"], unique: false, type: "btree" },
    },
    foreign_keys: {
        system_foreign_keys!("organization_contact_user_roles"),
        fk_organization_contact_user_roles_organization_contact_id: { columns: ["organization_contact_id"], foreign_table: "organization_contacts", foreign_columns: ["id"], on_delete: "no action", on_update: "no action" },
        fk_organization_contact_user_roles_user_role_id: { columns: ["user_role_id"], foreign_table: "user_roles", foreign_columns: ["id"], on_delete: "no action", on_update: "no action" },
    }
}
