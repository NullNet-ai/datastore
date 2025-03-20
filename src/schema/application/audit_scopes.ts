import { pgTable, text, date, time, json } from 'drizzle-orm/pg-core';
import {
  getConfigDefaults,
  system_fields,
} from '@dna-platform/crdt-lww-postgres/build/schema/system';

const config = getConfigDefaults.byIndex('crawls');

export const table = pgTable(
  'crawls',
  {
    ...system_fields,
    website_id: text('website_id'),
    audit_id: text('audit_id'),
    id: text().unique(),
    name: text('name'),
  },
  config,
);
