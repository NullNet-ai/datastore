-- This file should undo anything in `up.sql`
-- =============================================
-- down.sql – Reverse the objects created in the
-- associated "up" migration.
-- =============================================
-- NOTE: All objects are dropped in dependency‑safe
--       reverse order. CASCADE ensures that any
--       foreign‑key constraints, indexes, and
--       sequences created alongside the tables are
--       removed automatically.

BEGIN;

-- Highest‑level dependents first
DROP TABLE IF EXISTS "role_permissions" CASCADE;
DROP TABLE IF EXISTS "data_permissions" CASCADE;

-- Stand‑alone or lower‑level dependents
DROP TABLE IF EXISTS "sessions"         CASCADE;
DROP TABLE IF EXISTS "encryption_keys"  CASCADE;
DROP TABLE IF EXISTS "permissions"      CASCADE;
DROP TABLE IF EXISTS "entity_fields"    CASCADE;
DROP TABLE IF EXISTS "entities"         CASCADE;

COMMIT;
