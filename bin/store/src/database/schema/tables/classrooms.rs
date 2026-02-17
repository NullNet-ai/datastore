use crate::define_table_schema;
use crate::schema::generator::diesel_schema_definition::{types::*, DieselTableDefinition};
use crate::{system_fields, system_foreign_keys, system_indexes};

/**
 * For defining proper diesel types check it here: bin/store/src/builders/generator/README.md
 */

/// Classrooms table for managing classroom information
pub struct ClassroomsTable;

define_table_schema! {
    hypertable: false,
    fields: {
        // System fields - common across all tables ( REQUIRED )
        system_fields!(),

        // Classrooms fields
        name: nullable(text()),
        description: nullable(text()),
        grade: nullable(text()),
        color: nullable(text()),
        avatar: nullable(text()),
        department_id: nullable(text()),
        district_id: nullable(text()),
        school_id: nullable(text()),
        teacher_id: nullable(text()),
    },
    indexes: {
        // System field indexes ( REQUIRED )
        system_indexes!("classrooms"),

        // Custom table-specific indexes - all non-primary key fields
        idx_classrooms_name: {
            columns: ["name"],
            unique: false,
            type: "btree"
        },
        idx_classrooms_description: {
            columns: ["description"],
            unique: false,
            type: "btree"
        },
        idx_classrooms_grade: {
            columns: ["grade"],
            unique: false,
            type: "btree"
        },
        idx_classrooms_color: {
            columns: ["color"],
            unique: false,
            type: "btree"
        },
        idx_classrooms_avatar: {
            columns: ["avatar"],
            unique: false,
            type: "btree"
        },
        idx_classrooms_department_id: {
            columns: ["department_id"],
            unique: false,
            type: "btree"
        },
        idx_classrooms_district_id: {
            columns: ["district_id"],
            unique: false,
            type: "btree"
        },
        idx_classrooms_school_id: {
            columns: ["school_id"],
            unique: false,
            type: "btree"
        },
        idx_classrooms_teacher_id: {
            columns: ["teacher_id"],
            unique: false,
            type: "btree"
        },
    },
    foreign_keys: {
        // System field foreign keys ( REQUIRED )
        system_foreign_keys!("classrooms"),

        // Custom foreign keys - all fields ending with "_id"
        fk_classrooms_department_id: {
            columns: ["department_id"],
            foreign_table: "organizations",
            foreign_columns: ["id"],
            on_delete: "no action",
            on_update: "no action"
        },
        fk_classrooms_district_id: {
            columns: ["district_id"],
            foreign_table: "organizations",
            foreign_columns: ["id"],
            on_delete: "no action",
            on_update: "no action"
        },
        fk_classrooms_school_id: {
            columns: ["school_id"],
            foreign_table: "organizations",
            foreign_columns: ["id"],
            on_delete: "no action",
            on_update: "no action"
        },
        fk_classrooms_teacher_id: {
            columns: ["teacher_id"],
            foreign_table: "contacts",
            foreign_columns: ["id"],
            on_delete: "no action",
            on_update: "no action"
        },
    }
}
