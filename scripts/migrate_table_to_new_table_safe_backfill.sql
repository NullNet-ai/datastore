-- Standalone safer backfill function for PostgreSQL table -> Timescale hypertable migration.
-- This implementation advances checkpoint by scanned rows, so it does not stall when
-- ON CONFLICT DO NOTHING skips inserts for already-copied rows.

CREATE TABLE IF NOT EXISTS migration_checkpoints (
  source_table text NOT NULL,
  dest_table text NOT NULL,
  time_column text NOT NULL,
  id_column text NOT NULL,
  last_ts timestamp,
  last_id text,
  PRIMARY KEY (source_table, dest_table, time_column, id_column)
);

DROP FUNCTION IF EXISTS backfill_batch_safe;
CREATE OR REPLACE FUNCTION backfill_batch_safe(
  p_source_table text,
  p_dest_table text,
  p_time_column text,
  p_id_column text,
  p_batch_size integer
)
RETURNS TABLE (
  checkpoint_ts timestamp,
  checkpoint_id text,
  scanned_count bigint,
  inserted_count bigint
)
LANGUAGE plpgsql
AS $$
DECLARE
  current_checkpoint_ts timestamp;
  current_checkpoint_id text;
  max_scanned_ts timestamp;
  max_scanned_id text;
  cols text;
  id_type text;
BEGIN
  IF p_batch_size IS NULL OR p_batch_size <= 0 THEN
    RAISE EXCEPTION 'p_batch_size must be > 0';
  END IF;

  SELECT last_ts, last_id INTO current_checkpoint_ts, current_checkpoint_id
  FROM migration_checkpoints
  WHERE migration_checkpoints.source_table = p_source_table
    AND migration_checkpoints.dest_table = p_dest_table
    AND migration_checkpoints.time_column = p_time_column
    AND migration_checkpoints.id_column = p_id_column
  FOR UPDATE;

  IF current_checkpoint_ts IS NULL THEN
    current_checkpoint_ts := '-infinity'::timestamp;
    current_checkpoint_id := NULL;

    INSERT INTO migration_checkpoints (source_table, dest_table, time_column, id_column, last_ts, last_id)
    VALUES (p_source_table, p_dest_table, p_time_column, p_id_column, current_checkpoint_ts, current_checkpoint_id)
    ON CONFLICT (source_table, dest_table, time_column, id_column) DO NOTHING;

    SELECT last_ts, last_id INTO current_checkpoint_ts, current_checkpoint_id
    FROM migration_checkpoints
    WHERE migration_checkpoints.source_table = p_source_table
      AND migration_checkpoints.dest_table = p_dest_table
      AND migration_checkpoints.time_column = p_time_column
      AND migration_checkpoints.id_column = p_id_column
    FOR UPDATE;
  END IF;

  SELECT string_agg(quote_ident(c.column_name), ', ' ORDER BY c.ordinal_position)
  INTO cols
  FROM information_schema.columns c
  JOIN information_schema.columns s
    ON s.column_name = c.column_name
   AND s.table_schema = c.table_schema
  WHERE c.table_schema = 'public'
    AND s.table_schema = 'public'
    AND c.table_name = p_dest_table
    AND s.table_name = p_source_table;

  IF cols IS NULL THEN
    RAISE EXCEPTION 'No shared columns found between % and %', p_source_table, p_dest_table;
  END IF;

  SELECT format_type(a.atttypid, a.atttypmod)
  INTO id_type
  FROM pg_attribute a
  JOIN pg_class c ON c.oid = a.attrelid
  JOIN pg_namespace n ON n.oid = c.relnamespace
  WHERE n.nspname = 'public'
    AND c.relname = p_source_table
    AND a.attname = p_id_column;

  IF id_type IS NULL THEN
    RAISE EXCEPTION 'Could not resolve id column type for %.%', p_source_table, p_id_column;
  END IF;

  EXECUTE format(
    'WITH src AS (
       SELECT %1$I AS ts_src, %2$I AS id_src, %3$s
       FROM %4$I
       WHERE (%1$I > $1)
          OR (%1$I = $1 AND ($2 IS NULL OR %2$I > $2::%5$s))
       ORDER BY %1$I, %2$I
       LIMIT $3
     ),
     ins AS (
       INSERT INTO %6$I (%3$s)
       SELECT %3$s FROM src
       ON CONFLICT (%2$I, %1$I) DO NOTHING
       RETURNING 1
     ),
     max_src AS (
       SELECT ts_src, id_src
       FROM src
       ORDER BY ts_src DESC, id_src DESC
       LIMIT 1
     )
     SELECT
       (SELECT ts_src FROM max_src),
       (SELECT id_src::text FROM max_src),
       (SELECT count(*) FROM src),
       (SELECT count(*) FROM ins)',
    p_time_column,
    p_id_column,
    cols,
    p_source_table,
    id_type,
    p_dest_table
  )
  USING current_checkpoint_ts, current_checkpoint_id, p_batch_size
  INTO max_scanned_ts, max_scanned_id, scanned_count, inserted_count;

  IF COALESCE(scanned_count, 0) = 0 THEN
    checkpoint_ts := current_checkpoint_ts;
    checkpoint_id := current_checkpoint_id;
    scanned_count := 0;
    inserted_count := 0;
    RETURN NEXT;
    RETURN;
  END IF;

  UPDATE migration_checkpoints
  SET last_ts = max_scanned_ts,
      last_id = max_scanned_id
  WHERE migration_checkpoints.source_table = p_source_table
    AND migration_checkpoints.dest_table = p_dest_table
    AND migration_checkpoints.time_column = p_time_column
    AND migration_checkpoints.id_column = p_id_column;

  checkpoint_ts := max_scanned_ts;
  checkpoint_id := max_scanned_id;
  RETURN NEXT;
END;
$$;

-- Example:
-- SELECT * FROM backfill_batch_safe('test_hypertables', 'test_hypertables_new', 'timestamp', 'id', 1000000);
