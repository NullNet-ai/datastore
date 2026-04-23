-- This file should undo anything in `up.sql`
-- Drop foreign key constraints first
ALTER TABLE IF EXISTS "sync_queue_items" 
DROP CONSTRAINT IF EXISTS "sync_queue_items_group_id_sync_queue_items_id_fk";

ALTER TABLE IF EXISTS "sync_endpoint_groups" 
DROP CONSTRAINT IF EXISTS "sync_endpoint_groups_sync_endpoint_id_sync_endpoints_id_fk";

-- Drop crdt_messages indexes before dropping the table
DROP INDEX IF EXISTS "idx_crdt_messages_group_id_timestamp_client_id";
DROP INDEX IF EXISTS "idx_crdt_messages_group_id_client_id";

-- Drop crdt_client_messages indexes before dropping the table
DROP INDEX IF EXISTS "idx_crdt_client_messages_client_id_record_id";

-- Drop tables in reverse order of creation
DROP TABLE IF EXISTS "sync_transactions";
DROP TABLE IF EXISTS "sync_endpoint_groups";
DROP TABLE IF EXISTS "sync_endpoints";
DROP TABLE IF EXISTS "sync_queues";
DROP TABLE IF EXISTS "sync_queue_items";
DROP TABLE IF EXISTS "crdt_client_messages";
DROP TABLE IF EXISTS "crdt_messages";
DROP TABLE IF EXISTS "crdt_messages_merkles";