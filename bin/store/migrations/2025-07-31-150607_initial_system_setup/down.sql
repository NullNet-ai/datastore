-- This file should undo anything in `up.sql`
-- Drop session indexes
DROP INDEX IF EXISTS "idx_sessions_sensitivity_level";
DROP INDEX IF EXISTS "idx_sessions_code";
DROP INDEX IF EXISTS "idx_sessions_categories";
DROP INDEX IF EXISTS "idx_sessions_tags";
DROP INDEX IF EXISTS "idx_sessions_requested_by";
DROP INDEX IF EXISTS "idx_sessions_deleted_by";
DROP INDEX IF EXISTS "idx_sessions_updated_by";
DROP INDEX IF EXISTS "idx_sessions_created_by";
DROP INDEX IF EXISTS "idx_sessions_organization_id";
DROP INDEX IF EXISTS "idx_sessions_updated_date";
DROP INDEX IF EXISTS "idx_sessions_created_date";
DROP INDEX IF EXISTS "idx_sessions_version";
DROP INDEX IF EXISTS "idx_sessions_previous_status";
DROP INDEX IF EXISTS "idx_sessions_tombstone";

DROP INDEX IF EXISTS idx_stream_queue_last_accessed;
DROP INDEX IF EXISTS idx_stream_queue_created_at;
DROP INDEX IF EXISTS idx_stream_queue_name;
DROP TABLE IF EXISTS stream_queue;
DROP INDEX IF EXISTS idx_stream_queue_items_timestamp;
DROP INDEX IF EXISTS idx_stream_queue_items_queue_name;
DROP TABLE IF EXISTS stream_queue_items;

-- This file should undo anything in `up.sql`
ALTER TABLE "user_roles" DROP COLUMN "sync_status";
ALTER TABLE "user_roles" DROP COLUMN "is_batch";

ALTER TABLE "external_contacts" DROP COLUMN "sync_status";
ALTER TABLE "external_contacts" DROP COLUMN "is_batch";

ALTER TABLE "organizations" DROP COLUMN "sync_status";
ALTER TABLE "organizations" DROP COLUMN "is_batch";

ALTER TABLE "organization_contacts" DROP COLUMN "sync_status";
ALTER TABLE "organization_contacts" DROP COLUMN "is_batch";

ALTER TABLE "organization_accounts" DROP COLUMN "sync_status";
ALTER TABLE "organization_accounts" DROP COLUMN "is_batch";

ALTER TABLE "account_organizations" DROP COLUMN "sync_status";
ALTER TABLE "account_organizations" DROP COLUMN "is_batch";

ALTER TABLE "account_profiles" DROP COLUMN "sync_status";
ALTER TABLE "account_profiles" DROP COLUMN "is_batch";

ALTER TABLE "organization_domains" DROP COLUMN "sync_status";
ALTER TABLE "organization_domains" DROP COLUMN "is_batch";

ALTER TABLE "addresses" DROP COLUMN "sync_status";
ALTER TABLE "addresses" DROP COLUMN "is_batch";

ALTER TABLE "devices" DROP COLUMN "sync_status";
ALTER TABLE "devices" DROP COLUMN "is_batch";


ALTER TABLE "postgres_channels" DROP COLUMN "sync_status";
ALTER TABLE "postgres_channels" DROP COLUMN "is_batch";

ALTER TABLE "contacts" DROP COLUMN "sync_status";
ALTER TABLE "contacts" DROP COLUMN "is_batch";

ALTER TABLE "contact_phone_numbers" DROP COLUMN "sync_status";
ALTER TABLE "contact_phone_numbers" DROP COLUMN "is_batch";

ALTER TABLE "contact_emails" DROP COLUMN "sync_status";
ALTER TABLE "contact_emails" DROP COLUMN "is_batch";