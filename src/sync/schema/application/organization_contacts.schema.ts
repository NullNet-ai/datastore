import { integer, sqliteTable, text } from 'drizzle-orm/sqlite-core';
import { contacts } from './contacts';
import { organizations } from './organizations.schema';
import { sql } from 'drizzle-orm';

export const organization_contacts = sqliteTable(`organization_contacts`, {
  id: text().primaryKey(),
  categories: text({ mode: 'json' })
    .$type<string[]>()
    .default(sql`(json_array())`),
  contact_id: text().references(() => contacts.id),
  organization_id: text().references(() => organizations.id),
  tombstone: integer().default(0),
  status: text().default('Active'),
  created_date: text(),
  created_time: text(),
  updated_date: text(),
  updated_time: text(),
});
