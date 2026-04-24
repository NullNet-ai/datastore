use crate::define_table_schema;
use crate::generated::schema::generator::diesel_schema_definition::{
    types::*, DieselTableDefinition,
};
use crate::{system_fields, system_foreign_keys, system_indexes};

/**
 * Auto-converted from TypeScript schema source.
 */
pub struct ContactPhoneNumbersTable;

define_table_schema! {
    hypertable: false,
    fields: {
        // System fields - common across all tables ( REQUIRED )
        system_fields!(),

        contact_id: nullable(text()),
        raw_phone_number: nullable(text()),
        iso_code: nullable(text()),
        country_code: nullable(text()),
        is_primary: nullable(boolean()), default "false",
    },
    indexes: {
        // System field indexes ( REQUIRED )
        system_indexes!("contact_phone_numbers"),
    },
    foreign_keys: {
        // System field foreign keys ( REQUIRED )
        system_foreign_keys!("contact_phone_numbers"),
        fk_contact_phone_numbers_contact_id: { columns: ["contact_id"], foreign_table: "contacts", foreign_columns: ["id"], on_delete: "no action", on_update: "no action" },
    }
}
