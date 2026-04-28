-- This file should undo anything in `up.sql`

ALTER TABLE "temp_aliases" ADD COLUMN "table" TEXT;
ALTER TABLE "temp_aliases" ADD COLUMN "type" TEXT;
ALTER TABLE "temp_aliases" DROP COLUMN IF EXISTS "table_";
ALTER TABLE "temp_aliases" DROP COLUMN IF EXISTS "type_";
ALTER TABLE "temp_device_filter_rules" ADD COLUMN "table" TEXT;
ALTER TABLE "temp_device_filter_rules" DROP COLUMN IF EXISTS "table_";
ALTER TABLE "temp_device_nat_rules" ADD COLUMN "table" TEXT;
ALTER TABLE "temp_device_nat_rules" DROP COLUMN IF EXISTS "table_";
ALTER TABLE "aliases" ADD COLUMN "table" TEXT;
ALTER TABLE "aliases" ADD COLUMN "type" TEXT;
ALTER TABLE "aliases" DROP COLUMN IF EXISTS "table_";
ALTER TABLE "aliases" DROP COLUMN IF EXISTS "type_";
