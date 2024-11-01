import { sqliteTable, text } from 'drizzle-orm/sqlite-core';
import { table as contacts } from './contacts';
import { sql } from 'drizzle-orm';
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
    contact_id: text().references(() => contacts.id),
    phone_number_raw: text(),
  },
  getDefaultIndices,
);
