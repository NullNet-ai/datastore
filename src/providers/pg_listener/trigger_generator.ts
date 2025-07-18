import { util } from 'zod';
import objectKeys = util.objectKeys;
export function generateTableTriggerSQL(
  tableName: string,
  tableSchema: any,
  triggerName: string,
  channelName: string = 'objects',
): string {
  const functionName = `${tableName}_change`;
  // Extract fields from the table schema columns
  delete tableSchema['enableRLS'];
  const fields = objectKeys(tableSchema);
  const safeFields = fields.filter(
    (field) =>
      !field.includes('.') && !field.includes(' ') && !field.includes('-'),
  );

  const fieldsJson = safeFields
    .map((field) => `'${field}', NEW."${field}"`)
    .join(',\n          ');

  return `CREATE OR REPLACE FUNCTION ${functionName}()
      RETURNS trigger AS $$
      DECLARE
        payload text;
        channel text := '${channelName}';
      BEGIN
        SELECT json_build_object(
          'type', 'layerchange',
          'layer', TG_TABLE_NAME,
          'change', TG_OP,
          ${fieldsJson}
        )::text
        INTO payload;

        PERFORM pg_notify(channel, payload);
        RETURN NEW;
      END;
      $$ LANGUAGE plpgsql;

-- Create trigger for ${tableName}
DROP TRIGGER IF EXISTS ${triggerName} ON "${tableName}";
CREATE TRIGGER ${triggerName}
AFTER INSERT ON "${tableName}"
FOR EACH ROW EXECUTE FUNCTION ${functionName}();`;
}
