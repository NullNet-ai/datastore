import { sqliteTable, text } from 'drizzle-orm/sqlite-core';
import * as path from 'path';
import { getDefaultIndices, system_fields } from '../system_fields';
const filename = path
  .basename(__filename)
  .replace('.ts', '')
  .replace('.js', '');
const indices = getDefaultIndices(filename);
export const table = sqliteTable(
  filename,
  {
    ...system_fields,
    type: text(),
    value: text(),
  },
  indices,
);
