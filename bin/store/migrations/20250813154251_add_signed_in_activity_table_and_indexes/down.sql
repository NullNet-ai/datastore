-- Drop foreign key constraints
ALTER TABLE "signed_in_activities" DROP CONSTRAINT IF EXISTS "signed_in_activities_session_id_sessions_id_fk";
ALTER TABLE "signed_in_activities" DROP CONSTRAINT IF EXISTS "signed_in_activities_requested_by_account_organizations_id_fk";
ALTER TABLE "signed_in_activities" DROP CONSTRAINT IF EXISTS "signed_in_activities_deleted_by_account_organizations_id_fk";
ALTER TABLE "signed_in_activities" DROP CONSTRAINT IF EXISTS "signed_in_activities_updated_by_account_organizations_id_fk";
ALTER TABLE "signed_in_activities" DROP CONSTRAINT IF EXISTS "signed_in_activities_created_by_account_organizations_id_fk";
ALTER TABLE "signed_in_activities" DROP CONSTRAINT IF EXISTS "signed_in_activities_organization_id_organizations_id_fk";

-- Drop indexes for signed_in_activities table
DROP INDEX IF EXISTS "idx_signed_in_activities_session_id";
DROP INDEX IF EXISTS "idx_signed_in_activities_sensitivity_level";
DROP INDEX IF EXISTS "idx_signed_in_activities_code";
DROP INDEX IF EXISTS "idx_signed_in_activities_categories";
DROP INDEX IF EXISTS "idx_signed_in_activities_tags";
DROP INDEX IF EXISTS "idx_signed_in_activities_requested_by";
DROP INDEX IF EXISTS "idx_signed_in_activities_deleted_by";
DROP INDEX IF EXISTS "idx_signed_in_activities_updated_by";
DROP INDEX IF EXISTS "idx_signed_in_activities_created_by";
DROP INDEX IF EXISTS "idx_signed_in_activities_organization_id";
DROP INDEX IF EXISTS "idx_signed_in_activities_updated_date";
DROP INDEX IF EXISTS "idx_signed_in_activities_created_date";
DROP INDEX IF EXISTS "idx_signed_in_activities_version";
DROP INDEX IF EXISTS "idx_signed_in_activities_previous_status";
DROP INDEX IF EXISTS "idx_signed_in_activities_status";
DROP INDEX IF EXISTS "idx_signed_in_activities_tombstone";

-- Drop signed_in_activities table
DROP TABLE IF EXISTS "signed_in_activities";