use crate::schema::generator::diesel_schema_definition::{
    boolean, define_table_schema, foreign_key, index, integer, jsonb, nullable, text, timestamp,
};

define_table_schema! {
    hypertable: false,
    fields: {
        system_fields!(),
        smtp_payload_id: nullable(text()),
        transaction_status: nullable(text()),
        response_message: nullable(text()),
        trigger_type: nullable(text()), default "system",
    },
    indexes: {
        system_indexes!("smtp_transactions"),
        idx_smtp_transactions_smtp_payload_id: {
            columns: ["smtp_payload_id"],
            unique: false,
            type: "btree"
        },
        idx_smtp_transactions_transaction_status: {
            columns: ["transaction_status"],
            unique: false,
            type: "btree"
        },
        idx_smtp_transactions_response_message: {
            columns: ["response_message"],
            unique: false,
            type: "btree"
        },
        idx_smtp_transactions_trigger_type: {
            columns: ["trigger_type"],
            unique: false,
            type: "btree"
        },
    },
    foreign_keys: {
        system_foreign_keys!("smtp_transactions"),
        fk_smtp_transactions_smtp_payload_id: { columns: ["smtp_payload_id"], foreign_table: "smtp_payloads", foreign_columns: ["id"], on_delete: "no action", on_update: "no action" },
    }
}
