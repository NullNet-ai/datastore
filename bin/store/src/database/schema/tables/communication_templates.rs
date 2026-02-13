use crate::define_table_schema;
use crate::schema::generator::diesel_schema_definition::{types::*, DieselTableDefinition};
use crate::{system_fields, system_foreign_keys, system_indexes};

/**
 * For defining proper diesel types check it here: bin/store/src/builders/generator/README.md
 */

/// Communication templates table for storing email/SMS templates
pub struct CommunicationTemplatesTable;

define_table_schema! {
    hypertable: false,
    fields: {
        // System fields - common across all tables ( REQUIRED )
        system_fields!(),

        // Communication template fields
        name: nullable(text()),
        communication_template_status: nullable(text()),
        event: nullable(text()),
        content: nullable(text()),
        subject: nullable(text()),
    },
    indexes: {
        // System field indexes ( REQUIRED )
        system_indexes!("communication_templates"),

        // Custom table-specific indexes
        idx_name: {
            columns: ["name"],
            unique: false,
            type: "btree"
        },
        idx_communication_template_status: {
            columns: ["communication_template_status"],
            unique: false,
            type: "btree"
        },
        idx_event: {
            columns: ["event"],
            unique: false,
            type: "btree"
        },
        idx_content: {
            columns: ["content"],
            unique: false,
            type: "btree"
        },
        idx_subject: {
            columns: ["subject"],
            unique: false,
            type: "btree"
        },
    },
    foreign_keys: {
        // System field foreign keys ( REQUIRED )
        system_foreign_keys!("communication_templates"),

        // Custom foreign keys - no fields ending with "_id" in this table
    }
}