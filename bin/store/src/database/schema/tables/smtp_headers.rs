use crate::define_table_schema;
use crate::schema::generator::diesel_schema_definition::{types::*, DieselTableDefinition};
use crate::{system_fields, system_foreign_keys, system_indexes};

/**
 * For defining proper diesel types check it here: bin/store/src/builders/generator/README.md
 */

/// SMTP headers table for storing email header information
pub struct SmtpHeadersTable;

define_table_schema! {
    hypertable: false,
    fields: {
        // System fields - common across all tables ( REQUIRED )
        system_fields!(),

        // SMTP headers fields
        header_key: nullable(text()),
        header_value: nullable(text()),
        smtp_payload_id: nullable(text()),
    },
    indexes: {
        // System field indexes ( REQUIRED )
        system_indexes!("smtp_headers"),

        // Custom table-specific indexes - all non-primary key fields
        idx_smtp_headers_header_key: {
            columns: ["header_key"],
            unique: false,
            type: "btree"
        },
        idx_smtp_headers_header_value: {
            columns: ["header_value"],
            unique: false,
            type: "btree"
        },
        idx_smtp_headers_smtp_payload_id: {
            columns: ["smtp_payload_id"],
            unique: false,
            type: "btree"
        },
    },
    foreign_keys: {
        // System field foreign keys ( REQUIRED )
        system_foreign_keys!("smtp_headers"),

        // Custom foreign keys
        fk_smtp_headers_smtp_payload_id: { columns: ["smtp_payload_id"], foreign_table: "smtp_payloads", foreign_columns: ["id"], on_delete: "no action", on_update: "no action" },
    }
}