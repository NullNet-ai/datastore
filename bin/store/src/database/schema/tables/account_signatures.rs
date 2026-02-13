use crate::define_table_schema;
use crate::generated::schema::generator::diesel_schema_definition::{
    types::*, DieselTableDefinition,
};
use crate::{system_fields, system_foreign_keys, system_indexes};

/// Account signatures table for storing user signatures
pub struct AccountSignaturesTable;

define_table_schema! {
    hypertable: false,
    fields: {
        // System fields - common across all tables ( REQUIRED )
        system_fields!(),

        // Account signatures specific fields
        account_profile_id: nullable(text()),
        name: nullable(text()),
        signature: nullable(varchar(Some(300))),

    },
    indexes: {
        // System field indexes ( REQUIRED )
        system_indexes!("account_signatures"),
    },
    foreign_keys: {
        // System foreign keys ( REQUIRED )
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
