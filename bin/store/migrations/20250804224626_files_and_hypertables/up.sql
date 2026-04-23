-- Your SQL goes here

CREATE TABLE "files" (
    "image_url" TEXT,
    "fieldname" TEXT,
    "originalname" TEXT,
    "encoding" TEXT,
    "mimetype" TEXT,
    "destination" TEXT,
    "filename" TEXT,
    "path" TEXT,
    "size" INTEGER,
    "uploaded_by" TEXT,
    "downloaded_by" TEXT,
    "etag" TEXT,
    "version_id" TEXT,
    "download_path" TEXT,
    "presigned_url" TEXT,
    "presigned_url_expire" INTEGER,
    "tombstone" INTEGER DEFAULT 0,
    "status" TEXT DEFAULT 'Active',
    "previous_status" TEXT,
    "version" integer DEFAULT 0,
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
CREATE TABLE "test_hypertables" (
    "timestamp" TIMESTAMPTZ,
    "hypertable_timestamp" TEXT,
    "sensor_id" TEXT,
    "temperature" INTEGER,
    "humidity" INTEGER,
    "location" TEXT,
    "tombstone" INTEGER DEFAULT 0,
    "status" TEXT DEFAULT 'Active',
    "previous_status" TEXT,
    "version" integer DEFAULT 0,
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
    "id" TEXT,
    "sensitivity_level" INTEGER,
    "sync_status" TEXT DEFAULT 'in_process',
    "is_batch" BOOLEAN DEFAULT false,
    "image_url" TEXT,
    PRIMARY KEY ("timestamp", "id")
);
--> statement-breakpoint
SELECT create_hypertable('test_hypertables', 'timestamp', chunk_time_interval => INTERVAL '1 day', if_not_exists => TRUE);
--> statement-breakpoint
CREATE INDEX "idx_files_tombstone" ON "files" USING btree(tombstone);
--> statement-breakpoint
CREATE INDEX "idx_files_status" ON "files" USING btree(status);
--> statement-breakpoint
CREATE INDEX "idx_files_previous_status" ON "files" USING btree(previous_status);
--> statement-breakpoint
CREATE INDEX "idx_files_version" ON "files" USING btree(version);
--> statement-breakpoint
CREATE INDEX "idx_files_created_date" ON "files" USING btree(created_date);
--> statement-breakpoint
CREATE INDEX "idx_files_updated_date" ON "files" USING btree(updated_date);
--> statement-breakpoint
CREATE INDEX "idx_files_organization_id" ON "files" USING btree(organization_id);
--> statement-breakpoint
CREATE INDEX "idx_files_created_by" ON "files" USING btree(created_by);
--> statement-breakpoint
CREATE INDEX "idx_files_updated_by" ON "files" USING btree(updated_by);
--> statement-breakpoint
CREATE INDEX "idx_files_deleted_by" ON "files" USING btree(deleted_by);
--> statement-breakpoint
CREATE INDEX "idx_files_requested_by" ON "files" USING btree(requested_by);
--> statement-breakpoint
CREATE INDEX "idx_files_tags" ON "files" USING btree(tags);
--> statement-breakpoint
CREATE INDEX "idx_files_categories" ON "files" USING btree(categories);
--> statement-breakpoint
CREATE INDEX "idx_files_code" ON "files" USING btree(code);
--> statement-breakpoint
CREATE INDEX "idx_files_sensitivity_level" ON "files" USING btree(sensitivity_level);
--> statement-breakpoint
CREATE INDEX "idx_files_filename" ON "files" USING btree(filename);
--> statement-breakpoint
CREATE INDEX "idx_files_etag" ON "files" USING btree(etag);
--> statement-breakpoint
CREATE INDEX "idx_files_mimetype" ON "files" USING btree(mimetype);
--> statement-breakpoint
CREATE INDEX "idx_files_image_url" ON "files" USING btree(image_url);
--> statement-breakpoint
-- Your SQL goes here

CREATE INDEX "idx_test_hypertables_tombstone" ON "test_hypertables" USING btree("tombstone");
--> statement-breakpoint
CREATE INDEX "idx_test_hypertables_status" ON "test_hypertables" USING btree("status");
--> statement-breakpoint
CREATE INDEX "idx_test_hypertables_previous_status" ON "test_hypertables" USING btree("previous_status");
--> statement-breakpoint
CREATE INDEX "idx_test_hypertables_version" ON "test_hypertables" USING btree("version");
--> statement-breakpoint
CREATE INDEX "idx_test_hypertables_created_date" ON "test_hypertables" USING btree("created_date");
--> statement-breakpoint
CREATE INDEX "idx_test_hypertables_updated_date" ON "test_hypertables" USING btree("updated_date");
--> statement-breakpoint
CREATE INDEX "idx_test_hypertables_organization_id" ON "test_hypertables" USING btree("organization_id");
--> statement-breakpoint
CREATE INDEX "idx_test_hypertables_created_by" ON "test_hypertables" USING btree("created_by");
--> statement-breakpoint
CREATE INDEX "idx_test_hypertables_updated_by" ON "test_hypertables" USING btree("updated_by");
--> statement-breakpoint
CREATE INDEX "idx_test_hypertables_deleted_by" ON "test_hypertables" USING btree("deleted_by");
--> statement-breakpoint
CREATE INDEX "idx_test_hypertables_requested_by" ON "test_hypertables" USING btree("requested_by");
--> statement-breakpoint
CREATE INDEX "idx_test_hypertables_tags" ON "test_hypertables" USING btree("tags");
--> statement-breakpoint
CREATE INDEX "idx_test_hypertables_categories" ON "test_hypertables" USING btree("categories");
--> statement-breakpoint
CREATE INDEX "idx_test_hypertables_code" ON "test_hypertables" USING btree("code");
--> statement-breakpoint
CREATE INDEX "idx_test_hypertables_sensitivity_level" ON "test_hypertables" USING btree("sensitivity_level");
--> statement-breakpoint
CREATE INDEX "idx_test_hypertables_sensor" ON "test_hypertables" USING btree("sensor_id");
--> statement-breakpoint
CREATE INDEX "idx_test_hypertables_location" ON "test_hypertables" USING btree("location");

ALTER TABLE "test_hypertables" ADD CONSTRAINT "test_hypertable_organization_id_organizations_id_fk" FOREIGN KEY ("organization_id") REFERENCES "public"."organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "test_hypertables" ADD CONSTRAINT "test_hypertables_created_by_account_organizations_id_fk" FOREIGN KEY ("created_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "test_hypertables" ADD CONSTRAINT "test_hypertables_updated_by_account_organizations_id_fk" FOREIGN KEY ("updated_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "test_hypertables" ADD CONSTRAINT "test_hypertables_deleted_by_account_organizations_id_fk" FOREIGN KEY ("deleted_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint
ALTER TABLE "test_hypertables" ADD CONSTRAINT "test_hypertables_requested_by_account_organizations_id_fk" FOREIGN KEY ("requested_by") REFERENCES "public"."account_organizations"("id") ON DELETE no action ON UPDATE no action;
--> statement-breakpoint