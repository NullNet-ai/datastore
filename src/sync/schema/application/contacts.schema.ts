import { sql } from 'drizzle-orm';
import { integer, sqliteTable, text } from 'drizzle-orm/sqlite-core';

export const contacts = sqliteTable(`contacts`, {
  id: text().primaryKey(),
  categories: text({ mode: 'json' })
    .$type<string[]>()
    .default(sql`(json_array())`),
  first_name: text(),
  middle_name: text(),
  last_name: text(),
  date_of_birth: text(),
  tombstone: integer().default(0),
  status: text().default('Active'),
  created_date: text(),
  created_time: text(),
  updated_date: text(),
  updated_time: text(),
});
