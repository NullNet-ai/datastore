use crate::schema::generator::diesel_schema_definition::{
    DieselTableDefinition, types::*
};
use crate::define_table_schema;
use crate::{system_fields, system_indexes, system_foreign_keys};

/// Account signatures table for storing user signatures
pub struct AccountSignaturesTable;

define_table_schema! {
    hypertable: false,
    fields: {
        // System fields - common across all tables
        system_fields!(),
        
        // Account signatures specific fields
        account_profile_id: nullable(text()),
        name: nullable(text()),
        signature: nullable(varchar(Some(300))),

    },
    indexes: {
        // System field indexes
        system_indexes!("account_signatures"),
    },
    foreign_keys: {
        // System foreign keys
        system_foreign_keys!("account_signatures"),
        
        // Custom foreign keys
        fk_account_signatures_profile_id: {
            columns: ["account_profile_id"],
            foreign_table: "account_profiles",
            foreign_columns: ["id"],
            on_delete: "no action",
            on_update: "no action"
        }
    }
}