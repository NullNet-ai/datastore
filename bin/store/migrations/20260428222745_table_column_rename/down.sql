-- This file should undo anything in `up.sql`

ALTER TABLE "device_nat_rules" ADD COLUMN "table" TEXT;
ALTER TABLE "device_nat_rules" DROP COLUMN IF EXISTS "table_";
ALTER TABLE "device_filter_rules" ADD COLUMN "table" TEXT;
ALTER TABLE "device_filter_rules" DROP COLUMN IF EXISTS "table_";
