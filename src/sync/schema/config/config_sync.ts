import { sqliteTable, text } from 'drizzle-orm/sqlite-core';
import { getDefaultIndices, system_fields } from '../system_fields';
import * as path from 'path';
const filename = path.basename(__filename).replace('.js', '');
export const table = sqliteTable(
  filename,
  {
    ...system_fields,
    type: text(),
    value: text(),
  },
  getDefaultIndices,
);
