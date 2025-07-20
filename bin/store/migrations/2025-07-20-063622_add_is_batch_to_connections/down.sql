-- This file should undo anything in `up.sql`
ALTER TABLE "connections" 
DROP COLUMN "is_batch";

ALTER TABLE "temp_connections" 
DROP COLUMN "is_batch";