-- Create signed_in_activity table with same schema as sessions but without application_accessed field
CREATE TABLE "signed_in_activity" (
	"id" text PRIMARY KEY NOT NULL,
	"tombstone" integer,
	"status" text,
	"previous_status" text,
	"version" integer,
	"created_date" text,
	"created_time" text,
	"updated_date" text,
	"updated_time" text,
	"organization_id" text,
	"created_by" text,
	"updated_by" text,
	"deleted_by" text,
	"requested_by" text,
	"timestamp" timestamp with time zone,
	"tags" text[],
	"categories" text[],
	"code" text,
	"sensitivity_level" integer,
	"sync_status" text,
	"is_batch" boolean,
	"account_profile_id" text,
	"device_name" text,
	"browser_name" text,
	"operating_system" text,
	"authentication_method" text,
	"location" text,
	"ip_address" text,
	"session_started" timestamp,
	"remark" text,
	"session_id" text
);

-- Add required indexes for signed_in_activity table
CREATE INDEX "idx_signed_in_activity_tombstone" ON "signed_in_activity" USING btree ("tombstone");
CREATE INDEX "idx_signed_in_activity_status" ON "signed_in_activity" USING btree ("status");
CREATE INDEX "idx_signed_in_activity_previous_status" ON "signed_in_activity" USING btree ("previous_status");
CREATE INDEX "idx_signed_in_activity_version" ON "signed_in_activity" USING btree ("version");
CREATE INDEX "idx_signed_in_activity_created_date" ON "signed_in_activity" USING btree ("created_date");
CREATE INDEX "idx_signed_in_activity_updated_date" ON "signed_in_activity" USING btree ("updated_date");
CREATE INDEX "idx_signed_in_activity_organization_id" ON "signed_in_activity" USING btree ("organization_id");
CREATE INDEX "idx_signed_in_activity_created_by" ON "signed_in_activity" USING btree ("created_by");
CREATE INDEX "idx_signed_in_activity_updated_by" ON "signed_in_activity" USING btree ("updated_by");
CREATE INDEX "idx_signed_in_activity_deleted_by" ON "signed_in_activity" USING btree ("deleted_by");
CREATE INDEX "idx_signed_in_activity_requested_by" ON "signed_in_activity" USING btree ("requested_by");
CREATE INDEX "idx_signed_in_activity_tags" ON "signed_in_activity" USING btree ("tags");
CREATE INDEX "idx_signed_in_activity_categories" ON "signed_in_activity" USING btree ("categories");
CREATE INDEX "idx_signed_in_activity_code" ON "signed_in_activity" USING btree ("code");
CREATE INDEX "idx_signed_in_activity_sensitivity_level" ON "signed_in_activity" USING btree ("sensitivity_level");
CREATE INDEX "idx_signed_in_activity_session_id" ON "signed_in_activity" USING btree ("session_id");

-- Add foreign key constraints
ALTER TABLE "signed_in_activity" ADD CONSTRAINT "signed_in_activity_organization_id_organizations_id_fk" FOREIGN KEY ("organization_id") REFERENCES "public"."organizations"("id") ON DELETE no action ON UPDATE no action;
ALTER TABLE "signed_in_activity" ADD CONSTRAINT "signed_in_activity_created_by_account_organizations_id_fk" FOREIGN KEY ("created_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
ALTER TABLE "signed_in_activity" ADD CONSTRAINT "signed_in_activity_updated_by_account_organizations_id_fk" FOREIGN KEY ("updated_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
ALTER TABLE "signed_in_activity" ADD CONSTRAINT "signed_in_activity_deleted_by_account_organizations_id_fk" FOREIGN KEY ("deleted_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
ALTER TABLE "signed_in_activity" ADD CONSTRAINT "signed_in_activity_requested_by_account_organizations_id_fk" FOREIGN KEY ("requested_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;