-- Drop the hypertable in one go so TimescaleDB cleans up chunks and catalog.
-- Manually dropping indexes/constraints first can leave _timescaledb_internal chunk
-- references orphaned, causing "cache lookup failed for relation _hyper_*_chunk".
DROP TABLE IF EXISTS "signed_in_activities" CASCADE;