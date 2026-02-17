use crate::define_table_schema;
use crate::schema::generator::diesel_schema_definition::{types::*, DieselTableDefinition};

pub struct AvatarSelectionsTable;

define_table_schema! {
    hypertable: false,
    fields: {
        system_fields!(),
        user_id: nullable(text()),
        gender: nullable(text()),
        skin_color: nullable(text()),
        hair_color: nullable(text()),
        eye_color: nullable(text()),
        hair_style: nullable(text()),
        clothing: nullable(text()),
    },
    indexes: {
        system_indexes!("avatar_selections"),
        idx_avatar_selections_user_id: { columns: ["user_id"], unique: false, type: "btree" },
        idx_avatar_selections_gender: { columns: ["gender"], unique: false, type: "btree" },
        idx_avatar_selections_skin_color: { columns: ["skin_color"], unique: false, type: "btree" },
        idx_avatar_selections_hair_color: { columns: ["hair_color"], unique: false, type: "btree" },
        idx_avatar_selections_eye_color: { columns: ["eye_color"], unique: false, type: "btree" },
        idx_avatar_selections_hair_style: { columns: ["hair_style"], unique: false, type: "btree" },
        idx_avatar_selections_clothing: { columns: ["clothing"], unique: false, type: "btree" },
    },
    foreign_keys: {
        system_foreign_keys!("avatar_selections"),
        fk_avatar_selections_user_id: { columns: ["user_id"], foreign_table: "contacts", foreign_columns: ["id"], on_delete: "no action", on_update: "no action" },
    }
}
