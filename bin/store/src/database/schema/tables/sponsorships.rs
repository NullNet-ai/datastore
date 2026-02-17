use crate::define_table_schema;
use crate::schema::generator::diesel_schema_definition::{types::*, DieselTableDefinition};

pub struct SponsorshipsTable;

define_table_schema! {
    hypertable: false,
    fields: {
        system_fields!(),
        name: nullable(text()),
        sponsor_organization_id: nullable(text()),
        sponsor_sub_organization_id: nullable(text()),
        sponsor_website: nullable(text()),
        sponsor_status: nullable(text()),
        start_date: nullable(text()),
        start_time: nullable(text()), default: "'00:00'",
        end_date: nullable(text()),
        end_time: nullable(text()), default: "'00:00'",
    },
    indexes: {
        system_indexes!("sponsorships"),
        idx_sponsorships_name: { columns: ["name"], unique: false, type: "btree" },
        idx_sponsorships_sponsor_organization_id: { columns: ["sponsor_organization_id"], unique: false, type: "btree" },
        idx_sponsorships_sponsor_sub_organization_id: { columns: ["sponsor_sub_organization_id"], unique: false, type: "btree" },
        idx_sponsorships_sponsor_website: { columns: ["sponsor_website"], unique: false, type: "btree" },
        idx_sponsorships_sponsor_status: { columns: ["sponsor_status"], unique: false, type: "btree" },
        idx_sponsorships_start_date: { columns: ["start_date"], unique: false, type: "btree" },
        idx_sponsorships_start_time: { columns: ["start_time"], unique: false, type: "btree" },
        idx_sponsorships_end_date: { columns: ["end_date"], unique: false, type: "btree" },
        idx_sponsorships_end_time: { columns: ["end_time"], unique: false, type: "btree" },
    },
    foreign_keys: {
        system_foreign_keys!("sponsorships"),
        fk_sponsorships_sponsor_organization_id: { columns: ["sponsor_organization_id"], foreign_table: "organizations", foreign_columns: ["id"], on_delete: "no action", on_update: "no action" },
        fk_sponsorships_sponsor_sub_organization_id: { columns: ["sponsor_sub_organization_id"], foreign_table: "organizations", foreign_columns: ["id"], on_delete: "no action", on_update: "no action" },
    }
}
