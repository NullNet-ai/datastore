use crate::define_table_schema;
use crate::schema::generator::diesel_schema_definition::{types::*, DieselTableDefinition};

pub struct ConversationTopicsTable;

define_table_schema! {
    hypertable: false,
    fields: {
        system_fields!(),
        title: nullable(text()),
        topic_contact_categories: nullable(array(text())), default: "[]",
        topic_contact_roles: nullable(array(text())), default: "[]",
        age_start: nullable(integer()),
        age_end: nullable(integer()),
        topic_status: nullable(text()),
        is_show_to_new_contacts_only: nullable(boolean()),
        order: nullable(integer()), default: "0",
        source: nullable(text()),
    },
    indexes: {
        system_indexes!("conversation_topics"),
        idx_conversation_topics_title: { columns: ["title"], unique: false, type: "btree" },
        idx_conversation_topics_topic_contact_categories: { columns: ["topic_contact_categories"], unique: false, type: "btree" },
        idx_conversation_topics_topic_contact_roles: { columns: ["topic_contact_roles"], unique: false, type: "btree" },
        idx_conversation_topics_age_start: { columns: ["age_start"], unique: false, type: "btree" },
        idx_conversation_topics_age_end: { columns: ["age_end"], unique: false, type: "btree" },
        idx_conversation_topics_topic_status: { columns: ["topic_status"], unique: false, type: "btree" },
        idx_conversation_topics_is_show_to_new_contacts_only: { columns: ["is_show_to_new_contacts_only"], unique: false, type: "btree" },
        idx_conversation_topics_order: { columns: ["order"], unique: false, type: "btree" },
        idx_conversation_topics_source: { columns: ["source"], unique: false, type: "btree" },
    },
    foreign_keys: {
        system_foreign_keys!("conversation_topics"),
    }
}
