-- Your SQL goes here

CREATE TABLE "smtp_headers" (
    "header_key" TEXT,
    "header_value" TEXT,
    "smtp_payload_id" TEXT,
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
CREATE TABLE "school_admins" (
    "school_id" TEXT,
    "school_admin_id" TEXT,
    "department_id" TEXT,
    "district_id" TEXT,
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
CREATE TABLE "district_admins" (
    "district_id" TEXT,
    "district_admin_id" TEXT,
    "department_id" TEXT,
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
CREATE INDEX "idx_smtp_headers_tombstone" ON "smtp_headers" USING btree("tombstone");
--> statement-breakpoint
CREATE INDEX "idx_smtp_headers_status" ON "smtp_headers" USING btree("status");
--> statement-breakpoint
CREATE INDEX "idx_smtp_headers_previous_status" ON "smtp_headers" USING btree("previous_status");
--> statement-breakpoint
CREATE INDEX "idx_smtp_headers_version" ON "smtp_headers" USING btree("version");
--> statement-breakpoint
CREATE INDEX "idx_smtp_headers_created_date" ON "smtp_headers" USING btree("created_date");
--> statement-breakpoint
CREATE INDEX "idx_smtp_headers_updated_date" ON "smtp_headers" USING btree("updated_date");
--> statement-breakpoint
CREATE INDEX "idx_smtp_headers_organization_id" ON "smtp_headers" USING btree("organization_id");
--> statement-breakpoint
CREATE INDEX "idx_smtp_headers_created_by" ON "smtp_headers" USING btree("created_by");
--> statement-breakpoint
CREATE INDEX "idx_smtp_headers_updated_by" ON "smtp_headers" USING btree("updated_by");
--> statement-breakpoint
CREATE INDEX "idx_smtp_headers_deleted_by" ON "smtp_headers" USING btree("deleted_by");
--> statement-breakpoint
CREATE INDEX "idx_smtp_headers_requested_by" ON "smtp_headers" USING btree("requested_by");
--> statement-breakpoint
CREATE INDEX "idx_smtp_headers_tags" ON "smtp_headers" USING btree("tags");
--> statement-breakpoint
CREATE INDEX "idx_smtp_headers_categories" ON "smtp_headers" USING btree("categories");
--> statement-breakpoint
CREATE INDEX "idx_smtp_headers_code" ON "smtp_headers" USING btree("code");
--> statement-breakpoint
CREATE INDEX "idx_smtp_headers_sensitivity_level" ON "smtp_headers" USING btree("sensitivity_level");
--> statement-breakpoint
CREATE INDEX "idx_smtp_headers_header_key" ON "smtp_headers" USING btree("header_key");
--> statement-breakpoint
CREATE INDEX "idx_smtp_headers_header_value" ON "smtp_headers" USING btree("header_value");
--> statement-breakpoint
CREATE INDEX "idx_smtp_headers_smtp_payload_id" ON "smtp_headers" USING btree("smtp_payload_id");
--> statement-breakpoint
CREATE INDEX "idx_school_admins_tombstone" ON "school_admins" USING btree("tombstone");
--> statement-breakpoint
CREATE INDEX "idx_school_admins_status" ON "school_admins" USING btree("status");
--> statement-breakpoint
CREATE INDEX "idx_school_admins_previous_status" ON "school_admins" USING btree("previous_status");
--> statement-breakpoint
CREATE INDEX "idx_school_admins_version" ON "school_admins" USING btree("version");
--> statement-breakpoint
CREATE INDEX "idx_school_admins_created_date" ON "school_admins" USING btree("created_date");
--> statement-breakpoint
CREATE INDEX "idx_school_admins_updated_date" ON "school_admins" USING btree("updated_date");
--> statement-breakpoint
CREATE INDEX "idx_school_admins_organization_id" ON "school_admins" USING btree("organization_id");
--> statement-breakpoint
CREATE INDEX "idx_school_admins_created_by" ON "school_admins" USING btree("created_by");
--> statement-breakpoint
CREATE INDEX "idx_school_admins_updated_by" ON "school_admins" USING btree("updated_by");
--> statement-breakpoint
CREATE INDEX "idx_school_admins_deleted_by" ON "school_admins" USING btree("deleted_by");
--> statement-breakpoint
CREATE INDEX "idx_school_admins_requested_by" ON "school_admins" USING btree("requested_by");
--> statement-breakpoint
CREATE INDEX "idx_school_admins_tags" ON "school_admins" USING btree("tags");
--> statement-breakpoint
CREATE INDEX "idx_school_admins_categories" ON "school_admins" USING btree("categories");
--> statement-breakpoint
CREATE INDEX "idx_school_admins_code" ON "school_admins" USING btree("code");
--> statement-breakpoint
CREATE INDEX "idx_school_admins_sensitivity_level" ON "school_admins" USING btree("sensitivity_level");
--> statement-breakpoint
CREATE INDEX "idx_school_admins_school_id" ON "school_admins" USING btree("school_id");
--> statement-breakpoint
CREATE INDEX "idx_school_admins_school_admin_id" ON "school_admins" USING btree("school_admin_id");
--> statement-breakpoint
CREATE INDEX "idx_school_admins_department_id" ON "school_admins" USING btree("department_id");
--> statement-breakpoint
CREATE INDEX "idx_school_admins_district_id" ON "school_admins" USING btree("district_id");
--> statement-breakpoint
CREATE INDEX "idx_school_admins_school_id_school_admin_id" ON "school_admins" USING btree("school_id", "school_admin_id");
--> statement-breakpoint
CREATE INDEX "idx_district_admins_tombstone" ON "district_admins" USING btree("tombstone");
--> statement-breakpoint
CREATE INDEX "idx_district_admins_status" ON "district_admins" USING btree("status");
--> statement-breakpoint
CREATE INDEX "idx_district_admins_previous_status" ON "district_admins" USING btree("previous_status");
--> statement-breakpoint
CREATE INDEX "idx_district_admins_version" ON "district_admins" USING btree("version");
--> statement-breakpoint
CREATE INDEX "idx_district_admins_created_date" ON "district_admins" USING btree("created_date");
--> statement-breakpoint
CREATE INDEX "idx_district_admins_updated_date" ON "district_admins" USING btree("updated_date");
--> statement-breakpoint
CREATE INDEX "idx_district_admins_organization_id" ON "district_admins" USING btree("organization_id");
--> statement-breakpoint
CREATE INDEX "idx_district_admins_created_by" ON "district_admins" USING btree("created_by");
--> statement-breakpoint
CREATE INDEX "idx_district_admins_updated_by" ON "district_admins" USING btree("updated_by");
--> statement-breakpoint
CREATE INDEX "idx_district_admins_deleted_by" ON "district_admins" USING btree("deleted_by");
--> statement-breakpoint
CREATE INDEX "idx_district_admins_requested_by" ON "district_admins" USING btree("requested_by");
--> statement-breakpoint
CREATE INDEX "idx_district_admins_tags" ON "district_admins" USING btree("tags");
--> statement-breakpoint
CREATE INDEX "idx_district_admins_categories" ON "district_admins" USING btree("categories");
--> statement-breakpoint
CREATE INDEX "idx_district_admins_code" ON "district_admins" USING btree("code");
--> statement-breakpoint
CREATE INDEX "idx_district_admins_sensitivity_level" ON "district_admins" USING btree("sensitivity_level");
--> statement-breakpoint
CREATE INDEX "idx_district_admins_district_id" ON "district_admins" USING btree("district_id");
--> statement-breakpoint
CREATE INDEX "idx_district_admins_district_admin_id" ON "district_admins" USING btree("district_admin_id");
--> statement-breakpoint
CREATE INDEX "idx_district_admins_department_id" ON "district_admins" USING btree("department_id");
--> statement-breakpoint
CREATE INDEX "idx_district_admins_district_id_district_admin_id" ON "district_admins" USING btree("district_id", "district_admin_id");
--> statement-breakpoint
ALTER TABLE "smtp_headers" ADD CONSTRAINT "fk_smtp_headers_organization_id" FOREIGN KEY ("organization_id") REFERENCES "public"."organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "smtp_headers" ADD CONSTRAINT "fk_smtp_headers_created_by" FOREIGN KEY ("created_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "smtp_headers" ADD CONSTRAINT "fk_smtp_headers_updated_by" FOREIGN KEY ("updated_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "smtp_headers" ADD CONSTRAINT "fk_smtp_headers_deleted_by" FOREIGN KEY ("deleted_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "smtp_headers" ADD CONSTRAINT "fk_smtp_headers_requested_by" FOREIGN KEY ("requested_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "smtp_headers" ADD CONSTRAINT "fk_smtp_headers_smtp_payload_id" FOREIGN KEY ("smtp_payload_id") REFERENCES "public"."smtp_payloads"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "school_admins" ADD CONSTRAINT "fk_school_admins_organization_id" FOREIGN KEY ("organization_id") REFERENCES "public"."organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "school_admins" ADD CONSTRAINT "fk_school_admins_created_by" FOREIGN KEY ("created_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "school_admins" ADD CONSTRAINT "fk_school_admins_updated_by" FOREIGN KEY ("updated_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "school_admins" ADD CONSTRAINT "fk_school_admins_deleted_by" FOREIGN KEY ("deleted_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "school_admins" ADD CONSTRAINT "fk_school_admins_requested_by" FOREIGN KEY ("requested_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "school_admins" ADD CONSTRAINT "fk_school_admins_school_id" FOREIGN KEY ("school_id") REFERENCES "public"."organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "school_admins" ADD CONSTRAINT "fk_school_admins_school_admin_id" FOREIGN KEY ("school_admin_id") REFERENCES "public"."contacts"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "school_admins" ADD CONSTRAINT "fk_school_admins_department_id" FOREIGN KEY ("department_id") REFERENCES "public"."organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "school_admins" ADD CONSTRAINT "fk_school_admins_district_id" FOREIGN KEY ("district_id") REFERENCES "public"."organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "district_admins" ADD CONSTRAINT "fk_district_admins_organization_id" FOREIGN KEY ("organization_id") REFERENCES "public"."organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "district_admins" ADD CONSTRAINT "fk_district_admins_created_by" FOREIGN KEY ("created_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "district_admins" ADD CONSTRAINT "fk_district_admins_updated_by" FOREIGN KEY ("updated_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "district_admins" ADD CONSTRAINT "fk_district_admins_deleted_by" FOREIGN KEY ("deleted_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "district_admins" ADD CONSTRAINT "fk_district_admins_requested_by" FOREIGN KEY ("requested_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "district_admins" ADD CONSTRAINT "fk_district_admins_district_id" FOREIGN KEY ("district_id") REFERENCES "public"."organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "district_admins" ADD CONSTRAINT "fk_district_admins_district_admin_id" FOREIGN KEY ("district_admin_id") REFERENCES "public"."contacts"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "district_admins" ADD CONSTRAINT "fk_district_admins_department_id" FOREIGN KEY ("department_id") REFERENCES "public"."organizations"("id") ON DELETE no action ON UPDATE no action;
