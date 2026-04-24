use crate::define_table_schema;
use crate::generated::schema::generator::diesel_schema_definition::{
    types::*, DieselTableDefinition,
};
use crate::{system_fields, system_foreign_keys, system_indexes};

/**
 * Auto-converted from TypeScript schema source.
 */
pub struct GridFiltersTable;

define_table_schema! {
    hypertable: false,
    fields: {
        // System fields - common across all tables ( REQUIRED )
        system_fields!(),

        name: nullable(text()),
        grid_id: nullable(text()),
        link: nullable(text()), default "",
        is_current: nullable(boolean()), default "false",
        is_default: nullable(boolean()), default "false",
        contact_id: nullable(text()),
        account_organization_id: nullable(text()),
        entity: nullable(text()),
        columns: nullable(jsonb()), default "'[]'::jsonb",
        groups: nullable(jsonb()), default "'[]'::jsonb",
        sorts: nullable(jsonb()), default "'[]'::jsonb",
        default_sorts: nullable(jsonb()), default "'[]'::jsonb",
        advance_filters: nullable(jsonb()), default "'[]'::jsonb",
        group_advance_filters: nullable(jsonb()), default "'[]'::jsonb",
        filter_groups: nullable(jsonb()), default "'[]'::jsonb",
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
        fk_grid_filters_contact_id: { columns: ["contact_id"], foreign_table: "contacts", foreign_columns: ["id"], on_delete: "no action", on_update: "no action" },
    }
}
