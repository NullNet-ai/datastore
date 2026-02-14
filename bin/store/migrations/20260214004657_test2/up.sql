-- Your SQL goes here

ALTER TABLE "demo_items" ADD COLUMN "name" TEXT;
--> statement-breakpoint
CREATE INDEX "idx_demo_items_name" ON "demo_items" USING btree("name");
