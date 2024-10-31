import { sql } from 'drizzle-orm';
import { integer, sqliteTable, text } from 'drizzle-orm/sqlite-core';

export const organizations = sqliteTable(`organizations`, {
  id: text().primaryKey(),
  categories: text({ mode: 'json' })
    .$type<string[]>()
    .default(sql`(json_array())`),
  parent_organization_id: text()
    .references(() => organizations.id)
    .default(sql`(null)`),
  name: text(),
  tombstone: integer().default(0),
  status: text().default('Active'),
  created_date: text(),
  created_time: text(),
  updated_date: text(),
  updated_time: text(),
});
