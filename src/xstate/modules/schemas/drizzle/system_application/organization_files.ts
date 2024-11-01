import { sqliteTable, text } from 'drizzle-orm/sqlite-core';
import { table as organization_contacts } from './organization_contacts';
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
    organizaion_id: text().references(() => organizations.id),
    organization_contact_id: text().references(() => organization_contacts.id),
    url: text(),
    name: text(),
    mime_type: text(),
    size: text(),
    type: text(),
  },
  config,
);
