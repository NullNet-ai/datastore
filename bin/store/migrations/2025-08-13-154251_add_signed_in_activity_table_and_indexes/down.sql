-- Drop indexes for signed_in_activity table
DROP INDEX IF EXISTS "idx_signed_in_activity_sensitivity_level";
DROP INDEX IF EXISTS "idx_signed_in_activity_code";
DROP INDEX IF EXISTS "idx_signed_in_activity_categories";
DROP INDEX IF EXISTS "idx_signed_in_activity_tags";
DROP INDEX IF EXISTS "idx_signed_in_activity_requested_by";
DROP INDEX IF EXISTS "idx_signed_in_activity_deleted_by";
DROP INDEX IF EXISTS "idx_signed_in_activity_updated_by";
DROP INDEX IF EXISTS "idx_signed_in_activity_created_by";
DROP INDEX IF EXISTS "idx_signed_in_activity_organization_id";
DROP INDEX IF EXISTS "idx_signed_in_activity_updated_date";
DROP INDEX IF EXISTS "idx_signed_in_activity_created_date";
DROP INDEX IF EXISTS "idx_signed_in_activity_version";
DROP INDEX IF EXISTS "idx_signed_in_activity_previous_status";
DROP INDEX IF EXISTS "idx_signed_in_activity_status";
DROP INDEX IF EXISTS "idx_signed_in_activity_tombstone";

-- Drop signed_in_activity table
DROP TABLE IF EXISTS "signed_in_activity";