use crate::schema::generator::diesel_schema_definition::{
    DieselTableDefinition, types::*
};
use crate::define_table_schema;

pub struct TransportProviderCredentialsTable;

define_table_schema! {
    hypertable: false,
    fields: {
        system_fields!(),
        transport_provider_id: nullable(text()),
        host: nullable(text()),
        port: nullable(integer()),
        is_secure: nullable(boolean()), default: "false",
        username: nullable(text()),
        password: nullable(text()),
        api_key: nullable(text()),
        additional_config: nullable(jsonb()), default: "'{}'::jsonb",
    },
    indexes: {
        system_indexes!("transport_provider_credentials"),
        idx_transport_provider_credentials_transport_provider_id: { columns: ["transport_provider_id"], unique: false, type: "btree" },
        idx_transport_provider_credentials_host: { columns: ["host"], unique: false, type: "btree" },
        idx_transport_provider_credentials_port: { columns: ["port"], unique: false, type: "btree" },
        idx_transport_provider_credentials_is_secure: { columns: ["is_secure"], unique: false, type: "btree" },
        idx_transport_provider_credentials_username: { columns: ["username"], unique: false, type: "btree" },
        idx_transport_provider_credentials_password: { columns: ["password"], unique: false, type: "btree" },
        idx_transport_provider_credentials_api_key: { columns: ["api_key"], unique: false, type: "btree" },
    },
    foreign_keys: {
        system_foreign_keys!("transport_provider_credentials"),
        fk_transport_provider_credentials_transport_provider_id: { columns: ["transport_provider_id"], foreign_table: "transport_providers", foreign_columns: ["id"], on_delete: "no action", on_update: "no action" },
    }
}