-- Create signed_in_activities table with same schema as sessions but without application_accessed field
CREATE TABLE "signed_in_activities" (
	"id" text NOT NULL,
	"tombstone" integer,
	"status" text,
	"previous_status" text,
	"version" integer default 0,
	"created_date" text,
	"created_time" text,
	"updated_date" text,
	"updated_time" text,
	"organization_id" text,
	"created_by" text,
	"updated_by" text,
	"deleted_by" text,
	"requested_by" text,
	"timestamp" timestamp with time zone NOT NULL,
	"tags" text[],
	"categories" text[],
	"code" text,
	"sensitivity_level" integer,
	"sync_status" text,
	"is_batch" boolean,
	"image_url" varchar(300),
	"account_organization_id" text,
	"device_name" text,
	"browser_name" text,
	"operating_system" text,
	"authentication_method" text,
	"location" text,
	"ip_address" text,
	"session_started" timestamp,
	"remark" text,
	"session_id" text,
	"hypertable_timestamp" text,
	PRIMARY KEY ("timestamp","id")
);

-- Convert to TimescaleDB hypertable

-- Add required indexes for signed_in_activities table
CREATE INDEX "idx_signed_in_activities_tombstone" ON "signed_in_activities" USING btree ("tombstone");
CREATE INDEX "idx_signed_in_activities_status" ON "signed_in_activities" USING btree ("status");
CREATE INDEX "idx_signed_in_activities_previous_status" ON "signed_in_activities" USING btree ("previous_status");
CREATE INDEX "idx_signed_in_activities_version" ON "signed_in_activities" USING btree ("version");
CREATE INDEX "idx_signed_in_activities_created_date" ON "signed_in_activities" USING btree ("created_date");
CREATE INDEX "idx_signed_in_activities_updated_date" ON "signed_in_activities" USING btree ("updated_date");
CREATE INDEX "idx_signed_in_activities_organization_id" ON "signed_in_activities" USING btree ("organization_id");
CREATE INDEX "idx_signed_in_activities_created_by" ON "signed_in_activities" USING btree ("created_by");
CREATE INDEX "idx_signed_in_activities_updated_by" ON "signed_in_activities" USING btree ("updated_by");
CREATE INDEX "idx_signed_in_activities_deleted_by" ON "signed_in_activities" USING btree ("deleted_by");
CREATE INDEX "idx_signed_in_activities_requested_by" ON "signed_in_activities" USING btree ("requested_by");
CREATE INDEX "idx_signed_in_activities_tags" ON "signed_in_activities" USING btree ("tags");
CREATE INDEX "idx_signed_in_activities_categories" ON "signed_in_activities" USING btree ("categories");
CREATE INDEX "idx_signed_in_activities_code" ON "signed_in_activities" USING btree ("code");
CREATE INDEX "idx_signed_in_activities_sensitivity_level" ON "signed_in_activities" USING btree ("sensitivity_level");
CREATE INDEX "idx_signed_in_activities_session_id" ON "signed_in_activities" USING btree ("session_id");

-- Convert to TimescaleDB hypertable FIRST
SELECT create_hypertable('signed_in_activities', 'timestamp', chunk_time_interval => INTERVAL '1 day', if_not_exists => TRUE);

