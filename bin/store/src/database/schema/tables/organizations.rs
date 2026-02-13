use crate::define_table_schema;
use crate::generated::schema::generator::diesel_schema_definition::{
    types::*, DieselTableDefinition,
};
use crate::{system_fields, system_foreign_keys, system_indexes};

/// Organizations table for hierarchical organization structure
pub struct OrganizationsTable;

define_table_schema! {
    hypertable: false,
    fields: {
        // System fields - common across all tables
        system_fields!(),

        // Organizations specific fields
        name: nullable(text()), default: "''",
        skyll_id: nullable(text()),
        department_id: nullable(text()),
        district_id: nullable(text()),
        parent_organization_id: nullable(text()),
        school_id: nullable(text()),
        city: nullable(text()),
        county: nullable(text()),
        state: nullable(text()), default: "''",
        school_identifier: nullable(text()),
        district_identifier: nullable(text()),
        organization_level: nullable(integer()), default: "0",
        root_organization_id: nullable(text()),
        path_level: nullable(jsonb()),
        superintendent_id: nullable(text()),
        principal_id: nullable(text()),
    },
    indexes: {
        // System field indexes
        system_indexes!("organizations"),

        // Custom table-specific indexes
        idx_organizations_name: {
            columns: ["name"],
            unique: false,
            type: "btree"
        },
        idx_organizations_parent_organization_id: {
            columns: ["parent_organization_id"],
            unique: false,
            type: "btree"
        },
        idx_organizations_root_organization_id: {
            columns: ["root_organization_id"],
            unique: false,
            type: "btree"
        },
        idx_organizations_skyll_id: {
            columns: ["skyll_id"],
            unique: false,
            type: "btree"
        },
        idx_organizations_school_id: {
            columns: ["school_id"],
            unique: false,
            type: "btree"
        },
        idx_organizations_district_id: {
            columns: ["district_id"],
            unique: false,
            type: "btree"
        },
        idx_organizations_department_id: {
            columns: ["department_id"],
            unique: false,
            type: "btree"
        },
        idx_organizations_city: {
            columns: ["city"],
            unique: false,
            type: "btree"
        },
        idx_organizations_county: {
            columns: ["county"],
            unique: false,
            type: "btree"
        },
        idx_organizations_state: {
            columns: ["state"],
            unique: false,
            type: "btree"
        },
        idx_organizations_school_identifier: {
            columns: ["school_identifier"],
            unique: false,
            type: "btree"
        },
        idx_organizations_district_identifier: {
            columns: ["district_identifier"],
            unique: false,
            type: "btree"
        },
        idx_organizations_organization_level: {
            columns: ["organization_level"],
            unique: false,
            type: "btree"
        },
        idx_organizations_path_level: {
            columns: ["path_level"],
            unique: false,
            type: "btree"
        },
        idx_organizations_superintendent_id: {
            columns: ["superintendent_id"],
            unique: false,
            type: "btree"
        },
        idx_organizations_principal_id: {
            columns: ["principal_id"],
            unique: false,
            type: "btree"
        },
    },
    foreign_keys: {
        // System field foreign keys
        system_foreign_keys!("organizations"),

        // Self-referencing foreign keys
        fk_organizations_parent_organization_id: {
            columns: ["parent_organization_id"],
            foreign_table: "organizations",
            foreign_columns: ["id"],
            on_delete: "no action",
            on_update: "no action"
        },
        fk_organizations_root_organization_id: {
            columns: ["root_organization_id"],
            foreign_table: "organizations",
            foreign_columns: ["id"],
            on_delete: "no action",
            on_update: "no action"
        }
    }
}
