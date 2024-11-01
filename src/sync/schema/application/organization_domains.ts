import { sqliteTable, text } from 'drizzle-orm/sqlite-core';
import { table as organizations } from './organizations';
import { sql } from 'drizzle-orm';
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
    categories: text({ mode: 'json' })
      .$type<string[]>()
      .default(sql`(json_array())`),
    domain_name: text().unique(),
    organization_id: text().references(() => organizations.id),
  },
  indices,
);
