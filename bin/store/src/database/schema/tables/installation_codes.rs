use crate::define_table_schema;
use crate::generated::schema::generator::diesel_schema_definition::{
    types::*, DieselTableDefinition,
};
use crate::{system_fields, system_foreign_keys, system_indexes};

/**
 * Auto-converted from TypeScript schema source.
 */
pub struct InstallationCodesTable;

define_table_schema! {
    hypertable: false,
    fields: {
        // System fields - common across all tables ( REQUIRED )
        system_fields!(),

        device_id: nullable(text()),
        device_code: nullable(text()),
        redeemed: nullable(boolean()), default "false",
        auto_authorization: nullable(boolean()), default "false",
        token: nullable(text()), default "sql`substring(md5(random(",
    },
    indexes: {
        // System field indexes ( REQUIRED )
        system_indexes!("installation_codes"),
    },
    foreign_keys: {
        // System field foreign keys ( REQUIRED )
        system_foreign_keys!("installation_codes"),
        fk_installation_codes_device_id: { columns: ["device_id"], foreign_table: "devices", foreign_columns: ["id"], on_delete: "no action", on_update: "no action" },
    }
}
