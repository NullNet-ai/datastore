use crate::define_table_schema;
use crate::schema::generator::diesel_schema_definition::{types::*, DieselTableDefinition};
use crate::{system_fields, system_foreign_keys, system_indexes};

/**
 * For defining proper diesel types check it here: bin/store/src/builders/generator/README.md
 */

/// Report child victims table for managing child victim information
pub struct ReportChildVictimsTable;

define_table_schema! {
    hypertable: false,
    fields: {
        // System fields - common across all tables ( REQUIRED )
        system_fields!(),

        // Report child victims fields
        report_id: nullable(text()),
        first_name: nullable(text()), default: "''",
        last_name: nullable(text()), default: "''",
        birth_date: nullable(text()), default: "''", // Ex: 2012-10-15
        email: nullable(text()), default: "''",
        // Address
        address_street: nullable(text()), default: "''",
        address_city: nullable(text()), default: "''",
        address_zip_code: nullable(text()), default: "''",
        address_state: nullable(text()), default: "''",
        address_non_usa_state: nullable(text()), default: "''",
        address_country: nullable(text()), default: "''",
        // Phone
        phone_country_code: nullable(text()), default: "''", // Ex: 1
        phone_extension: nullable(text()), default: "''", // Ex: 1234567
        screen_name: nullable(text()), default: "''",
    },
    indexes: {
        // System field indexes ( REQUIRED )
        system_indexes!("report_child_victims"),

        // Custom table-specific indexes - all non-primary key fields
        idx_report_id: {
            columns: ["report_id"],
            unique: false,
            type: "btree"
        },
        idx_first_name: {
            columns: ["first_name"],
            unique: false,
            type: "btree"
        },
        idx_last_name: {
            columns: ["last_name"],
            unique: false,
            type: "btree"
        },
        idx_birth_date: {
            columns: ["birth_date"],
            unique: false,
            type: "btree"
        },
        idx_email: {
            columns: ["email"],
            unique: false,
            type: "btree"
        },
        idx_address_street: {
            columns: ["address_street"],
            unique: false,
            type: "btree"
        },
        idx_address_city: {
            columns: ["address_city"],
            unique: false,
            type: "btree"
        },
        idx_address_zip_code: {
            columns: ["address_zip_code"],
            unique: false,
            type: "btree"
        },
        idx_address_state: {
            columns: ["address_state"],
            unique: false,
            type: "btree"
        },
        idx_address_non_usa_state: {
            columns: ["address_non_usa_state"],
            unique: false,
            type: "btree"
        },
        idx_address_country: {
            columns: ["address_country"],
            unique: false,
            type: "btree"
        },
        idx_phone_country_code: {
            columns: ["phone_country_code"],
            unique: false,
            type: "btree"
        },
        idx_phone_extension: {
            columns: ["phone_extension"],
            unique: false,
            type: "btree"
        },
        idx_screen_name: {
            columns: ["screen_name"],
            unique: false,
            type: "btree"
        },
    },
    foreign_keys: {
        // System field foreign keys ( REQUIRED )
        system_foreign_keys!("report_child_victims"),

        // Custom foreign keys - all fields ending with "_id"
        fk_report_id: {
            columns: ["report_id"],
            foreign_table: "reports",
            foreign_columns: ["id"],
            on_delete: "no action",
            on_update: "no action"
        },
    }
}