-- This file should undo anything in `up.sql`

ALTER TABLE "account_profiles" 
DROP COLUMN "date_of_birth",
DROP COLUMN "middle_name",
DROP COLUMN "auth_preference";