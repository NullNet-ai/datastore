use crate::define_table_schema;
use crate::generated::schema::generator::diesel_schema_definition::{
    types::*, DieselTableDefinition,
};
use crate::{system_fields, system_foreign_keys, system_indexes};

/**
 * For defining proper diesel types check it here: bin/store/src/builders/generator/README.md
 */

/// Grid filters table for storing grid view configurations
pub struct GridFiltersTable;

define_table_schema! {
    hypertable: false,
    fields: {
        // System fields - common across all tables ( REQUIRED )
        system_fields!(),

        // Grid filter fields
        name: nullable(text()),
        grid_id: nullable(text()),
        link: nullable(text()), default: "''",
        is_current: nullable(boolean()), default: "false",
        is_default: nullable(boolean()), default: "false",
        contact_id: nullable(text()),
        account_organization_id: nullable(text()),
        entity: nullable(text()),
        columns: nullable(jsonb()), default: "'[]'::jsonb",
        groups: nullable(jsonb()), default: "'[]'::jsonb",
        sorts: nullable(jsonb()), default: "'[]'::jsonb",
        default_sorts: nullable(jsonb()), default: "'[]'::jsonb",
        advance_filters: nullable(jsonb()), default: "'[]'::jsonb",
        group_advance_filters: nullable(jsonb()), default: "'[]'::jsonb",
        filter_groups: nullable(jsonb()), default: "'[]'::jsonb",
    },
    indexes: {
        // System field indexes ( REQUIRED )
        system_indexes!("grid_filters"),

        // Custom table-specific indexes
        idx_grid_filters_name: {
            columns: ["name"],
            unique: false,
            type: "btree"
        },
        idx_grid_filters_grid_id: {
            columns: ["grid_id"],
            unique: false,
            type: "btree"
        },
        idx_grid_filters_link: {
            columns: ["link"],
            unique: false,
            type: "btree"
        },
        idx_grid_filters_is_current: {
            columns: ["is_current"],
            unique: false,
            type: "btree"
        },
        idx_grid_filters_is_default: {
            columns: ["is_default"],
            unique: false,
            type: "btree"
        },
        idx_grid_filters_contact_id: {
            columns: ["contact_id"],
            unique: false,
            type: "btree"
        },
        idx_grid_filters_entity: {
            columns: ["entity"],
            unique: false,
            type: "btree"
        },
    },
    foreign_keys: {
        // System field foreign keys ( REQUIRED )
        system_foreign_keys!("grid_filters"),

        // Custom foreign keys
        fk_grid_filters_contact_id: {
            columns: ["contact_id"],
            foreign_table: "contacts",
            foreign_columns: ["id"],
            on_delete: "no action",
            on_update: "no action"
        },
        fk_grid_filters_account_organization_id: {
            columns: ["account_organization_id"],
            foreign_table: "account_organizations",
            foreign_columns: ["id"],
            on_delete: "no action",
            on_update: "no action"
        },
    }
}
