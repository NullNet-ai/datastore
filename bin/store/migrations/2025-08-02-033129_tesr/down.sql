-- This file should undo anything in `up.sql`

DROP INDEX IF EXISTS "idx_files_idx_files_tags";
DROP INDEX IF EXISTS "idx_files_idx_files_mimetype";
DROP INDEX IF EXISTS "idx_files_idx_files_etag";
DROP INDEX IF EXISTS "idx_files_idx_files_filename";
