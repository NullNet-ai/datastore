use crate::define_table_schema;
use crate::generated::schema::generator::diesel_schema_definition::{
    types::*, DieselTableDefinition,
};
use crate::{system_fields, system_foreign_keys, system_indexes};

/**
 * For defining proper diesel types check it here: bin/store/src/builders/generator/README.md
 */

/// Invitations table for user account invitations
pub struct InvitationsTable;

define_table_schema! {
    hypertable: false,
    fields: {
        // System fields - common across all tables ( REQUIRED )
        system_fields!(),

        // File-specific fields
        account_id: nullable(text()),
        expiration_date: nullable(text()),
        expiration_time: nullable(text()),
        account_organization_id: nullable(text()),
    },
    indexes: {
        // System field indexes ( REQUIRED )
        system_indexes!("organization_contact_user_roles"),

        // Custom table-specific indexes
        idx_account_id: {
            columns: ["account_id"],
            unique: false,
            type: "btree"
        },
        idx_account_organization_id: {
            columns: ["account_organization_id"],
            unique: false,
            type: "btree"
        },
        idx_expiration_date: {
            columns: ["expiration_date"],
            unique: false,
            type: "btree"
        },
        idx_expiration_time: {
            columns: ["expiration_time"],
            unique: false,
            type: "btree"
        },
    },
    foreign_keys: {
        // System field foreign keys ( REQUIRED )
        system_foreign_keys!("organization_contact_user_roles"),

         // Custom foreign keys
        fk_account_id: {
            columns: ["account_id"],
            foreign_table: "accounts",
            foreign_columns: ["id"],
            on_delete: "no action",
            on_update: "no action"
        },
        fk_account_organization_id: {
            columns: ["account_organization_id"],
            foreign_table: "account_organizations",
            foreign_columns: ["id"],
            on_delete: "no action",
            on_update: "no action"
        },
    }
}
