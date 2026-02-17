-- Your SQL goes here

CREATE TABLE "checkpoint_samples" (
    "label" TEXT,
    "value" INTEGER,
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
CREATE INDEX "idx_checkpoint_samples_tombstone" ON "checkpoint_samples" USING btree("tombstone");
--> statement-breakpoint
CREATE INDEX "idx_checkpoint_samples_status" ON "checkpoint_samples" USING btree("status");
--> statement-breakpoint
CREATE INDEX "idx_checkpoint_samples_previous_status" ON "checkpoint_samples" USING btree("previous_status");
--> statement-breakpoint
CREATE INDEX "idx_checkpoint_samples_version" ON "checkpoint_samples" USING btree("version");
--> statement-breakpoint
CREATE INDEX "idx_checkpoint_samples_created_date" ON "checkpoint_samples" USING btree("created_date");
--> statement-breakpoint
CREATE INDEX "idx_checkpoint_samples_updated_date" ON "checkpoint_samples" USING btree("updated_date");
--> statement-breakpoint
CREATE INDEX "idx_checkpoint_samples_organization_id" ON "checkpoint_samples" USING btree("organization_id");
--> statement-breakpoint
CREATE INDEX "idx_checkpoint_samples_created_by" ON "checkpoint_samples" USING btree("created_by");
--> statement-breakpoint
CREATE INDEX "idx_checkpoint_samples_updated_by" ON "checkpoint_samples" USING btree("updated_by");
--> statement-breakpoint
CREATE INDEX "idx_checkpoint_samples_deleted_by" ON "checkpoint_samples" USING btree("deleted_by");
--> statement-breakpoint
CREATE INDEX "idx_checkpoint_samples_requested_by" ON "checkpoint_samples" USING btree("requested_by");
--> statement-breakpoint
CREATE INDEX "idx_checkpoint_samples_tags" ON "checkpoint_samples" USING btree("tags");
--> statement-breakpoint
CREATE INDEX "idx_checkpoint_samples_categories" ON "checkpoint_samples" USING btree("categories");
--> statement-breakpoint
CREATE INDEX "idx_checkpoint_samples_code" ON "checkpoint_samples" USING btree("code");
--> statement-breakpoint
CREATE INDEX "idx_checkpoint_samples_sensitivity_level" ON "checkpoint_samples" USING btree("sensitivity_level");
--> statement-breakpoint
CREATE INDEX "idx_checkpoint_samples_label" ON "checkpoint_samples" USING btree("label");
--> statement-breakpoint
ALTER TABLE "checkpoint_samples" ADD CONSTRAINT "fk_checkpoint_samples_organization_id" FOREIGN KEY ("organization_id") REFERENCES "public"."organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "checkpoint_samples" ADD CONSTRAINT "fk_checkpoint_samples_created_by" FOREIGN KEY ("created_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "checkpoint_samples" ADD CONSTRAINT "fk_checkpoint_samples_updated_by" FOREIGN KEY ("updated_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "checkpoint_samples" ADD CONSTRAINT "fk_checkpoint_samples_deleted_by" FOREIGN KEY ("deleted_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "checkpoint_samples" ADD CONSTRAINT "fk_checkpoint_samples_requested_by" FOREIGN KEY ("requested_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
