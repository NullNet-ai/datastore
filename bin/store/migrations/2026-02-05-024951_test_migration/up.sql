-- Your SQL goes here

CREATE TABLE "jean" (
    "first_name" TEXT,
    "is_hungry" BOOLEAN,
    "is_sleepy" BOOLEAN DEFAULT true,
    "eats_candies" BOOLEAN DEFAULT true,
    "age" INTEGER,
    "tombstone" INTEGER DEFAULT 0,
    "status" TEXT DEFAULT 'Active',
    "previous_status" TEXT,
    "version" INTEGER DEFAULT 0,
    "created_date" TEXT,
    "created_time" TEXT,
    "updated_date" TEXT,
    "updated_time" TEXT,
    "organization_id" TEXT,
    "created_by" TEXT,
    "updated_by" TEXT,
    "deleted_by" TEXT,
    "requested_by" TEXT,
    "timestamp" TIMESTAMP,
    "tags" TEXT[],
    "categories" TEXT[],
    "code" TEXT,
    "id" TEXT,
    "sensitivity_level" INTEGER DEFAULT 1000,
    "sync_status" TEXT DEFAULT 'in_process',
    "is_batch" BOOLEAN DEFAULT false,
    PRIMARY KEY ("id")
);
--> statement-breakpoint
CREATE TABLE "jean_hypertable" (
    "timestamp" TIMESTAMPTZ NOT NULL,
    "id" TEXT NOT NULL,
    "hypertable_timestamp" TEXT,
    "first_name" TEXT,
    "is_hungry" BOOLEAN,
    "is_sleepy" BOOLEAN DEFAULT true,
    "eats_candies" BOOLEAN DEFAULT true,
    "age" INTEGER,
    "tombstone" INTEGER DEFAULT 0,
    "status" TEXT DEFAULT 'Active',
    "previous_status" TEXT,
    "version" INTEGER DEFAULT 0,
    "created_date" TEXT,
    "created_time" TEXT,
    "updated_date" TEXT,
    "updated_time" TEXT,
    "organization_id" TEXT,
    "created_by" TEXT,
    "updated_by" TEXT,
    "deleted_by" TEXT,
    "requested_by" TEXT,
    "tags" TEXT[],
    "categories" TEXT[],
    "code" TEXT,
    "sensitivity_level" INTEGER DEFAULT 1000,
    "sync_status" TEXT DEFAULT 'in_process',
    "is_batch" BOOLEAN DEFAULT false,
    PRIMARY KEY ("timestamp", "id")
);
--> statement-breakpoint
SELECT create_hypertable('jean_hypertable', 'timestamp', chunk_time_interval => INTERVAL '1 day', if_not_exists => TRUE);
--> statement-breakpoint
CREATE INDEX "idx_jean_tombstone" ON "jean" USING btree(tombstone);
--> statement-breakpoint
CREATE INDEX "idx_jean_status" ON "jean" USING btree(status);
--> statement-breakpoint
CREATE INDEX "idx_jean_previous_status" ON "jean" USING btree(previous_status);
--> statement-breakpoint
CREATE INDEX "idx_jean_version" ON "jean" USING btree(version);
--> statement-breakpoint
CREATE INDEX "idx_jean_created_date" ON "jean" USING btree(created_date);
--> statement-breakpoint
CREATE INDEX "idx_jean_updated_date" ON "jean" USING btree(updated_date);
--> statement-breakpoint
CREATE INDEX "idx_jean_organization_id" ON "jean" USING btree(organization_id);
--> statement-breakpoint
CREATE INDEX "idx_jean_created_by" ON "jean" USING btree(created_by);
--> statement-breakpoint
CREATE INDEX "idx_jean_updated_by" ON "jean" USING btree(updated_by);
--> statement-breakpoint
CREATE INDEX "idx_jean_deleted_by" ON "jean" USING btree(deleted_by);
--> statement-breakpoint
CREATE INDEX "idx_jean_requested_by" ON "jean" USING btree(requested_by);
--> statement-breakpoint
CREATE INDEX "idx_jean_tags" ON "jean" USING btree(tags);
--> statement-breakpoint
CREATE INDEX "idx_jean_categories" ON "jean" USING btree(categories);
--> statement-breakpoint
CREATE INDEX "idx_jean_code" ON "jean" USING btree(code);
--> statement-breakpoint
CREATE INDEX "idx_jean_sensitivity_level" ON "jean" USING btree(sensitivity_level);
--> statement-breakpoint
CREATE INDEX "idx_jean_hypertable_tombstone" ON "jean_hypertable" USING btree(tombstone);
--> statement-breakpoint
CREATE INDEX "idx_jean_hypertable_status" ON "jean_hypertable" USING btree(status);
--> statement-breakpoint
CREATE INDEX "idx_jean_hypertable_previous_status" ON "jean_hypertable" USING btree(previous_status);
--> statement-breakpoint
CREATE INDEX "idx_jean_hypertable_version" ON "jean_hypertable" USING btree(version);
--> statement-breakpoint
CREATE INDEX "idx_jean_hypertable_created_date" ON "jean_hypertable" USING btree(created_date);
--> statement-breakpoint
CREATE INDEX "idx_jean_hypertable_updated_date" ON "jean_hypertable" USING btree(updated_date);
--> statement-breakpoint
CREATE INDEX "idx_jean_hypertable_organization_id" ON "jean_hypertable" USING btree(organization_id);
--> statement-breakpoint
CREATE INDEX "idx_jean_hypertable_created_by" ON "jean_hypertable" USING btree(created_by);
--> statement-breakpoint
CREATE INDEX "idx_jean_hypertable_updated_by" ON "jean_hypertable" USING btree(updated_by);
--> statement-breakpoint
CREATE INDEX "idx_jean_hypertable_deleted_by" ON "jean_hypertable" USING btree(deleted_by);
--> statement-breakpoint
CREATE INDEX "idx_jean_hypertable_requested_by" ON "jean_hypertable" USING btree(requested_by);
--> statement-breakpoint
CREATE INDEX "idx_jean_hypertable_tags" ON "jean_hypertable" USING btree(tags);
--> statement-breakpoint
CREATE INDEX "idx_jean_hypertable_categories" ON "jean_hypertable" USING btree(categories);
--> statement-breakpoint
CREATE INDEX "idx_jean_hypertable_code" ON "jean_hypertable" USING btree(code);
--> statement-breakpoint
CREATE INDEX "idx_jean_hypertable_sensitivity_level" ON "jean_hypertable" USING btree(sensitivity_level);
--> statement-breakpoint
CREATE INDEX "idx_jean_hypertable_idx_jean_hypertable_first_name" ON "jean_hypertable" USING btree(first_name);
--> statement-breakpoint
ALTER TABLE "jean" ADD CONSTRAINT "jean_organization_id_organizations_id_fk" FOREIGN KEY ("organization_id") REFERENCES "public"."organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "jean" ADD CONSTRAINT "jean_created_by_account_organizations_id_fk" FOREIGN KEY ("created_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "jean" ADD CONSTRAINT "jean_updated_by_account_organizations_id_fk" FOREIGN KEY ("updated_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "jean" ADD CONSTRAINT "jean_deleted_by_account_organizations_id_fk" FOREIGN KEY ("deleted_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "jean" ADD CONSTRAINT "jean_requested_by_account_organizations_id_fk" FOREIGN KEY ("requested_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
