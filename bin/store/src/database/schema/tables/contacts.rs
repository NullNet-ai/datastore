use crate::define_table_schema;
use crate::generated::schema::generator::diesel_schema_definition::{
    types::*, DieselTableDefinition,
};
use crate::{system_fields, system_foreign_keys, system_indexes};

/**
 * Auto-converted from TypeScript schema source.
 */
pub struct ContactsTable;

define_table_schema! {
    hypertable: false,
    fields: {
        // System fields - common across all tables ( REQUIRED )
        system_fields!(),

        first_name: nullable(text()),
        middle_name: nullable(text()),
        last_name: nullable(text()),
        date_of_birth: nullable(text()),
        address_id: nullable(text()),
        account_id: nullable(text()),
    },
    indexes: {
        // System field indexes ( REQUIRED )
        system_indexes!("contacts"),
        // Custom table-specific indexes
        idx_contacts_first_name: {
            columns: ["first_name"],
            unique: false,
            type: "btree"
        },
        idx_contacts_last_name: {
            columns: ["last_name"],
            unique: false,
            type: "btree"
        },
        idx_contacts_account_id: {
            columns: ["account_id"],
            unique: false,
            type: "btree"
        },
        idx_contacts_address_id: {
            columns: ["address_id"],
            unique: false,
            type: "btree"
        },
        idx_contacts_date_of_birth: {
            columns: ["date_of_birth"],
            unique: false,
            type: "btree"
        },
        idx_contacts_middle_name: {
            columns: ["middle_name"],
            unique: false,
            type: "btree"
        },
    },
    foreign_keys: {
        // System field foreign keys ( REQUIRED )
        system_foreign_keys!("contacts"),
        fk_contacts_address_id: { columns: ["address_id"], foreign_table: "addresses", foreign_columns: ["id"], on_delete: "no action", on_update: "no action" },
        fk_contacts_account_id: { columns: ["account_id"], foreign_table: "accounts", foreign_columns: ["id"], on_delete: "no action", on_update: "no action" },
    }
}
