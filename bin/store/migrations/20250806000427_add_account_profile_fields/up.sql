-- Your SQL goes here

ALTER TABLE "account_profiles" 
ADD COLUMN "date_of_birth" TIMESTAMP,
ADD COLUMN "middle_name" TEXT,
ADD COLUMN "auth_preference" TEXT;