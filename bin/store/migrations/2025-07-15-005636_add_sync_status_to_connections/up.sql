-- Your SQL goes here
ALTER TABLE "connections" 
ADD COLUMN "sync_status" text DEFAULT "in-process";