-- Add a BIGSERIAL position column so chunk rows can be fetched in strict insertion order.
-- This fixes the FK-violation bug where messages were returned out of dependency order
-- because ORDER BY record_id (ULID) was non-deterministic within the same millisecond.
ALTER TABLE crdt_client_messages ADD COLUMN position BIGSERIAL;

-- Replace the old (client_id, record_id) index with one on (client_id, position).
-- The new index satisfies:
--   WHERE client_id = ?  ORDER BY position  OFFSET n LIMIT m   (get_chunk)
--   WHERE client_id = ?  COUNT(*)                               (get_chunk_status)
DROP INDEX IF EXISTS idx_crdt_client_messages_client_id_record_id;
CREATE INDEX idx_crdt_client_messages_client_id_position
    ON crdt_client_messages (client_id, position);
