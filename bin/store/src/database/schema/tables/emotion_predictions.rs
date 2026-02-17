use crate::define_table_schema;
use crate::schema::generator::diesel_schema_definition::{types::*, DieselTableDefinition};

pub struct EmotionPredictionsTable;

define_table_schema! {
    hypertable: false,
    fields: {
        system_fields!(),
        conversation_id: nullable(text()),
        model: nullable(text()),
        prob: nullable(double_precision()),
        box: nullable(jsonb()), default: "'{}'::jsonb",
        emotions: nullable(jsonb()), default: "'[]'::jsonb",
        time: nullable(double_precision()),
        id: nullable(text()),
        timestamp: nullable(timestamptz()),
        hypertable_timestamp: nullable(text()),
    },
    indexes: {
        system_indexes!("emotion_predictions"),
        idx_emotion_predictions_conversation_id: { columns: ["conversation_id"], unique: false, type: "btree" },
        idx_emotion_predictions_model: { columns: ["model"], unique: false, type: "btree" },
        idx_emotion_predictions_prob: { columns: ["prob"], unique: false, type: "btree" },
        idx_emotion_predictions_box: { columns: ["box"], unique: false, type: "btree" },
        idx_emotion_predictions_emotions: { columns: ["emotions"], unique: false, type: "btree" },
        idx_emotion_predictions_time: { columns: ["time"], unique: false, type: "btree" },
        idx_emotion_predictions_id: { columns: ["id"], unique: false, type: "btree" },
        idx_emotion_predictions_timestamp: { columns: ["timestamp"], unique: false, type: "btree" },
        idx_emotion_predictions_hypertable_timestamp: { columns: ["hypertable_timestamp"], unique: false, type: "btree" },
    },
    foreign_keys: {
        system_foreign_keys!("emotion_predictions"),
        fk_emotion_predictions_conversation_id: { columns: ["conversation_id"], foreign_table: "conversations", foreign_columns: ["id"], on_delete: "no action", on_update: "no action" },
    }
}
