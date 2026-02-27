-- This file should undo anything in `up.sql`

ALTER TABLE "sample_checks" DROP CONSTRAINT IF EXISTS "fk_sample_checks_requested_by";
ALTER TABLE "sample_checks" DROP CONSTRAINT IF EXISTS "fk_sample_checks_deleted_by";
ALTER TABLE "sample_checks" DROP CONSTRAINT IF EXISTS "fk_sample_checks_updated_by";
ALTER TABLE "sample_checks" DROP CONSTRAINT IF EXISTS "fk_sample_checks_created_by";
ALTER TABLE "sample_checks" DROP CONSTRAINT IF EXISTS "fk_sample_checks_organization_id";
DROP INDEX IF EXISTS "idx_sample_checks_idx_sample_checks_sensitivity_level";
DROP INDEX IF EXISTS "idx_sample_checks_idx_sample_checks_code";
DROP INDEX IF EXISTS "idx_sample_checks_idx_sample_checks_categories";
DROP INDEX IF EXISTS "idx_sample_checks_idx_sample_checks_tags";
DROP INDEX IF EXISTS "idx_sample_checks_idx_sample_checks_requested_by";
DROP INDEX IF EXISTS "idx_sample_checks_idx_sample_checks_deleted_by";
DROP INDEX IF EXISTS "idx_sample_checks_idx_sample_checks_updated_by";
DROP INDEX IF EXISTS "idx_sample_checks_idx_sample_checks_created_by";
DROP INDEX IF EXISTS "idx_sample_checks_idx_sample_checks_organization_id";
DROP INDEX IF EXISTS "idx_sample_checks_idx_sample_checks_updated_date";
DROP INDEX IF EXISTS "idx_sample_checks_idx_sample_checks_created_date";
DROP INDEX IF EXISTS "idx_sample_checks_idx_sample_checks_version";
DROP INDEX IF EXISTS "idx_sample_checks_idx_sample_checks_previous_status";
DROP INDEX IF EXISTS "idx_sample_checks_idx_sample_checks_status";
DROP INDEX IF EXISTS "idx_sample_checks_idx_sample_checks_tombstone";
DROP TABLE IF EXISTS "sample_checks";
