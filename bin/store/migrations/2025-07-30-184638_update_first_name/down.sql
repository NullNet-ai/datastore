-- This file should undo anything in `up.sql`

ALTER TABLE "products" DROP CONSTRAINT IF EXISTS "fk_products_category_id";
DROP INDEX IF EXISTS "idx_products_idx_products_price";
DROP INDEX IF EXISTS "idx_products_idx_products_metadata_gin";
DROP INDEX IF EXISTS "idx_products_idx_products_tags_gin";
DROP INDEX IF EXISTS "idx_products_idx_products_active_featured";
DROP INDEX IF EXISTS "idx_products_idx_products_category";
DROP INDEX IF EXISTS "idx_products_idx_products_sku";
ALTER TABLE "products" DROP COLUMN IF EXISTS "first_name";
