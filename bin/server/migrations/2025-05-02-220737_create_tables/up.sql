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
CREATE TABLE IF NOT EXISTS "crdt_client_messages" (
	"record_id" text NOT NULL,
	"client_id" text NOT NULL,
	"message" text NOT NULL,
	CONSTRAINT "crdt_client_messages_record_id_pk" PRIMARY KEY("record_id")
);
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
