-- Your SQL goes here

CREATE TABLE "demo_items" (
    "title" TEXT,
    "description" TEXT,
    "quantity" INTEGER,
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
CREATE INDEX "idx_demo_items_tombstone" ON "demo_items" USING btree("tombstone");
--> statement-breakpoint
CREATE INDEX "idx_demo_items_status" ON "demo_items" USING btree("status");
--> statement-breakpoint
CREATE INDEX "idx_demo_items_previous_status" ON "demo_items" USING btree("previous_status");
--> statement-breakpoint
CREATE INDEX "idx_demo_items_version" ON "demo_items" USING btree("version");
--> statement-breakpoint
CREATE INDEX "idx_demo_items_created_date" ON "demo_items" USING btree("created_date");
--> statement-breakpoint
CREATE INDEX "idx_demo_items_updated_date" ON "demo_items" USING btree("updated_date");
--> statement-breakpoint
CREATE INDEX "idx_demo_items_organization_id" ON "demo_items" USING btree("organization_id");
--> statement-breakpoint
CREATE INDEX "idx_demo_items_created_by" ON "demo_items" USING btree("created_by");
--> statement-breakpoint
CREATE INDEX "idx_demo_items_updated_by" ON "demo_items" USING btree("updated_by");
--> statement-breakpoint
CREATE INDEX "idx_demo_items_deleted_by" ON "demo_items" USING btree("deleted_by");
--> statement-breakpoint
CREATE INDEX "idx_demo_items_requested_by" ON "demo_items" USING btree("requested_by");
--> statement-breakpoint
CREATE INDEX "idx_demo_items_tags" ON "demo_items" USING btree("tags");
--> statement-breakpoint
CREATE INDEX "idx_demo_items_categories" ON "demo_items" USING btree("categories");
--> statement-breakpoint
CREATE INDEX "idx_demo_items_code" ON "demo_items" USING btree("code");
--> statement-breakpoint
CREATE INDEX "idx_demo_items_sensitivity_level" ON "demo_items" USING btree("sensitivity_level");
--> statement-breakpoint
CREATE INDEX "idx_demo_items_title" ON "demo_items" USING btree("title");
--> statement-breakpoint
ALTER TABLE "demo_items" ADD CONSTRAINT "demo_items_organization_id_organizations_id_fk" FOREIGN KEY ("organization_id") REFERENCES "public"."organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "demo_items" ADD CONSTRAINT "demo_items_created_by_account_organizations_id_fk" FOREIGN KEY ("created_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "demo_items" ADD CONSTRAINT "demo_items_updated_by_account_organizations_id_fk" FOREIGN KEY ("updated_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "demo_items" ADD CONSTRAINT "demo_items_deleted_by_account_organizations_id_fk" FOREIGN KEY ("deleted_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "demo_items" ADD CONSTRAINT "demo_items_requested_by_account_organizations_id_fk" FOREIGN KEY ("requested_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
