use crate::define_table_schema;
use crate::schema::generator::diesel_schema_definition::{types::*, DieselTableDefinition};
use crate::{system_fields, system_foreign_keys, system_indexes};

/**
 * For defining proper diesel types check it here: bin/store/src/builders/generator/README.md
 */

/// Courses table for managing course information
pub struct CoursesTable;

define_table_schema! {
    hypertable: false,
    fields: {
        // System fields - common across all tables ( REQUIRED )
        system_fields!(),

        // Courses fields
        title: nullable(text()),
        order: nullable(integer()), default: "0",
        description: nullable(text()),
        is_show_assistant: nullable(boolean()), default: "true",
    },
    indexes: {
        // System field indexes ( REQUIRED )
        system_indexes!("courses"),

        // Custom table-specific indexes - all non-primary key fields
        idx_courses_title: {
            columns: ["title"],
            unique: false,
            type: "btree"
        },
        idx_courses_order: {
            columns: ["order"],
            unique: false,
            type: "btree"
        },
        idx_courses_description: {
            columns: ["description"],
            unique: false,
            type: "btree"
        },
        idx_courses_is_show_assistant: {
            columns: ["is_show_assistant"],
            unique: false,
            type: "btree"
        },
    },
    foreign_keys: {
        // System field foreign keys ( REQUIRED )
        system_foreign_keys!("courses"),

        // Custom foreign keys - no fields ending with "_id" in this table
    }
}
