use crate::define_table_schema;
use crate::schema::generator::diesel_schema_definition::{types::*, DieselTableDefinition};
use crate::{system_fields, system_foreign_keys, system_indexes};

/**
 * For defining proper diesel types check it here: bin/store/src/builders/generator/README.md
 */

/// District admins table for managing district admin information
pub struct DistrictAdminsTable;

define_table_schema! {
    hypertable: false,
    fields: {
        // System fields - common across all tables ( REQUIRED )
        system_fields!(),

        // District admins fields
        district_id: nullable(text()),
        district_admin_id: nullable(integer()), default: "0",
        department_id: nullable(text()),
    },
    indexes: {
        // System field indexes ( REQUIRED )
        system_indexes!("district_admins"),

        // Custom table-specific indexes - all non-primary key fields
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
        // System field foreign keys ( REQUIRED )
        system_foreign_keys!("district_admins"),

        // Custom foreign keys - no fields ending with "_id" in this table
        fk_district_admins_district_id: { columns: ["district_id"], foreign_table: "organizations", foreign_columns: ["id"], on_delete: "no action", on_update: "no action" },
        fk_district_admins_district_admin_id: { columns: ["district_admin_id"], foreign_table: "contacts", foreign_columns: ["id"], on_delete: "no action", on_update: "no action" },
        fk_district_admins_department_id: { columns: ["department_id"], foreign_table: "organizations", foreign_columns: ["id"], on_delete: "no action", on_update: "no action" },
    }
}
