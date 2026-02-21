use crate::define_table_schema;
use crate::schema::generator::diesel_schema_definition::{types::*, DieselTableDefinition};

pub struct OrganizationAccountsTable;

define_table_schema! {
    hypertable: false,
    fields: {
        system_fields!(),
        organization_contact_id: nullable(text()),
        organization_id: nullable(text()),
        contact_id: nullable(text()),
        email: nullable(text()),
        password: nullable(text()),
        account_id: nullable(text()),
        account_secret: nullable(text()),
        role_id: nullable(text()),
        account_organization_id: nullable(text()),
        is_new_user: nullable(boolean()), default: "false",
        account_status: nullable(text()),
        external_contact_id: nullable(text()),
        // Additional fields from original database schema
        image_url: nullable(text()),
        device_id: nullable(text()),
    },
    indexes: {
        system_indexes!("organization_accounts"),
        idx_organization_accounts_organization_contact_id: { columns: ["organization_contact_id"], unique: false, type: "btree" },
        idx_organization_accounts_contact_id: { columns: ["contact_id"], unique: false, type: "btree" },
        idx_organization_accounts_email: { columns: ["email"], unique: false, type: "btree" },
        idx_organization_accounts_password: { columns: ["password"], unique: false, type: "btree" },
        idx_organization_accounts_account_id: { columns: ["account_id"], unique: false, type: "btree" },
        idx_organization_accounts_account_secret: { columns: ["account_secret"], unique: false, type: "btree" },
        idx_organization_accounts_role_id: { columns: ["role_id"], unique: false, type: "btree" },
        idx_organization_accounts_account_organization_id: { columns: ["account_organization_id"], unique: false, type: "btree" },
        idx_organization_accounts_is_new_user: { columns: ["is_new_user"], unique: false, type: "btree" },
        idx_organization_accounts_account_status: { columns: ["account_status"], unique: false, type: "btree" },
        idx_organization_accounts_external_contact_id: { columns: ["external_contact_id"], unique: false, type: "btree" },
        idx_organization_accounts_image_url: { columns: ["image_url"], unique: false, type: "btree" },
        idx_organization_accounts_device_id: { columns: ["device_id"], unique: false, type: "btree" },
    },
    foreign_keys: {
        system_foreign_keys!("organization_accounts"),
        fk_organization_accounts_organization_contact_id: { columns: ["organization_contact_id"], foreign_table: "organization_contacts", foreign_columns: ["id"], on_delete: "no action", on_update: "no action" },
        fk_organization_accounts_organization_id: { columns: ["organization_id"], foreign_table: "organizations", foreign_columns: ["id"], on_delete: "no action", on_update: "no action" },
        fk_organization_accounts_contact_id: { columns: ["contact_id"], foreign_table: "contacts", foreign_columns: ["id"], on_delete: "no action", on_update: "no action" },
        fk_organization_accounts_role_id: { columns: ["role_id"], foreign_table: "user_roles", foreign_columns: ["id"], on_delete: "no action", on_update: "no action" },
        fk_organization_accounts_account_organization_id: { columns: ["account_organization_id"], foreign_table: "organizations", foreign_columns: ["id"], on_delete: "no action", on_update: "no action" },
        fk_organization_accounts_device_id: { columns: ["device_id"], foreign_table: "devices", foreign_columns: ["id"], on_delete: "set null", on_update: "no action" },
    }
}
