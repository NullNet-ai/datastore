-- This file should undo anything in `up.sql`
ALTER TABLE "connections" 
DROP COLUMN "sync_status";

ALTER TABLE "temp_connections"
DROP COLUMN "sync_status";