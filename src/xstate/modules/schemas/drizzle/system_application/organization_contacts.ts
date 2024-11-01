import { sqliteTable, text } from 'drizzle-orm/sqlite-core';
import { table as contacts } from './contacts';
import { table as organizations } from './organizations';
import { sql } from 'drizzle-orm';
import * as path from 'path';
import { fileRegex, getConfigDefaults, system_fields } from '../system';
const filename = path.basename(__filename).replace(fileRegex, '');
const config = getConfigDefaults.byIndex(filename);
export const table = sqliteTable(
  filename,
  {
    ...system_fields,
    categories: text({ mode: 'json' })
      .$type<string[]>()
      .default(sql`(json_array())`),
    contact_id: text().references(() => contacts.id),
    organization_id: text().references(() => organizations.id),
  },
  config,
);
