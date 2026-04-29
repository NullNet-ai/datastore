-- Drizzle ORM's IndexBuilder (pg-core) does not expose an .include() method,
-- so the INCLUDE clause cannot be expressed in the schema definition and will
-- never be emitted by drizzle:generate / drizzle:migrate. Run this script
-- manually via: npm run sql:exec ./src/schema/sql/indexes/connections_include_total_byte.sql
CREATE INDEX IF NOT EXISTS connections_device_id_source_ip_timestamp_include_total_byte_idx
  ON connections (device_id, source_ip, timestamp DESC)
  INCLUDE (total_byte);
