DROP INDEX IF EXISTS idx_crdt_client_messages_client_id_position;
ALTER TABLE crdt_client_messages DROP COLUMN IF EXISTS position;
CREATE INDEX idx_crdt_client_messages_client_id_record_id
    ON crdt_client_messages (client_id, record_id);
