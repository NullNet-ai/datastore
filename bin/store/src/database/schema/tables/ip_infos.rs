use crate::define_table_schema;
use crate::generated::schema::generator::diesel_schema_definition::{
    types::*, DieselTableDefinition,
};
use crate::{system_fields, system_foreign_keys, system_indexes};

/**
 * Auto-converted from TypeScript schema source.
 */
pub struct IpInfosTable;

define_table_schema! {
    hypertable: false,
    fields: {
        // System fields - common across all tables ( REQUIRED )
        system_fields!(),

        ip: nullable(text()),
        country: nullable(text()),
        asn: nullable(text()),
        org: nullable(text()),
        continent_code: nullable(text()),
        city: nullable(text()),
        region: nullable(text()),
        postal: nullable(text()),
        timezone: nullable(text()),
    },
    indexes: {
        // System field indexes ( REQUIRED )
        system_indexes!("ip_infos"),
        // Custom table-specific indexes
        idx_ip_infos_ip: {
            columns: ["ip"],
            unique: false,
            type: "btree"
        },
        idx_ip_infos_country: {
            columns: ["country"],
            unique: false,
            type: "btree"
        },
    },
    foreign_keys: {
        // System field foreign keys ( REQUIRED )
        system_foreign_keys!("ip_infos"),
    }
}
