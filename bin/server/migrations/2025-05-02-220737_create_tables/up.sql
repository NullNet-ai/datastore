-- Your SQL goes here
CREATE TABLE IF NOT EXISTS "crdt_messages_merkles" (
	"group_id" text NOT NULL,
	"merkle" text NOT NULL,
	CONSTRAINT "crdt_messages_merkles_group_id_pk" PRIMARY KEY("group_id")
);
--> statement-breakpoint
CREATE TABLE IF NOT EXISTS "crdt_messages" (
	"database" text,
	"dataset" text NOT NULL,
	"group_id" text NOT NULL,
	"timestamp" text NOT NULL,
	"row" text NOT NULL,
	"column" text NOT NULL,
	"client_id" text NOT NULL,
	"value" text NOT NULL,
	"hypertable_timestamp" text,
	CONSTRAINT "crdt_messages_timestamp_group_id_row_column_pk" PRIMARY KEY("timestamp","group_id","row","column")
);
--> statement-breakpoint
-- Index for the hot query path: get_all_messages_from_timestamp
--   WHERE group_id = ? AND timestamp > ? AND client_id != ?  ORDER BY timestamp ASC
-- Column order: group_id first (equality), then timestamp (range + sort), then client_id (inequality filter).
-- This lets Postgres satisfy the WHERE and ORDER BY entirely from the index with no heap sort.
CREATE INDEX IF NOT EXISTS "idx_crdt_messages_group_id_timestamp_client_id"
    ON "crdt_messages" ("group_id", "timestamp" ASC, "client_id");
--> statement-breakpoint
-- Covering index for bootstrap path (client merkle empty):
--   get_all_messages_from_timestamp called with timestamp = "" (i.e. timestamp > '')
-- Same shape as above; this alias makes EXPLAIN output easier to read when debugging bootstrap vs diff paths.
-- (Postgres will pick one; keep both only if you want explicit partition of the two explain plans.)
-- Alternatively, drop this and rely solely on idx_crdt_messages_group_id_timestamp_client_id.
CREATE INDEX IF NOT EXISTS "idx_crdt_messages_group_id_client_id"
    ON "crdt_messages" ("group_id", "client_id");
--> statement-breakpoint
CREATE TABLE IF NOT EXISTS "crdt_client_messages" (
	"record_id" text NOT NULL,
	"client_id" text NOT NULL,
	"message" text NOT NULL,
	CONSTRAINT "crdt_client_messages_record_id_pk" PRIMARY KEY("record_id")
);
--> statement-breakpoint
-- Index for crdt_client_messages: all three query patterns filter by client_id.
-- Including record_id satisfies ORDER BY record_id in get_chunk without a sort step,
-- and allows COUNT(record_id) to be answered from the index alone (index-only scan).
CREATE INDEX IF NOT EXISTS "idx_crdt_client_messages_client_id_record_id"
    ON "crdt_client_messages" ("client_id", "record_id");
--> statement-breakpoint
CREATE TABLE IF NOT EXISTS "sync_queue_items" (
	"id" text PRIMARY KEY NOT NULL,
	"order" integer NOT NULL,
	"group_id" text NOT NULL,
	"value" text NOT NULL
);
--> statement-breakpoint
CREATE TABLE IF NOT EXISTS "sync_queues" (
	"group_id" text PRIMARY KEY NOT NULL,
	"count" integer NOT NULL,
	"size" integer NOT NULL
);
--> statement-breakpoint
CREATE TABLE IF NOT EXISTS "sync_endpoints" (
	"id" serial PRIMARY KEY NOT NULL,
	"url" text NOT NULL,
	"auth_username" text NOT NULL,
	"auth_password" text NOT NULL,
	"sync_interval" integer NOT NULL
);
--> statement-breakpoint
CREATE TABLE IF NOT EXISTS "sync_endpoint_groups" (
	"sync_endpoint_id" integer NOT NULL,
	"group_id" text NOT NULL,
	"status" text DEFAULT 'active' NOT NULL,
	CONSTRAINT "sync_endpoint_groups_group_id_sync_endpoint_id_pk" PRIMARY KEY("group_id","sync_endpoint_id")
);
--> statement-breakpoint
CREATE TABLE IF NOT EXISTS "sync_transactions" (
	"id" text PRIMARY KEY NOT NULL,
	"timestamp" text NOT NULL,
	"group_id" text NOT NULL,
	"sync_endpoint_id" integer NOT NULL,
	"status" text DEFAULT 'Active' NOT NULL,
	"expiry" bigint
);
--> statement-breakpoint
DO $$ BEGIN
 ALTER TABLE "sync_queue_items" ADD CONSTRAINT "sync_queue_items_group_id_sync_queue_items_id_fk" FOREIGN KEY ("group_id") REFERENCES "public"."sync_queue_items"("id") ON DELETE no action ON UPDATE no action;
EXCEPTION
 WHEN duplicate_object THEN null;
END $$;
--> statement-breakpoint
DO $$ BEGIN
 ALTER TABLE "sync_endpoint_groups" ADD CONSTRAINT "sync_endpoint_groups_sync_endpoint_id_sync_endpoints_id_fk" FOREIGN KEY ("sync_endpoint_id") REFERENCES "public"."sync_endpoints"("id") ON DELETE no action ON UPDATE no action;
EXCEPTION
 WHEN duplicate_object THEN null;
END $$;
