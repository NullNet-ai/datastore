import { integer, sqliteTable, text } from 'drizzle-orm/sqlite-core';
import { contacts } from './contacts.schema';
import { sql } from 'drizzle-orm';

export const contact_phone_numbers = sqliteTable(`contact_phone_numbers`, {
  id: text().primaryKey(),
  categories: text({ mode: 'json' })
    .$type<string[]>()
    .default(sql`(json_array())`),
  contact_id: text().references(() => contacts.id),
  phone_number_raw: text(),
  tombstone: integer().default(0),
  status: text().default('Active'),
  created_date: text(),
  created_time: text(),
  updated_date: text(),
  updated_time: text(),
});
