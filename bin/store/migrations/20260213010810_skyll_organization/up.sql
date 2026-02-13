-- Your SQL goes here

ALTER TABLE "organizations" ADD COLUMN "skyll_id" TEXT;
--> statement-breakpoint
ALTER TABLE "organizations" ADD COLUMN "department_id" TEXT;
--> statement-breakpoint
ALTER TABLE "organizations" ADD COLUMN "district_id" TEXT;
--> statement-breakpoint
ALTER TABLE "organizations" ADD COLUMN "school_id" TEXT;
--> statement-breakpoint
ALTER TABLE "organizations" ADD COLUMN "city" TEXT;
--> statement-breakpoint
ALTER TABLE "organizations" ADD COLUMN "county" TEXT;
--> statement-breakpoint
ALTER TABLE "organizations" ADD COLUMN "state" TEXT;
--> statement-breakpoint
ALTER TABLE "organizations" ADD COLUMN "school_identifier" TEXT;
--> statement-breakpoint
ALTER TABLE "organizations" ADD COLUMN "district_identifier" TEXT;
--> statement-breakpoint
ALTER TABLE "organizations" ADD COLUMN "superintendent_id" TEXT;
--> statement-breakpoint
ALTER TABLE "organizations" ADD COLUMN "principal_id" TEXT;
--> statement-breakpoint
CREATE INDEX "idx_organizations_name" ON "organizations" USING btree(name);
--> statement-breakpoint
CREATE INDEX "idx_organizations_parent_organization_id" ON "organizations" USING btree(parent_organization_id);
--> statement-breakpoint
CREATE INDEX "idx_organizations_root_organization_id" ON "organizations" USING btree(root_organization_id);
--> statement-breakpoint
CREATE INDEX "idx_organizations_skyll_id" ON "organizations" USING btree(skyll_id);
--> statement-breakpoint
CREATE INDEX "idx_organizations_school_id" ON "organizations" USING btree(school_id);
--> statement-breakpoint
CREATE INDEX "idx_organizations_district_id" ON "organizations" USING btree(district_id);
--> statement-breakpoint
CREATE INDEX "idx_organizations_department_id" ON "organizations" USING btree(department_id);
--> statement-breakpoint
CREATE INDEX "idx_organizations_city" ON "organizations" USING btree(city);
--> statement-breakpoint
CREATE INDEX "idx_organizations_county" ON "organizations" USING btree(county);
--> statement-breakpoint
CREATE INDEX "idx_organizations_state" ON "organizations" USING btree(state);
--> statement-breakpoint
CREATE INDEX "idx_organizations_school_identifier" ON "organizations" USING btree(school_identifier);
--> statement-breakpoint
CREATE INDEX "idx_organizations_district_identifier" ON "organizations" USING btree(district_identifier);
--> statement-breakpoint
CREATE INDEX "idx_organizations_organization_level" ON "organizations" USING btree(organization_level);
--> statement-breakpoint
CREATE INDEX "idx_organizations_path_level" ON "organizations" USING btree(path_level);
--> statement-breakpoint
CREATE INDEX "idx_organizations_superintendent_id" ON "organizations" USING btree(superintendent_id);
--> statement-breakpoint
CREATE INDEX "idx_organizations_principal_id" ON "organizations" USING btree(principal_id);
