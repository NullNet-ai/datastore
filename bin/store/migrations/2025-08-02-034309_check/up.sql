-- Your SQL goes here

CREATE TABLE "test_hypertable" (
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
    "timestamp" TIMESTAMPTZ,
    "hypertable_timestamp" TEXT,
    "sensor_id" TEXT,
    "temperature" INTEGER,
    "humidity" INTEGER,
    "location" TEXT,
    PRIMARY KEY ("id", "timestamp")
);
--> statement-breakpoint
SELECT create_hypertable('test_hypertable', 'timestamp', chunk_time_interval => INTERVAL '1 day', if_not_exists => TRUE);
