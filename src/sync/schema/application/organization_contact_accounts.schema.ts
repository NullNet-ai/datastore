import { integer, sqliteTable, text } from 'drizzle-orm/sqlite-core';
import { organization_contacts } from './organization_contacts.schema';
import { organizations } from './organizations.schema';
import { sql } from 'drizzle-orm';

export const organization_contact_accounts = sqliteTable(
  `organization_contact_accounts`,
  {
    id: text().primaryKey(),
    categories: text({ mode: 'json' })
      .$type<string[]>()
      .default(sql`(json_array())`),
    organization_contact_id: text().references(() => organization_contacts.id),
    organization_id: text().references(() => organizations.id),
    email: text(),
    password: text(),
    tombstone: integer().default(0),
    status: text().default('Active'),
    created_date: text(),
    created_time: text(),
    updated_date: text(),
    updated_time: text(),
  },
);
