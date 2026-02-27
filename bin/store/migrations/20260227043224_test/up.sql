-- Your SQL goes here

CREATE TABLE "sample_checks" (
    "some_test_field" TIMESTAMP,
    "timestamp2" TIMESTAMPTZ,
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
    "image_url" TEXT,
    PRIMARY KEY ("id")
);
--> statement-breakpoint
CREATE INDEX "idx_sample_checks_tombstone" ON "sample_checks" USING btree("tombstone");
--> statement-breakpoint
CREATE INDEX "idx_sample_checks_status" ON "sample_checks" USING btree("status");
--> statement-breakpoint
CREATE INDEX "idx_sample_checks_previous_status" ON "sample_checks" USING btree("previous_status");
--> statement-breakpoint
CREATE INDEX "idx_sample_checks_version" ON "sample_checks" USING btree("version");
--> statement-breakpoint
CREATE INDEX "idx_sample_checks_created_date" ON "sample_checks" USING btree("created_date");
--> statement-breakpoint
CREATE INDEX "idx_sample_checks_updated_date" ON "sample_checks" USING btree("updated_date");
--> statement-breakpoint
CREATE INDEX "idx_sample_checks_organization_id" ON "sample_checks" USING btree("organization_id");
--> statement-breakpoint
CREATE INDEX "idx_sample_checks_created_by" ON "sample_checks" USING btree("created_by");
--> statement-breakpoint
CREATE INDEX "idx_sample_checks_updated_by" ON "sample_checks" USING btree("updated_by");
--> statement-breakpoint
CREATE INDEX "idx_sample_checks_deleted_by" ON "sample_checks" USING btree("deleted_by");
--> statement-breakpoint
CREATE INDEX "idx_sample_checks_requested_by" ON "sample_checks" USING btree("requested_by");
--> statement-breakpoint
CREATE INDEX "idx_sample_checks_tags" ON "sample_checks" USING btree("tags");
--> statement-breakpoint
CREATE INDEX "idx_sample_checks_categories" ON "sample_checks" USING btree("categories");
--> statement-breakpoint
CREATE INDEX "idx_sample_checks_code" ON "sample_checks" USING btree("code");
--> statement-breakpoint
CREATE INDEX "idx_sample_checks_sensitivity_level" ON "sample_checks" USING btree("sensitivity_level");
--> statement-breakpoint
ALTER TABLE "sample_checks" ADD CONSTRAINT "fk_sample_checks_organization_id" FOREIGN KEY ("organization_id") REFERENCES "public"."organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "sample_checks" ADD CONSTRAINT "fk_sample_checks_created_by" FOREIGN KEY ("created_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "sample_checks" ADD CONSTRAINT "fk_sample_checks_updated_by" FOREIGN KEY ("updated_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "sample_checks" ADD CONSTRAINT "fk_sample_checks_deleted_by" FOREIGN KEY ("deleted_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "sample_checks" ADD CONSTRAINT "fk_sample_checks_requested_by" FOREIGN KEY ("requested_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
