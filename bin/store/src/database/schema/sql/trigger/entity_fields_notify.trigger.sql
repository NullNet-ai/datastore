DROP TRIGGER IF EXISTS entity_fields_notify_trigger ON entity_fields;

-- 3. Create the trigger
CREATE TRIGGER entity_fields_notify_trigger
AFTER INSERT ON entity_fields
FOR EACH ROW
EXECUTE FUNCTION notify_entity_fields_changes();