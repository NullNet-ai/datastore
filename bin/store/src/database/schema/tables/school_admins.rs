use crate::define_table_schema;
use crate::generated::schema::generator::diesel_schema_definition::{
    types::*, DieselTableDefinition,
};
use crate::{system_fields, system_foreign_keys, system_indexes};

/// School admins table for managing school-level administrators
pub struct SchoolAdminsTable;

define_table_schema! {
    hypertable: false,
    fields: {
        // System fields - common across all tables
        system_fields!(),

        // School admins specific fields
        school_id: nullable(text()),
        school_admin_id: nullable(text()),
        department_id: nullable(text()),
        district_id: nullable(text()),
    },
    indexes: {
        // System field indexes
        system_indexes!("school_admins"),

        // Custom table-specific indexes
        idx_school_admins_school_id: {
            columns: ["school_id"],
            unique: false,
            type: "btree"
        },
        idx_school_admins_school_admin_id: {
            columns: ["school_admin_id"],
            unique: false,
            type: "btree"
        },
        idx_school_admins_department_id: {
            columns: ["department_id"],
            unique: false,
            type: "btree"
        },
        idx_school_admins_district_id: {
            columns: ["district_id"],
            unique: false,
            type: "btree"
        },
    },
    foreign_keys: {
        // System field foreign keys
        system_foreign_keys!("school_admins"),

        // School admin foreign keys
        fk_school_admins_school_id: {
            columns: ["school_id"],
            foreign_table: "organizations",
            foreign_columns: ["id"],
            on_delete: "no action",
            on_update: "no action"
        },
        fk_school_admins_school_admin_id: {
            columns: ["school_admin_id"],
            foreign_table: "contacts",
            foreign_columns: ["id"],
            on_delete: "no action",
            on_update: "no action"
        },
        fk_school_admins_department_id: {
            columns: ["department_id"],
            foreign_table: "organizations",
            foreign_columns: ["id"],
            on_delete: "no action",
            on_update: "no action"
        },
        fk_school_admins_district_id: {
            columns: ["district_id"],
            foreign_table: "organizations",
            foreign_columns: ["id"],
            on_delete: "no action",
            on_update: "no action"
        },
    }
}