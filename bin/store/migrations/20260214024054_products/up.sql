-- Your SQL goes here

CREATE TABLE "test_products" (
    "sku" TEXT,
    "price" INTEGER,
    "in_stock" TEXT,
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
CREATE INDEX "idx_test_products_tombstone" ON "test_products" USING btree("tombstone");
--> statement-breakpoint
CREATE INDEX "idx_test_products_status" ON "test_products" USING btree("status");
--> statement-breakpoint
CREATE INDEX "idx_test_products_previous_status" ON "test_products" USING btree("previous_status");
--> statement-breakpoint
CREATE INDEX "idx_test_products_version" ON "test_products" USING btree("version");
--> statement-breakpoint
CREATE INDEX "idx_test_products_created_date" ON "test_products" USING btree("created_date");
--> statement-breakpoint
CREATE INDEX "idx_test_products_updated_date" ON "test_products" USING btree("updated_date");
--> statement-breakpoint
CREATE INDEX "idx_test_products_organization_id" ON "test_products" USING btree("organization_id");
--> statement-breakpoint
CREATE INDEX "idx_test_products_created_by" ON "test_products" USING btree("created_by");
--> statement-breakpoint
CREATE INDEX "idx_test_products_updated_by" ON "test_products" USING btree("updated_by");
--> statement-breakpoint
CREATE INDEX "idx_test_products_deleted_by" ON "test_products" USING btree("deleted_by");
--> statement-breakpoint
CREATE INDEX "idx_test_products_requested_by" ON "test_products" USING btree("requested_by");
--> statement-breakpoint
CREATE INDEX "idx_test_products_tags" ON "test_products" USING btree("tags");
--> statement-breakpoint
CREATE INDEX "idx_test_products_categories" ON "test_products" USING btree("categories");
--> statement-breakpoint
CREATE INDEX "idx_test_products_code" ON "test_products" USING btree("code");
--> statement-breakpoint
CREATE INDEX "idx_test_products_sensitivity_level" ON "test_products" USING btree("sensitivity_level");
--> statement-breakpoint
CREATE INDEX "idx_test_products_sku" ON "test_products" USING btree("sku");
--> statement-breakpoint
CREATE INDEX "idx_test_products_price" ON "test_products" USING btree("price");
--> statement-breakpoint
ALTER TABLE "test_products" ADD CONSTRAINT "test_products_organization_id_organizations_id_fk" FOREIGN KEY ("organization_id") REFERENCES "public"."organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "test_products" ADD CONSTRAINT "test_products_created_by_account_organizations_id_fk" FOREIGN KEY ("created_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "test_products" ADD CONSTRAINT "test_products_updated_by_account_organizations_id_fk" FOREIGN KEY ("updated_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "test_products" ADD CONSTRAINT "test_products_deleted_by_account_organizations_id_fk" FOREIGN KEY ("deleted_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "test_products" ADD CONSTRAINT "test_products_requested_by_account_organizations_id_fk" FOREIGN KEY ("requested_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
