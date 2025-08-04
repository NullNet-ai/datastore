-- Your SQL goes here

CREATE TABLE "files" (
    "image_url" TEXT,
    "fieldname" TEXT,
    "originalname" TEXT,
    "encoding" TEXT,
    "mimetype" TEXT,
    "destination" TEXT,
    "filename" TEXT NOT NULL,
    "path" TEXT NOT NULL,
    "size" INTEGER NOT NULL,
    "uploaded_by" TEXT NOT NULL,
    "downloaded_by" TEXT NOT NULL,
    "etag" TEXT NOT NULL,
    "version_id" TEXT,
    "download_path" TEXT,
    "presigned_url" TEXT,
    "presigned_url_expires" INTEGER,
    "tombstone" INTEGER NOT NULL DEFAULT 0,
    "status" TEXT DEFAULT Active,
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
    "code" TEXT NOT NULL,
    "id" TEXT NOT NULL,
    "sensitivity_level" INTEGER,
    "sync_status" TEXT DEFAULT in_process,
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
    "tombstone" INTEGER NOT NULL DEFAULT 0,
    "status" TEXT DEFAULT Active,
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
    "code" TEXT NOT NULL,
    "id" TEXT NOT NULL,
    "sensitivity_level" INTEGER,
    "sync_status" TEXT DEFAULT in_process,
    "is_batch" BOOLEAN DEFAULT false,
    PRIMARY KEY ("timestamp", "id")
);
--> statement-breakpoint
SELECT create_hypertable('test_hypertable', 'timestamp', chunk_time_interval => INTERVAL '1 day', if_not_exists => TRUE);
--> statement-breakpoint
CREATE INDEX "idx_files_filename" ON "files" USING btree(filename);
--> statement-breakpoint
CREATE INDEX "idx_files_etag" ON "files" USING btree(etag);
--> statement-breakpoint
CREATE INDEX "idx_files_mimetype" ON "files" USING btree(mimetype);
--> statement-breakpoint
CREATE INDEX "idx_files_tags" ON "files" USING btree(tags);
--> statement-breakpoint
CREATE INDEX "idx_test_hypertable_sensor" ON "test_hypertable" USING btree(sensor_id);
--> statement-breakpoint
CREATE INDEX "idx_test_hypertable_location" ON "test_hypertable" USING btree(location);
