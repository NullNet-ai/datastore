use crate::schema::generator::diesel_schema_definition::{
    define_table_schema, foreign_key, index, nullable, text, timestamp, jsonb, boolean, integer
};

define_table_schema! {
    hypertable: false,
    fields: {
        system_fields!(),
        payload_id: nullable(text()),
        message_id: nullable(text()),
        thread_id: nullable(text()),
        status: nullable(text()),
        response_code: nullable(integer()),
        response_message: nullable(text()),
        sent_at: nullable(timestamp()),
        delivered_at: nullable(timestamp()),
        opened_at: nullable(timestamp()),
        clicked_at: nullable(timestamp()),
        bounced_at: nullable(timestamp()),
        complained_at: nullable(timestamp()),
        unsubscribed_at: nullable(timestamp()),
        retry_count: nullable(integer()), default: "0",
        last_retry_at: nullable(timestamp()),
        error_details: nullable(jsonb()), default: "'{}'::jsonb",
        metadata: nullable(jsonb()), default: "'{}'::jsonb",
    },
    indexes: {
        system_indexes!("smtp_transactions"),
        idx_smtp_transactions_payload_id: { columns: ["payload_id"], unique: false, type: "btree" },
        idx_smtp_transactions_message_id: { columns: ["message_id"], unique: false, type: "btree" },
        idx_smtp_transactions_thread_id: { columns: ["thread_id"], unique: false, type: "btree" },
        idx_smtp_transactions_status: { columns: ["status"], unique: false, type: "btree" },
        idx_smtp_transactions_response_code: { columns: ["response_code"], unique: false, type: "btree" },
        idx_smtp_transactions_sent_at: { columns: ["sent_at"], unique: false, type: "btree" },
        idx_smtp_transactions_delivered_at: { columns: ["delivered_at"], unique: false, type: "btree" },
        idx_smtp_transactions_opened_at: { columns: ["opened_at"], unique: false, type: "btree" },
        idx_smtp_transactions_clicked_at: { columns: ["clicked_at"], unique: false, type: "btree" },
        idx_smtp_transactions_bounced_at: { columns: ["bounced_at"], unique: false, type: "btree" },
        idx_smtp_transactions_complained_at: { columns: ["complained_at"], unique: false, type: "btree" },
        idx_smtp_transactions_unsubscribed_at: { columns: ["unsubscribed_at"], unique: false, type: "btree" },
        idx_smtp_transactions_retry_count: { columns: ["retry_count"], unique: false, type: "btree" },
        idx_smtp_transactions_last_retry_at: { columns: ["last_retry_at"], unique: false, type: "btree" },
    },
    foreign_keys: {
        system_foreign_keys!("smtp_transactions"),
        fk_smtp_transactions_payload_id: { columns: ["payload_id"], foreign_table: "smtp_payloads", foreign_columns: ["id"], on_delete: "no action", on_update: "no action" },
    }
}