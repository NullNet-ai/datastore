use crate::define_table_schema;
use crate::schema::generator::diesel_schema_definition::{types::*, DieselTableDefinition};
use crate::{system_fields, system_foreign_keys, system_indexes};

/**
 * For defining proper diesel types check it here: bin/store/src/builders/generator/README.md
 */

/// Teacher schools table for managing teacher-school assignments
pub struct TeacherSchoolsTable;

define_table_schema! {
    hypertable: false,
    fields: {
        // System fields - common across all tables ( REQUIRED )
        system_fields!(),

        // Teacher schools fields
        school_id: nullable(text()),
        teacher_id: nullable(text()),
        district_id: nullable(text()),
        department_id: nullable(text()),
    },
    indexes: {
        // System field indexes ( REQUIRED )
        system_indexes!("teacher_schools"),

        // Custom table-specific indexes - all non-primary key fields
        idx_school_id: {
            columns: ["school_id"],
            unique: false,
            type: "btree"
        },
        idx_teacher_id: {
            columns: ["teacher_id"],
            unique: false,
            type: "btree"
        },
        idx_district_id: {
            columns: ["district_id"],
            unique: false,
            type: "btree"
        },
        idx_department_id: {
            columns: ["department_id"],
            unique: false,
            type: "btree"
        },
    },
    foreign_keys: {
        // System field foreign keys ( REQUIRED )
        system_foreign_keys!("teacher_schools"),

        // Custom foreign keys - all fields ending with "_id"
        fk_school_id: {
            columns: ["school_id"],
            foreign_table: "organizations",
            foreign_columns: ["id"],
            on_delete: "no action",
            on_update: "no action"
        },
        fk_teacher_id: {
            columns: ["teacher_id"],
            foreign_table: "contacts",
            foreign_columns: ["id"],
            on_delete: "no action",
            on_update: "no action"
        },
        fk_district_id: {
            columns: ["district_id"],
            foreign_table: "organizations",
            foreign_columns: ["id"],
            on_delete: "no action",
            on_update: "no action"
        },
        fk_department_id: {
            columns: ["department_id"],
            foreign_table: "organizations",
            foreign_columns: ["id"],
            on_delete: "no action",
            on_update: "no action"
        },
    }
}
