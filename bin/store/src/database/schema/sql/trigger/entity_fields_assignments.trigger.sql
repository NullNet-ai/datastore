DROP TRIGGER IF EXISTS entity_fields_assignments_trigger ON entity_fields;
CREATE TRIGGER entity_fields_assignments_trigger
AFTER INSERT ON entity_fields
REFERENCING NEW TABLE AS new_entity_fields
FOR EACH STATEMENT
EXECUTE FUNCTION assignPermission();