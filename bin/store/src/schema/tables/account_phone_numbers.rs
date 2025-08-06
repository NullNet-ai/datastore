use crate::schema::generator::diesel_schema_definition::{
    DieselTableDefinition, types::*
};
use crate::define_table_schema;
use crate::{system_fields, system_indexes, system_foreign_keys};

/// Account phone numbers table for storing phone number information
pub struct AccountPhoneNumbersTable;

define_table_schema! {
    hypertable: false,
    fields: {
        // System fields - common across all tables
        system_fields!(),
        
        // Account phone numbers specific fields
        account_profile_id: nullable(text()),
        raw_phone_number: nullable(text()),
        is_primary: nullable(boolean()),
        iso_code: nullable(text()),
        country_code: nullable(text()),
    },
    indexes: {
        // System field indexes
        system_indexes!("account_phone_numbers"),
    },
    foreign_keys: {
        // System foreign keys
        system_foreign_keys!("account_phone_numbers"),
        
        // Custom foreign keys
        fk_account_phone_numbers_profile_id: {
            columns: ["account_profile_id"],
            foreign_table: "account_profiles",
            foreign_columns: ["id"],
            on_delete: "no action",
            on_update: "no action"
        }
    }
}