use crate::define_table_schema;
use crate::schema::generator::diesel_schema_definition::{types::*, DieselTableDefinition};
use crate::{system_fields, system_foreign_keys, system_indexes};

/**
 * For defining proper diesel types check it here: bin/store/src/builders/generator/README.md
 */

/// Classroom course stories table for managing story assignments
pub struct ClassroomCourseStoriesTable;

define_table_schema! {
    hypertable: false,
    fields: {
        // System fields - common across all tables ( REQUIRED )
        system_fields!(),

        // Classroom course stories fields
        classroom_id: nullable(text()),
        course_id: nullable(text()),
        story_id: nullable(text()),
        start_date: nullable(text()),
        order: nullable(integer()), default: "0",
    },
    indexes: {
        // System field indexes ( REQUIRED )
        system_indexes!("classroom_course_stories"),

        // Custom table-specific indexes - all non-primary key fields
        idx_classroom_course_stories_classroom_id: {
            columns: ["classroom_id"],
            unique: false,
            type: "btree"
        },
        idx_classroom_course_stories_course_id: {
            columns: ["course_id"],
            unique: false,
            type: "btree"
        },
        idx_classroom_course_stories_story_id: {
            columns: ["story_id"],
            unique: false,
            type: "btree"
        },
        idx_classroom_course_stories_start_date: {
            columns: ["start_date"],
            unique: false,
            type: "btree"
        },
        idx_classroom_course_stories_order: {
            columns: ["order"],
            unique: false,
            type: "btree"
        },
    },
    foreign_keys: {
        // System field foreign keys ( REQUIRED )
        system_foreign_keys!("classroom_course_stories"),

        // Custom foreign keys - all fields ending with "_id"
        fk_classroom_course_stories_classroom_id: {
            columns: ["classroom_id"],
            foreign_table: "classrooms",
            foreign_columns: ["id"],
            on_delete: "no action",
            on_update: "no action"
        },
        fk_classroom_course_stories_course_id: {
            columns: ["course_id"],
            foreign_table: "courses",
            foreign_columns: ["id"],
            on_delete: "no action",
            on_update: "no action"
        },
        fk_classroom_course_stories_story_id: {
            columns: ["story_id"],
            foreign_table: "stories",
            foreign_columns: ["id"],
            on_delete: "no action",
            on_update: "no action"
        },
    }
}
