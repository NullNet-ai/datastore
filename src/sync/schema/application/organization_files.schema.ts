import { integer, sqliteTable, text } from 'drizzle-orm/sqlite-core';
import { organization_contacts } from './organization_contacts.schema';
import { organizations } from './organizations.schema';
import { sql } from 'drizzle-orm';

export const organization_files = sqliteTable(`organization_files`, {
  id: text().primaryKey(),
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
  tombstone: integer().default(0),
  status: text().default('active'),
  created_date: text(),
  created_time: text(),
  updated_date: text(),
  updated_time: text(),
});
