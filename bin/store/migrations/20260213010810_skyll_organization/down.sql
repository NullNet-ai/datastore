-- This file should undo anything in `up.sql`

DROP INDEX IF EXISTS "idx_organizations_idx_organizations_principal_id";
DROP INDEX IF EXISTS "idx_organizations_idx_organizations_superintendent_id";
DROP INDEX IF EXISTS "idx_organizations_idx_organizations_path_level";
DROP INDEX IF EXISTS "idx_organizations_idx_organizations_organization_level";
DROP INDEX IF EXISTS "idx_organizations_idx_organizations_district_identifier";
DROP INDEX IF EXISTS "idx_organizations_idx_organizations_school_identifier";
DROP INDEX IF EXISTS "idx_organizations_idx_organizations_state";
DROP INDEX IF EXISTS "idx_organizations_idx_organizations_county";
DROP INDEX IF EXISTS "idx_organizations_idx_organizations_city";
DROP INDEX IF EXISTS "idx_organizations_idx_organizations_department_id";
DROP INDEX IF EXISTS "idx_organizations_idx_organizations_district_id";
DROP INDEX IF EXISTS "idx_organizations_idx_organizations_school_id";
DROP INDEX IF EXISTS "idx_organizations_idx_organizations_skyll_id";
DROP INDEX IF EXISTS "idx_organizations_idx_organizations_root_organization_id";
DROP INDEX IF EXISTS "idx_organizations_idx_organizations_parent_organization_id";
DROP INDEX IF EXISTS "idx_organizations_idx_organizations_name";
ALTER TABLE "organizations" DROP COLUMN IF EXISTS "principal_id";
ALTER TABLE "organizations" DROP COLUMN IF EXISTS "superintendent_id";
ALTER TABLE "organizations" DROP COLUMN IF EXISTS "district_identifier";
ALTER TABLE "organizations" DROP COLUMN IF EXISTS "school_identifier";
ALTER TABLE "organizations" DROP COLUMN IF EXISTS "state";
ALTER TABLE "organizations" DROP COLUMN IF EXISTS "county";
ALTER TABLE "organizations" DROP COLUMN IF EXISTS "city";
ALTER TABLE "organizations" DROP COLUMN IF EXISTS "school_id";
ALTER TABLE "organizations" DROP COLUMN IF EXISTS "district_id";
ALTER TABLE "organizations" DROP COLUMN IF EXISTS "department_id";
ALTER TABLE "organizations" DROP COLUMN IF EXISTS "skyll_id";
