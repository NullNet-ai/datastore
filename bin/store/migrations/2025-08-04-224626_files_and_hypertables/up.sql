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
CREATE TABLE "test_hypertable" (
    "timestamp" TIMESTAMPTZ,
    "hypertable_timestamp" TEXT,
    "sensor_id" TEXT,
    "temperature" INTEGER,
    "humidity" INTEGER,
    "location" TEXT,
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
    "tags" TEXT[],
    "categories" TEXT[],
    "code" TEXT,
    "id" TEXT,
    "sensitivity_level" INTEGER,
    "sync_status" TEXT DEFAULT 'in_process',
    "is_batch" BOOLEAN DEFAULT false,
    PRIMARY KEY ("timestamp", "id")
);
--> statement-breakpoint
SELECT create_hypertable('test_hypertable', 'timestamp', chunk_time_interval => INTERVAL '1 day', if_not_exists => TRUE);
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
CREATE INDEX "idx_test_hypertable_tombstone" ON "test_hypertable" USING btree(tombstone);
--> statement-breakpoint
CREATE INDEX "idx_test_hypertable_status" ON "test_hypertable" USING btree(status);
--> statement-breakpoint
CREATE INDEX "idx_test_hypertable_previous_status" ON "test_hypertable" USING btree(previous_status);
--> statement-breakpoint
CREATE INDEX "idx_test_hypertable_version" ON "test_hypertable" USING btree(version);
--> statement-breakpoint
CREATE INDEX "idx_test_hypertable_created_date" ON "test_hypertable" USING btree(created_date);
--> statement-breakpoint
CREATE INDEX "idx_test_hypertable_updated_date" ON "test_hypertable" USING btree(updated_date);
--> statement-breakpoint
CREATE INDEX "idx_test_hypertable_organization_id" ON "test_hypertable" USING btree(organization_id);
--> statement-breakpoint
CREATE INDEX "idx_test_hypertable_created_by" ON "test_hypertable" USING btree(created_by);
--> statement-breakpoint
CREATE INDEX "idx_test_hypertable_updated_by" ON "test_hypertable" USING btree(updated_by);
--> statement-breakpoint
CREATE INDEX "idx_test_hypertable_deleted_by" ON "test_hypertable" USING btree(deleted_by);
--> statement-breakpoint
CREATE INDEX "idx_test_hypertable_requested_by" ON "test_hypertable" USING btree(requested_by);
--> statement-breakpoint
CREATE INDEX "idx_test_hypertable_tags" ON "test_hypertable" USING btree(tags);
--> statement-breakpoint
CREATE INDEX "idx_test_hypertable_categories" ON "test_hypertable" USING btree(categories);
--> statement-breakpoint
CREATE INDEX "idx_test_hypertable_code" ON "test_hypertable" USING btree(code);
--> statement-breakpoint
CREATE INDEX "idx_test_hypertable_sensitivity_level" ON "test_hypertable" USING btree(sensitivity_level);
--> statement-breakpoint
CREATE INDEX "idx_test_hypertable_sensor" ON "test_hypertable" USING btree(sensor_id);
--> statement-breakpoint
CREATE INDEX "idx_test_hypertable_location" ON "test_hypertable" USING btree(location);
