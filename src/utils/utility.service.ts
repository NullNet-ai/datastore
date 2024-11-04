import * as schema from '../schema';
export class Utility {
  public static parsePluckedFields(
    table: string,
    pluck?: string,
  ): Record<string, any> | null {
    const table_schema = schema[table];

    if (pluck === '' || !pluck) {
      return null;
    }
    const _plucked_fields = pluck.split(',').reduce((acc, field) => {
      if (table_schema[field]) {
        return {
          ...acc,
          [field]: table_schema[field],
        };
      }
      return acc;
    }, {});

    if (Object.keys(_plucked_fields).length === 0) {
      return null;
    }
    return _plucked_fields;
  }
}
