use crate::schema::generator::diesel_schema_definition::{
    DieselTableDefinition, types::*
};
use crate::define_table_schema;

pub struct ConversationMessagesTable;

define_table_schema! {
    hypertable: false,
    fields: {
        system_fields!(),
        conversation_id: nullable(text()),
        content: nullable(text()),
        role: nullable(text()),
    },
    indexes: {
        system_indexes!("conversation_messages"),
        idx_conversation_messages_conversation_id: { columns: ["conversation_id"], unique: false, type: "btree" },
        idx_conversation_messages_content: { columns: ["content"], unique: false, type: "btree" },
        idx_conversation_messages_role: { columns: ["role"], unique: false, type: "btree" },
    },
    foreign_keys: {
        system_foreign_keys!("conversation_messages"),
        fk_conversation_messages_conversation_id: { columns: ["conversation_id"], foreign_table: "conversations", foreign_columns: ["id"], on_delete: "no action", on_update: "no action" },
    }
}