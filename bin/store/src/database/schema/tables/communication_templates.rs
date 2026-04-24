use crate::define_table_schema;
use crate::generated::schema::generator::diesel_schema_definition::{
    types::*, DieselTableDefinition,
};
use crate::{system_fields, system_foreign_keys, system_indexes};

/**
 * Auto-converted from TypeScript schema source.
 */
pub struct CommunicationTemplatesTable;

define_table_schema! {
    hypertable: false,
    fields: {
        // System fields - common across all tables ( REQUIRED )
        system_fields!(),

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
        idx_communication_templates_name: {
            columns: ["name"],
            unique: false,
            type: "btree"
        },
        idx_communication_templates_communication_template_status: {
            columns: ["communication_template_status"],
            unique: false,
            type: "btree"
        },
        idx_communication_templates_event: {
            columns: ["event"],
            unique: false,
            type: "btree"
        },
        idx_communication_templates_content: {
            columns: ["content"],
            unique: false,
            type: "btree"
        },
        idx_communication_templates_subject: {
            columns: ["subject"],
            unique: false,
            type: "btree"
        },
    },
    foreign_keys: {
        // System field foreign keys ( REQUIRED )
        system_foreign_keys!("communication_templates"),
    }
}
