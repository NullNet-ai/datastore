use crate::define_table_schema;
use crate::generated::schema::generator::diesel_schema_definition::{
    types::*, DieselTableDefinition,
};
use crate::{system_fields, system_foreign_keys, system_indexes};

/// District admins table for managing district-level administrators
pub struct DistrictAdminsTable;

define_table_schema! {
    hypertable: false,
    fields: {
        // System fields - common across all tables
        system_fields!(),

        // District admins specific fields
        district_id: nullable(text()),
        district_admin_id: nullable(text()),
        department_id: nullable(text()),
    },
    indexes: {
        // System field indexes
        system_indexes!("district_admins"),

        // Custom table-specific indexes
        idx_district_admins_district_id: {
            columns: ["district_id"],
            unique: false,
            type: "btree"
        },
        idx_district_admins_district_admin_id: {
            columns: ["district_admin_id"],
            unique: false,
            type: "btree"
        },
        idx_district_admins_department_id: {
            columns: ["department_id"],
            unique: false,
            type: "btree"
        },
    },
    foreign_keys: {
        // System field foreign keys
        system_foreign_keys!("district_admins"),

        // District admin foreign keys
        fk_district_admins_district_id: {
            columns: ["district_id"],
            foreign_table: "organizations",
            foreign_columns: ["id"],
            on_delete: "no action",
            on_update: "no action"
        },
        fk_district_admins_district_admin_id: {
            columns: ["district_admin_id"],
            foreign_table: "contacts",
            foreign_columns: ["id"],
            on_delete: "no action",
            on_update: "no action"
        },
        fk_district_admins_department_id: {
            columns: ["department_id"],
            foreign_table: "organizations",
            foreign_columns: ["id"],
            on_delete: "no action",
            on_update: "no action"
        },
    }
}