-- Your SQL goes here
ALTER TABLE "connections" 
ADD COLUMN "is_batch" boolean DEFAULT false;

ALTER TABLE "temp_connections" 
ADD COLUMN "is_batch" boolean DEFAULT false;