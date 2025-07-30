-- Your SQL goes here

CREATE TABLE "products" (
    "id" TEXT NOT NULL,
    "sku" TEXT NOT NULL,
    "name" TEXT NOT NULL,
    "description" TEXT NOT NULL,
    "category_id" INTEGER NOT NULL,
    "price" TEXT NOT NULL,
    "currency" TEXT NOT NULL,
    "stock_quantity" INTEGER NOT NULL,
    "is_active" BOOLEAN NOT NULL,
    "is_featured" BOOLEAN NOT NULL,
    "weight_kg" TEXT,
    "dimensions" JSONB,
    "tags" TEXT NOT NULL,
    "metadata" JSONB,
    "image_urls" TEXT NOT NULL,
    "supplier_info" JSONB,
    "created_at" TIMESTAMPTZ NOT NULL,
    "updated_at" TIMESTAMPTZ NOT NULL,
    "deleted_at" TIMESTAMPTZ NOT NULL
);
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
