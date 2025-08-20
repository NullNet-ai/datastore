-- Your SQL goes here

CREATE TABLE "account_phone_numbers" (
    "account_profile_id" TEXT,
    "raw_phone_number" TEXT,
    "is_primary" BOOLEAN,
    "iso_code" TEXT,
    "country_code" TEXT,
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
    "sensitivity_level" INTEGER,
    "sync_status" TEXT DEFAULT 'in_process',
    "is_batch" BOOLEAN DEFAULT false,
    PRIMARY KEY ("id")
);
--> statement-breakpoint
CREATE TABLE "account_signatures" (
    "account_profile_id" TEXT,
    "name" TEXT,
    "signature" VARCHAR(300),
    "tombstone" INTEGER DEFAULT 0,
    "status" TEXT DEFAULT 'Active',
    "previous_status" TEXT,
    "version" INTEGER,
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
    "sensitivity_level" INTEGER,
    "sync_status" TEXT DEFAULT 'in_process',
    "is_batch" BOOLEAN DEFAULT false,
    PRIMARY KEY ("id")
);
--> statement-breakpoint
CREATE INDEX "idx_account_phone_numbers_tombstone" ON "account_phone_numbers" USING btree(tombstone);
--> statement-breakpoint
CREATE INDEX "idx_account_phone_numbers_status" ON "account_phone_numbers" USING btree(status);
--> statement-breakpoint
CREATE INDEX "idx_account_phone_numbers_previous_status" ON "account_phone_numbers" USING btree(previous_status);
--> statement-breakpoint
CREATE INDEX "idx_account_phone_numbers_version" ON "account_phone_numbers" USING btree(version);
--> statement-breakpoint
CREATE INDEX "idx_account_phone_numbers_created_date" ON "account_phone_numbers" USING btree(created_date);
--> statement-breakpoint
CREATE INDEX "idx_account_phone_numbers_updated_date" ON "account_phone_numbers" USING btree(updated_date);
--> statement-breakpoint
CREATE INDEX "idx_account_phone_numbers_organization_id" ON "account_phone_numbers" USING btree(organization_id);
--> statement-breakpoint
CREATE INDEX "idx_account_phone_numbers_created_by" ON "account_phone_numbers" USING btree(created_by);
--> statement-breakpoint
CREATE INDEX "idx_account_phone_numbers_updated_by" ON "account_phone_numbers" USING btree(updated_by);
--> statement-breakpoint
CREATE INDEX "idx_account_phone_numbers_deleted_by" ON "account_phone_numbers" USING btree(deleted_by);
--> statement-breakpoint
CREATE INDEX "idx_account_phone_numbers_requested_by" ON "account_phone_numbers" USING btree(requested_by);
--> statement-breakpoint
CREATE INDEX "idx_account_phone_numbers_tags" ON "account_phone_numbers" USING btree(tags);
--> statement-breakpoint
CREATE INDEX "idx_account_phone_numbers_categories" ON "account_phone_numbers" USING btree(categories);
--> statement-breakpoint
CREATE INDEX "idx_account_phone_numbers_code" ON "account_phone_numbers" USING btree(code);
--> statement-breakpoint
CREATE INDEX "idx_account_phone_numbers_sensitivity_level" ON "account_phone_numbers" USING btree(sensitivity_level);
--> statement-breakpoint
CREATE INDEX "idx_account_signatures_tombstone" ON "account_signatures" USING btree(tombstone);
--> statement-breakpoint
CREATE INDEX "idx_account_signatures_status" ON "account_signatures" USING btree(status);
--> statement-breakpoint
CREATE INDEX "idx_account_signatures_previous_status" ON "account_signatures" USING btree(previous_status);
--> statement-breakpoint
CREATE INDEX "idx_account_signatures_version" ON "account_signatures" USING btree(version);
--> statement-breakpoint
CREATE INDEX "idx_account_signatures_created_date" ON "account_signatures" USING btree(created_date);
--> statement-breakpoint
CREATE INDEX "idx_account_signatures_updated_date" ON "account_signatures" USING btree(updated_date);
--> statement-breakpoint
CREATE INDEX "idx_account_signatures_organization_id" ON "account_signatures" USING btree(organization_id);
--> statement-breakpoint
CREATE INDEX "idx_account_signatures_created_by" ON "account_signatures" USING btree(created_by);
--> statement-breakpoint
CREATE INDEX "idx_account_signatures_updated_by" ON "account_signatures" USING btree(updated_by);
--> statement-breakpoint
CREATE INDEX "idx_account_signatures_deleted_by" ON "account_signatures" USING btree(deleted_by);
--> statement-breakpoint
CREATE INDEX "idx_account_signatures_requested_by" ON "account_signatures" USING btree(requested_by);
--> statement-breakpoint
CREATE INDEX "idx_account_signatures_tags" ON "account_signatures" USING btree(tags);
--> statement-breakpoint
CREATE INDEX "idx_account_signatures_categories" ON "account_signatures" USING btree(categories);
--> statement-breakpoint
CREATE INDEX "idx_account_signatures_code" ON "account_signatures" USING btree(code);
--> statement-breakpoint
CREATE INDEX "idx_account_signatures_sensitivity_level" ON "account_signatures" USING btree(sensitivity_level);
--> statement-breakpoint
ALTER TABLE "account_phone_numbers" ADD CONSTRAINT "account_phone_numbers_organization_id_organizations_id_fk" FOREIGN KEY ("organization_id") REFERENCES "public"."organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "account_phone_numbers" ADD CONSTRAINT "account_phone_numbers_created_by_account_organizations_id_fk" FOREIGN KEY ("created_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "account_phone_numbers" ADD CONSTRAINT "account_phone_numbers_updated_by_account_organizations_id_fk" FOREIGN KEY ("updated_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "account_phone_numbers" ADD CONSTRAINT "account_phone_numbers_deleted_by_account_organizations_id_fk" FOREIGN KEY ("deleted_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "account_phone_numbers" ADD CONSTRAINT "account_phone_numbers_requested_by_account_organizations_id_fk" FOREIGN KEY ("requested_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "account_phone_numbers" ADD CONSTRAINT "account_phone_numbers_account_profile_id_account_profiles_id_fk" FOREIGN KEY ("account_profile_id") REFERENCES "public"."account_profiles"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "files" ADD CONSTRAINT "files_organization_id_organizations_id_fk" FOREIGN KEY ("organization_id") REFERENCES "public"."organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "files" ADD CONSTRAINT "files_created_by_account_organizations_id_fk" FOREIGN KEY ("created_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "files" ADD CONSTRAINT "files_updated_by_account_organizations_id_fk" FOREIGN KEY ("updated_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "files" ADD CONSTRAINT "files_deleted_by_account_organizations_id_fk" FOREIGN KEY ("deleted_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "files" ADD CONSTRAINT "files_requested_by_account_organizations_id_fk" FOREIGN KEY ("requested_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "test_hypertable" ADD CONSTRAINT "test_hypertable_organization_id_organizations_id_fk" FOREIGN KEY ("organization_id") REFERENCES "public"."organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "test_hypertable" ADD CONSTRAINT "test_hypertable_created_by_account_organizations_id_fk" FOREIGN KEY ("created_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "test_hypertable" ADD CONSTRAINT "test_hypertable_updated_by_account_organizations_id_fk" FOREIGN KEY ("updated_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "test_hypertable" ADD CONSTRAINT "test_hypertable_deleted_by_account_organizations_id_fk" FOREIGN KEY ("deleted_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "test_hypertable" ADD CONSTRAINT "test_hypertable_requested_by_account_organizations_id_fk" FOREIGN KEY ("requested_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "account_signatures" ADD CONSTRAINT "account_signatures_organization_id_organizations_id_fk" FOREIGN KEY ("organization_id") REFERENCES "public"."organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "account_signatures" ADD CONSTRAINT "account_signatures_created_by_account_organizations_id_fk" FOREIGN KEY ("created_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "account_signatures" ADD CONSTRAINT "account_signatures_updated_by_account_organizations_id_fk" FOREIGN KEY ("updated_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "account_signatures" ADD CONSTRAINT "account_signatures_deleted_by_account_organizations_id_fk" FOREIGN KEY ("deleted_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "account_signatures" ADD CONSTRAINT "account_signatures_requested_by_account_organizations_id_fk" FOREIGN KEY ("requested_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "account_signatures" ADD CONSTRAINT "account_signatures_account_profile_id_account_profiles_id_fk" FOREIGN KEY ("account_profile_id") REFERENCES "public"."account_profiles"("id") ON DELETE no action ON UPDATE no action;
