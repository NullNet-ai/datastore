CREATE OR REPLACE FUNCTION notify_entity_fields_changes()
RETURNS trigger AS $$
BEGIN
    IF (TG_OP = 'INSERT') THEN
        PERFORM pg_notify('entity_fields_update', row_to_json(NEW)::text);
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;