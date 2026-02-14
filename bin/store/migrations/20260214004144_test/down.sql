-- This file should undo anything in `up.sql`

ALTER TABLE "demo_items" DROP CONSTRAINT IF EXISTS "demo_items_requested_by_account_organizations_id_fk";
ALTER TABLE "demo_items" DROP CONSTRAINT IF EXISTS "demo_items_deleted_by_account_organizations_id_fk";
ALTER TABLE "demo_items" DROP CONSTRAINT IF EXISTS "demo_items_updated_by_account_organizations_id_fk";
ALTER TABLE "demo_items" DROP CONSTRAINT IF EXISTS "demo_items_created_by_account_organizations_id_fk";
ALTER TABLE "demo_items" DROP CONSTRAINT IF EXISTS "demo_items_organization_id_organizations_id_fk";
DROP INDEX IF EXISTS "idx_demo_items_idx_demo_items_title";
DROP INDEX IF EXISTS "idx_demo_items_idx_demo_items_sensitivity_level";
DROP INDEX IF EXISTS "idx_demo_items_idx_demo_items_code";
DROP INDEX IF EXISTS "idx_demo_items_idx_demo_items_categories";
DROP INDEX IF EXISTS "idx_demo_items_idx_demo_items_tags";
DROP INDEX IF EXISTS "idx_demo_items_idx_demo_items_requested_by";
DROP INDEX IF EXISTS "idx_demo_items_idx_demo_items_deleted_by";
DROP INDEX IF EXISTS "idx_demo_items_idx_demo_items_updated_by";
DROP INDEX IF EXISTS "idx_demo_items_idx_demo_items_created_by";
DROP INDEX IF EXISTS "idx_demo_items_idx_demo_items_organization_id";
DROP INDEX IF EXISTS "idx_demo_items_idx_demo_items_updated_date";
DROP INDEX IF EXISTS "idx_demo_items_idx_demo_items_created_date";
DROP INDEX IF EXISTS "idx_demo_items_idx_demo_items_version";
DROP INDEX IF EXISTS "idx_demo_items_idx_demo_items_previous_status";
DROP INDEX IF EXISTS "idx_demo_items_idx_demo_items_status";
DROP INDEX IF EXISTS "idx_demo_items_idx_demo_items_tombstone";
DROP TABLE IF EXISTS "demo_items";
