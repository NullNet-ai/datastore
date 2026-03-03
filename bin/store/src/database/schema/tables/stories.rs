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
        description: nullable(text()), default: "''",
        story_identifier: nullable(text()), default: "''",
        bundle_file_name: nullable(text()), default: "''",
        allowed_grades: nullable(jsonb()), default: "'[]'::jsonb",
        birthdate_cutoff: nullable(text()), default: "''",
        must_be_born_on_or_after_birthdate_cutoff: nullable(text()), default: "''",
        must_be_born_before_birthdate_cutoff: nullable(text()), default: "''",
        allowed_ages: nullable(jsonb()), default: "'[]'::jsonb",
    },
    indexes: {
        // System field indexes ( REQUIRED )
        system_indexes!("stories"),

        // Custom table-specific indexes - all non-primary key fields
        idx_stories_name: {
            columns: ["name"],
            unique: false,
            type: "btree"
        },
        idx_stories_course_id: {
            columns: ["course_id"],
            unique: false,
            type: "btree"
        },
        idx_stories_order: {
            columns: ["order"],
            unique: false,
            type: "btree"
        },
        idx_stories_description: {
            columns: ["description"],
            unique: false,
            type: "btree"
        },
        idx_stories_story_identifier: {
            columns: ["story_identifier"],
            unique: false,
            type: "btree"
        },
        idx_stories_bundle_file_name: {
            columns: ["bundle_file_name"],
            unique: false,
            type: "btree"
        },
        idx_stories_allowed_grades: {
            columns: ["allowed_grades"],
            unique: false,
            type: "btree"
        },
        idx_stories_birthdate_cutoff: {
            columns: ["birthdate_cutoff"],
            unique: false,
            type: "btree"
        },
        idx_stories_allowed_ages: {
            columns: ["allowed_ages"],
            unique: false,
            type: "btree"
        },
    },
    foreign_keys: {
        // System field foreign keys ( REQUIRED )
        system_foreign_keys!("stories"),

        // Custom foreign keys - all fields ending with "_id"
        fk_stories_course_id: {
            columns: ["course_id"],
            foreign_table: "courses",
            foreign_columns: ["id"],
            on_delete: "no action",
            on_update: "no action"
        },
    }
}
