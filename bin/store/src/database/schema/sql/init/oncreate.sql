CREATE OR REPLACE FUNCTION on_create_table()
RETURNS event_trigger AS $$
BEGIN
  RAISE NOTICE 'A table was created: %', tg_tag;
END;
$$ LANGUAGE plpgsql;
CREATE EVENT TRIGGER trg_on_create_table
ON sql_drop  -- or use ddl_command_end for CREATE
WHEN TAG IN ('CREATE TABLE')
EXECUTE FUNCTION on_create_table();

CREATE EVENT TRIGGER trg_on_create_table
ON sql_drop  -- or use ddl_command_end for CREATE
WHEN TAG IN ('CREATE TABLE')
EXECUTE FUNCTION on_create_table();


CREATE OR REPLACE FUNCTION log_created_table()
RETURNS event_trigger AS $$
DECLARE
  obj RECORD;
  col RECORD;
BEGIN
  FOR obj IN SELECT * FROM pg_event_trigger_ddl_commands() LOOP
    IF obj.object_type = 'table' THEN
      FOR col IN
        SELECT column_name, data_type
        FROM information_schema.columns
        WHERE table_schema = obj.schema_name
          AND table_name = obj.object_name
      LOOP
        INSERT INTO table_creation_log(table_name, column_name, data_type)
        VALUES (obj.object_name, col.column_name, col.data_type);
      END LOOP;
    END IF;
  END LOOP;
END;
$$ LANGUAGE plpgsql;