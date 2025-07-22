-- Your SQL goes here
ALTER TABLE "connections" 
ADD COLUMN "sync_status" text DEFAULT 'in-process';


-- Your SQL goes here
ALTER TABLE "temp_connections" 
ADD COLUMN "sync_status" text DEFAULT 'in-process';