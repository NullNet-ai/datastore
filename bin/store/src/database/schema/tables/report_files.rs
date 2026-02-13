use crate::define_table_schema;
use crate::schema::generator::diesel_schema_definition::{types::*, DieselTableDefinition};
use crate::{system_fields, system_foreign_keys, system_indexes};

/**
 * For defining proper diesel types check it here: bin/store/src/builders/generator/README.md
 */

/// Report files table for managing report file attachments
pub struct ReportFilesTable;

define_table_schema! {
    hypertable: false,
    fields: {
        // System fields - common across all tables ( REQUIRED )
        system_fields!(),

        // Report files fields
        report_id: nullable(text()),
        cybertipline_report_file_id: nullable(text()), default: "''",
        evidence_url: nullable(text()), default: "''",
    },
    indexes: {
        // System field indexes ( REQUIRED )
        system_indexes!("report_files"),

        // Custom table-specific indexes - all non-primary key fields
        idx_report_id: {
            columns: ["report_id"],
            unique: false,
            type: "btree"
        },
        idx_cybertipline_report_file_id: {
            columns: ["cybertipline_report_file_id"],
            unique: false,
            type: "btree"
        },
        idx_evidence_url: {
            columns: ["evidence_url"],
            unique: false,
            type: "btree"
        },
    },
    foreign_keys: {
        // System field foreign keys ( REQUIRED )
        system_foreign_keys!("report_files"),

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