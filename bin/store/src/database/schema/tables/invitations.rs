use crate::define_table_schema;
use crate::generated::schema::generator::diesel_schema_definition::{
    types::*, DieselTableDefinition,
};
use crate::{system_fields, system_foreign_keys, system_indexes};

/**
 * Auto-converted from TypeScript schema source.
 */
pub struct InvitationsTable;

define_table_schema! {
    hypertable: false,
    fields: {
        // System fields - common across all tables ( REQUIRED )
        system_fields!(),

        account_id: nullable(text()),
        expiration_date: nullable(text()),
        expiration_time: nullable(text()),
        account_organization_id: nullable(text()),
    },
    indexes: {
        // System field indexes ( REQUIRED )
        system_indexes!("invitations"),
        // Custom table-specific indexes
        idx_invitations_account_id: {
            columns: ["account_id"],
            unique: false,
            type: "btree"
        },
        idx_invitations_expiration_date: {
            columns: ["expiration_date"],
            unique: false,
            type: "btree"
        },
    },
    foreign_keys: {
        // System field foreign keys ( REQUIRED )
        system_foreign_keys!("invitations"),
        fk_invitations_account_id: { columns: ["account_id"], foreign_table: "organization_accounts", foreign_columns: ["id"], on_delete: "no action", on_update: "no action" },
        fk_invitations_account_organization_id: { columns: ["account_organization_id"], foreign_table: "account_organizations", foreign_columns: ["id"], on_delete: "no action", on_update: "no action" },
    }
}
