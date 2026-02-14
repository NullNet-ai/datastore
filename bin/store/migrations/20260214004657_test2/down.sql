-- This file should undo anything in `up.sql`

DROP INDEX IF EXISTS "idx_demo_items_idx_demo_items_name";
ALTER TABLE "demo_items" DROP COLUMN IF EXISTS "name";
