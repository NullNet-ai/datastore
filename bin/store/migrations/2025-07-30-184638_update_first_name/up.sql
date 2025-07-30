-- Your SQL goes here

ALTER TABLE "products" ADD COLUMN "first_name" TEXT NOT NULL;
--> statement-breakpoint
CREATE INDEX "idx_products_sku" ON "products" USING btree(sku);
--> statement-breakpoint
CREATE INDEX "idx_products_category" ON "products" USING btree(category_id);
--> statement-breakpoint
CREATE INDEX "idx_products_active_featured" ON "products" USING btree(is_active,is_featured);
--> statement-breakpoint
CREATE INDEX "idx_products_tags_gin" ON "products" USING gin(tags);
--> statement-breakpoint
CREATE INDEX "idx_products_metadata_gin" ON "products" USING gin(metadata);
--> statement-breakpoint
CREATE INDEX "idx_products_price" ON "products" USING btree(price);
--> statement-breakpoint
ALTER TABLE "products" ADD CONSTRAINT "fk_products_category_id" FOREIGN KEY ("category_id") REFERENCES "categories" ("id");
