use crate::define_table_schema;
use crate::schema::generator::diesel_schema_definition::{types::*, DieselTableDefinition};
use crate::{system_fields, system_foreign_keys, system_indexes};

/**
 * For defining proper diesel types check it here: bin/store/src/builders/generator/README.md
 */

/// Stories table for managing story information
pub struct StoriesTable;

define_table_schema! {
    hypertable: false,
    fields: {
        // System fields - common across all tables ( REQUIRED )
        system_fields!(),

        // Stories fields
        name: nullable(text()),
        course_id: nullable(text()),
        order: nullable(integer()), default: "0",
    },
    indexes: {
        // System field indexes ( REQUIRED )
        system_indexes!("stories"),

        // Custom table-specific indexes - all non-primary key fields
        idx_name: {
            columns: ["name"],
            unique: false,
            type: "btree"
        },
        idx_course_id: {
            columns: ["course_id"],
            unique: false,
            type: "btree"
        },
        idx_order: {
            columns: ["order"],
            unique: false,
            type: "btree"
        },
    },
    foreign_keys: {
        // System field foreign keys ( REQUIRED )
        system_foreign_keys!("stories"),

        // Custom foreign keys - all fields ending with "_id"
        fk_course_id: {
            columns: ["course_id"],
            foreign_table: "courses",
            foreign_columns: ["id"],
            on_delete: "no action",
            on_update: "no action"
        },
    }
}