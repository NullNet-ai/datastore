-- Your SQL goes here

ALTER TABLE "device_filter_rules" ADD COLUMN "table_" TEXT;
--> statement-breakpoint
ALTER TABLE "device_nat_rules" ADD COLUMN "table_" TEXT;
--> statement-breakpoint
ALTER TABLE "device_filter_rules" DROP COLUMN "table";
--> statement-breakpoint
ALTER TABLE "device_nat_rules" DROP COLUMN "table";
