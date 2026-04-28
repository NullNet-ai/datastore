-- Your SQL goes here

ALTER TABLE "aliases" ADD COLUMN "type_" TEXT;
--> statement-breakpoint
ALTER TABLE "aliases" ADD COLUMN "table_" TEXT;
--> statement-breakpoint
ALTER TABLE "temp_device_nat_rules" ADD COLUMN "table_" TEXT;
--> statement-breakpoint
ALTER TABLE "temp_device_filter_rules" ADD COLUMN "table_" TEXT;
--> statement-breakpoint
ALTER TABLE "temp_aliases" ADD COLUMN "type_" TEXT;
--> statement-breakpoint
ALTER TABLE "temp_aliases" ADD COLUMN "table_" TEXT;
--> statement-breakpoint
ALTER TABLE "aliases" DROP COLUMN "type";
--> statement-breakpoint
ALTER TABLE "aliases" DROP COLUMN "table";
--> statement-breakpoint
ALTER TABLE "temp_device_nat_rules" DROP COLUMN "table";
--> statement-breakpoint
ALTER TABLE "temp_device_filter_rules" DROP COLUMN "table";
--> statement-breakpoint
ALTER TABLE "temp_aliases" DROP COLUMN "type";
--> statement-breakpoint
ALTER TABLE "temp_aliases" DROP COLUMN "table";
