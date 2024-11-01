import { sql } from 'drizzle-orm';
import { sqliteTable, text } from 'drizzle-orm/sqlite-core';
import { getDefaultIndices, system_fields } from '../system_fields';
import * as path from 'path';
const filename = path.basename(__filename).replace('.js', '');
export const table = sqliteTable(
  filename,
  {
    ...system_fields,
    categories: text({ mode: 'json' })
      .$type<string[]>()
      .default(sql`(json_array())`),
    first_name: text(),
    middle_name: text(),
    last_name: text(),
    date_of_birth: text(),
  },
  getDefaultIndices,
);
