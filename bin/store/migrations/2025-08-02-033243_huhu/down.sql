-- This file should undo anything in `up.sql`

DROP INDEX IF EXISTS "idx_test_hypertable_idx_test_hypertable_location";
DROP INDEX IF EXISTS "idx_test_hypertable_idx_test_hypertable_sensor";
DROP TABLE IF EXISTS "test_hypertable";
