use crate::define_table_schema;
use crate::schema::generator::diesel_schema_definition::{types::*, DieselTableDefinition};

pub struct UserSettingsTable;

define_table_schema! {
    hypertable: false,
    fields: {
        system_fields!(),
        user_id: nullable(text()),
        sound: nullable(text()),
    },
    indexes: {
        system_indexes!("user_settings"),
        idx_user_settings_user_id: { columns: ["user_id"], unique: false, type: "btree" },
        idx_user_settings_sound: { columns: ["sound"], unique: false, type: "btree" },
    },
    foreign_keys: {
        system_foreign_keys!("user_settings"),
        fk_user_settings_user_id: { columns: ["user_id"], foreign_table: "contacts", foreign_columns: ["id"], on_delete: "no action", on_update: "no action" },
    }
}
