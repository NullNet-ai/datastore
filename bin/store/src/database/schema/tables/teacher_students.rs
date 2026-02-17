use crate::define_table_schema;
use crate::schema::generator::diesel_schema_definition::{types::*, DieselTableDefinition};
use crate::{system_fields, system_foreign_keys, system_indexes};

/**
 * For defining proper diesel types check it here: bin/store/src/builders/generator/README.md
 */

/// Teacher students table for managing teacher-student assignments
pub struct TeacherStudentsTable;

define_table_schema! {
    hypertable: false,
    fields: {
        // System fields - common across all tables ( REQUIRED )
        system_fields!(),

        // Teacher students fields
        teacher_id: nullable(text()),
        student_id: nullable(text()),
    },
    indexes: {
        // System field indexes ( REQUIRED )
        system_indexes!("teacher_students"),

        // Custom table-specific indexes - all non-primary key fields
        idx_teacher_students_teacher_id: {
            columns: ["teacher_id"],
            unique: false,
            type: "btree"
        },
        idx_teacher_students_student_id: {
            columns: ["student_id"],
            unique: false,
            type: "btree"
        },
    },
    foreign_keys: {
        // System field foreign keys ( REQUIRED )
        system_foreign_keys!("teacher_students"),

        // Custom foreign keys - all fields ending with "_id"
        fk_teacher_students_teacher_id: {
            columns: ["teacher_id"],
            foreign_table: "contacts",
            foreign_columns: ["id"],
            on_delete: "no action",
            on_update: "no action"
        },
        fk_teacher_students_student_id: {
            columns: ["student_id"],
            foreign_table: "contacts",
            foreign_columns: ["id"],
            on_delete: "no action",
            on_update: "no action"
        },
    }
}
