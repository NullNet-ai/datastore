-- This file should undo anything in `up.sql`

ALTER TABLE "checkpoint_samples" DROP CONSTRAINT IF EXISTS "fk_checkpoint_samples_requested_by";
ALTER TABLE "checkpoint_samples" DROP CONSTRAINT IF EXISTS "fk_checkpoint_samples_deleted_by";
ALTER TABLE "checkpoint_samples" DROP CONSTRAINT IF EXISTS "fk_checkpoint_samples_updated_by";
ALTER TABLE "checkpoint_samples" DROP CONSTRAINT IF EXISTS "fk_checkpoint_samples_created_by";
ALTER TABLE "checkpoint_samples" DROP CONSTRAINT IF EXISTS "fk_checkpoint_samples_organization_id";
DROP INDEX IF EXISTS "idx_checkpoint_samples_idx_checkpoint_samples_label";
DROP INDEX IF EXISTS "idx_checkpoint_samples_idx_checkpoint_samples_sensitivity_level";
DROP INDEX IF EXISTS "idx_checkpoint_samples_idx_checkpoint_samples_code";
DROP INDEX IF EXISTS "idx_checkpoint_samples_idx_checkpoint_samples_categories";
DROP INDEX IF EXISTS "idx_checkpoint_samples_idx_checkpoint_samples_tags";
DROP INDEX IF EXISTS "idx_checkpoint_samples_idx_checkpoint_samples_requested_by";
DROP INDEX IF EXISTS "idx_checkpoint_samples_idx_checkpoint_samples_deleted_by";
DROP INDEX IF EXISTS "idx_checkpoint_samples_idx_checkpoint_samples_updated_by";
DROP INDEX IF EXISTS "idx_checkpoint_samples_idx_checkpoint_samples_created_by";
DROP INDEX IF EXISTS "idx_checkpoint_samples_idx_checkpoint_samples_organization_id";
DROP INDEX IF EXISTS "idx_checkpoint_samples_idx_checkpoint_samples_updated_date";
DROP INDEX IF EXISTS "idx_checkpoint_samples_idx_checkpoint_samples_created_date";
DROP INDEX IF EXISTS "idx_checkpoint_samples_idx_checkpoint_samples_version";
DROP INDEX IF EXISTS "idx_checkpoint_samples_idx_checkpoint_samples_previous_status";
DROP INDEX IF EXISTS "idx_checkpoint_samples_idx_checkpoint_samples_status";
DROP INDEX IF EXISTS "idx_checkpoint_samples_idx_checkpoint_samples_tombstone";
DROP TABLE IF EXISTS "checkpoint_samples";
