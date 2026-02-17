use crate::define_table_schema;
use crate::schema::generator::diesel_schema_definition::{types::*, DieselTableDefinition};

pub struct ContactPhoneNumbersTable;

define_table_schema! {
    hypertable: false,
    fields: {
        system_fields!(),
        contact_id: nullable(text()),
        raw_phone_number: nullable(text()),
        iso_code: nullable(text()),
        country_code: nullable(text()),
        is_primary: nullable(boolean()), default: "false",
    },
    indexes: {
        system_indexes!("contact_phone_numbers"),
        idx_contact_phone_numbers_contact_id: { columns: ["contact_id"], unique: false, type: "btree" },
        idx_contact_phone_numbers_raw_phone_number: { columns: ["raw_phone_number"], unique: false, type: "btree" },
        idx_contact_phone_numbers_iso_code: { columns: ["iso_code"], unique: false, type: "btree" },
        idx_contact_phone_numbers_country_code: { columns: ["country_code"], unique: false, type: "btree" },
        idx_contact_phone_numbers_is_primary: { columns: ["is_primary"], unique: false, type: "btree" },
    },
    foreign_keys: {
        system_foreign_keys!("contact_phone_numbers"),
        fk_contact_phone_numbers_contact_id: { columns: ["contact_id"], foreign_table: "contacts", foreign_columns: ["id"], on_delete: "no action", on_update: "no action" },
    }
}
