-- This file should undo anything in `up.sql`

ALTER TABLE "test_products" DROP CONSTRAINT IF EXISTS "test_products_requested_by_account_organizations_id_fk";
ALTER TABLE "test_products" DROP CONSTRAINT IF EXISTS "test_products_deleted_by_account_organizations_id_fk";
ALTER TABLE "test_products" DROP CONSTRAINT IF EXISTS "test_products_updated_by_account_organizations_id_fk";
ALTER TABLE "test_products" DROP CONSTRAINT IF EXISTS "test_products_created_by_account_organizations_id_fk";
ALTER TABLE "test_products" DROP CONSTRAINT IF EXISTS "test_products_organization_id_organizations_id_fk";
DROP INDEX IF EXISTS "idx_test_products_idx_test_products_price";
DROP INDEX IF EXISTS "idx_test_products_idx_test_products_sku";
DROP INDEX IF EXISTS "idx_test_products_idx_test_products_sensitivity_level";
DROP INDEX IF EXISTS "idx_test_products_idx_test_products_code";
DROP INDEX IF EXISTS "idx_test_products_idx_test_products_categories";
DROP INDEX IF EXISTS "idx_test_products_idx_test_products_tags";
DROP INDEX IF EXISTS "idx_test_products_idx_test_products_requested_by";
DROP INDEX IF EXISTS "idx_test_products_idx_test_products_deleted_by";
DROP INDEX IF EXISTS "idx_test_products_idx_test_products_updated_by";
DROP INDEX IF EXISTS "idx_test_products_idx_test_products_created_by";
DROP INDEX IF EXISTS "idx_test_products_idx_test_products_organization_id";
DROP INDEX IF EXISTS "idx_test_products_idx_test_products_updated_date";
DROP INDEX IF EXISTS "idx_test_products_idx_test_products_created_date";
DROP INDEX IF EXISTS "idx_test_products_idx_test_products_version";
DROP INDEX IF EXISTS "idx_test_products_idx_test_products_previous_status";
DROP INDEX IF EXISTS "idx_test_products_idx_test_products_status";
DROP INDEX IF EXISTS "idx_test_products_idx_test_products_tombstone";
DROP TABLE IF EXISTS "test_products";
