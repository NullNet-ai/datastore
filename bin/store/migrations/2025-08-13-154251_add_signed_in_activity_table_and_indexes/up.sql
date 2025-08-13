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
	"timestamp" timestamp,
	"tags" text[],
	"categories" text[],
	"code" text,
	"sensitivity_level" integer,
	"sync_status" text,
	"is_batch" boolean,
	"account_profile_id" integer,
	"device_name" text,
	"browser_name" text,
	"operating_system" text,
	"authentication_method" text,
	"location" text,
	"ip_address" text,
	"session_started" timestamp,
	"remarks" text,
	"user_role_id" text,
	"user_account_id" text,
	"user_is_root_user" boolean,
	"token" text,
	"cookie_path" text,
	"cookie_expire" text,
	"cookie_http_only" boolean,
	"cookie_original_max_age" bigint,
	"origin_url" text,
	"origin_host" text,
	"origin_user_agent" text,
	"valid_pass_key" text,
	"role_permission" text,
	"field_permission" text,
	"record_permission" text,
	"expire" timestamp,
	"last_accessed" timestamp with time zone
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