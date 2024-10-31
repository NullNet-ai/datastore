import { integer, sqliteTable, text } from 'drizzle-orm/sqlite-core';
import { contacts } from './contacts.schema';
import { sql } from 'drizzle-orm';

export const contact_emails = sqliteTable(`contact_emails`, {
  id: text().primaryKey(),
  categories: text({ mode: 'json' })
    .$type<string[]>()
    .default(sql`(json_array())`),
  contact_id: text().references(() => contacts.id),
  email: text(),
  tombstone: integer().default(0),
  status: text().default('Active'),
  created_date: text(),
  created_time: text(),
  updated_date: text(),
  updated_time: text(),
});
