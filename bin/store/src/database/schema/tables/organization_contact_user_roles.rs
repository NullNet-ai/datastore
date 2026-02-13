use crate::define_table_schema;
use crate::generated::schema::generator::diesel_schema_definition::{
    types::*, DieselTableDefinition,
};
use crate::{system_fields, system_foreign_keys, system_indexes};

/// Files table for file storage and management
pub struct FilesTable;

define_table_schema! {
    hypertable: false,
    fields: {
        // System fields - common across all tables ( REQUIRED )
        system_fields!(),

        // File-specific fields
        organization_contact_id: nullable(text()),
        user_role_id: nullable(text()),
    },
    indexes: {
        // System field indexes ( REQUIRED )
        system_indexes!("organization_contact_user_roles"),

        // Custom table-specific indexes
        idx_organization_contact_id: {
            columns: ["organization_contact_id"],
            unique: false,
            type: "btree"
        },
        idx_user_role_id: {
            columns: ["user_role_id"],
            unique: false,
            type: "btree"
        },

    },
    foreign_keys: {
        // System field foreign keys ( REQUIRED )
        system_foreign_keys!("organization_contact_user_roles"),

         // Custom foreign keys
        fk_organization_contact_id: {
            columns: ["organization_contact_id"],
            foreign_table: "organization_contacts",
            foreign_columns: ["id"],
            on_delete: "no action",
            on_update: "no action"
        },
        fk_user_role_id: {
            columns: ["user_role_id"],
            foreign_table: "user_roles",
            foreign_columns: ["id"],
            on_delete: "no action",
            on_update: "no action"
        },
    }
}
