CREATE OR REPLACE FUNCTION forward_to_hypertable()
RETURNS TRIGGER
LANGUAGE plpgsql
AS $$
DECLARE
  from_table text;
  to_table text;
BEGIN
  IF TG_NARGS >= 2 THEN
    from_table := TG_ARGV[0];
    to_table := TG_ARGV[1];
  ELSE
    from_table := NULL;
    to_table := TG_ARGV[0];
  END IF;

  IF from_table IS NOT NULL AND TG_TABLE_NAME <> from_table THEN
    RAISE EXCEPTION 'Trigger source mismatch: expected %, got %', from_table, TG_TABLE_NAME;
  END IF;
  INSERT INTO test_hypertables_new
  VALUES (NEW.*);
  RETURN NEW;
END;
$$;

-- Phase 1  - for old data

INSERT INTO test_hypertables_new
SELECT *
FROM test_hypertables
WHERE timestamp < now() - interval '5 minutes'
-- Add this if re-migrating
-- ON CONFLICT (id, timestamp) DO NOTHING;

-- Phase 2 - for new data
DROP TRIGGER IF EXISTS trg_forward_to_hypertable ON test_hypertables;
CREATE TRIGGER trg_forward_to_hypertable
AFTER INSERT ON test_hypertables
FOR EACH ROW
EXECUTE FUNCTION forward_to_hypertable('test_hypertables', 'test_hypertables_new');

-- phase 3 - for backfill data
CREATE TABLE IF NOT EXISTS migration_checkpoints (
  source_table text NOT NULL,
  dest_table text NOT NULL,
  time_column text NOT NULL,
  id_column text NOT NULL,
  last_ts timestamp,
  last_id text,
  PRIMARY KEY (source_table, dest_table, time_column, id_column)
);

DROP FUNCTION IF EXISTS backfill_batch;
CREATE OR REPLACE FUNCTION backfill_batch(p_source_table text, p_dest_table text, p_time_column text, p_id_column text, p_batch_size integer)
RETURNS timestamp
LANGUAGE plpgsql
AS $$
DECLARE
  checkpoint_ts timestamp;
  new_checkpoint_ts timestamp;
  checkpoint_id text;
  new_checkpoint_id text;
  cols text;
  id_type text;
BEGIN
  SELECT last_ts, last_id INTO checkpoint_ts, checkpoint_id
  FROM migration_checkpoints
  WHERE migration_checkpoints.source_table = p_source_table
    AND migration_checkpoints.dest_table = p_dest_table
    AND migration_checkpoints.time_column = p_time_column
    AND migration_checkpoints.id_column = p_id_column
  FOR UPDATE;

  IF checkpoint_ts IS NULL THEN
    checkpoint_ts := '-infinity'::timestamp;
    checkpoint_id := NULL;
    INSERT INTO migration_checkpoints (source_table, dest_table, time_column, id_column, last_ts, last_id)
    VALUES (p_source_table, p_dest_table, p_time_column, p_id_column, checkpoint_ts, checkpoint_id)
    ON CONFLICT (source_table, dest_table, time_column, id_column) DO NOTHING;

    SELECT last_ts, last_id INTO checkpoint_ts, checkpoint_id
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
    AND s.table_name = p_source_table
    AND c.column_name = s.column_name;

  SELECT format_type(a.atttypid, a.atttypmod)
  INTO id_type
  FROM pg_attribute a
  JOIN pg_class c ON c.oid = a.attrelid
  JOIN pg_namespace n ON n.oid = c.relnamespace
  WHERE n.nspname = 'public'
    AND c.relname = p_source_table
    AND a.attname = p_id_column;

  EXECUTE format(
    'WITH ins AS (
       INSERT INTO %I (%s)
       SELECT %s FROM %I
       WHERE (%I > $1)
          OR (%I = $1 AND ($2 IS NULL OR %I > $2::%s))
       ORDER BY %I, %I
       LIMIT $3
       ON CONFLICT (%I, %I) DO NOTHING
       RETURNING %I AS ts_ret, %I AS id_ret
     )
     SELECT ts_ret, id_ret FROM ins ORDER BY ts_ret DESC, id_ret DESC LIMIT 1',
    p_dest_table, cols, cols, p_source_table, p_time_column, p_time_column, p_id_column, id_type,
    p_time_column, p_id_column,
    p_id_column, p_time_column,
    p_time_column, p_id_column
  )
  USING checkpoint_ts, checkpoint_id, p_batch_size
  INTO new_checkpoint_ts, new_checkpoint_id;

  IF new_checkpoint_ts IS NULL THEN
    RETURN checkpoint_ts;
  END IF;

  UPDATE migration_checkpoints
  SET last_ts = new_checkpoint_ts,
      last_id = new_checkpoint_id
  WHERE migration_checkpoints.source_table = p_source_table
    AND migration_checkpoints.dest_table = p_dest_table
    AND migration_checkpoints.time_column = p_time_column
    AND migration_checkpoints.id_column = p_id_column;

  RETURN new_checkpoint_ts;
END;
$$;

SELECT backfill_batch('test_hypertables', 'test_hypertables_new', 'timestamp', 'id', 1000000);

-- Phase 4 - verify
-- detect missing rows
SELECT *
FROM test_hypertables
EXCEPT
SELECT *
FROM test_hypertables_new;
SELECT *
FROM test_hypertables_new
EXCEPT
SELECT *
FROM test_hypertables;

SELECT count(*) FROM test_hypertables;
SELECT count(*) FROM test_hypertables_new;
-- Phase 5
-- Switch ORM operation to use new table
